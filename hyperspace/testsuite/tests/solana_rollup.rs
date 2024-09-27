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

use anchor_lang::prelude::*;
use core::time::Duration;
use futures::StreamExt;
use hyperspace_core::{
	chain::{AnyAssetId, AnyChain, AnyConfig},
	logging,
	substrate::DefaultConfig,
};
use hyperspace_rollup::client::{RollupClient, RollupClientConfig};
use hyperspace_primitives::{utils::create_clients, CommonClientConfig, IbcProvider, KeyProvider};
use hyperspace_solana::{client::SolanaClient, SolanaClientConfig};
use hyperspace_testsuite::{
	ibc_channel_close, ibc_messaging_packet_height_timeout_with_connection_delay,
	ibc_messaging_packet_timeout_on_channel_close,
	ibc_messaging_packet_timestamp_timeout_with_connection_delay,
	ibc_messaging_with_connection_delay, misbehaviour::ibc_messaging_submit_misbehaviour,
	setup_connection_and_channel,
};
use ibc::core::{
	ics02_client::msgs::update_client::MsgUpdateAnyClient,
	ics24_host::identifier::{ClientId, PortId},
};
use ibc_proto::ibc::core::client::v1::MsgUpdateClient;
use sp_core::hashing::sha2_256;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Args {
	pub chain_a: String,
	pub chain_b: String,
	pub connection_prefix_a: String,
	pub connection_prefix_b: String,
	pub solana_ws: String,
	pub rollup_ws: String,
	pub rollup_trie_db_rpc: String,
}

