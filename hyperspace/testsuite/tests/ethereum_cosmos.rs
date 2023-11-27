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

use crate::utils::ETH_NODE_PORT_WS;
use core::time::Duration;
use ethers::{abi::Token, prelude::ContractInstance, utils::AnvilInstance};
use ethers_solc::ProjectCompileOutput;
use hyperspace_core::{
	chain::{AnyAssetId, AnyChain, AnyConfig},
	logging,
};
use hyperspace_cosmos::client::{CosmosClient, CosmosClientConfig};
use hyperspace_ethereum::{
	client::EthereumClient,
	config::EthereumClientConfig,
	ibc_provider::{Ics20BankAbi, SendPacketFilter},
	mock::{
		utils,
		utils::{hyperspace_ethereum_client_fixture, ETH_NODE_PORT, USE_GETH},
	},
	utils::{check_code_size, deploy_contract, DeployYuiIbc, ProviderImpl},
};
use hyperspace_parachain::{finality_protocol::FinalityProtocol, ParachainClientConfig};
use hyperspace_primitives::{utils::create_clients, Chain, CommonClientConfig, IbcProvider};
use hyperspace_testsuite::{
	ibc_channel_close, ibc_messaging_packet_height_timeout_with_connection_delay,
	ibc_messaging_packet_timeout_on_channel_close,
	ibc_messaging_packet_timestamp_timeout_with_connection_delay,
	ibc_messaging_with_connection_delay, setup_connection_and_channel,
};
use ibc::core::{ics23_commitment::specs::ProofSpecs, ics24_host::identifier::PortId};
use log::info;
use sp_core::hashing::sha2_256;
use std::{
	future::Future,
	path::PathBuf,
	str::FromStr,
	sync::{Arc, Mutex},
};
use tokio::time::sleep;

const USE_CONFIG: bool = false;
const SAVE_TO_CONFIG: bool = true;

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
			chain_a: format!("ws://{eth}:{ETH_NODE_PORT_WS}"),
			chain_b: format!("http://{cosmos}:36657"),
			connection_prefix_a: "ibc/".to_string(),
			connection_prefix_b: "ibc".to_string(),
			cosmos_grpc: format!("http://{cosmos}:1090"),
			cosmos_ws: format!("ws://{cosmos}:36657/websocket"),
			ethereum_rpc: format!("http://{eth}:{}", ETH_NODE_PORT),
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
	println!("path: {:?}", path);
	let project_output = hyperspace_ethereum::utils::compile_yui(&path, "contracts/core");
	let diamond_project_output =
		hyperspace_ethereum::utils::compile_yui(&path, "contracts/diamond");
	let project_output1 = hyperspace_ethereum::utils::compile_yui(&path, "contracts/clients");
	let (anvil, client) = utils::spawn_anvil().await;
	log::warn!("endpoint: {}, chain id: {}", anvil.endpoint(), anvil.chain_id());
	let mut yui_ibc = hyperspace_ethereum::utils::deploy_yui_ibc(
		&project_output,
		&diamond_project_output,
		client.clone(),
	)
	.await;

	check_code_size(project_output1.artifacts());

	let ics23_contract =
		deploy_contract("Ics23Contract", &[&project_output1], (), client.clone()).await;

	let update_client_delegate_contract =
		deploy_contract("DelegateTendermintUpdate", &[&project_output1], (), client.clone()).await;

	let tendermint_light_client = deploy_contract(
		"TendermintLightClientSimple",
		&[&project_output1],
		(
			Token::Address(yui_ibc.diamond.address()),
			Token::Address(update_client_delegate_contract.address()),
			Token::Address(ics23_contract.address()),
		),
		client.clone(),
	)
	.await;

	sleep(Duration::from_secs(10)).await;
	let _ = yui_ibc
		.register_client("07-tendermint", tendermint_light_client.address())
		.await;

	yui_ibc.tendermint = Some(tendermint_light_client.clone());

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
) -> impl Future<
	Output = (
		ContractInstance<Arc<ProviderImpl>, ProviderImpl>,
		ContractInstance<Arc<ProviderImpl>, ProviderImpl>,
	),
