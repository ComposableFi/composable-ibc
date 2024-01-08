use crate::{
	client::{ClientError, EthRpcClient},
	config::EthereumClientConfig,
	utils::{deploy_client, deploy_ibc, deploy_transfer_module, DeployYuiIbc},
};
use anyhow::{anyhow, bail};
use clap::{Args, Parser, Subcommand};
use ethers_providers::Provider;
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
		config.ibc_core_diamond_address = Some(yui_ibc.ibc_core_diamond.address());
		config.ibc_core_facets = yui_ibc
			.ibc_core_facets
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
		let diamond_addr = config.ibc_core_diamond_address.ok_or_else(|| {
			anyhow!("Diamond contract should be deployed first (use 'deploy core' subcommand)")
		})?;
		let facets = config.ibc_core_facets.clone();
		if facets.is_empty() {
			bail!("Diamond facets are empty. Make sure to deploy the core first ('deploy core')")
		};
		let yui_ibc = DeployYuiIbc::<_, EthRpcClient>::from_addresses(
			client.clone(),
			diamond_addr,
			facets,
			None,
			vec![],
			None,
			vec![],
			None,
			vec![],
			None,
		)
		.await?;
		let (contract, facets) = deploy_client(
			path,
			yui_ibc.clone(),
			self.client_type.clone(),
			&self.client_name,
			client,
		)
		.await?;
		config.tendermint_diamond_address = Some(contract.address());
		config.tendermint_facets = facets
			.into_iter()
			.map(|contract| (contract.abi_name(), contract.contract().address()))
			.collect();
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
		client.address();
		let path = &self.yui_solidity_path;
		let diamond_addr = config.ibc_core_diamond_address.ok_or_else(|| {
			anyhow!("Diamond contract should be deployed first (use 'deploy core' subcommand)")
		})?;
		let facets = config.ibc_core_facets.clone();
		if facets.is_empty() {
			bail!("Diamond facets are empty. Make sure to deploy the core first ('deploy core')")
		};
		let yui_ibc = DeployYuiIbc::<_, EthRpcClient>::from_addresses(
			client.clone(),
			diamond_addr,
			facets,
			None,
			vec![],
			None,
			vec![],
			None,
			vec![],
			None,
		)
		.await?;

		let (ibc_transfer_bank_diamond, transfer_bank_facets, bank_diamond, bank_facets) =
			deploy_transfer_module::<Provider<_>, _>(path, yui_ibc, diamond_addr, client).await?;
		config.ibc_transfer_diamond_address = Some(ibc_transfer_bank_diamond.address());
		config.ibc_transfer_facets = transfer_bank_facets
			.into_iter()
			.map(|f| (f.abi_name(), f.contract().address()))
			.collect();
		config.bank_diamond_address = Some(bank_diamond.address());
		config.bank_facets = bank_facets
			.into_iter()
			.map(|f| (f.abi_name(), f.contract().address()))
			.collect();
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
