use anyhow::Result;
use clap::Parser;
use hyperspace::{
	command::{Cli, Subcommand},
	logging,
};

#[tokio::main]
async fn main() -> Result<()> {
	logging::setup_logging();
	let cli = Cli::parse();

	match &cli.subcommand {
		Subcommand::Relay(cmd) => cmd.run().await,
		Subcommand::CreateClients(cmd) => cmd.create_clients().await,
		Subcommand::CreateConnection(cmd) => cmd.create_connection().await,
		Subcommand::CreateChannel(cmd) => cmd.create_channel().await,
	}
}