> + '_ {
	async move {
		let path = utils::yui_ibc_solidity_path();
		let project_output =
			hyperspace_ethereum::utils::compile_yui(&path, "contracts/apps/20-transfer");

		let ics20_bank_contract = deploy_contract(
			"ICS20Bank",
			&[&project_output],
			Token::String("ETH".to_string()),
			deploy.client.clone(),
		)
		.await;
		println!("Bank module address: {:?}", ics20_bank_contract.address());
		let constructor_args = (
			Token::Address(deploy.yui_ibc.diamond.address()),
			Token::Address(ics20_bank_contract.address()),
		);
		let ics20_bank_transfer_contract = deploy_contract(
			"ICS20TransferBank",
			&[&project_output],
			constructor_args,
			deploy.client.clone(),
		)
		.await;
		(ics20_bank_contract, ics20_bank_transfer_contract)
	}
}

async fn setup_clients() -> (AnyChain, AnyChain) {
	log::info!(target: "hyperspace", "=========================== Starting Test ===========================");
	let args = Args::default();

	// Create client configurations
	let config_a = if USE_CONFIG {
		toml::from_str(include_str!("../../../config/ethereum-local.toml")).unwrap()
	} else {
		let deploy = deploy_yui_ibc_and_tendermint_client_fixture().await;
		let (ics20_bank_contract, ics20_bank_trasnfer_contract) =
			deploy_transfer_module_fixture(&deploy).await;
		let DeployYuiIbcTendermintClient {
			anvil,
			tendermint_client,
			ics20_module: _,
			mut yui_ibc,
			..
		} = deploy;
		yui_ibc.bind_port("transfer", ics20_bank_trasnfer_contract.address()).await;
		info!(target: "hyperspace", "Deployed diamond: {:?}, tendermint client: {:?}, bank: {:?}", yui_ibc.diamond.address(), tendermint_client.address(), ics20_bank_contract.address());
		yui_ibc.ics20_transfer_bank = Some(ics20_bank_trasnfer_contract);
		yui_ibc.ics20_bank = Some(ics20_bank_contract);

		//replace the tendermint client address in hyperspace config with a real one
		let tendermint_address = yui_ibc.tendermint.as_ref().map(|x| x.address());
		let mut config_a = hyperspace_ethereum_client_fixture(
			&anvil,
			yui_ibc,
			"pg://postgres:password@localhost/postgres",
			"redis://localhost:6379",
		)
		.await;
		config_a.tendermint_address = tendermint_address;
		if !USE_GETH {
			config_a.ws_rpc_url = anvil.ws_endpoint().parse().unwrap();
		}
		config_a.anvil = Some(Arc::new(Mutex::new(anvil)));

		if SAVE_TO_CONFIG {
			let config_path = PathBuf::from_str("../../config/ethereum-local.toml").unwrap();
			let config_a_str = toml::to_string_pretty(&config_a).unwrap();
			std::fs::write(config_path, config_a_str).unwrap();
		}
		config_a
	};

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

	let chain_b = CosmosClient::<()>::new(config_b.clone()).await.unwrap();

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

	let mut clients_on_a =
		chain_a_wrapped.query_clients(&"07-tendermint".to_string()).await.unwrap();
	let mut clients_on_b = chain_b_wrapped.query_clients(&"08-wasm".to_string()).await.unwrap();

	let mut client_id_a = None;
	let mut client_id_b = None;
	if !clients_on_a.is_empty() && !clients_on_b.is_empty() {
		let client_a = clients_on_b.pop().unwrap();
		let client_b = clients_on_a.pop().unwrap();
		info!(target: "hyperspace", "Reusing clients A: {client_a:?} B: {client_b:?}");
		// client_id_a = Some(client_a);
		// client_id_b = Some(client_b);
	}

	if client_id_a.is_none() || client_id_b.is_none() {
		let (client_b, client_a) =
			create_clients(&mut chain_b_wrapped, &mut chain_a_wrapped).await.unwrap();
		info!(target: "hyperspace", "Created clients A: {client_a:?} B: {client_b:?}");
		client_id_a = Some(client_a);
		client_id_b = Some(client_b);
	}

	chain_a_wrapped.set_client_id(client_id_a.unwrap());
	chain_b_wrapped.set_client_id(client_id_b.unwrap());
	(chain_a_wrapped, chain_b_wrapped)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
#[ignore]
async fn ethereum_to_cosmos_ibc_messaging_full_integration_test() {
	logging::setup_logging();
	let asset_str = "pica".to_string();
	let asset_native_str = "ETH".to_string();
	let asset_id_a = AnyAssetId::Ethereum(asset_str.clone());
	let asset_id_native_a = AnyAssetId::Ethereum(asset_native_str.clone());
	let (mut chain_a, mut chain_b) = setup_clients().await;
	sleep(Duration::from_secs(60)).await;
	let (handle, channel_a, channel_b, connection_id_a, connection_id_b) =
		setup_connection_and_channel(&mut chain_a, &mut chain_b, Duration::from_secs(1)).await;
	handle.abort();
	let asset_id_a = AnyAssetId::Ethereum(asset_str.clone());
	// let asset_id_a = AnyAssetId::Ethereum(format!("transfer/{}/{}", channel_a,
	// asset_str.clone()));

	log::info!(target: "hyperspace", "Conn A: {connection_id_a:?} B: {connection_id_b:?}");
	log::info!(target: "hyperspace", "Chann A: {channel_a:?} B: {channel_b:?}");

	let asset_id_b = AnyAssetId::Cosmos(format!(
		"ibc/{}",
		hex::encode(&sha2_256(
			format!("{}/{channel_b}/{asset_str}", PortId::transfer()).as_bytes()
		))
		.to_uppercase()
	));

	let asset_id_native_b: AnyAssetId = AnyAssetId::Cosmos(format!(
		"ibc/{}",
		hex::encode(&sha2_256(
			format!("{}/{channel_b}/{asset_native_str}", PortId::transfer()).as_bytes()
		))
		.to_uppercase()
	));

	log::info!(target: "hyperspace", "Asset A: {asset_id_a:?} B: {asset_id_b:?}");

	// Set connections and channel whitelist
	chain_a.set_connection_id(connection_id_a);
	chain_b.set_connection_id(connection_id_b);

	chain_a.set_channel_whitelist(vec![(channel_a, PortId::transfer())].into_iter().collect());
	chain_b.set_channel_whitelist(vec![(channel_b, PortId::transfer())].into_iter().collect());

	// Run tests sequentially
	// no timeouts + connection delay
	// ibc_messaging_with_connection_delay(
	// 	&mut chain_a,
	// 	&mut chain_b,
	// 	asset_id_native_a.clone(),
	// 	asset_id_native_b.clone(),
	// 	channel_a,
	// 	channel_b,
	// )
	// .await;
	//
	// ibc_messaging_with_connection_delay(
	// 	&mut chain_a,
	// 	&mut chain_b,
	// 	asset_id_a.clone(),
	// 	asset_id_b.clone(),
	// 	channel_a,
	// 	channel_b,
	// )
	// .await;

	// timeouts + connection delay
	ibc_messaging_packet_height_timeout_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_a.clone(),
		channel_a,
		channel_b,
	)
	.await;

	// timeouts + connection delay
	// ibc_messaging_packet_height_timeout_with_connection_delay(
	// 	&mut chain_a,
	// 	&mut chain_b,
	// 	asset_id_native_a.clone(),
	// 	channel_a,
	// 	channel_b,
	// )
	// .await;

	ibc_messaging_packet_timestamp_timeout_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_a.clone(),
		channel_a,
		channel_b,
	)
	.await;

	// ibc_messaging_packet_timestamp_timeout_with_connection_delay(
	// 	&mut chain_a,
	// 	&mut chain_b,
	// 	asset_id_native_a.clone(),
	// 	channel_a,
	// 	channel_b,
	// )
	// .await;

	// channel closing semantics
	ibc_messaging_packet_timeout_on_channel_close(
		&mut chain_a,
		&mut chain_b,
		asset_id_a.clone(),
		channel_a,
	)
	.await;

	ibc_channel_close(&mut chain_a, &mut chain_b).await;

	// TODO: ethereum misbehaviour?
	// ibc_messaging_submit_misbehaviour(&mut chain_a, &mut chain_b).await;
}
/*
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
	// let asset_id_b = AnyAssetId::Ethereum("pica".to_string());
	//
	// // Run tests sequentially
	//
	// // no timeouts + connection delay
	// ibc_messaging_with_connection_delay(
	// 	&mut chain_a,
	// 	&mut chain_b,
	// 	asset_id_a.clone(),
	// 	asset_id_b.clone(),
	// 	channel_a,
	// 	channel_b,
	// )
	// .await;
	//
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
	// // channel closing semantics (doesn't work on cosmos)
	// // ibc_messaging_packet_timeout_on_channel_close(&mut chain_a, &mut chain_b,
	// asset_id_a.clone()) // 	.await;
	// // ibc_channel_close(&mut chain_a, &mut chain_b).await;
	//
	// ibc_messaging_submit_misbehaviour(&mut chain_a, &mut chain_b).await;
}

mod xx {
	use super::*;
	use ethers::prelude::{
		Address, BlockNumber, Filter, Http, Middleware, Provider, TransactionRequest, H160, U256,
	};
	use hyperspace_ethereum::{
		client::EthereumClient, config::EthereumClientConfig, ibc_provider::Ics20BankAbi,
	};
	// use hyperspace_testsuite::send_transfer_to;
	use ibc::signer::Signer;
	use log::error;
	use std::fmt::Debug;

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn devnet() -> anyhow::Result<()> {
		logging::setup_logging();

		let config_a = toml::from_str::<EthereumClientConfig>(include_str!(
			"../../../config/ethereum-goerli.toml"
		))
		.unwrap();
		let config_b = toml::from_str::<CosmosClientConfig>(include_str!(
			"../../../config/centauri-goerli.toml"
		))
		.unwrap();

		let (mut client_a, mut client_b) = (
			EthereumClient::new(config_a).await.unwrap(),
			CosmosClient::<()>::new(config_b).await.unwrap(),
		);

		{
			let diff = 1000u32;
			let sepolia_height = 4574369 - diff;
			let goerli_height = 9940653 - diff;
			let filter = Filter::new()
				.from_block(BlockNumber::Number(goerli_height.into()))
				//.address(ValueOrArray::Value(self.yui.diamond.address()))
				//.from_block(BlockNumber::Earliest)
				// .from_block(from_block)
				.to_block(BlockNumber::Latest)
				.address(client_a.yui.diamond.address())
				.event("OpenInitChannel(string,string)");

			let ankr_url = "https://rpc.ankr.com/eth_goerli/ad0182ed132fa31b17f0ef9e8fcfcc540c3508a582d1e618563b4a9040c047ca";
			let client = Provider::<Http>::try_from(ankr_url.to_string()).unwrap();

			let t0 = std::time::Instant::now();
			let logs = client.get_logs(&filter).await.unwrap();
			let t1 = std::time::Instant::now();
			log::info!("Got {} logs from ankr in {}ms", logs.len(), (t1 - t0).as_millis());

			let t0 = std::time::Instant::now();
			let logs = client_a.client().get_logs(&filter).await.unwrap();
			let t1 = std::time::Instant::now();
			log::info!("Got {} logs in {}ms", logs.len(), (t1 - t0).as_millis());

			// client_a.client().
		}
		// let id = client_a.client_id();
		// client_a.set_client_id(client_b.client_id());
		// client_b.set_client_id(id);
		let client = client_a.client();
		let asset_str = "ppica".to_string();
		let asset_id_a = AnyAssetId::Ethereum(asset_str.clone());
		let asset_id_b_atom = AnyAssetId::Cosmos("uatom".to_string());
		let asset_id_b_pica = AnyAssetId::Cosmos("ppica".to_string());
		// let channel_id = ChannelId::new(0);
		let port_id = PortId::transfer();

		let users = [
			"0xF66605eDE7BfCCc460097CAFD34B4924f1C6969D",
			"0x7C12ff36c44c1B10c13cC76ea8A3aEba0FFf6403",
			"0xD36554eF26E9B2ad72f2b53986469A8180522E5F",
		];
		let pica_amt = 10000000000000000000000u128;
		let atom_amt = 10000000000000000u128;
		// let pica_amt = 100_000000000000u128;
		// let atom_amt = 10000000000u128;
		let a = &mut AnyChain::Cosmos(client_b);
		let b = &mut AnyChain::Ethereum(client_a);
		let abi = Ics20BankAbi::new(
			Address::from_str("0x5eea0c4ed157d60bbeeec84ad25ce05357c2ff2c").unwrap(),
			client,
		);
		// dbg!(
		// 	get_balance(
		// 		&abi,
		// 		Address::from_str("0xF66605eDE7BfCCc460097CAFD34B4924f1C6969D").unwrap()
		// 	)
		// 	.await
		// );

		// while send_transfer_to(
		// 	b,
		// 	a,
		// 	AnyAssetId::Ethereum("transfer/channel-0/ppica".to_owned()),
		// 	b.channel_whitelist().iter().next().unwrap().0,
		// 	None,
		// 	Signer::from_str("centauri10556m38z4x6pqalr9rl5ytf3cff8q46nk85k9m").unwrap(),
		// 	pica_amt / 100000000,
		// )
		// .await
		// .is_err()
		// {
		// 	tokio::time::sleep(Duration::from_secs(2)).await;
		// }

		// while send_transfer_to(
		// 	a,
		// 	b,
		// 	AnyAssetId::Cosmos("ppica".to_owned()).clone(),
		// 	a.channel_whitelist().iter().next().unwrap().0,
		// 	None,
		// 	Signer::from_str("0xF66605eDE7BfCCc460097CAFD34B4924f1C6969D").unwrap(),
		// 	pica_amt / 100000000000,
		// )
		// .await
		// .map_err(|e| {
		// 	error!("{e}");
		// })
		// .is_err()
		// {
		// 	tokio::time::sleep(Duration::from_secs(2)).await;
		// }

		// dbg!(
		// 	get_balance(
		// 		&abi,
		// 		Address::from_str("0x7C12ff36c44c1B10c13cC76ea8A3aEba0FFf6403").unwrap()
		// 	)
		// 	.await
		// );

		// 70000000000000000ppica
		for user in users {
			let x = [
				(pica_amt, asset_id_b_pica.clone()),
				// (atom_amt, asset_id_b_atom.clone())
			];
			// for (amt, denom) in x {
			// 	// dbg!(user, get_balance(&abi, Address::from_str(user).unwrap()).await);
			// 	while send_transfer_to(
			// 		a,
			// 		b,
			// 		denom.clone(),
			// 		a.channel_whitelist().iter().next().unwrap().0,
			// 		None,
			// 		Signer::from_str(&user.clone()).unwrap(),
			// 		amt,
			// 	)
			// 	.await
			// 	.map_err(|e| {
			// 		error!("{e}");
			// 	})
			// 	.is_err()
			// 	{
			// 		tokio::time::sleep(Duration::from_secs(2)).await;
			// 	}
			// 	dbg!(user, get_balance(&abi, Address::from_str(user).unwrap()).await);
			// }
		}

		async fn get_balance<M>(abi: &Ics20BankAbi<M>, acc: H160) -> U256
		where
			M: Middleware + Debug + Send + Sync,
		{
			abi.method("balanceOf", (acc, "transfer/channel-3/ppica".to_string()))
				.unwrap()
				.call()
				.await
				.unwrap()
		};
		// dbg!(
		// 	get_balance(&abi,
		// Address::from_str("0xF66605eDE7BfCCc460097CAFD34B4924f1C6969D").unwrap()) 		.await
		// );

		// let tx = client_a
		// 	.client()
		// 	.get_transaction_receipt(
		// 		H256::from_str("0x0ca7e6f45de3bffeaf93995748a181b4d469b2d7936218bdcc4927fde78ce831")
		// 			.unwrap(),
		// 	)
		// 	.await
		// 	.unwrap()
		// 	.unwrap();
		// // let ev = client_a.yui.event_for_name("TransferInitiated").unwrap();
		// // ev.filter.signature()
		// dbg!(SendPacketFilter::signature());
		// dbg!(TransferInitiatedFilter::signature());
		// tx.logs.iter().for_each(|x| {
		// 	// SendPacketFilter::
		// 	// TransferInitiatedFilter::new
		// 	println!("{:?}", x);
		// });

		// client_a.send_transfer()

		// let block = client_a
		// 	.client()
		// 	.get_block(H256::from_str(
		// 		"0xe44b85448b031c68a2e3b7377b895750bed23ea21bff086360443caeb82d8e62",
		// 	)?)
		// 	.await?
		// 	.unwrap();
		// dbg!(block.transactions.len());
		//
		// let tx = client_a
		// 	.client()
		// 	.get_transaction_receipt(H256::from_str(
		// 		"0x9af4ef7c3c1c1f27d426480ee1348740023131d3eb06988a7fc62d92f173b5fc",
		// 	)?)
		// 	.await?
		// 	.unwrap();
		// tx.logs.iter().for_each(|x| {
		// 	println!("{:?}", x);
		// });
		//
		// let (height, _) = client_a.latest_height_and_timestamp().await.unwrap();
		//
		// let seqs = client_a.query_packet_commitments(height, channel_id, port_id.clone()).await?;
		// seqs.iter().for_each(|x| {
		// 	println!("{:?}", x);
		// });
		//
		// let ps = client_a
		// 	.query_send_packets(height, channel_id, port_id, vec![0, 1, 2, 3])
		// 	.await
		// 	.unwrap();
		// dbg!(ps);

		/*
		Sender account: 0x73db010c3275eb7a92e5c38770316248f4c644ee
		Diamond init address: 0x4d9654e1da9826361519be28c6db135e560f20a0
		Deployed IBCClient on 0xb7198a3674e37433579be45aa9dd09f5ab4b314a
		Deployed IBCConnection on 0xb26397cfa7e111e844086bdd3da5080f9de65cb7
		Deployed IBCChannelHandshake on 0xfbf766071d0fdee42b78ab029b97194543b6d7a5
		Deployed IBCPacket on 0x844d2447e6c00cf6a5fbe9ad5eebebe31e40368e
		Deployed IBCQuerier on 0x992966599e81b9d4a3ef92172b9fa162d2e50d5b
		Deployed DiamondCutFacet on 0x3bf46cf159422e1791d20d45683b21f34ecae4be
		Deployed DiamondLoupeFacet on 0xb16af4cfc553ae0a8f43e812e22dc6caabdf5e63
		Deployed OwnershipFacet on 0x4f6e145fbaf72be9ea283f5793e70a1c594d5ceb
		Deployed update client delegate contract address: 0xe566a7e344f2aef783319a76233e54e7f8b47823
		Deployed light client address: 0x56378f9b88f341b1913a2fc6ac2bcbaa1b9a9f9f
		Deployed Bank module address: 0x0486ee42d89d569c4d8143e47a82c4b14545ae43
		Deployed ICS-20 Transfer module address: 0x4976bb932815783f092dd0e3cca567d5502be46e
		 */

		// relay(client_a, client_b, None, None, None).await.unwrap();
		Ok(())
	}

	#[tokio::test]
	async fn send_tokens() {
		let config = toml::from_str::<EthereumClientConfig>(
			&std::fs::read_to_string("../../config/ethereum-testnet.toml").unwrap(),
		)
		.unwrap();
		let mut client = EthereumClient::new(config).await.unwrap();
		let abi = Ics20BankAbi::new(
			Address::from_str("0x0486ee42d89d569c4d8143e47a82c4b14545ae43").unwrap(),
			client.client(),
		);
		let from = Address::from_str("0xF66605eDE7BfCCc460097CAFD34B4924f1C6969D").unwrap();
		let to = Address::from_str("0x5c1c17fBe28B4c2a2b67048cCe256B83FC65e181").unwrap();

		// async fn get_balance<M>(abi: &Ics20BankAbi<M>, acc: H160) -> U256
		// where
		// 	M: Middleware + Debug + Send + Sync,
		// {
		// 	abi.method("balanceOf", (acc, "pica".to_string()))
		// 		.unwrap()
		// 		.call()
		// 		.await
		// 		.unwrap()
		// };
		// dbg!(get_balance(&abi, from).await);
		// dbg!(get_balance(&abi, to).await);

		dbg!(abi.client().get_balance(from, None).await.unwrap());
		dbg!(abi.client().get_balance(to, None).await.unwrap());
		let tx = TransactionRequest::new().to(to).value(100000000000000000u64).from(from);
		let tx = abi.client().send_transaction(tx, None).await.unwrap().await.unwrap().unwrap();
		// let tx = abi
		// 	.method::<_, ()>("transferFrom", (from, to, "pica".to_string(), U256::from(10000000u32)))
		// 	.unwrap()
		// 	.send()
		// 	.await
		// 	.unwrap()
		// 	.await
		// 	.unwrap()
		// 	.unwrap();
		assert_eq!(tx.status, Some(1u32.into()));

		dbg!(tx.transaction_hash);

		// dbg!(get_balance(&abi, from).await);
		// dbg!(get_balance(&abi, to).await);
	}
}
 */
