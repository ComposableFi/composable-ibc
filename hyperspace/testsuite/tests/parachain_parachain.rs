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
use hyperspace_core::{logging, substrate::DefaultConfig};
use hyperspace_parachain::{
	finality_protocol::FinalityProtocol, ParachainClient, ParachainClientConfig,
};
use hyperspace_primitives::{utils::create_clients, IbcProvider, TestProvider};
use hyperspace_testsuite::{
	client_synchronization_test, ibc_channel_close,
	ibc_messaging_packet_height_timeout_with_connection_delay,
	ibc_messaging_packet_timeout_on_channel_close,
	ibc_messaging_packet_timestamp_timeout_with_connection_delay,
	ibc_messaging_with_connection_delay, misbehaviour::ibc_messaging_submit_misbehaviour,
};
use std::time::Duration;

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

async fn setup_clients() -> (ParachainClient<DefaultConfig>, ParachainClient<DefaultConfig>) {
	log::info!(target: "hyperspace", "=========================== Starting Test ===========================");
	let args = Args::default();

	// Create client configurations
	let config_a = ParachainClientConfig {
		name: format!("9988"),
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
		wasm_code_hash: None,
	};
	let config_b = ParachainClientConfig {
		name: format!("9188"),
		para_id: args.para_id_b,
		parachain_rpc_url: args.chain_b,
		relay_chain_rpc_url: args.relay_chain,
		client_id: None,
		connection_id: None,
		commitment_prefix: args.connection_prefix_b.as_bytes().to_vec().into(),
		private_key: "//Alice".to_string(),
		ss58_version: 42,
		channel_whitelist: vec![],
		finality_protocol: FinalityProtocol::Grandpa,
		key_type: "sr25519".to_string(),
		wasm_code_hash: None,
	};

	let mut chain_a = ParachainClient::<DefaultConfig>::new(config_a).await.unwrap();
	let mut chain_b = ParachainClient::<DefaultConfig>::new(config_b).await.unwrap();

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

	// We need to make difference between the chains' counters to ensure that
	// proper values are used for source/sink client, connection, channel (etc.) ids.
	chain_a.increase_counters().await.unwrap();

	let clients_on_a = chain_a.query_clients().await.unwrap();
	let clients_on_b = chain_b.query_clients().await.unwrap();

	let (client_a, client_b) = if !clients_on_a.is_empty() && !clients_on_b.is_empty() {
		(clients_on_b[0].clone(), clients_on_b[0].clone())
	} else {
		create_clients(&mut chain_a, &mut chain_b).await.unwrap()
	};

	log::info!(target: "hyperspace_parachain", "Client IDs: {client_a}, {client_b}");
	chain_a.set_client_id(client_a);
	chain_b.set_client_id(client_b);
	(chain_a, chain_b)
}

#[tokio::test]
async fn parachain_to_parachain_ibc_messaging_full_integration_test() {
	logging::setup_logging();
	use hyperspace_testsuite::setup_connection_and_channel;
	use ibc::core::ics24_host::identifier::PortId;
	let (mut chain_a, mut chain_b) = setup_clients().await;
	let mut chain_aa = chain_a.clone();
	let mut chain_bb = chain_b.clone();
	//set up connection only once!!!
	let (handle, channel_a, channel_b, connection_id_a, connection_id_b) =
		setup_connection_and_channel(&mut chain_a, &mut chain_b, Duration::from_secs(60 * 2)).await;
	handle.abort();

	// Set connections and channel whitelist
	chain_a.set_connection_id(connection_id_a);
	chain_b.set_connection_id(connection_id_b);

	chain_a.set_channel_whitelist(vec![(channel_a, PortId::transfer())].into_iter().collect());
	chain_b.set_channel_whitelist(vec![(channel_b, PortId::transfer())].into_iter().collect());

	let asset_id = 1;

	let mut join_set = tokio::task::JoinSet::new();

	// no timeouts + connection delay
	let mut c1 = chain_a.clone();
	let mut c2 = chain_b.clone();
	join_set.spawn(async move {
		ibc_messaging_with_connection_delay(
			&mut c1, &mut c2, asset_id, asset_id, channel_a, channel_b,
		)
		.await;
		log::info!(target: "hyperspace", "🚀🚀 finished connection delay");
	});

	// timeouts + connection delay
	let mut c1 = chain_a.clone();
	let mut c2 = chain_b.clone();
	join_set.spawn(async move {
		ibc_messaging_packet_height_timeout_with_connection_delay(
			&mut c1, &mut c2, asset_id, channel_a, channel_b,
		)
		.await;
		log::info!(target: "hyperspace", "🚀🚀 finished packet height timeout");

		ibc_messaging_packet_timestamp_timeout_with_connection_delay(
			&mut c1, &mut c2, asset_id, channel_a, channel_b,
		)
		.await;
		log::info!(target: "hyperspace", "🚀🚀 finished packet timestamp timeout");
	});

	log::info!(target: "hyperspace", "🚀🚀 Waiting for connection delay and timeout checks to finish");
	while let Some(res) = join_set.join_next().await {
		res.unwrap();
	}

	// channel closing semantics
	let mut join_set = tokio::task::JoinSet::new();
	let mut c1 = chain_a.clone();
	let mut c2 = chain_b.clone();
	join_set.spawn(async move {
		ibc_messaging_packet_timeout_on_channel_close(&mut c1, &mut c2, asset_id, channel_a).await;
		log::info!(target: "hyperspace", "🚀🚀 finished packet timeout on channel close");
	});
	join_set.spawn(async move {
		ibc_channel_close(&mut chain_aa, &mut chain_bb).await;
		log::info!(target: "hyperspace", "🚀🚀 finished channel close");
	});

	log::info!(target: "hyperspace", "🚀🚀 Waiting for channel close semantics to finish");
	while let Some(res) = join_set.join_next().await {
		res.unwrap();
	}

	// Test sync abilities, run this before misbehaviour test
	client_synchronization_test(&mut chain_a, &mut chain_b).await;

	// misbehaviour
	ibc_messaging_submit_misbehaviour(&mut chain_a, &mut chain_b).await;
	log::info!(target: "hyperspace", "🚀🚀 Waiting for misbehaviour to be submitted");
}
