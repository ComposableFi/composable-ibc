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

use async_trait::async_trait;
use futures::StreamExt;
use hyperspace_core::{
	chain::{AnyAssetId, AnyChain, AnyConfig},
	logging,
};
use hyperspace_cosmos::client::{ConfigKeyEntry, CosmosClient, CosmosClientConfig};
use hyperspace_parachain::{
	config, config::CustomExtrinsicParams, finality_protocol::FinalityProtocol,
	ParachainClientConfig,
};
use hyperspace_primitives::{utils::create_clients, IbcProvider};
use hyperspace_testsuite::{
	ibc_channel_close, ibc_messaging_packet_height_timeout_with_connection_delay,
	ibc_messaging_packet_timeout_on_channel_close,
	ibc_messaging_packet_timestamp_timeout_with_connection_delay,
	ibc_messaging_with_connection_delay, misbehaviour::ibc_messaging_submit_misbehaviour,
};
use sp_core::hashing::sha2_256;
use subxt::{
	config::{
		extrinsic_params::Era,
		polkadot::{PolkadotExtrinsicParams, PolkadotExtrinsicParamsBuilder},
	},
	Error, OnlineClient,
};

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

#[derive(Debug, Clone)]
pub enum DefaultConfig {}

#[async_trait]
impl config::Config for DefaultConfig {
	type AssetId = u128;
	type Signature = <Self as subxt::Config>::Signature;
	type Address = <Self as subxt::Config>::Address;

	async fn custom_extrinsic_params(
		client: &OnlineClient<Self>,
	) -> Result<CustomExtrinsicParams<Self>, Error> {
		let params =
			PolkadotExtrinsicParamsBuilder::new().era(Era::Immortal, client.genesis_hash());
		Ok(params.into())
	}
}

impl subxt::Config for DefaultConfig {
	type Index = u32;
	type BlockNumber = u32;
	type Hash = sp_core::H256;
	type AccountId = sp_runtime::AccountId32;
	type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
	type Header = subxt::config::substrate::SubstrateHeader<
		Self::BlockNumber,
		subxt::config::substrate::BlakeTwo256,
	>;
	type Signature = sp_runtime::MultiSignature;
	type ExtrinsicParams = PolkadotExtrinsicParams<Self>;
	type Hasher = subxt::config::substrate::BlakeTwo256;
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
		keybase: ConfigKeyEntry {
			public_key: "spub4W7TSjsuqcUE17mSB2ajhZsbwkefsHWKsXCbERimu3z2QLN9EFgqqpppiBn4tTNPFoNVTo1b3BgCZAaFJuUgTZeFhzJjUHkK8X7kSC5c7yn".to_string(),
			private_key: "sprv8H873EM21Euvndgy513jLRvsPipBTpnUWJGzS3KALiT3XY2zgiNbJ2WLrvPzRhg7GuAoujHd5d6cpBe887vTbJghja8kmRdkHoNgamx6WWr".to_string(),
			account: "cosmos1nnypkcfrvu3e9dhzeggpn4kh622l4cq7wwwrn0".to_string(),
			address: vec![156, 200, 27, 97, 35, 103, 35, 146, 182, 226, 202, 16, 25, 214, 215, 210, 149, 250, 224, 30],
		},
		wasm_code_id: None,
		channel_whitelist: vec![],
	};

	let chain_b = CosmosClient::<DefaultConfig>::new(config_b.clone()).await.unwrap();

	let wasm_data = tokio::fs::read(&args.wasm_path).await.expect("Failed to read wasm file");
	let code_id = match chain_b.upload_wasm(wasm_data.clone()).await {
		Ok(code_id) => code_id,
		Err(e) => {
			let e_str = format!("{:?}", e);
			if !e_str.contains("wasm code already exists") {
				panic!("Failed to upload wasm: {}", e_str);
			}
			sha2_256(&wasm_data).to_vec()
		},
	};
	let code_id_str = hex::encode(code_id);
	config_b.wasm_code_id = Some(code_id_str);

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

	let (client_b, client_a) = create_clients(&chain_b_wrapped, &chain_a_wrapped).await.unwrap();
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

	// channel closing semantics
	ibc_messaging_packet_timeout_on_channel_close(&mut chain_a, &mut chain_b, asset_id_a.clone())
		.await;
	ibc_channel_close(&mut chain_a, &mut chain_b).await;

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
