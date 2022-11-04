use clap::Parser;
use codegen::{codegen, fetch_metadata_ws};
use std::{fs, path::PathBuf};

#[derive(Debug, Parser)]
pub struct Cli {
	#[clap(long)]
	pub path: PathBuf,
	#[clap(long, env = "RELAY_HOST", default_value = "127.0.0.1")]
	pub relay_host: String,
	#[clap(long, default_value = "9944")]
	pub relay_port: String,
	#[clap(long, env = "PARA_HOST", default_value = "127.0.0.1")]
	pub para_host: String,
	#[clap(long, default_value = "9188")]
	pub para_port: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	let cli = Cli::parse();

	let runtimes = [
		(format!("ws://{}:{}", cli.relay_host, cli.relay_port), "rococo"),
		(format!("ws://{}:{}", cli.relay_host, cli.para_port), "parachain"),
	];

	for (url, runtime) in runtimes {
		let metadata = fetch_metadata_ws(&url).await?;
		let code = codegen(&mut &metadata[..])?;
		let path = cli.path.join(format!("{}.rs", runtime));
		fs::write(&path, &code)?;
	}

	Ok(())
}
