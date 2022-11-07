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
