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
	chain::{AnyAssetId, AnyChain, AnyConfig},
	logging,
	packets::utils::get_key_path,
	substrate::DefaultConfig,
};
use hyperspace_cosmos::client::{ConfigKeyEntry, CosmosClient, CosmosClientConfig};
use hyperspace_parachain::{finality_protocol::FinalityProtocol, ParachainClientConfig};
use hyperspace_primitives::{utils::create_clients, IbcProvider};
use hyperspace_testsuite::{
	ibc_channel_close, ibc_messaging_packet_height_timeout_with_connection_delay,
	ibc_messaging_packet_timeout_on_channel_close,
	ibc_messaging_packet_timestamp_timeout_with_connection_delay,
	ibc_messaging_with_connection_delay, misbehaviour::ibc_messaging_submit_misbehaviour,
};
use ibc::{core::ics23_commitment::commitment::CommitmentPrefix, Height};
use sp_core::hashing::sha2_256;

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
	pub wasm_path: String,
}

impl Default for Args {
	fn default() -> Self {
		let relay = std::env::var("RELAY_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		let para = std::env::var("PARA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		let cosmos = std::env::var("COSMOS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		let wasm_path = std::env::var("WASM_PATH").unwrap_or_else(|_| {
			"../../target/wasm32-unknown-unknown/release/ics10_grandpa_cw.wasm".to_string()
		});

		Args {
			chain_a: format!("ws://{para}:9188"),
			chain_b: format!("http://{cosmos}:26657"),
			relay_chain: format!("ws://{relay}:9944"),
			para_id: 2000,
			connection_prefix_a: "ibc/".to_string(),
			connection_prefix_b: "ibc".to_string(),
			cosmos_grpc: format!("http://{cosmos}:9090"),
			cosmos_ws: format!("ws://{cosmos}:26657/websocket"),
			wasm_path,
		}
	}
}

async fn setup_clients() -> (AnyChain, AnyChain) {
	log::info!(target: "hyperspace", "=========================== Starting Test ===========================");
	let args = Args::default();

	// Create client configurations
	let config_a = ParachainClientConfig {
		name: format!("parachain"),
		para_id: args.para_id,
		parachain_rpc_url: args.chain_a,
		relay_chain_rpc_url: args.relay_chain.clone(),
		client_id: None,
		connection_id: None,
		commitment_prefix: args.connection_prefix_a.as_bytes().to_vec().into(),
		ss58_version: 42,
		channel_whitelist: vec![],
		finality_protocol: FinalityProtocol::Grandpa,
		private_key: "//Alice".to_string(),
		key_type: "sr25519".to_string(),
		wasm_code_id: None,
	};

	let mut config_b = CosmosClientConfig {
		name: "cosmos".to_string(),
		rpc_url: args.chain_b.clone().parse().unwrap(),
		grpc_url: args.cosmos_grpc.clone().parse().unwrap(),
		websocket_url: args.cosmos_ws.clone().parse().unwrap(),
		chain_id: "ibcgo-1".to_string(),
		client_id: None,
		connection_id: None,
		account_prefix: "cosmos".to_string(),
		fee_denom: "stake".to_string(),
		fee_amount: "4000".to_string(),
		gas_limit: (i64::MAX - 1) as u64,
		store_prefix: args.connection_prefix_b,
		max_tx_size: 200000,
		mnemonic:
			"oxygen fall sure lava energy veteran enroll frown question detail include maximum"
				.to_string(),
		wasm_code_id: None,
		channel_whitelist: vec![],
	};

	let chain_b = CosmosClient::<DefaultConfig>::new(config_b.clone()).await.unwrap();

	// let wasm_data = tokio::fs::read(&args.wasm_path).await.expect("Failed to read wasm file");
	// let code_id = match chain_b.upload_wasm(wasm_data.clone()).await {
	// 	Ok(code_id) => code_id,
	// 	Err(e) => {
	// 		let e_str = format!("{:?}", e);
	// 		if !e_str.contains("wasm code already exists") {
	// 			panic!("Failed to upload wasm: {}", e_str);
	// 		}
	// 		sha2_256(&wasm_data).to_vec()
	// 	},
	// };
	// let code_id_str = hex::encode(code_id);
	// config_b.wasm_code_id = Some(code_id_str);

	let mut chain_a_wrapped = AnyConfig::Parachain(config_a).into_client().await.unwrap();
	let mut chain_b_wrapped = AnyConfig::Cosmos(config_b).into_client().await.unwrap();

	let AnyChain::Parachain(chain_a) = &mut chain_a_wrapped else { unreachable!() };

	// Wait until for parachains to start producing blocks
	log::info!(target: "hyperspace", "Waiting for block production from parachain");
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
	log::info!(target: "hyperspace", "Parachain have started block production");

	let clients_on_a = chain_a_wrapped.query_clients().await.unwrap();
	let clients_on_b = chain_b_wrapped.query_clients().await.unwrap();

	if !clients_on_a.is_empty() && !clients_on_b.is_empty() {
		chain_a_wrapped.set_client_id(clients_on_b[0].clone());
		chain_b_wrapped.set_client_id(clients_on_a[0].clone());
		return (chain_a_wrapped, chain_b_wrapped)
	}

	let (client_b, client_a) =
		create_clients(&mut chain_b_wrapped, &mut chain_a_wrapped).await.unwrap();
	chain_a_wrapped.set_client_id(client_a);
	chain_b_wrapped.set_client_id(client_b);
	(chain_a_wrapped, chain_b_wrapped)
}

