use clap::Parser;
use codegen::{codegen, fetch_metadata_ws};
use std::{fs, path::PathBuf};

#[derive(Debug, Parser)]
pub struct Cli {
	#[clap(long)]
	pub path: PathBuf,
	#[clap(long, default_value = "9944")]
	pub relay_port: String,
	#[clap(long, default_value = "9188")]
	pub para_port: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	let cli = Cli::parse();

	let runtimes = [
		(format!("ws://127.0.0.1:{}", cli.relay_port), "rococo"),
		(format!("ws://127.0.0.1:{}", cli.para_port), "parachain"),
	];

	for (url, runtime) in runtimes {
		let metadata = fetch_metadata_ws(&url).await?;
		let code = codegen(&mut &metadata[..])?;
		let path = cli.path.join(format!("{}.rs", runtime));
		fs::write(&path, &code)?;
	}

	Ok(())
}
