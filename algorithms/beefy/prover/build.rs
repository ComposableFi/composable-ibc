// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use once_cell::sync::Lazy;

static RELAY_URL: Lazy<String> = Lazy::new(|| {
	let ip = std::env::var("RELAY_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
	format!("ws://{}:9944", ip)
});

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	if cfg!(feature = "build-metadata-from-ws") {
		subxt_codegen::build_script(&RELAY_URL, "runtime").await?;
	}
	Ok(())
}