#[tokio::test]
#[ignore]
async fn parachain_to_cosmos_ibc_messaging_full_integration_test() {
	logging::setup_logging();

	let asset_id_a = AnyAssetId::Parachain(1);
	let asset_id_b = AnyAssetId::Cosmos(
		"ibc/47B97D8FF01DA03FCB2F4B1FFEC931645F254E21EF465FA95CBA6888CB964DC4".to_string(),
	);
	let (mut chain_a, mut chain_b) = setup_clients().await;

	let proof_height = Height::new(1, 8779);

	let key = "acks/ports/transfer/channels/channel-1/sequences/1".to_string();
	let proof = chain_b.query_proof(proof_height, vec![key.into_bytes()]).await.unwrap();
	println!("{}", hex::encode(&proof));

	let key = "acks/ports/transfer/channels/channel-1/sequences/2".to_string();
	let proof = chain_b.query_proof(proof_height, vec![key.into_bytes()]).await.unwrap();
	println!("{}", hex::encode(&proof));

	let key = "acks/ports/transfer/channels/channel-3/sequences/1".to_string();
	let proof = chain_b.query_proof(proof_height, vec![key.into_bytes()]).await.unwrap();
	println!("{}", hex::encode(&proof));

	// let prefix = CommitmentPrefix::try_from(hex::decode("696263").unwrap()).unwrap();
	// // let path = "acks/ports/transfer/channels/channel-3/sequences/1".to_string(); // INCORRECT
	// let path = "acks/ports/transfer/channels/channel-1/sequences/1".to_string();
	// let merkle_path = apply_prefix(&prefix, vec![path]);
	// let proof =
	// CommitmentProofBytes::try_from(hex::decode("
	// 0aef040aec040a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d312f73657175656e6365732f311220439dd0ea54f168850977fce615993ac34f8e2d238c76c0c9328a744e89a2230d1a0c0801180120012a040002c028222d080112060406948901201a212018b8b1cd4e5487dd06c9d630e90e4db05476e5c379cff0c346173b70cfa0983e222b08011227060a94890120505f0e6b3039e050189315bd9a868a00274aacabd9ae85abbd58d857a8f5ec5620222d08011206081a948901201a21208261cb18738d315a27474946d85c0ae81715858dee97e6abda2892ccafa18776222d080112060a2a948901201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222d080112060c56948901201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222d080112060e74948901201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222e0801120712bc02948901201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222e0801120714d604948901201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222e0801120716b609948901201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222e0801120718ee13948901201a2120763fe0dc26765a9069f0198cb4dc24ed387a710ba0bf4f75ae6e6ab3a51d80ec222e080112071aac22968901201a2120000544f6164eb772b5e4d861e753b4ad813d7b2e26489d28e22414d04c24ba670afe010afb010a03696263122002d1c5b1ce8dc9ffe45249153d97fd23379cfdb48cc4863004a9d746062fa3121a090801180120012a0100222708011201011a2052532327b15377f3e4610721f7e69778833f471f6af4d8f3f9f06c81c9ab70ec222508011221013d62060c06c86ae55dc892cbefc1d5a63c65a638f8b613ceecc55250d0af20c5222708011201011a2044ba93e2aa5de099a7e0a4cc79a5c0911956d3557cf22ba26b16117da845bdb8222508011221014ccbbe24a16228b2bc74e21d028b42b03753c679ca0dab083a621c561ba27b11222708011201011a2014a66aa0ada7b629a21909b3f812c0dfc91bb73da10c6bc9ac9eddf3f05efd31"
	// ).unwrap()).unwrap(); let merkle_proof: MerkleProof<HostFunctionsManager> =
	// 	RawMerkleProof::try_from(proof.clone()).unwrap().into();
	// dbg!(&merkle_proof);
	// let root = CommitmentRoot::from_bytes(
	// 	&hex::decode("07b49df5ea9c2d9ee2b3a67dac9830e411434e61625016a4512a368c1f39597a").unwrap(),
	// );
	// let value =
	// 	hex::decode("08f7557ed51826fe18d84512bf24ec75001edbaf2123a477df72a0a9f3640a7c").unwrap();
	// let ps = ProofSpecs::default();
	// merkle_proof
	// 	.verify_membership(&ps, root.clone().into(), merkle_path, value, 0)
	// 	.unwrap();

	// Run tests sequentially

	// no timeouts + connection delay
	// ibc_messaging_with_connection_delay(
	// 	&mut chain_a,
	// 	&mut chain_b,
	// 	asset_id_a.clone(),
	// 	asset_id_b.clone(),
	// )
	// .await;
	//
	// // timeouts + connection delay
	// ibc_messaging_packet_height_timeout_with_connection_delay(
	// 	&mut chain_a,
	// 	&mut chain_b,
	// 	asset_id_a.clone(),
	// )
	// .await;
	// ibc_messaging_packet_timestamp_timeout_with_connection_delay(
	// 	&mut chain_a,
	// 	&mut chain_b,
	// 	asset_id_a.clone(),
	// )
	// .await;
	//
	// // channel closing semantics
	// ibc_messaging_packet_timeout_on_channel_close(&mut chain_a, &mut chain_b, asset_id_a.clone())
	// 	.await;
	// ibc_channel_close(&mut chain_a, &mut chain_b).await;

	// TODO: tendermint misbehaviour?
	// ibc_messaging_submit_misbehaviour(&mut chain_a, &mut chain_b).await;
}

#[tokio::test]
#[ignore]
async fn cosmos_to_parachain_ibc_messaging_full_integration_test() {
	logging::setup_logging();

	let (chain_a, chain_b) = setup_clients().await;
	let (mut chain_b, mut chain_a) = (chain_a, chain_b);

	let asset_id_a = AnyAssetId::Cosmos("stake".to_string());
	let asset_id_b = AnyAssetId::Parachain(2);

	// Run tests sequentially

	// no timeouts + connection delay
	ibc_messaging_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_a.clone(),
		asset_id_b.clone(),
	)
	.await;

	// timeouts + connection delay
	ibc_messaging_packet_height_timeout_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_a.clone(),
	)
	.await;
	ibc_messaging_packet_timestamp_timeout_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_a.clone(),
	)
	.await;

	// channel closing semantics (doesn't work on cosmos)
	// ibc_messaging_packet_timeout_on_channel_close(&mut chain_a, &mut chain_b, asset_id_a.clone())
	// 	.await;
	// ibc_channel_close(&mut chain_a, &mut chain_b).await;

	ibc_messaging_submit_misbehaviour(&mut chain_a, &mut chain_b).await;
}
