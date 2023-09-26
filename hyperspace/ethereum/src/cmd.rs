use crate::{
	client::{ClientError, EthRpcClient},
	config::EthereumClientConfig,
	utils::{deploy_client, deploy_ibc, deploy_transfer_module, DeployYuiIbc},
};
use anyhow::{anyhow, bail};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
pub enum EthereumCmd {
	/// Deploy contracts
	#[command(subcommand)]
	Deploy(DeployCmd),
}

#[derive(Debug, Clone, Subcommand)]
pub enum DeployCmd {
	/// Deploy core contracts
	Core(DeployCoreCmd),
	/// Deploy ICS-20 Transfer module contracts
	TransferModule(DeployTransferModuleCmd),
	/// Deploy client contracts
	Client(DeployClientCmd),
}

#[derive(Debug, Clone, Args)]
pub struct DeployCoreCmd {
	#[clap(long)]
	pub yui_solidity_path: PathBuf,
}

impl DeployCoreCmd {
	pub async fn run(
		&self,
		mut config: EthereumClientConfig,
	) -> anyhow::Result<EthereumClientConfig> {
		let client = config.client().await?;
		let path = &self.yui_solidity_path;
		let yui_ibc = deploy_ibc::<EthRpcClient>(path, client).await?;
		config.diamond_address = Some(yui_ibc.diamond.address());
		config.diamond_facets = yui_ibc
			.deployed_facets
			.iter()
			.map(|x| (x.abi_name(), x.contract().address()))
			.collect();
		Ok(config)
	}
}

#[derive(Debug, Clone, Parser)]
pub struct DeployClientCmd {
	/// Path to yui-solidity source code
	#[clap(long)]
	pub yui_solidity_path: PathBuf,
	/// Client type (e.g. '07-tendermint')
	#[clap(long)]
	pub client_type: String,
	/// Name of the delegate update contract
	#[clap(long)]
	pub delegate_update_name: String,
	/// Name of the client contract
	#[clap(long)]
	pub client_name: String,
}

impl DeployClientCmd {
	pub async fn run(
		&self,
		mut config: EthereumClientConfig,
	) -> anyhow::Result<EthereumClientConfig> {
		let client = config.client().await?;
		let path = &self.yui_solidity_path;
		let diamond_addr = config.diamond_address.ok_or_else(|| {
			anyhow!("Diamond contract should be deployed first (use 'deploy core' subcommand)")
		})?;
		let facets = config.diamond_facets.clone();
		if facets.is_empty() {
			bail!("Diamond facets are empty. Make sure to deploy the core first ('deploy core')")
		};
		let yui_ibc = DeployYuiIbc::<_, EthRpcClient>::from_addresses(
			client.clone(),
			diamond_addr,
			None,
			None,
			facets,
		)?;
		let contract = deploy_client(
			path,
			yui_ibc.clone(),
			self.client_type.clone(),
			&self.delegate_update_name,
			&self.client_name,
			client,
		)
		.await?;
		config.tendermint_address = Some(contract.address());
		Ok(config)
	}
}

#[derive(Debug, Clone, Parser)]
pub struct DeployTransferModuleCmd {
	#[clap(long)]
	pub yui_solidity_path: PathBuf,
}

impl DeployTransferModuleCmd {
	pub async fn run(
		&self,
		mut config: EthereumClientConfig,
	) -> anyhow::Result<EthereumClientConfig> {
		let client = config.client().await?;
		let path = &self.yui_solidity_path;
		let diamond_addr = config.diamond_address.ok_or_else(|| {
			anyhow!("Diamond contract should be deployed first (use 'deploy core' subcommand)")
		})?;
		let diamond_addr = config.diamond_address.ok_or_else(|| {
			anyhow!("Diamond contract should be deployed first (use 'deploy core' subcommand)")
		})?;
		let facets = config.diamond_facets.clone();
		if facets.is_empty() {
			bail!("Diamond facets are empty. Make sure to deploy the core first ('deploy core')")
		};
		let yui_ibc = DeployYuiIbc::<_, EthRpcClient>::from_addresses(
			client.clone(),
			diamond_addr,
			None,
			None,
			facets,
		)?;

		let contract =
			deploy_transfer_module::<EthRpcClient>(path, yui_ibc, diamond_addr, client).await?;
		config.bank_address = Some(contract.address());
		Ok(config)
	}
}

impl EthereumCmd {
	pub async fn run(&self, config: EthereumClientConfig) -> anyhow::Result<EthereumClientConfig> {
		match self {
			EthereumCmd::Deploy(cmd) => match cmd {
				DeployCmd::Core(cmd) => cmd.run(config).await,
				DeployCmd::Client(cmd) => cmd.run(config).await,
				DeployCmd::TransferModule(cmd) => cmd.run(config).await,
			},
		}
	}
}
