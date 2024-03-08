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

use anyhow::anyhow;
use clap::Parser;
use codec::Encode;
use codegen::{codegen, fetch_metadata_ws};
use http::uri::{Authority, Uri};
use std::{path::PathBuf, str::FromStr};
use wasm_testbed::WasmTestBed;

#[derive(Debug, Parser)]
pub struct Cli {
	#[clap(long)]
	pub path: PathBuf,
	#[clap(long, env = "RELAY_HOST", default_value = "ws://127.0.0.1:9944")]
	pub relay_url: Uri,
	#[clap(long, env = "PARA_HOST", default_value = "ws://127.0.0.1:9988")]
	pub para_url: Uri,
	/// Path to parachain runtime wasm blob
	#[clap(long)]
	pub parachain_wasm: Option<PathBuf>,
	/// Path to relaychain runtime wasm
	#[clap(long)]
	pub relaychain_wasm: Option<PathBuf>,
}

fn format_uri(uri: Uri) -> anyhow::Result<Uri> {
	let default_port = |scheme| match scheme {
		"http" | "ws" => Some(80u16),
		"https" | "wss" => Some(443),
		_ => None,
	};

	let scheme = uri.scheme_str().unwrap_or("wss");
	let authority = uri.authority().ok_or_else(|| anyhow!("No authority in {uri}"))?;
	let port = match authority.port_u16() {
		None => default_port(scheme)
			.ok_or_else(|| anyhow!("No default port for scheme {} in {uri}", scheme))?,
		Some(p) => p,
	};
	let mut builder = Uri::builder().scheme(scheme).authority(Authority::from_str(&format!(
		"{}:{}",
		authority.host(),
		port
	))?);
	if let Some(path) = uri.path_and_query().cloned() {
		builder = builder.path_and_query(path);
	}
	Ok(builder.build()?)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	let mut cli = Cli::parse();

	if let Some((parachain_wasm, relaychain_wasm)) = cli.parachain_wasm.zip(cli.relaychain_wasm) {
		let runtimes = vec![(parachain_wasm, "parachain"), (relaychain_wasm, "relaychain")];
		for (wasm_file, runtime) in runtimes {
			let source = FromStr::from_str(wasm_file.as_path().to_str().unwrap()).unwrap();
			let testbed = WasmTestBed::new(&source).expect("wasm file should be loaded");
			let metadata = testbed.runtime_metadata_prefixed().encode();
			let code = codegen(&mut &metadata[..])?;
			let path = cli.path.join(format!("{runtime}.rs"));
			tokio::fs::write(&path, &code).await?;
		}
	} else {
		cli.relay_url = format_uri(cli.relay_url)?;
		cli.para_url = format_uri(cli.para_url)?;

		let runtimes = [(cli.relay_url, "relaychain"), (cli.para_url, "parachain")];

		for (url, runtime) in runtimes {
			let url_str = url.to_string();
			let metadata = fetch_metadata_ws(&url_str)
				.await
				.map_err(|e| anyhow!("Failed to fetch metadata from {}: {}", url_str, e))?;
			let code = codegen(&mut &metadata[..])?;
			let path = cli.path.join(format!("{runtime}.rs"));
			tokio::fs::write(&path, &code).await?;
		}
	}

	Ok(())
}
