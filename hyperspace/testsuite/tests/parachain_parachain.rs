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

use futures::StreamExt;
use hyperspace_core::{
	logging,
	substrate::{ComposableConfig, DaliConfig, DefaultConfig, PicassoConfig},
};
use hyperspace_parachain::{
	finality_protocol::FinalityProtocol, ParachainClient, ParachainClientConfig,
};
use hyperspace_primitives::{utils::create_clients, IbcProvider};
use hyperspace_testsuite::{
	client_synchronization_test, ibc_channel_close,
	ibc_messaging_packet_height_timeout_with_connection_delay,
	ibc_messaging_packet_timeout_on_channel_close,
	ibc_messaging_packet_timestamp_timeout_with_connection_delay,
	ibc_messaging_with_connection_delay, misbehaviour::ibc_messaging_submit_misbehaviour,
};

#[derive(Debug, Clone)]
pub struct Args {
	pub chain_a: String,
	pub chain_b: String,
	pub relay_chain: String,
	pub para_id_a: u32,
	pub para_id_b: u32,
	pub connection_prefix_a: String,
	pub connection_prefix_b: String,
}

impl Default for Args {
	fn default() -> Self {
		let relay = std::env::var("RELAY_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		let para = std::env::var("PARA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

		Args {
			chain_a: format!("ws://{para}:9988"),
			chain_b: format!("ws://{para}:9188"),
			relay_chain: format!("ws://{relay}:9944"),
			para_id_a: 2001,
			para_id_b: 2000,
			connection_prefix_a: "ibc/".to_string(),
			connection_prefix_b: "ibc/".to_string(),
		}
	}
}

impl Args {
	fn composable() -> Self {
		let relay = std::env::var("RELAY_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		let para = std::env::var("PARA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		Args {
			chain_a: format!("ws://{para}:8988"),
			chain_b: format!("ws://{para}:8188"),
			relay_chain: format!("ws://{relay}:8944"),
			para_id_a: 10009,
			para_id_b: 10009,
			connection_prefix_a: "ibc/".to_string(),
			connection_prefix_b: "ibc/".to_string(),
		}
	}

	fn dali() -> Self {
		let relay = std::env::var("RELAY_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		let para = std::env::var("PARA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

		Args {
			chain_a: format!("ws://{para}:8988"),
			chain_b: format!("ws://{para}:8188"),
			relay_chain: format!("ws://{relay}:8944"),
			para_id_a: 2000,
			para_id_b: 2001,
			connection_prefix_a: "ibc/".to_string(),
			connection_prefix_b: "ibc/".to_string(),
		}
	}

	fn picasso() -> Self {
		let relay = std::env::var("RELAY_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		let para = std::env::var("PARA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

		Args {
			chain_a: format!("ws://{para}:7988"),
			chain_b: format!("ws://{para}:7188"),
			relay_chain: format!("ws://{relay}:9944"),
			para_id_a: 2087,
			para_id_b: 2001,
			connection_prefix_a: "ibc/".to_string(),
			connection_prefix_b: "ibc/".to_string(),
		}
	}
}

async fn setup_clients() -> (ParachainClient<DefaultConfig>, ParachainClient<PicassoConfig>) {
	log::info!(target: "hyperspace", "=========================== Starting Test ===========================");
	let args = Args::default();
	let args_composable = Args::picasso();

	// Create client configurations
	let config_a = ParachainClientConfig {
		name: format!("A"),
		para_id: args.para_id_a,
		parachain_rpc_url: args.chain_a,
		relay_chain_rpc_url: args.relay_chain.clone(),
		client_id: None,
		connection_id: None,
		commitment_prefix: args.connection_prefix_b.as_bytes().to_vec().into(),
		ss58_version: 42,
		channel_whitelist: vec![],
		finality_protocol: FinalityProtocol::Grandpa,
		private_key: "//Alice".to_string(),
		key_type: "sr25519".to_string(),
	};
	let config_b = ParachainClientConfig {
		name: format!("B"),
		para_id: args_composable.para_id_a,
		parachain_rpc_url: args_composable.chain_a,
		relay_chain_rpc_url: args_composable.relay_chain,
		client_id: None,
		connection_id: None,
		commitment_prefix: args_composable.connection_prefix_b.as_bytes().to_vec().into(),
		private_key: "//Alice".to_string(),
		ss58_version: 42,
		channel_whitelist: vec![],
		finality_protocol: FinalityProtocol::Grandpa,
		key_type: "sr25519".to_string(),
	};

	let mut chain_a = ParachainClient::<DefaultConfig>::new(config_a).await.unwrap();
	let mut chain_b = ParachainClient::<PicassoConfig>::new(config_b).await.unwrap();

	// Wait until for parachains to start producing blocks
	log::info!(target: "hyperspace", "Waiting for  block production from parachains");
	let session_length = chain_a.grandpa_prover().session_length().await.unwrap();
	let _ = chain_a
		.relay_client
		.rpc()
		.subscribe_finalized_block_headers()
		.await
		.unwrap()
		.filter_map(|result| futures::future::ready(result.ok()))
		.skip_while(|h| futures::future::ready(h.number < (session_length * 2) + 10))
		.take(1)
		.collect::<Vec<_>>()
		.await;
	log::info!(target: "hyperspace", "Parachains have started block production");

	let clients_on_a = chain_a.query_clients().await.unwrap();
	let clients_on_b = chain_b.query_clients().await.unwrap();

	if !clients_on_a.is_empty() && !clients_on_b.is_empty() {
		chain_a.set_client_id(clients_on_b[0].clone());
		chain_b.set_client_id(clients_on_b[0].clone());
		return (chain_a, chain_b)
	}

	let (client_a, client_b) = create_clients(&chain_a, &chain_b).await.unwrap();
	chain_a.set_client_id(client_a);
	chain_b.set_client_id(client_b);
	(chain_a, chain_b)
}

#[tokio::test]
async fn parachain_to_parachain_ibc_messaging_full_integration_test() {
	logging::setup_logging();
	let (mut chain_a, mut chain_b) = setup_clients().await;
	// Run tests sequentially

	let asset_id = 1u128;

	// no timeouts + connection delay
	ibc_messaging_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id.into(),
		asset_id.into(),
	)
	.await;

	// timeouts + connection delay
	ibc_messaging_packet_height_timeout_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id.into(),
	)
	.await;
	ibc_messaging_packet_timestamp_timeout_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id.into(),
	)
	.await;

	// channel closing semantics
	ibc_messaging_packet_timeout_on_channel_close(&mut chain_a, &mut chain_b, asset_id.into())
		.await;
	ibc_channel_close(&mut chain_a, &mut chain_b).await;

	// Test sync abilities, run this before misbehaviour test
	client_synchronization_test(&chain_a, &chain_b).await;

	// misbehaviour
	ibc_messaging_submit_misbehaviour(&mut chain_a, &mut chain_b).await;
}
