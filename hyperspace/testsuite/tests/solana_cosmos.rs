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
use hyperspace_cosmos::client::{CosmosClient, CosmosClientConfig};
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
	pub relay_chain: String,
	pub para_id: u32,
	pub connection_prefix_a: String,
	pub connection_prefix_b: String,
	pub cosmos_grpc: String,
	pub cosmos_ws: String,
	pub solana_ws: String,
	pub wasm_path: String,
}

impl Default for Args {
	fn default() -> Self {
		let relay = std::env::var("RELAY_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		let solana = std::env::var("SOLANA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		let cosmos = std::env::var("COSMOS_HOST").unwrap_or_else(|_| "10.132.0.6".to_string());
		let wasm_path = std::env::var("WASM_PATH").unwrap_or_else(|_| {
			"../../target/wasm32-unknown-unknown/release/ics07_guest_cw.wasm".to_string()
		});

		Args {
			// chain_a: format!("https://devnet.helius-rpc.com/?api-key=bc5c0cfc-46df-4781-978f-af6ca7a202c2"),
			chain_a: format!("http://{solana}:8899"),
			// chain_b: format!("http://{cosmos}:26657"),
			// chain_b: format!("http://34.34.178.141:26657"),
			// chain_b: format!("http://10.132.0.13:26657/"), // testnet
			chain_b: format!("http://10.132.0.6:26657/"), // mainnet
			// chain_b: format!("https://rpc-testnet5.composable-cosmos.composablenodes.tech"),
			relay_chain: format!("ws://{relay}:9944"),
			para_id: 2000,
			connection_prefix_a: "ibc".to_string(),
			connection_prefix_b: "ibc".to_string(),
			// cosmos_grpc: format!("http://34.34.182.7:9098"),
			// cosmos_grpc: format!("http://{cosmos}:9999"),
			// cosmos_grpc: format!("http://34.34.178.141:9999"),
			// cosmos_grpc: format!("http://10.132.0.13:9999/"), // testnet
			cosmos_grpc: format!("http://10.132.0.6:9999/"), // mainnet
			// cosmos_ws: format!("ws://{cosmos}:26657/websocket"),
			// cosmos_ws: format!("ws://34.34.178.141:26657/websocket"),
			// cosmos_ws:
			// format!("wss://rpc-testnet5.composable-cosmos.composablenodes.tech/websocket"),
			// cosmos_ws: format!("ws://10.132.0.13:26657/websocket"),
			cosmos_ws: format!("ws://10.132.0.6:26657/websocket"), // mainnet
			solana_ws: format!("ws://{solana}:8900"),
			// solana_ws:
			// format!("wss://devnet.helius-rpc.com/?api-key=bc5c0cfc-46df-4781-978f-af6ca7a202c2"),
			wasm_path,
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
		commitment_prefix: args.connection_prefix_a.as_bytes().to_vec(),
		wasm_code_id: None,
		rpc_url: args.chain_a.clone().parse().unwrap(),
		ws_url: args.solana_ws.clone().parse().unwrap(),
		chain_id: "solana-1".to_string(),
		account_prefix: args.connection_prefix_a.clone(),
		fee_denom: "stake".to_string(),
		fee_amount: "4000".to_string(),
		gas_limit: 100000000,
		store_prefix: args.connection_prefix_a,
		max_tx_size: 320000,
		common_state_config: CommonClientConfig {
			skip_optional_client_updates: true,
			max_packets_to_process: 1,
		},
		channel_whitelist: vec![],
		commitment_level: "confirmed".to_string(),
		private_key: vec![
			48, 123, 8, 80, 248, 0, 217, 142, 124, 193, 95, 24, 168, 139, 214, 136, 147, 210, 168,
			135, 26, 36, 162, 89, 150, 185, 99, 191, 247, 135, 78, 111, 12, 8, 4, 81, 129, 165,
			153, 230, 192, 225, 51, 119, 216, 14, 69, 225, 73, 7, 204, 144, 39, 213, 91, 255, 136,
			38, 95, 131, 197, 4, 101, 186,
		],
		solana_ibc_program_id: "3MZrLWwMvD9mcMiNd7mnbrqmZDzjg29qmLo7FMTqr3qT".to_string(),
		write_program_id: "FttaQtn8T8CnDCXd7JwxvkkKSYgVi7XwwyY7p2b6TCUt".to_string(),
		signature_verifier_program_id: "2G9Wsz1LfzJ2gpVbeXuSciih2s3wKdj4fcTjeD1JJ3M1".to_string(),
	};

	let mut config_b = CosmosClientConfig {
        name: "centauri".to_string(),
        rpc_url: args.chain_b.clone().parse().unwrap(),
        grpc_url: args.cosmos_grpc.clone().parse().unwrap(),
        websocket_url: args.cosmos_ws.clone().parse().unwrap(),
        // chain_id: "banksy-testnet-5".to_string(),
        chain_id: "centauri-1".to_string(),
        client_id: None,
        connection_id: None,
        account_prefix: "centauri".to_string(),
        fee_denom: "ppica".to_string(),
        fee_amount: "10000000".to_string(),
        gas_limit: 100000000,
        store_prefix: args.connection_prefix_b,
        max_tx_size: 200000,
        mnemonic:
        // centauri1g5r2vmnp6lta9cpst4lzc4syy3kcj2ljte3tlh
        // "decorate bright ozone fork gallery riot bus exhaust worth way bone indoor calm squirrel merry zero scheme cotton until shop any excess stage laundry"
				// "peace cash suffer celery broken blade fame fiscal lesson fancy virus bless recipe inherit reason cart mask mask absurd venture culture problem reward crew"
				"scissors enroll comfort wrist eight catch decide stage squirrel phrase close december staff baby stable mirror hand allow sort dish wrist gas quantum puppy"
            .to_string(),
        wasm_code_id: None,
        channel_whitelist: vec![],
        common: CommonClientConfig {
            skip_optional_client_updates: true,
            max_packets_to_process: 200,
        },
        skip_tokens_list: Some(vec!["uosmo".to_string()]),
    };

	// println!("This is config b {:?}", config_b);

	let chain_a = SolanaClient::new(config_a.clone()).await.expect("Solana error");
	let chain_b = CosmosClient::<DefaultConfig>::new(config_b.clone())
		.await
		.map_err(|e| println!("{:?}", e))
		.unwrap();

	println!("This is chain b prefix {:?}", chain_b.commitment_prefix.as_bytes());

	// let wasm_data = tokio::fs::read(&args.wasm_path).await.expect("Failed to read wasm file");
	// let code_id = match chain_b.upload_wasm(wasm_data.clone()).await {
	// 	Ok(code_id) => {
	// 		log::info!("wasm was uploaded");
	// 		code_id
	// 	},
	// 	Err(e) => {
	// 		let e_str = format!("{e:?}");
	// 		if !e_str.contains("wasm code already exists") {
	// 			panic!("Failed to upload wasm: {e_str}");
	// 		}
	// 		sha2_256(&wasm_data).to_vec()
	// 	},
	// };
	// let code_id_str = hex::encode(code_id);
	let code_id_str =
		String::from("66ce7420d21e2555b0e6ce952c0826590fb5f6508a9ac84a5c11178cec58a303");
	log::info!("This is wasm checksum {:?}", code_id_str);
	config_b.wasm_code_id = Some(code_id_str);

	let mut chain_a_wrapped = AnyConfig::Solana(config_a).into_client().await.unwrap();
	let mut chain_b_wrapped = AnyConfig::Cosmos(config_b).into_client().await.unwrap();

	let AnyChain::Solana(chain_a) = &mut chain_a_wrapped else { unreachable!() };

	// let clients_on_a = chain_a_wrapped.query_clients().await.unwrap();
	// let clients_on_b = chain_b_wrapped.query_clients().await.unwrap();

	// if !clients_on_a.is_empty() && !clients_on_b.is_empty() {
	// 	chain_a_wrapped.set_client_id(clients_on_b[0].clone());
	// 	chain_b_wrapped.set_client_id(clients_on_a[0].clone());
	// 	return (chain_a_wrapped, chain_b_wrapped)
	// }

	let (client_a, client_b) =
		create_clients(&mut chain_a_wrapped, &mut chain_b_wrapped).await.unwrap();
	chain_a_wrapped.set_client_id(client_a);
	chain_b_wrapped.set_client_id(client_b);
	// chain_b_wrapped.set_client_id(ClientId::new("07-tendermint", 0).unwrap());
	// chain_a_wrapped.set_client_id(ClientId::new("08-wasm", 137).unwrap());
	(chain_a_wrapped, chain_b_wrapped)
}

// #[tokio::test]
#[tokio::test(flavor = "multi_thread", worker_threads = 12)]
// #[ignore]
async fn solana_to_cosmos_ibc_messaging_full_integration_test() {
	use ibc::core::ics24_host::identifier::{ChannelId, ConnectionId};
	use std::str::FromStr;
	logging::setup_logging();

	let asset_id_a = AnyAssetId::Solana("33WVSef9zaw49KbNdPGTmACVRnAXzN3o1fsqbUrLp2mh".to_string());
	let asset_id_b = AnyAssetId::Cosmos("ppica".to_string());
	let (mut chain_a, mut chain_b) = setup_clients().await;
	let (handle, channel_a, channel_b, connection_id_a, connection_id_b) =
		setup_connection_and_channel(&mut chain_a, &mut chain_b, Duration::from_secs(0)).await;

	handle.abort();

	// let connection_id_a = ConnectionId::from_str("connection-0").unwrap();
	// let connection_id_b = ConnectionId::from_str("connection-0").unwrap();

	// let channel_a = ChannelId::from_str("channel-0").unwrap();
	// let channel_b = ChannelId::from_str("channel-0").unwrap();

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
async fn cosmos_to_solana_ibc_messaging_full_integration_test() {
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

	let asset_id_a = AnyAssetId::Cosmos("stake".to_string());
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