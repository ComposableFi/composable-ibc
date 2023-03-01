// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
	chain::{AnyConfig, Config, CoreConfig},
	fish, relay, Mode,
};
use anyhow::{anyhow, Result};
use clap::Parser;
use ibc::core::{ics04_channel::channel::Order, ics24_host::identifier::PortId};
use metrics::{data::Metrics, handler::MetricsHandler, init_prometheus};
use primitives::{
	utils::{create_channel, create_clients, create_connection},
	Chain, IbcProvider,
};
use prometheus::Registry;
use std::{path::PathBuf, str::FromStr, time::Duration};

#[derive(Debug, Parser)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Subcommand,
}

/// Possible subcommands of the main binary.
#[derive(Debug, Parser)]
pub enum Subcommand {
	#[clap(name = "relay", about = "Start relaying messages between two chains")]
	Relay(Cmd),
	#[clap(name = "upload-wasm", about = "Upload a WASM blob to the chain")]
	UploadWasm(UploadWasmCmd),
	#[clap(
		name = "fish",
		about = "Start the relayer in fishing mode (catching malicious transactions)"
	)]
	Fish(Cmd),
	#[clap(name = "create-clients", about = "Creates light clients on both chains")]
	CreateClients(Cmd),
	#[clap(name = "create-connection", about = "Creates a connection between both chains")]
	CreateConnection(Cmd),
	#[clap(name = "create-channel", about = "Creates a channel on the specified port")]
	CreateChannel(Cmd),
}

#[derive(Debug, Clone, Parser)]
pub struct Cmd {
	/// Relayer chain A config path.
	#[clap(long)]
	pub config_a: String,
	/// Relayer chain B config path.
	#[clap(long)]
	config_b: String,
	/// Relayer core config path.
	#[clap(long)]
	config_core: String,
	/// Port id for channel creation
	#[clap(long)]
	port_id: Option<String>,
	/// Connection delay period in seconds
	#[clap(long)]
	delay_period: Option<u32>,
	/// Channel order
	#[clap(long)]
	order: Option<String>,
	/// Channel version
	#[clap(long)]
	version: Option<String>,
	/// New config path for A to avoid overriding existing configuration
	#[clap(long)]
	pub out_config_a: Option<String>,
	/// New config path for B to avoid overriding existing configuration
	#[clap(long)]
	pub out_config_b: Option<String>,
}

#[derive(Debug, Clone, Parser)]
pub struct UploadWasmCmd {
	/// Relayer chain config path.
	#[clap(long)]
	config: String,
	/// New config path to avoid overriding existing configuration.
	#[clap(long)]
	pub out_config: Option<String>,
	/// Path to the wasm file.
	#[clap(long)]
	wasm_path: PathBuf,
}

impl UploadWasmCmd {
	pub async fn run(&self) -> Result<AnyConfig> {
		use tokio::fs::read_to_string;
		let path: PathBuf = self.config.parse()?;
		let file_content = read_to_string(path).await?;
		let mut config: AnyConfig = toml::from_str(&file_content)?;
		let client = config.clone().into_client().await?;
		let wasm = tokio::fs::read(&self.wasm_path).await?;
		let code_id = client.upload_wasm(wasm).await?;
		let code_id_str = hex::encode(&code_id);
		println!("{}", code_id_str);
		config.set_wasm_code_id(code_id_str);
		Ok(config)
	}

	pub async fn save_config(&self, new_config: &AnyConfig) -> Result<()> {
		let path = self.out_config.as_ref().cloned().unwrap_or_else(|| self.config.clone());
		write_config(path, &new_config).await
	}
}

impl Cmd {
	async fn parse_config(&self) -> Result<Config> {
		use tokio::fs::read_to_string;
		let path_a: PathBuf = self.config_a.parse()?;
		let path_b: PathBuf = self.config_b.parse()?;
		let path_core: PathBuf = self.config_core.parse()?;
		let file_content = read_to_string(path_a).await?;
		let config_a: AnyConfig = toml::from_str(&file_content)?;
		let file_content = read_to_string(path_b).await?;
		let config_b: AnyConfig = toml::from_str(&file_content)?;
		let file_content = read_to_string(path_core).await?;
		let config_core: CoreConfig = toml::from_str(&file_content)?;

		Ok(Config { chain_a: config_a, chain_b: config_b, core: config_core })
	}

	// todo: IntoClient, since clients are generic, users must configure clients themselves.
	/// Run the command
	pub async fn run(&self) -> Result<()> {
		let config = self.parse_config().await?;
		let chain_a = config.chain_a.into_client().await?;
		let chain_b = config.chain_b.into_client().await?;

		let registry =
			Registry::new_custom(None, None).expect("this can only fail if the prefix is empty");
		let metrics_a = Metrics::register(chain_a.name(), &registry)?;
		let metrics_b = Metrics::register(chain_b.name(), &registry)?;
		let mut metrics_handler_a = MetricsHandler::new(registry.clone(), metrics_a);
		let mut metrics_handler_b = MetricsHandler::new(registry.clone(), metrics_b);
		metrics_handler_a.link_with_counterparty(&mut metrics_handler_b);

		if let Some(addr) = config.core.prometheus_endpoint.map(|s| s.parse().ok()).flatten() {
			tokio::spawn(init_prometheus(addr, registry.clone()));
		}

		relay(chain_a, chain_b, Some(metrics_handler_a), Some(metrics_handler_b), None).await
	}