impl Default for Args {
	fn default() -> Self {
		let solana = std::env::var("SOLANA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		let rollup = std::env::var("ROLLUP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

		Args {
			// chain_a: format!("https://devnet.helius-rpc.com/?api-key=bc5c0cfc-46df-4781-978f-af6ca7a202c2"),
			chain_a: format!("http://{solana}:9000"),
			chain_b: format!("http://{rollup}:8899"),
			connection_prefix_a: "ibc".to_string(),
			connection_prefix_b: "ibc".to_string(),
			solana_ws: format!("ws://{solana}:8900"),
			// solana_ws:
			// format!("wss://devnet.helius-rpc.com/?api-key=bc5c0cfc-46df-4781-978f-af6ca7a202c2"),
			rollup_ws: format!("ws://{rollup}:9001"),
			rollup_trie_db_rpc: format!("http://{rollup}:42069")
		}
	}
}

async fn setup_clients() -> (AnyChain, AnyChain) {
	log::info!(target: "hyperspace", "=========================== Starting Test ===========================");
	let args = Args::default();

	// Create client configurations
	let config_a = SolanaClientConfig {
		name: "solana".to_string(),
		client_id: None,
		connection_id: None,
		commitment_prefix: args.connection_prefix_a.clone().as_bytes().to_vec(),
		wasm_checksum: None,
		rpc_url: args.chain_a.clone().parse().unwrap(),
		ws_url: args.solana_ws.clone().parse().unwrap(),
		chain_id: "solana-1".to_string(),
		account_prefix: args.connection_prefix_a.clone(),
		fee_denom: "stake".to_string(),
		fee_amount: "4000".to_string(),
		gas_limit: 100000000,
		store_prefix: args.connection_prefix_a.clone(),
		max_tx_size: 320000,
		common_state_config: CommonClientConfig {
			skip_optional_client_updates: true,
			max_packets_to_process: 1,
			client_update_interval_sec: 10,
			handshake_completed: false,
		},
		channel_whitelist: vec![],
		commitment_level: "confirmed".to_string(),
		private_key: vec![
			48, 123, 8, 80, 248, 0, 217, 142, 124, 193, 95, 24, 168, 139, 214, 136, 147, 210, 168,
			135, 26, 36, 162, 89, 150, 185, 99, 191, 247, 135, 78, 111, 12, 8, 4, 81, 129, 165,
			153, 230, 192, 225, 51, 119, 216, 14, 69, 225, 73, 7, 204, 144, 39, 213, 91, 255, 136,
			38, 95, 131, 197, 4, 101, 186,
		],
		solana_ibc_program_id: "2HLLVco5HvwWriNbUhmVwA2pCetRkpgrqwnjcsZdyTKT".to_string(),
		write_program_id: "FufGpHqMQgGVjtMH9AV8YMrJYq8zaK6USRsJkZP4yDjo".to_string(),
		signature_verifier_program_id: "C6r1VEbn3mSpecgrZ7NdBvWUtYVJWrDPv4uU9Xs956gc".to_string(),
		trie_db_path: "../../../solana-ibc-indexer/indexer.db3".to_string(),
		transaction_sender: "RPC".to_string(),
	};

	let mut config_b = RollupClientConfig {
		name: "mantis".to_string(),
		client_id: None,
		connection_id: None,
		commitment_prefix: args.connection_prefix_b.as_bytes().to_vec(),
		wasm_checksum: None,
		rpc_url: args.chain_b.clone().parse().unwrap(),
		ws_url: args.rollup_ws.clone().parse().unwrap(),
		chain_id: "mantis-1".to_string(),
		account_prefix: args.connection_prefix_b.clone(),
		fee_denom: "stake".to_string(),
		fee_amount: "4000".to_string(),
		gas_limit: 100000000,
		store_prefix: args.connection_prefix_b,
		max_tx_size: 320000,
		common_state_config: CommonClientConfig {
			skip_optional_client_updates: true,
			max_packets_to_process: 1,
			client_update_interval_sec: 10,
			handshake_completed: false,
		},
		channel_whitelist: vec![],
		commitment_level: "confirmed".to_string(),
		private_key: vec![
			48, 123, 8, 80, 248, 0, 217, 142, 124, 193, 95, 24, 168, 139, 214, 136, 147, 210, 168,
			135, 26, 36, 162, 89, 150, 185, 99, 191, 247, 135, 78, 111, 12, 8, 4, 81, 129, 165,
			153, 230, 192, 225, 51, 119, 216, 14, 69, 225, 73, 7, 204, 144, 39, 213, 91, 255, 136,
			38, 95, 131, 197, 4, 101, 186,
		],
		solana_ibc_program_id: "2HLLVco5HvwWriNbUhmVwA2pCetRkpgrqwnjcsZdyTKT".to_string(),
		write_program_id: "FufGpHqMQgGVjtMH9AV8YMrJYq8zaK6USRsJkZP4yDjo".to_string(),
		signature_verifier_program_id: "C6r1VEbn3mSpecgrZ7NdBvWUtYVJWrDPv4uU9Xs956gc".to_string(),
		trie_db_path: "../../../solana-ibc-indexer/indexer.db3".to_string(),
		transaction_sender: "RPC".to_string(),
    trie_rpc_url: args.rollup_trie_db_rpc.clone().parse().unwrap(),
	};

	// println!("This is config b {:?}", config_b);

	let chain_a = SolanaClient::new(config_a.clone()).await.expect("Solana error");
	let chain_b = RollupClient::new(config_b.clone())
		.await
		.map_err(|e| println!("{:?}", e))
		.unwrap();

	println!("This is chain b prefix {:?}", chain_b.commitment_prefix.as_bytes());

	let mut chain_a_wrapped = AnyConfig::Solana(config_a).into_client().await.unwrap();
	let mut chain_b_wrapped = AnyConfig::Rollup(config_b).into_client().await.unwrap();

	let AnyChain::Solana(chain_a) = &mut chain_a_wrapped else { unreachable!() };

	let clients_on_a = chain_a_wrapped.query_clients().await.unwrap();
	let clients_on_b = chain_b_wrapped.query_clients().await.unwrap();

	// if !clients_on_a.is_empty() && !clients_on_b.is_empty() {
	// 	chain_a_wrapped.set_client_id(clients_on_b[0].clone());
	// 	chain_b_wrapped.set_client_id(clients_on_a[0].clone());
	// 	return (chain_a_wrapped, chain_b_wrapped)
	// }

	let (client_a, client_b) =
		create_clients(&mut chain_a_wrapped, &mut chain_b_wrapped).await.unwrap();
	chain_a_wrapped.set_client_id(client_a);
	chain_b_wrapped.set_client_id(client_b);
	// chain_b_wrapped.set_client_id(ClientId::new("cf-solana", 11).unwrap());
	// chain_a_wrapped.set_client_id(ClientId::new("cf-guest", 10).unwrap());
	(chain_a_wrapped, chain_b_wrapped)
}

// #[tokio::test]
#[tokio::test(flavor = "multi_thread", worker_threads = 12)]
// #[ignore]
async fn solana_to_rollup_ibc_messaging_full_integration_test() {
	use ibc::core::ics24_host::identifier::{ChannelId, ConnectionId};
	use std::str::FromStr;
	logging::setup_logging();

	let asset_id_a = AnyAssetId::Solana("33WVSef9zaw49KbNdPGTmACVRnAXzN3o1fsqbUrLp2mh".to_string());
	let asset_id_b = AnyAssetId::Rollup("33WVSef9zaw49KbNdPGTmACVRnAXzN3o1fsqbUrLp2mh".to_string());
	let (mut chain_a, mut chain_b) = setup_clients().await;
	let (handle, channel_a, channel_b, connection_id_a, connection_id_b) =
		setup_connection_and_channel(&mut chain_a, &mut chain_b, Duration::from_secs(10)).await;

	handle.abort();

	// let connection_id_a = ConnectionId::from_str("connection-2").unwrap();
	// let connection_id_b = ConnectionId::from_str("connection-1").unwrap();

	// let channel_a = ChannelId::from_str("channel-1").unwrap();
	// let channel_b = ChannelId::from_str("channel-1").unwrap();

	log::info!("Channel A: {:?}", channel_a);
	log::info!("Channel B: {:?}", channel_b);
	log::info!("Connection A: {:?}", connection_id_a);
	log::info!("Connection B: {:?}", connection_id_b);

	// Set connections and channel whitelist
	chain_a.set_connection_id(connection_id_a);
	chain_b.set_connection_id(connection_id_b);

	chain_a.set_channel_whitelist(vec![(channel_a, PortId::transfer())].into_iter().collect());
	chain_b.set_channel_whitelist(vec![(channel_b, PortId::transfer())].into_iter().collect());

	// Run tests sequentially

	// no timeouts + connection delay

	ibc_messaging_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_a.clone(),
		asset_id_b.clone(),
		channel_a,
		channel_b,
	)
	.await;

