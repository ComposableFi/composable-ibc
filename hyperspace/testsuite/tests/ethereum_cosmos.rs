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

use core::time::Duration;
use ethers::{
	abi::Token,
	prelude::{ContractFactory, ContractInstance},
	utils::AnvilInstance,
};
use ethers_solc::{Artifact, ProjectCompileOutput, ProjectPathsConfig};
use futures::StreamExt;
use hyperspace_core::{
	chain::{AnyAssetId, AnyChain, AnyConfig},
	logging,
	substrate::DefaultConfig,
};
use hyperspace_cosmos::client::{CosmosClient, CosmosClientConfig};
use hyperspace_ethereum::{
	config::EthereumClientConfig,
	mock::{utils, utils::hyperspace_ethereum_client_fixture},
	utils::{DeployYuiIbc, ProviderImpl},
};
use hyperspace_parachain::{finality_protocol::FinalityProtocol, ParachainClientConfig};
use hyperspace_primitives::{utils::create_clients, CommonClientConfig, IbcProvider};
use hyperspace_testsuite::{
	ibc_channel_close, ibc_messaging_packet_height_timeout_with_connection_delay,
	ibc_messaging_packet_timeout_on_channel_close,
	ibc_messaging_packet_timestamp_timeout_with_connection_delay,
	ibc_messaging_with_connection_delay, misbehaviour::ibc_messaging_submit_misbehaviour,
	setup_connection_and_channel,
};
use ibc::core::ics24_host::identifier::{ClientId, PortId};
use sp_core::hashing::sha2_256;
use std::{future::Future, path::PathBuf, str::FromStr, sync::Arc};
use subxt::utils::H160;

#[derive(Debug, Clone)]
pub struct Args {
	pub chain_a: String,
	pub chain_b: String,
	pub connection_prefix_a: String,
	pub connection_prefix_b: String,
	pub cosmos_grpc: String,
	pub cosmos_ws: String,
	pub ethereum_rpc: String,
	pub wasm_path: String,
}

impl Default for Args {
	fn default() -> Self {
		let eth = std::env::var("ETHEREUM_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		let cosmos = std::env::var("COSMOS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
		let wasm_path = std::env::var("WASM_PATH").unwrap_or_else(|_| {
			"../../target/wasm32-unknown-unknown/release/icsxx_ethereum_cw.wasm".to_string()
		});

		Args {
			chain_a: format!("ws://{eth}:5001"),
			chain_b: format!("http://{cosmos}:26657"),
			connection_prefix_a: "ibc/".to_string(),
			connection_prefix_b: "ibc".to_string(),
			cosmos_grpc: format!("http://{cosmos}:9090"),
			cosmos_ws: format!("ws://{cosmos}:26657/websocket"),
			ethereum_rpc: format!("http://{eth}:6001"),
			wasm_path,
		}
	}
}

pub struct DeployYuiIbcTendermintClient {
	pub path: PathBuf,
	pub project_output: ProjectCompileOutput,
	pub anvil: AnvilInstance,
	pub client: Arc<ProviderImpl>,
	pub tendermint_client: ContractInstance<Arc<ProviderImpl>, ProviderImpl>,
	pub ics20_module: Option<ContractInstance<Arc<ProviderImpl>, ProviderImpl>>,
	pub yui_ibc: DeployYuiIbc<Arc<ProviderImpl>, ProviderImpl>,
}

pub async fn deploy_yui_ibc_and_tendermint_client_fixture() -> DeployYuiIbcTendermintClient {
	let path = utils::yui_ibc_solidity_path();
	let project_output = utils::compile_yui(&path, "contracts/core");
	let diamond_project_output = utils::compile_yui(&path, "contracts/diamond");
	let project_output1 = utils::compile_yui(&path, "contracts/clients");
	let (anvil, client) = utils::spawn_anvil();
	log::warn!("{}", anvil.endpoint());
	let yui_ibc =
		utils::deploy_yui_ibc(&project_output, &diamond_project_output, client.clone()).await;

	let upd = project_output1.find_first("DelegateTendermintUpdate").unwrap();
	let (abi, bytecode, _) = upd.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let update_client_delegate_contract = factory.deploy(()).unwrap().send().await.unwrap();

	let contract = project_output1.find_first("TendermintLightClientSimple").unwrap();
	let r = contract.clone();
	let (abi, bytecode, _) = r.into_parts();

	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let tendermint_light_client = factory
		.deploy((
			Token::Address(yui_ibc.diamond.address()),
			Token::Address(update_client_delegate_contract.address()),
		))
		.unwrap()
		.send()
		.await
		.unwrap();

	let _ = yui_ibc
		.register_client("07-tendermint", tendermint_light_client.address())
		.await;

	DeployYuiIbcTendermintClient {
		path,
		project_output,
		anvil,
		client,
		yui_ibc,
		tendermint_client: tendermint_light_client,
		ics20_module: None,
	}
}

#[track_caller]
fn deploy_transfer_module_fixture(
	deploy: &DeployYuiIbcTendermintClient,
) -> impl Future<Output = ContractInstance<Arc<ProviderImpl>, ProviderImpl>> + '_ {
	async move {
		let path = utils::yui_ibc_solidity_path();
		let project_output = utils::compile_yui(&path, "contracts/apps/20-transfer");

		let artifact =
			project_output.find_first("ICS20Bank").expect("no ICS20Bank in project output");
		let bank_contract = utils::deploy_contract(artifact, (), deploy.client.clone()).await;
		println!("Bank module address: {:?}", bank_contract.address());
		let artifact = project_output
			.find_first("ICS20TransferBank")
			.expect("no ICS20TransferBank in project output");
		let constructor_args = (
			Token::Address(deploy.yui_ibc.diamond.address()),
			Token::Address(bank_contract.address()),
		);
		let module_contract =
			utils::deploy_contract(artifact, constructor_args, deploy.client.clone()).await;
		module_contract
	}
}

