use clap::Parser;
use codegen::{codegen, fetch_metadata_ws};
use std::{fs, path::PathBuf};

#[derive(Debug, Parser)]
pub struct Cli {
	#[clap(long)]
	pub path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	let cli = Cli::parse();

	let runtimes = [("ws://127.0.0.1:9944", "rococo"), ("ws://127.0.0.1:9188", "parachain")];

	for (url, runtime) in runtimes {
		let metadata = fetch_metadata_ws(url).await?;
		let code = codegen(&mut &metadata[..])?;
		let path = cli.path.join(format!("{}.rs", runtime));
		fs::write(&path, &code)?;
	}

	Ok(())
}