	// timeouts + connection delay
	ibc_messaging_packet_height_timeout_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_a.clone(),
		channel_a,
		channel_b,
	)
	.await;
	ibc_messaging_packet_height_timeout_with_connection_delay(
		&mut chain_b,
		&mut chain_a,
		asset_id_b.clone(),
		channel_b,
		channel_a,
	)
	.await;
	ibc_messaging_packet_timestamp_timeout_with_connection_delay(
		&mut chain_b,
		&mut chain_a,
		asset_id_b.clone(),
		channel_b,
		channel_a,
	)
	.await;
	ibc_messaging_packet_timestamp_timeout_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_a.clone(),
		channel_a,
		channel_b,
	)
	.await;

	// channel closing semantics
	// ibc_messaging_packet_timeout_on_channel_close(
	// 	&mut chain_b,
	// 	&mut chain_a,
	// 	asset_id_b.clone(),
	// 	channel_b,
	// )
	// .await;
	// ibc_channel_close(&mut chain_a, &mut chain_b).await;

	// // TODO: tendermint misbehaviour?
	// // ibc_messaging_submit_misbehaviour(&mut chain_a, &mut chain_b).await;
}

// #[tokio::test]
#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
#[ignore]
async fn rollup_to_solana_ibc_messaging_full_integration_test() {
	logging::setup_logging();

	let (chain_a, chain_b) = setup_clients().await;
	let (mut chain_b, mut chain_a) = (chain_a, chain_b);

	let (handle, channel_a, channel_b, connection_id_a, connection_id_b) =
		setup_connection_and_channel(&mut chain_a, &mut chain_b, Duration::from_secs(20)).await;
	handle.abort();

	// Set connections and channel whitelist
	chain_a.set_connection_id(connection_id_a);
	chain_b.set_connection_id(connection_id_b);

	chain_a.set_channel_whitelist(vec![(channel_a, PortId::transfer())].into_iter().collect());
	chain_b.set_channel_whitelist(vec![(channel_b, PortId::transfer())].into_iter().collect());

	let asset_id_a = AnyAssetId::Rollup("33WVSef9zaw49KbNdPGTmACVRnAXzN3o1fsqbUrLp2mh".to_string());
	let asset_id_b = AnyAssetId::Solana("33WVSef9zaw49KbNdPGTmACVRnAXzN3o1fsqbUrLp2mh".to_string());

	// Run tests sequentially

	// no timeouts + connection delay
	ibc_messaging_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_a.clone(),
		asset_id_b.clone(),
		channel_a,
		channel_b,
	)
	.await;

	// // timeouts + connection delay
	// ibc_messaging_packet_height_timeout_with_connection_delay(
	// 	&mut chain_a,
	// 	&mut chain_b,
	// 	asset_id_a.clone(),
	// 	channel_a,
	// 	channel_b,
	// )
	// .await;
	// ibc_messaging_packet_timestamp_timeout_with_connection_delay(
	// 	&mut chain_a,
	// 	&mut chain_b,
	// 	asset_id_a.clone(),
	// 	channel_a,
	// 	channel_b,
	// )
	// .await;

	// // channel closing semantics (doesn't work on cosmos)
	// // ibc_messaging_packet_timeout_on_channel_close(&mut chain_a, &mut chain_b,
	// asset_id_a.clone()) // 	.await;
	// // ibc_channel_close(&mut chain_a, &mut chain_b).await;

	// ibc_messaging_submit_misbehaviour(&mut chain_a, &mut chain_b).await;
}