async fn setup_clients() -> (AnyChain, AnyChain) {
	log::info!(target: "hyperspace", "=========================== Starting Test ===========================");
	let args = Args::default();

	// Create client configurations

	let deploy = deploy_yui_ibc_and_tendermint_client_fixture().await;
	let bank = deploy_transfer_module_fixture(&deploy).await;
	let DeployYuiIbcTendermintClient {
		anvil, tendermint_client, ics20_module: _, mut yui_ibc, ..
	} = deploy;
	log::info!(target: "hyperspace", "Deployed diamond: {:?}, tendermint client: {:?}, bank: {:?}", yui_ibc.diamond.address(), tendermint_client.address(), bank.address());
	yui_ibc.bind_port("transfer", bank.address()).await;
	yui_ibc.bank = Some(bank);

	// Overrides
	// yui_ibc.diamond = ContractInstance::new(
	// 	H160::from_str("0xf02f5476535c0eea1e501d2c155c1e1f3055e752").unwrap(),
	// 	yui_ibc.diamond.abi().clone(),
	// 	yui_ibc.diamond.client(),
	// );
	// yui_ibc.tendermint = ContractInstance::new(
	// 	H160::from_str("0x5d2d6b3fd375fd77dc627d1ae955c441ec157847").unwrap(),
	// 	yui_ibc.tendermint.abi().clone(),
	// 	yui_ibc.tendermint.client(),
	// );
	// {
	// 	let bank = yui_ibc.bank.as_mut().unwrap();
	// 	*bank = ContractInstance::new(
	// 		H160::from_str("0xa4373e6da07ad62d43c3a74f5edb3d3aa6674c14").unwrap(),
	// 		bank.abi().clone(),
	// 		bank.client(),
	// 	);
	// }

	//replace the tendermint client address in hyperspace config with a real one
	let diamond_address = yui_ibc.diamond.address();
	let tendermint_address = yui_ibc.tendermint.address();
	let mut config_a = hyperspace_ethereum_client_fixture(&anvil, yui_ibc).await;
	config_a.tendermint_client_address = tendermint_address;
	config_a.ibc_handler_address = diamond_address;

	let mut config_b = CosmosClientConfig {
		name: "centauri".to_string(),
		rpc_url: args.chain_b.clone().parse().unwrap(),
		grpc_url: args.cosmos_grpc.clone().parse().unwrap(),
		websocket_url: args.cosmos_ws.clone().parse().unwrap(),
		chain_id: "centauri-testnet-1".to_string(),
		client_id: None,
		connection_id: None,
		account_prefix: "centauri".to_string(),
		fee_denom: "stake".to_string(),
		fee_amount: "4000".to_string(),
		gas_limit: (i64::MAX - 1) as u64,
		store_prefix: args.connection_prefix_b,
		max_tx_size: 200000,
		mnemonic:
			"sense state fringe stool behind explain area quit ugly affair develop thumb clinic weasel choice atom gesture spare sea renew penalty second upon peace"
				.to_string(),
		wasm_code_id: None,
		channel_whitelist: vec![],
		common: CommonClientConfig {
			skip_optional_client_updates: true,
			max_packets_to_process: 200,
		},
	};

	let chain_b = CosmosClient::<DefaultConfig>::new(config_b.clone()).await.unwrap();

	let wasm_data = tokio::fs::read(&args.wasm_path).await.expect("Failed to read wasm file");
	let code_id = match chain_b.upload_wasm(wasm_data.clone()).await {
		Ok(code_id) => code_id,
		Err(e) => {
			let e_str = format!("{e:?}");
			if !e_str.contains("wasm code already exists") {
				panic!("Failed to upload wasm: {e_str}");
			}
			sha2_256(&wasm_data).to_vec()
		},
	};
	let code_id_str = hex::encode(code_id);
	config_b.wasm_code_id = Some(code_id_str);

	let mut chain_a_wrapped = AnyConfig::Ethereum(config_a).into_client().await.unwrap();
	let mut chain_b_wrapped = AnyConfig::Cosmos(config_b).into_client().await.unwrap();

	let clients_on_a = chain_a_wrapped.query_clients().await.unwrap();
	let clients_on_b = chain_b_wrapped.query_clients().await.unwrap();

	// if !clients_on_a.is_empty() && !clients_on_b.is_empty() {
	// 	chain_a_wrapped.set_client_id(clients_on_b.last().unwrap().clone());
	// 	chain_b_wrapped.set_client_id(clients_on_a.last().unwrap().clone());
	// 	return (chain_a_wrapped, chain_b_wrapped)
	// }

	let (client_b, client_a) =
		create_clients(&mut chain_b_wrapped, &mut chain_a_wrapped).await.unwrap();
	// let (client_a, client_b): (ClientId, ClientId) =
	// 	("08-wasm-136".parse().unwrap(), "07-tendermint-0".parse().unwrap());

	log::info!(target: "hyperspace", "Client A: {client_a:?} B: {client_b:?}");
	chain_a_wrapped.set_client_id(client_a);
	chain_b_wrapped.set_client_id(client_b);
	(chain_a_wrapped, chain_b_wrapped)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn ethereum_to_cosmos_ibc_messaging_full_integration_test() {
	logging::setup_logging();

	let asset_id_a = AnyAssetId::Ethereum("pica".to_string());
	let asset_id_b = AnyAssetId::Cosmos(
		"ibc/47B97D8FF01DA03FCB2F4B1FFEC931645F254E21EF465FA95CBA6888CB964DC4".to_string(),
	);
	let (mut chain_a, mut chain_b) = setup_clients().await;
	let (handle, channel_a, channel_b, connection_id_a, connection_id_b) =
		setup_connection_and_channel(&mut chain_a, &mut chain_b, Duration::from_secs(60 * 2)).await;
	handle.abort();

	// let connection_id_a = "connection-0".parse().unwrap();
	// let connection_id_b = "connection-35".parse().unwrap();
	// let channel_a = "channel-0".parse().unwrap();
	// let channel_b = "channel-10".parse().unwrap();

	log::info!(target: "hyperspace", "Conn A: {connection_id_a:?} B: {connection_id_b:?}");
	log::info!(target: "hyperspace", "Chann A: {channel_a:?} B: {channel_b:?}");

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
	//
	// // channel closing semantics
	// ibc_messaging_packet_timeout_on_channel_close(
	// 	&mut chain_a,
	// 	&mut chain_b,
	// 	asset_id_a.clone(),
	// 	channel_a,
	// )
	// .await;
	// ibc_channel_close(&mut chain_a, &mut chain_b).await;

	// TODO: tendermint misbehaviour?
	// ibc_messaging_submit_misbehaviour(&mut chain_a, &mut chain_b).await;
}

#[tokio::test]
#[ignore]
async fn cosmos_to_ethereum_ibc_messaging_full_integration_test() {
	logging::setup_logging();

	let (chain_a, chain_b) = setup_clients().await;
	let (mut chain_b, mut chain_a) = (chain_a, chain_b);

	let (handle, channel_a, channel_b, connection_id_a, connection_id_b) =
		setup_connection_and_channel(&mut chain_a, &mut chain_b, Duration::from_secs(60 * 2)).await;
	handle.abort();

	// Set connections and channel whitelist
	chain_a.set_connection_id(connection_id_a);
	chain_b.set_connection_id(connection_id_b);

	chain_a.set_channel_whitelist(vec![(channel_a, PortId::transfer())].into_iter().collect());
	chain_b.set_channel_whitelist(vec![(channel_b, PortId::transfer())].into_iter().collect());

	let asset_id_a = AnyAssetId::Cosmos("stake".to_string());
	let asset_id_b = AnyAssetId::Parachain(2);

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
	ibc_messaging_packet_timestamp_timeout_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_a.clone(),
		channel_a,
		channel_b,
	)
	.await;

	// channel closing semantics (doesn't work on cosmos)
	// ibc_messaging_packet_timeout_on_channel_close(&mut chain_a, &mut chain_b, asset_id_a.clone())
	// 	.await;
	// ibc_channel_close(&mut chain_a, &mut chain_b).await;

	ibc_messaging_submit_misbehaviour(&mut chain_a, &mut chain_b).await;
}
