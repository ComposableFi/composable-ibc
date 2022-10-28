use anyhow::Result;
use clap::Parser;
use hyperspace::{
	command::{Cli, Subcommand},
	logging,
};
use subxt::tx::SubstrateExtrinsicParams;

#[derive(Debug, Clone)]
pub enum DefaultConfig {}

impl subxt::Config for DefaultConfig {
	type Index = u32;
	type BlockNumber = u32;
	type Hash = sp_core::H256;
	type Hashing = sp_runtime::traits::BlakeTwo256;
	type AccountId = sp_runtime::AccountId32;
	type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
	type Header = sp_runtime::generic::Header<Self::BlockNumber, sp_runtime::traits::BlakeTwo256>;
	type Signature = sp_runtime::MultiSignature;
	type Extrinsic = sp_runtime::OpaqueExtrinsic;
	type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
}

#[tokio::main]
async fn main() -> Result<()> {
	logging::setup_logging();
	let cli = Cli::parse();

	match &cli.subcommand {
		Subcommand::Relay(cmd) => cmd.run::<DefaultConfig>().await,
		Subcommand::CreateClients(cmd) => cmd.create_clients::<DefaultConfig>().await,
		Subcommand::CreateConnection(cmd) => cmd.create_connection::<DefaultConfig>().await,
		Subcommand::CreateChannel(cmd) => cmd.create_channel::<DefaultConfig>().await,
	}
}