	/// Run fisherman
	pub async fn fish(&self) -> Result<()> {
		let config = self.parse_config().await?;
		let chain_a = config.chain_a.into_client().await?;
		let chain_b = config.chain_b.into_client().await?;

		fish(chain_a, chain_b).await
	}

	pub async fn create_clients(&self) -> Result<Config> {
		let mut config = self.parse_config().await?;
		let chain_a = config.chain_a.clone().into_client().await?;
		let chain_b = config.chain_b.clone().into_client().await?;

		let (client_id_a_on_b, client_id_b_on_a) = create_clients(&chain_a, &chain_b).await?;
		log::info!(
			"ClientId for Chain {} on Chain {}: {}",
			chain_b.name(),
			chain_a.name(),
			client_id_b_on_a
		);
		log::info!(
			"ClientId for Chain {} on Chain {}: {}",
			chain_a.name(),
			chain_b.name(),
			client_id_a_on_b
		);
		config.chain_a.set_client_id(client_id_a_on_b);
		config.chain_b.set_client_id(client_id_b_on_a);

		Ok(config)
	}

	pub async fn create_connection(&self) -> Result<Config> {
		let delay = self
			.delay_period
			.expect("delay_period should be provided when creating a connection");
		let delay = Duration::from_secs(delay.into());
		let mut config = self.parse_config().await?;
		let chain_a = config.chain_a.clone().into_client().await?;
		let chain_b = config.chain_b.clone().into_client().await?;

		let chain_a_clone = chain_a.clone();
		let chain_b_clone = chain_b.clone();
		let handle = tokio::task::spawn(async move {
			relay(chain_a_clone, chain_b_clone, None, None, Some(Mode::Light))
				.await
				.unwrap();
		});

		let (connection_id_b, connection_id_a) =
			create_connection(&chain_a, &chain_b, delay).await?;
		log::info!("ConnectionId on Chain {}: {}", chain_b.name(), connection_id_b);
		log::info!("ConnectionId on Chain {}: {}", chain_a.name(), connection_id_a);
		handle.abort();

		config.chain_a.set_connection_id(connection_id_a);
		config.chain_b.set_connection_id(connection_id_b);

		Ok(config)
	}

	pub async fn create_channel(&self) -> Result<Config> {
		let port_id = PortId::from_str(
			self.port_id
				.as_ref()
				.expect("port_id must be specified when creating a channel")
				.as_str(),
		)
		.expect("Port id was invalid");
		let version = self
			.version
			.as_ref()
			.expect("version must be specified when creating a channel")
			.clone();
		let order = self.order.as_ref().expect("order must be specified when creating a channel, expected one of 'ordered' or 'unordered'").as_str();
		let mut config = self.parse_config().await?;
		let chain_a = config.chain_a.clone().into_client().await?;
		let chain_b = config.chain_b.clone().into_client().await?;

		let chain_a_clone = chain_a.clone();
		let chain_b_clone = chain_b.clone();
		let handle = tokio::task::spawn(async move {
			relay(chain_a_clone, chain_b_clone, None, None, Some(Mode::Light))
				.await
				.unwrap();
		});

		let order = Order::from_str(order).expect("Expected one of 'ordered' or 'unordered'");
		let (channel_id_a, channel_id_b) = create_channel(
			&chain_a,
			&chain_b,
			chain_a.connection_id(),
			port_id.clone(),
			version,
			order,
		)
		.await?;
		log::info!("ChannelId on Chain {}: {}", chain_a.name(), channel_id_a);
		log::info!("ChannelId on Chain {}: {}", chain_b.name(), channel_id_b);
		handle.abort();

		config.chain_a.set_channel_whitelist(channel_id_a, port_id.clone());
		config.chain_b.set_channel_whitelist(channel_id_b, port_id);

		Ok(config)
	}

	pub async fn save_config(&self, new_config: &Config) -> Result<()> {
		let path_a = self.out_config_a.as_ref().cloned().unwrap_or_else(|| self.config_a.clone());
		let path_b = self.out_config_b.as_ref().cloned().unwrap_or_else(|| self.config_b.clone());
		write_config(path_a, &new_config.chain_a).await?;
		write_config(path_b, &new_config.chain_b).await
	}
}

async fn write_config(path: String, config: &AnyConfig) -> Result<()> {
	tokio::fs::write(path.parse::<PathBuf>()?, toml::to_string(config)?)
		.await
		.map_err(|e| anyhow!(e))
}
