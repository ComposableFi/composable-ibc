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

use crate::utils::{BEACON_NODE_PORT, ETH_NODE_PORT_WS};
use core::time::Duration;
use ethers::{
	abi::{ethabi, Bytes, ParamType, StateMutability, Token},
	middleware::SignerMiddleware,
	prelude::{
		coins_bip39::{English, Mnemonic},
		transaction::eip2718::TypedTransaction,
		ContractInstance, Http, JsonRpcClient, LocalWallet, Middleware, MnemonicBuilder, Provider,
		Signer, H256,
	},
	types::{Address, BlockNumber, TransactionRequest, U256},
	utils::{keccak256, AnvilInstance},
};
use ethers_solc::ProjectCompileOutput;
use futures::StreamExt;
use hyperspace_core::{
	chain::{AnyAssetId, AnyChain, AnyConfig},
	logging,
};
use hyperspace_cosmos::{client::{CosmosClient, CosmosClientConfig}, eth_zk_utils::CreateProofInput};
use hyperspace_cosmos::eth_zk_utils::ZKProver;
use hyperspace_ethereum::{
	chain::client_state_from_abi_token, client::{ClientError, EthereumClient, COMMITMENTS_STORAGE_INDEX}, config::{ContractName, ContractName::ICS20Bank}, ibc_provider, ibc_provider::PublicKeyData, mock::{
		utils,
		utils::{hyperspace_ethereum_client_fixture, ETH_NODE_PORT, USE_GETH},
	}, utils::{check_code_size, deploy_contract, send_retrying, DeployYuiIbc, ProviderImpl}
};
use hyperspace_primitives::{utils::create_clients, Chain, CommonClientConfig, IbcProvider};
use hyperspace_testsuite::{
	assert_send_transfer, ibc_channel_close,
	ibc_messaging_packet_height_timeout_with_connection_delay,
	ibc_messaging_packet_height_timeout_with_connection_delay_native,
	ibc_messaging_packet_timeout_on_channel_close,
	ibc_messaging_packet_timestamp_timeout_with_connection_delay,
	ibc_messaging_with_connection_delay, send_transfer, setup_connection_and_channel,
};
use ibc::core::{ics02_client::client_state::ClientState, ics24_host::identifier::PortId};
use itertools::Itertools;
use log::{info, warn};
use pallet_ibc::light_clients::AnyClientState;
use sp_core::hashing::sha2_256;
use sp_runtime::print;
use std::{
	collections::{HashMap, HashSet},
	future::Future,
	path::PathBuf,
	str::FromStr,
	sync::{Arc, Mutex},
};
use tendermint::{validator::Set, PublicKey};
use tokio::{task::JoinHandle, time::sleep};

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
	let (anvil, client) = utils::spawn_anvil().await;
	warn!("endpoint: {}, chain id: {}", anvil.endpoint(), anvil.chain_id());

	let mut yui_ibc = hyperspace_ethereum::utils::deploy_ibc(&path, client.clone()).await.unwrap();
	let project_output = hyperspace_ethereum::utils::compile_yui(&path, "contracts/core");

	let (tendermint_diamond, tendermint_facets) = hyperspace_ethereum::utils::deploy_client(
		&path,
		yui_ibc.clone(),
		"07-tendermint".to_string(),
		"TendermintLightClientZK",
		client.clone(),
	)
	.await
	.unwrap();

	yui_ibc.tendermint_diamond = Some(tendermint_diamond.clone());
	yui_ibc.tendermint_facets = tendermint_facets;

	DeployYuiIbcTendermintClient {
		path,
		project_output,
		anvil,
		client,
		yui_ibc,
		tendermint_client: tendermint_diamond,
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
			(
				Token::String("ETH".to_string()),
				Token::Address(deploy.yui_ibc.gov_proxy.as_ref().unwrap().address()),
			),
			deploy.client.clone(),
		)
		.await;
		info!("Bank module address: {:?}", ics20_bank_contract.address());
		let constructor_args = (
			Token::Address(deploy.yui_ibc.ibc_core_diamond.address()),
			Token::Address(ics20_bank_contract.address()),
		);
		let ics20_bank_transfer_contract = deploy_contract(
			"ICS20TransferBank",
			&[&project_output],
			constructor_args,
			deploy.client.clone(),
		)
		.await;
		let method = ics20_bank_contract
			.method::<_, ()>(
				"transferRole",
				(keccak256("OWNER_ROLE"), ics20_bank_transfer_contract.address()),
			)
			.unwrap();
		send_retrying(&method).await.unwrap();
		(ics20_bank_contract, ics20_bank_transfer_contract)
	}
}

async fn get_current_validator_set(cosmos_client: &CosmosClient<()>, client_b: &impl Chain) -> Set {
	let (height, _) = client_b.latest_height_and_timestamp().await.unwrap();
	let client_state_response =
		client_b.query_client_state(height, cosmos_client.client_id()).await.unwrap();
	let client_state = client_state_response
		.client_state
		.map(AnyClientState::try_from)
		.unwrap()
		.unwrap();

	let height = client_state.latest_height().revision_height as u32;
	let header = cosmos_client
		.msg_update_client_header(height.into(), height.into(), client_state.latest_height())
		.await
		.unwrap()
		.pop()
		.unwrap()
		.0;
	header.validator_set
}

async fn test_call(
	contract_name: ContractName,
	function_name: &str,
	function_params: &[ParamType],
) -> Bytes {
	let abi = contract_name.to_abi();
	let functions = abi.functions_by_name(function_name).unwrap();
	assert_eq!(functions.len(), 1, "Expected one function with this name");
	let function = functions.first().unwrap();
	let mut args = function_params.into_iter().map(default_token).collect::<Vec<_>>();
	if contract_name == ICS20Bank && function_name == "transferFrom" {
		*(&mut args[1]) = Token::Address(
			Address::from_str("0x7C12ff36c44c1B10c13cC76ea8A3aEba0FFf6403").unwrap(),
		);
	}
	function.encode_input(&args).unwrap()
}

async fn owner_test_tx(
	eth_client: &EthereumClient,
	cosmos_client: &CosmosClient<()>,
	calldata: Bytes,
	diamond_address: Address,
) -> TypedTransaction {
	use crate::ibc_provider::LibGovernanceEcdsaSignature;
	use ethers::abi::Tokenizable;
	use hyperspace_ethereum::ibc_provider::SimpleValidatorData;

	let validator_set = get_current_validator_set(cosmos_client, eth_client).await;
	let validators = validator_set
		.validators()
		.into_iter()
		.map(|x| {
			let pub_key = match x.pub_key {
				PublicKey::Ed25519(pub_key) => PublicKeyData {
					ed_25519: pub_key.as_bytes().to_vec().into(),
					secp_25_6k_1: Default::default(),
					sr_25519: Default::default(),
				},
				PublicKey::Secp256k1(pub_key) => PublicKeyData {
					ed_25519: Default::default(),
					secp_25_6k_1: pub_key.to_bytes().to_vec().into(),
					sr_25519: Default::default(),
				},
				_ => panic!("Unsupported public key type"),
			};

			SimpleValidatorData { pub_key, voting_power: x.power.into() }
		})
		.collect::<Vec<_>>();
	let data =
		[calldata.as_slice(), 0u8.to_le_bytes().as_slice(), diamond_address.as_bytes()].concat();
	let keccak257 = keccak256(data.clone());
	let signature = eth_client.client().signer().sign_message(keccak257).await.unwrap();
	let r: [u8; 32] = signature.r.into();
	let s: [u8; 32] = signature.s.into();

	let mut method = eth_client
		.yui
		.method::<_, (bool, Vec<u8>)>(
			"execute",
			(
				Token::Bytes(calldata),
				Token::Uint(U256::zero()),
				validators,
				Token::Bytes(Bytes::new()),
				vec![LibGovernanceEcdsaSignature { r, s, v: signature.v as _ }],
			),
		)
		.unwrap();
	method.tx.set_to(diamond_address);
	method.tx
}

async fn test_tx(
	client: &EthereumClient,
	contract_name: ContractName,
	function_name: &str,
	function_params: &[ParamType],
) -> TypedTransaction {
	let data = test_call(contract_name, function_name, function_params).await;
	let mut method = client.yui.method::<_, ()>("addRelayer", Address::zero()).unwrap();
	method.tx.set_data(data.to_vec().into());
	method.tx
}

async fn setup_clients() -> (AnyChain, AnyChain, JoinHandle<()>) {
	info!(target: "hyperspace", "=========================== Starting Test ===========================");
	let args = Args::default();

	// Create client configurations
	let config_a = if USE_CONFIG {
		toml::from_str(include_str!("../../../config/ethereum-local.toml")).unwrap()
	} else {
		let deploy = deploy_yui_ibc_and_tendermint_client_fixture().await;
		let (ibc_transfer_diamond, ibc_transfer_facets, bank_diamond, bank_facets) =
			hyperspace_ethereum::utils::deploy_transfer_module(
				&deploy.path,
				deploy.yui_ibc.clone(),
				deploy.yui_ibc.ibc_core_diamond.address(),
				deploy.client.clone(),
				1,
			)
			.await
			.unwrap();
		let DeployYuiIbcTendermintClient {
			anvil,
			tendermint_client,
			ics20_module: _,
			mut yui_ibc,
			..
		} = deploy;
		let tendermint_address = yui_ibc.tendermint_diamond.as_ref().map(|x| x.address()).unwrap();

		info!(target: "hyperspace", "Deployed diamond: {:?}, tendermint client: {:?}, bank: {:?}", yui_ibc.ibc_core_diamond.address(), tendermint_client.address(), bank_diamond.address());
		info!(target: "hyperspace", "Using addr = {:?}", deploy.client.address());
		yui_ibc.ibc_transfer_diamond = Some(ibc_transfer_diamond);
		yui_ibc.ibc_transfer_facets = ibc_transfer_facets;
		yui_ibc.bank_diamond = Some(bank_diamond);
		yui_ibc.bank_facets = bank_facets;
		//Vlad suggested to comment for local testing
		// yui_ibc = hyperspace_ethereum::utils::deploy_governance(
		// 	&deploy.path,
		// 	deploy.client.clone(),
		// 	yui_ibc.clone(),
		// 	&[deploy.client.address()],
		// )
		// .await
		// .unwrap();

		let tendermint_facets = yui_ibc.tendermint_facets.clone();

		//replace the tendermint client address in hyperspace config with a real one
		let mut config_a = hyperspace_ethereum_client_fixture(
			&anvil,
			yui_ibc,
			"pg://postgres:password@localhost/postgres",
			"redis://localhost:6379",
		)
		.await;
		config_a.tendermint_diamond_address = Some(tendermint_address);
		config_a.tendermint_facets = tendermint_facets
			.into_iter()
			.map(|contract| (contract.abi_name(), contract.contract().address()))
			.collect();
		if !USE_GETH {
			config_a.ws_rpc_url = "ws://localhost:8545/".parse().unwrap(); //anvil.ws_endpoint().parse().unwrap();
			config_a.http_rpc_url = "http://localhost:8545/".parse().unwrap(); //anvil.ws_endpoint().parse().unwrap();
			config_a.anvil = Some(Arc::new(Mutex::new(anvil)));
		}

		if SAVE_TO_CONFIG {
			let config_path = PathBuf::from_str("../../config/ethereum-local.toml").unwrap();
			let config_a_str = toml::to_string_pretty(&config_a).unwrap();
			std::fs::write(config_path, config_a_str).unwrap();
		}
		config_a
	};

	let url = config_a.ws_rpc_url.to_string();
	let _h = tokio::spawn(async move {
		let mut p = Provider::connect(url).await.unwrap();
		loop {
			tokio::time::sleep(Duration::from_secs(5)).await;
			let res = p.request::<_, BlockNumber>("evm_mine", ()).await;
			info!(target: "hyperspace", "Mined: {res:?}");
		}
	});

	let db_url = config_a.indexer_pg_url.clone();
	let redis_url = config_a.indexer_redis_url.clone();
	let indexer_handle = tokio::spawn(async move {
		
		indexer::run_indexer(db_url, redis_url, config_a.ibc_core_diamond_address).await;
	});

	let mut config_b = CosmosClientConfig {
		name: "centauri".to_string(),
		rpc_url: args.chain_b.clone().parse().unwrap(),
		grpc_url: args.cosmos_grpc.clone().parse().unwrap(),
		websocket_url: args.cosmos_ws.clone().parse().unwrap(),
		chain_id: "centauri-testnet-1".to_string(),
		client_id: None,
		connection_id: None,
		account_prefix: "centauri".to_string(),
		fee_denom: "pica".to_string(),
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
			client_update_interval_sec: 10,
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
	(chain_a_wrapped, chain_b_wrapped, indexer_handle)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
#[ignore]
async fn zk_prover_bitmask() {
	//bitmask indicates which validators were included into voting power calculation

	//0 was not included into voting power calculation
	//1 was included into voting power calculation
	let expected_validators = vec![0, 1, 1, 0, 1, 1, 0]; //all validators
	let size = expected_validators.len();
    let mut bitmask: u64 = 0;

    for (index, &validator) in expected_validators.iter().enumerate() {
        if validator == 1 {
            bitmask |= 1 << index;
        }
    }

    println!("Bitmask: {}", bitmask);
    println!("Bitmask: {:064b}", bitmask);
	//bitmask will be sent to eth side together with zk proof


	// let bitmask: u64 = 0b00000000000001100110; 
    let mut actual_validators = vec![0; size]; // Assuming 7-bit bitmask
    
    for i in 0..size {
        if (bitmask >> i) & 1 == 1 {
            actual_validators[i] = 1;
        }
    }

    println!("Validators: {:?}", actual_validators);
	assert_eq!(actual_validators, expected_validators);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
#[ignore]
async fn zk_prover_integration_test() {
	//branch rustninja/compatibility
	let zk_prover = ZKProver::new("http://127.0.0.1:8000".to_string(), Duration::from_secs(60));
	let proof_input = CreateProofInput {
		signatures: vec![],
		msgs: vec![],
		public_keys: vec![]
	};
	let status = zk_prover.status().unwrap();
	println!("status: {:?}", status);
	let resp = zk_prover.create_proof(proof_input).unwrap();
	println!("resp: {:?}", resp);

	let proof = zk_prover.poll_proof(&resp.proof_id).unwrap();
	assert!(proof.is_none());
	std::thread::sleep(Duration::from_secs(33));
	let proof = zk_prover.poll_proof(&resp.proof_id).unwrap();
	assert!(proof.is_some());
	println!("proof: {:?}", proof.unwrap());
	return;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
#[ignore]
async fn decode_yui_error(){
	let error = "08c379a0000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000296865616465722063616e2774206265206578706972656463656e74617572692d746573746e65742d310000000000000000000000000000000000000000000000";
	let error = hex::decode(error).unwrap();
	let s = String::from_utf8_lossy(&error);
	println!("s: {:?}", s);
	// let error = ethabi::decode(&[ParamType::String], &error).unwrap();
	// println!("error: {:?}", error);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
#[ignore]
async fn ethereum_to_cosmos_ibc_messaging_full_integration_test() {
	std::env::set_var("KEY_PASS", "mybullishpassword ^_^");
	// std::env::set_var("RUST_LOG", "hyperspace_cosmos=error,hyperspace_ethereum=error,hyperspace=error,prover=error");
	std::env::set_var("RUST_LOG", "hyperspace_cosmos=debug,hyperspace_ethereum=debug,hyperspace=trace,prover=debug");
	// std::env::set_var("RUST_LOG", "hyperspace_cosmos=trace,hyperspace_ethereum=trace,hyperspace=trace,prover=debug,info");
	logging::setup_logging();
	

	let asset_str = "pica".to_string();
	let asset_native_str = "ETH".to_string();
	let asset_id_a = AnyAssetId::Ethereum("TST".to_string());
	let asset_id_native_a = AnyAssetId::Ethereum(asset_native_str.clone());
	let (mut chain_a, mut chain_b, _indexer_handle) = setup_clients().await;
	sleep(Duration::from_secs(12)).await;
	if !USE_GETH {
		let a = chain_a.clone();
		let _h = tokio::spawn(async move {
			let AnyChain::Ethereum(eth) = &a else { unreachable!() };

			let mut p = Provider::connect(eth.config.ws_rpc_url.to_string()).await.unwrap();
			loop {
				tokio::time::sleep(Duration::from_secs(5)).await;
				let res = p.request::<_, BlockNumber>("evm_mine", ()).await;
				info!(target: "hyperspace", "Mined: {res:?}");
			}
		});
	}

	let AnyChain::Ethereum(eth) = &chain_a else { unreachable!() };
	let addr = eth
		.yui
		.method_diamond::<_, Address>(
			"queryTokenContractFromDenom",
			"TST".to_string(),
			ContractName::BankDiamond,
		)
		.unwrap()
		.call()
		.await
		.unwrap();

	if addr == Address::zero() {
		let client = eth.client().clone();
		let path = utils::yui_ibc_solidity_path();
		let project_output =
			hyperspace_ethereum::utils::compile_yui(&path, "contracts/apps/20-transfer");
		let erc20_token = deploy_contract(
			"ERC20Token",
			&[&project_output],
			("Test Token".to_string(), "TST".to_string(), 100u32, 0u8),
			client,
		)
		.await;

		// let method = eth
		// 	.yui
		// 	.method_diamond::<_, ()>(
		// 		"whitelistToken",
		// 		(erc20_token.address(), "TST".to_string()),
		// 		ContractName::BankDiamond,
		// 	)
		// 	.unwrap();
		// send_retrying(&method).await.unwrap();
	}

	let (handle, channel_a, channel_b, connection_id_a, connection_id_b) =
		setup_connection_and_channel(&mut chain_a, &mut chain_b, Duration::from_secs(1)).await;

	let asset_id_b_on_a =
		AnyAssetId::Ethereum(format!("transfer/{}/{}", channel_a, asset_str.clone()));
	let asset_id_b = AnyAssetId::Cosmos(asset_str.to_string());

	log::info!(target: "hyperspace", "Conn A: {connection_id_a:?} B: {connection_id_b:?}");
	log::info!(target: "hyperspace", "Chann A: {channel_a:?} B: {channel_b:?}");

	let asset_id_native_b: AnyAssetId = AnyAssetId::Cosmos(format!(
		"ibc/{}",
		hex::encode(&sha2_256(
			format!("{}/{channel_b}/{asset_native_str}", PortId::transfer()).as_bytes()
		))
		.to_uppercase()
	));
	let asset_id_a_on_b: AnyAssetId = AnyAssetId::Cosmos(format!(
		"ibc/{}",
		hex::encode(&sha2_256(format!("{}/{channel_b}/TST", PortId::transfer()).as_bytes()))
			.to_uppercase()
	));

	log::info!(target: "hyperspace", "Asset A: {asset_id_a:?} B: {asset_id_b:?}");

	// Set connections and channel whitelist
	chain_a.set_connection_id(connection_id_a);
	chain_b.set_connection_id(connection_id_b);

	chain_a.set_channel_whitelist(vec![(channel_a, PortId::transfer())].into_iter().collect());
	chain_b.set_channel_whitelist(vec![(channel_b, PortId::transfer())].into_iter().collect());

	let (previous_balance, _) =
		send_transfer(&chain_b, &chain_a, asset_id_b.clone(), channel_b, None).await;
	assert_send_transfer(&chain_b, asset_id_b.clone(), previous_balance, 4800).await;
	handle.abort();

	// Run tests sequentially
	// no timeouts + connection delay
	ibc_messaging_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_native_a.clone(),
		asset_id_native_b.clone(),
		channel_a,
		channel_b,
	)
	.await;

	ibc_messaging_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_a.clone(),
		asset_id_a_on_b.clone(),
		channel_a,
		channel_b,
	)
	.await;

	ibc_messaging_with_connection_delay(
		&mut chain_b,
		&mut chain_a,
		asset_id_b.clone(),
		asset_id_b_on_a.clone(),
		channel_b,
		channel_a,
	)
	.await;

	// timeouts + connection delay
	ibc_messaging_packet_height_timeout_with_connection_delay(
		&mut chain_b,
		&mut chain_a,
		asset_id_b.clone(),
		channel_b,
		channel_a,
	)
	.await;

	// timeouts + connection delay
	ibc_messaging_packet_height_timeout_with_connection_delay_native(
		&mut chain_a,
		&mut chain_b,
		asset_id_native_a.clone(),
		channel_a,
		channel_b,
	)
	.await;

	ibc_messaging_packet_height_timeout_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_b_on_a.clone(),
		channel_a,
		channel_b,
	)
	.await;

	ibc_messaging_packet_height_timeout_with_connection_delay(
		&mut chain_a,
		&mut chain_b,
		asset_id_a.clone(),
		channel_a,
		channel_b,
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
		asset_id_native_a.clone(),
		channel_a,
		channel_b,
	)
	.await;

	// channel closing semantics
	ibc_messaging_packet_timeout_on_channel_close(
		&mut chain_a,
		&mut chain_b,
		asset_id_native_a.clone(),
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

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
#[ignore]
async fn ethereum_to_cosmos_governance_and_filters_test() {
	logging::setup_logging();
	let (chain_a, chain_b, indexer_handle) = setup_clients().await;
	sleep(Duration::from_secs(12)).await;
	indexer_handle.abort();
	use ibc_provider::{
		CALLBATCHFACETABI_ABI, DIAMONDABI_ABI, DIAMONDCUTFACETABI_ABI, DIAMONDLOUPEFACETABI_ABI,
		ERC20TOKENABI_ABI, GOVERNANCEFACETABI_ABI, IBCCHANNELABI_ABI, IBCCLIENTABI_ABI,
		IBCCONNECTIONABI_ABI, IBCPACKETABI_ABI, IBCQUERIERABI_ABI, ICS20BANKABI_ABI,
		ICS20TRANSFERBANKABI_ABI, OWNERSHIPFACETABI_ABI, RELAYERWHITELISTFACETABI_ABI,
		TENDERMINTCLIENTABI_ABI,
	};
	use CallableBy::*;
	use ContractName::*;

	let all_abis = [
		(IbcCoreDiamond, &DIAMONDABI_ABI),
		(DiamondCutFacet, &DIAMONDCUTFACETABI_ABI),
		(DiamondLoupeFacet, &DIAMONDLOUPEFACETABI_ABI),
		(ERC20Token, &ERC20TOKENABI_ABI),
		(GovernanceFacet, &GOVERNANCEFACETABI_ABI),
		(IBCChannelHandshake, &IBCCHANNELABI_ABI),
		(IBCClient, &IBCCLIENTABI_ABI),
		(IBCConnection, &IBCCONNECTIONABI_ABI),
		(IBCPacket, &IBCPACKETABI_ABI),
		(IBCQuerier, &IBCQUERIERABI_ABI),
		(ICS20Bank, &ICS20BANKABI_ABI),
		(ICS20TransferBank, &ICS20TRANSFERBANKABI_ABI),
		(OwnershipFacet, &OWNERSHIPFACETABI_ABI),
		(RelayerWhitelistFacet, &RELAYERWHITELISTFACETABI_ABI),
		(TendermintLightClientZK, &TENDERMINTCLIENTABI_ABI),
		(CallBatchFacet, &CALLBATCHFACETABI_ABI),
	];

	#[derive(Copy, Clone, Debug)]
	#[allow(dead_code)]
	enum CallableBy {
		Anyone,
		Relayer,
		Owner,
		Ibc,
		Module,
		Undefined,
	}

	let functions = [
		(CallBatchFacet, "callBatch", Anyone),
		(DiamondCutFacet, "diamondCut", Owner),
		(ERC20Token, "approve", Anyone),
		(ERC20Token, "burn", Module),
		(ERC20Token, "decreaseAllowance", Anyone),
		(ERC20Token, "increaseAllowance", Anyone),
		(ERC20Token, "mint", Module),
		(ERC20Token, "transfer", Anyone),
		(ERC20Token, "transferFrom", Anyone),
		(ERC20Token, "setDecimals", Module),
		(ERC20Token, "setName", Module),
		(ERC20Token, "setSymbol", Module),
		(ERC20Token, "renounceOwnership", Module),
		(ERC20Token, "transferOwnership", Module),
		(ERC20Token, "updateOwnership", Module),
		(GovernanceFacet, "execute", Anyone),
		(GovernanceFacet, "setTendermintClient", Owner),
		(GovernanceFacet, "setProxy", Owner),
		(GovernanceFacet, "addSignatory", Owner),
		(GovernanceFacet, "removeSignatory", Owner),
		(IBCChannelHandshake, "bindPort", Owner),
		(IBCChannelHandshake, "unbindPort", Owner),
		(IBCChannelHandshake, "setExpectedBlockTime", Owner),
		(IBCChannelHandshake, "channelCloseConfirm", Relayer),
		(IBCChannelHandshake, "channelCloseInit", Relayer),
		(IBCChannelHandshake, "channelOpenAck", Relayer),
		(IBCChannelHandshake, "channelOpenConfirm", Relayer),
		(IBCChannelHandshake, "channelOpenInit", Relayer),
		(IBCChannelHandshake, "channelOpenTry", Relayer),
		(IBCClient, "registerClient", Owner),
		(IBCClient, "createClient", Relayer),
		(IBCClient, "updateClient", Relayer),
		(IBCConnection, "connectionOpenAck", Relayer),
		(IBCConnection, "connectionOpenConfirm", Relayer),
		(IBCConnection, "connectionOpenInit", Relayer),
		(IBCConnection, "connectionOpenTry", Relayer),
		(IBCPacket, "acknowledgePacket", Relayer),
		(IBCPacket, "recvPacket", Relayer),
		(IBCPacket, "sendPacket", Module),
		(IBCPacket, "timeoutOnClose", Relayer),
		(IBCPacket, "timeoutPacket", Relayer),
		(IBCPacket, "writeAcknowledgement", Module),
		(IBCPacket, "setPaused", Relayer),
		(ICS20Bank, "init", Anyone),
		(ICS20Bank, "burn", Module),
		(ICS20Bank, "mint", Module),
		(ICS20Bank, "transfer", Module),
		(ICS20Bank, "transferFrom", Module), // Also Anyone, if `from` == sender
		(ICS20Bank, "renounceRole", Module),
		(ICS20Bank, "transferRole", Module),
		(ICS20Bank, "tokenUpdateOwnership", Owner),
		(ICS20Bank, "tokenSetDecimals", Owner),
		(ICS20Bank, "tokenSetName", Owner),
		(ICS20Bank, "tokenSetSymbol", Owner),
		(ICS20Bank, "whitelistToken", Owner),
		(ICS20Bank, "unwhitelistToken", Owner),
		(ICS20Bank, "unwhitelistToken", Owner),
		(ICS20TransferBank, "init", Anyone),
		(ICS20TransferBank, "onAcknowledgementPacket", Ibc),
		(ICS20TransferBank, "onChanCloseConfirm", Ibc),
		(ICS20TransferBank, "onChanCloseInit", Ibc),
		(ICS20TransferBank, "onChanOpenAck", Ibc),
		(ICS20TransferBank, "onChanOpenConfirm", Ibc),
		(ICS20TransferBank, "onChanOpenInit", Ibc),
		(ICS20TransferBank, "onChanOpenTry", Ibc),
		(ICS20TransferBank, "onRecvPacket", Ibc),
		(ICS20TransferBank, "onTimeoutPacket", Ibc),
		(ICS20TransferBank, "sendTransfer", Anyone),
		(ICS20TransferBank, "sendTransferNativeToken", Anyone),
		(OwnershipFacet, "transferOwnership", Owner),
		(RelayerWhitelistFacet, "addRelayer", Owner),
		(RelayerWhitelistFacet, "removeRelayer", Owner),
		(TendermintLightClientZK, "init", Anyone),
		(TendermintLightClientZK, "createClient", Ibc),
		(TendermintLightClientZK, "updateClient", Ibc),
	]
	.into_iter()
	.map(|(name, f_name, mode)| ((name, f_name), mode))
	.collect::<HashMap<_, _>>();

	let AnyChain::Ethereum(eth_client) = chain_a else { unreachable!() };
	let AnyChain::Wasm(wasm_chain) = chain_b else { unreachable!() };
	let AnyChain::Cosmos(cosmos_client) = *wasm_chain.inner else { unreachable!() };

	let relayer_client = eth_client.client();
	let user_client = {
		let client = Provider::<Http>::try_from(eth_client.config.http_rpc_url.to_string())
			.map_err(|_| ClientError::UriParseError(eth_client.config.http_rpc_url.clone()))
			.unwrap();
		let chain_id = client.get_chainid().await.unwrap();
		let mnemonic = Mnemonic::<English>::new(&mut rand::thread_rng());
		let wallet: LocalWallet = MnemonicBuilder::<English>::default()
			.phrase(mnemonic.to_phrase().as_str())
			.build()
			.unwrap();
		Arc::new(SignerMiddleware::new(client, wallet.with_chain_id(chain_id.as_u64())))
	};
	let fake_owner_client = {
		let client = Provider::<Http>::try_from(eth_client.config.http_rpc_url.to_string())
			.map_err(|_| ClientError::UriParseError(eth_client.config.http_rpc_url.clone()))
			.unwrap();
		let chain_id = client.get_chainid().await.unwrap();
		let mnemonic = Mnemonic::<English>::new(&mut rand::thread_rng());
		let wallet: LocalWallet = MnemonicBuilder::<English>::default()
			.phrase(mnemonic.to_phrase().as_str())
			.build()
			.unwrap();
		Arc::new(SignerMiddleware::new(client, wallet.with_chain_id(chain_id.as_u64())))
	};
	let transaction = TypedTransaction::Legacy(TransactionRequest {
		to: Some(fake_owner_client.address().into()),
		value: Some(100_000_000_000_000_000_000_u128.into()),
		..Default::default()
	});
	let _receipt = relayer_client
		.send_transaction(transaction, None)
		.await
		.unwrap()
		.await
		.unwrap()
		.unwrap();

	let path = utils::yui_ibc_solidity_path();
	let project_output =
		hyperspace_ethereum::utils::compile_yui(&path, "contracts/apps/20-transfer");
	let erc20_token = deploy_contract(
		"ERC20Token",
		&[&project_output],
		("Test Token".to_string(), "TEST".to_string(), 100u32, 0u8),
		fake_owner_client.clone(),
	)
	.await;

	let mut fns_to_check = functions.iter().map(|((x, y), _)| (*x, *y)).collect::<HashSet<_>>();
	for (name, abi) in all_abis {
		info!("{}:", name);
		for (f_name, fs) in &abi.functions {
			for f in fs {
				if f.state_mutability == StateMutability::NonPayable ||
					f.state_mutability == StateMutability::Payable
				{
					let mode = *functions
						.get(&(name, &f_name))
						.expect(format!("not defined {}:{}", name, f_name).as_str());
					info!(
						"\t{f_name} [{mode:?}]: {}",
						f.inputs.iter().map(|x| format!("{}:{}", x.name, x.kind)).join(", ")
					);
					assert!(fns_to_check.remove(&(name, f_name)), "duplicate function");
					let params = f.inputs.iter().map(|x| x.kind.clone()).collect::<Vec<_>>();
					let mut tx = test_tx(&eth_client, name, f_name, &params).await;
					match name {
						ERC20Token => {
							tx.set_to(erc20_token.address());
						},
						ICS20Bank => {
							tx.set_to(eth_client.yui.bank_diamond.as_ref().unwrap().address());
						},
						ICS20TransferBank => {
							tx.set_to(
								eth_client.yui.ibc_transfer_diamond.as_ref().unwrap().address(),
							);
						},
						TendermintLightClientZK => {
							tx.set_to(
								eth_client.yui.tendermint_diamond.as_ref().unwrap().address(),
							);
						},
						_ => (),
					}
					let owner_tx = owner_test_tx(
						&eth_client,
						&cosmos_client,
						test_call(name, f_name, &params).await,
						*tx.to().unwrap().as_address().unwrap(),
					)
					.await;

					warn!("Checking {}:{}", name, f_name);

					let not_contract_owner_err = "0xff4127cb";
					let not_contract_owner_err2 = "caller is not the owner";
					let not_contract_owner_err3 = "caller is not owner";
					let not_whitelisted_err = "Relayer not whitelisted";
					let no_capability_err = "NoCapability";
					let unauthorized_err = "unauthorized";
					let not_ibc_err = "caller is not the IBC contract";
					let not_enough_signatories = "not enough signatories";
					let invalid_additional_sigs = "Invalid additional signatures";
					match mode {
						Anyone => {
							let result = user_client.call(&tx, None).await;
							if let Err(e) = result {
								let string = e.to_string();
								info!("{string}");
								assert!(!string.contains(not_whitelisted_err));
							}
						},
						Relayer => {
							let result = relayer_client.call(&tx, None).await;
							if let Err(e) = result {
								let string = e.to_string();
								info!("{string}");
								assert!(!string.contains(not_whitelisted_err));
							}
							let err = user_client.call(&tx, None).await.unwrap_err().to_string();
							info!("{err}");
							assert!(err.contains(not_whitelisted_err));
						},
						Owner => {
							let err = relayer_client.call(&tx, None).await.unwrap_err().to_string();
							info!("{err}");
							assert!(
								err.contains(not_contract_owner_err) ||
									err.contains(not_contract_owner_err2) ||
									err.contains(not_contract_owner_err3)
							);
							let err = user_client.call(&tx, None).await.unwrap_err().to_string();
							info!("{err}");
							assert!(
								err.contains(not_contract_owner_err) ||
									err.contains(not_contract_owner_err2) ||
									err.contains(not_contract_owner_err3)
							);
							if name == ERC20Token {
								warn!("Skipping governance call to {}:{}", name, f_name);
								continue;
							}
							let err = get_error_from_call(user_client.call(&owner_tx, None).await);
							info!("{err}");
							assert!(
								!err.contains(not_whitelisted_err) &&
									!err.contains(not_contract_owner_err) &&
									!err.contains(not_contract_owner_err2) &&
									!err.contains(not_contract_owner_err3) &&
									!err.contains(not_enough_signatories) &&
									!err.contains(invalid_additional_sigs) &&
									!err.contains("message already executed") &&
									!err.contains("validators hash mismatch")
							);
						},
						Undefined | Module | Ibc => {
							warn!("{}:{} not defined", name, f_name);
							// At least it shouldn't be callable by basic users:
							let err = get_error_from_call(user_client.call(&tx, None).await);
							info!("{err}");
							assert!(
								err.contains(not_whitelisted_err) ||
									err.contains(not_contract_owner_err) || err
									.contains(not_contract_owner_err2) || err
									.contains(not_contract_owner_err3) || err
									.contains(no_capability_err) || err.contains(unauthorized_err) ||
									err.contains(not_ibc_err)
							);
						},
					}
				}
			}
		}
	}
	if !fns_to_check.is_empty() {
		panic!("not all functions checked: {:?}", fns_to_check);
	}
}

fn default_token(param: &ParamType) -> Token {
	match param {
		ParamType::Address => Token::Address(Address::zero()),
		ParamType::Bytes => Token::Bytes(vec![]),
		ParamType::Int(_) => Token::Int(0.into()),
		ParamType::Uint(_) => Token::Uint(0.into()),
		ParamType::Bool => Token::Bool(false),
		ParamType::String => Token::String("".to_string()),
		ParamType::Array(_) => Token::Array(vec![]),
		ParamType::FixedBytes(n) => Token::FixedBytes(vec![0u8; *n]),
		ParamType::FixedArray(p, n) => Token::FixedArray(vec![default_token(&*p); *n]),
		ParamType::Tuple(ps) => Token::Tuple(ps.into_iter().map(|p| default_token(p)).collect()),
	}
}

fn get_error_from_call<E: ToString>(result: Result<ethers::types::Bytes, E>) -> String {
	match result {
		Ok(bytes) => format!(
			"{}, (0x{})",
			String::from_utf8_lossy(bytes.as_ref()).to_string(),
			hex::encode(bytes.as_ref())
		),
		Err(e) => e.to_string(),
	}
}

mod indexer {
	use ethers::types::Address;
	use evm_indexer::{
		chains::chains::ETHEREUM_DEVNET, configs::indexer_config::EVMIndexerConfig,
		db::db::Database, rpc::rpc::Rpc,
	};
	use log::info;

	pub async fn run_indexer(db_url: String, redis_url: String, contract_address: Option<Address>) {

		for i in 0..20{
			std::thread::sleep(std::time::Duration::from_millis(5));
			println!("Waiting for block production from parachain: {}", i);
			info!(target: "hyperspace", "Waiting for block production from parachain");
		}
		let config = EVMIndexerConfig {
			start_block: 0,
			db_url,
			redis_url,
			debug: false,
			chain: ETHEREUM_DEVNET,
			batch_size: 200,
			reset: false,
			rpcs: vec!["http://127.0.0.1:8545".to_string()],
			recalc_blocks_indexer: false,
			contract_addresses: contract_address.into_iter().collect(),
			block_confirmation_length: 14,
		};

		for i in 0..20{
			std::thread::sleep(std::time::Duration::from_millis(5));
			println!("Waiting for block production from parachain: {}", i);
			info!(target: "hyperspace", "Waiting for block production from parachain");
		}

		for i in 0..20{
			std::thread::sleep(std::time::Duration::from_millis(5));
			info!(target: "hyperspace", "Starting EVM Indexer.");
			info!(target: "hyperspace", "Syncing chain {}.", config.chain.name);
		}

		info!(target: "hyperspace", "Starting EVM Indexer.");
		info!(target: "hyperspace", "Syncing chain {}.", config.chain.name);

		let rpc = Rpc::new(&config).await.expect("Unable to start RPC client.");

		info!(target: "hyperspace", "Connected EVM Indexer.");
		info!(target: "hyperspace", "Connected Syncing chain {}.", config.chain.name);

		let db =
			Database::new(config.db_url.clone(), config.redis_url.clone(), config.chain.clone())
				.await
				.expect("Unable to start DB connection.");

		info!(target: "hyperspace", "DB created EVM Indexer.");
		info!(target: "hyperspace", "DB created Syncing chain {}.", config.chain.name);

		loop {
			let mut indexed_blocks = db.get_indexed_blocks().await.unwrap();
			evm_indexer::indexer::sync_chain(&rpc, &db, &config, &mut indexed_blocks).await;
			tokio::time::sleep(std::time::Duration::from_millis(50)).await;
		}
	}
}
mod xx {
	use super::*;
	use ethers::prelude::{Address, Middleware, TransactionRequest};
	use hyperspace_core::relay;
	use hyperspace_ethereum::{
		client::EthereumClient, config::EthereumClientConfig, ibc_provider::Ics20BankAbi,
	};

	// #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	// async fn devnet() -> anyhow::Result<()> {
	// 	logging::setup_logging();

	// 	let config_a = toml::from_str::<EthereumClientConfig>(include_str!(
	// 		"../../../config/ethereum-goerli.toml"
	// 	))
	// 	.unwrap();
	// 	let config_b = toml::from_str::<CosmosClientConfig>(include_str!(
	// 		"../../../config/centauri-goerli.toml"
	// 	))
	// 	.unwrap();

	// 	let (client_a, client_b) = (
	// 		EthereumClient::new(config_a).await.unwrap(),
	// 		CosmosClient::<()>::new(config_b).await.unwrap(),
	// 	);
	// 	// let id = client_a.client_id();
	// 	// client_a.set_client_id(client_b.client_id());
	// 	// client_b.set_client_id(id);
	// 	let _client = client_a.client();
	// 	let asset_str = "ppica".to_string();
	// 	let _asset_id_a = AnyAssetId::Ethereum(asset_str.clone());
	// 	let _asset_id_b_atom = AnyAssetId::Cosmos("uatom".to_string());
	// 	let asset_id_b_pica = AnyAssetId::Cosmos("ppica".to_string());
	// 	// let channel_id = ChannelId::new(0);
	// 	let _port_id = PortId::transfer();

	// 	let users = [
	// 		"0xF66605eDE7BfCCc460097CAFD34B4924f1C6969D",
	// 		"0x7C12ff36c44c1B10c13cC76ea8A3aEba0FFf6403",
	// 		"0xD36554eF26E9B2ad72f2b53986469A8180522E5F",
	// 	];
	// 	let pica_amt = 10000000000000000000000u128;
	// 	let _atom_amt = 10000000000000000u128;
	// 	// let pica_amt = 100_000000000000u128;
	// 	// let atom_amt = 10000000000u128;
	// 	// let a = &mut AnyChain::Cosmos(client_b);
	// 	// let b = &mut AnyChain::Ethereum(client_a);
	// 	relay(AnyChain::Ethereum(client_a), AnyChain::Cosmos(client_b), None, None, None)
	// 		.await
	// 		.unwrap();
	// 	// let abi = Ics20BankAbi::new(
	// 	// 	Address::from_str("0x136484d4a64b3a53a82b13b0fb1ea7c79517be9f").unwrap(),
	// 	// 	client,
	// 	// );
	// 	// dbg!(
	// 	// 	get_balance(
	// 	// 		&abi,
	// 	// 		Address::from_str("0xF66605eDE7BfCCc460097CAFD34B4924f1C6969D").unwrap()
	// 	// 	)
	// 	// 	.await
	// 	// );

	// 	// while send_transfer_to(
	// 	// 	b,
	// 	// 	a,
	// 	// 	AnyAssetId::Ethereum("transfer/channel-0/ppica".to_owned()),
	// 	// 	b.channel_whitelist().iter().next().unwrap().0,
	// 	// 	None,
	// 	// 	Signer::from_str("centauri10556m38z4x6pqalr9rl5ytf3cff8q46nk85k9m").unwrap(),
	// 	// 	pica_amt / 100000000,
	// 	// )
	// 	// .await
	// 	// .is_err()
	// 	// {
	// 	// 	tokio::time::sleep(Duration::from_secs(2)).await;
	// 	// }

	// 	// while send_transfer_to(
	// 	// 	a,
	// 	// 	b,
	// 	// 	AnyAssetId::Cosmos("ppica".to_owned()).clone(),
	// 	// 	a.channel_whitelist().iter().next().unwrap().0,
	// 	// 	None,
	// 	// 	Signer::from_str("0xF66605eDE7BfCCc460097CAFD34B4924f1C6969D").unwrap(),
	// 	// 	pica_amt / 1000000,
	// 	// )
	// 	// .await
	// 	// .map_err(|e| {
	// 	// 	error!("{e}");
	// 	// })
	// 	// .is_err()
	// 	// {
	// 	// 	tokio::time::sleep(Duration::from_secs(2)).await;
	// 	// }

	// 	// dbg!(
	// 	// 	get_balance(
	// 	// 		&abi,
	// 	// 		Address::from_str("0x7C12ff36c44c1B10c13cC76ea8A3aEba0FFf6403").unwrap()
	// 	// 	)
	// 	// 	.await
	// 	// );

	// 	// 70000000000000000ppica
	// 	for _user in users {
	// 		let _x = [
	// 			(pica_amt, asset_id_b_pica.clone()),
	// 			// (atom_amt, asset_id_b_atom.clone())
	// 		];
	// 		// for (amt, denom) in x {
	// 		// 	// dbg!(user, get_balance(&abi, Address::from_str(user).unwrap()).await);
	// 		// 	while send_transfer_to(
	// 		// 		a,
	// 		// 		b,
	// 		// 		denom.clone(),
	// 		// 		a.channel_whitelist().iter().next().unwrap().0,
	// 		// 		None,
	// 		// 		Signer::from_str(&user.clone()).unwrap(),
	// 		// 		amt,
	// 		// 	)
	// 		// 	.await
	// 		// 	.map_err(|e| {
	// 		// 		error!("{e}");
	// 		// 	})
	// 		// 	.is_err()
	// 		// 	{
	// 		// 		tokio::time::sleep(Duration::from_secs(2)).await;
	// 		// 	}
	// 		// 	dbg!(user, get_balance(&abi, Address::from_str(user).unwrap()).await);
	// 		// }
	// 	}

	// 	// async fn get_balance<M>(abi: &Ics20BankAbi<M>, acc: H160) -> U256
	// 	// where
	// 	// 	M: Middleware + Debug + Send + Sync,
	// 	// {
	// 	// 	abi.method("balanceOf", (acc, "transfer/channel-0/ppica".to_string()))
	// 	// 		.unwrap()
	// 	// 		.call()
	// 	// 		.await
	// 	// 		.unwrap()
	// 	// };
	// 	// dbg!(
	// 	// 	get_balance(&abi,
	// 	// Address::from_str("0xF66605eDE7BfCCc460097CAFD34B4924f1C6969D").unwrap()) 		.await
	// 	// );

	// 	// let tx = client_a
	// 	// 	.client()
	// 	// 	.get_transaction_receipt(
	// 	// 		H256::from_str("0x0ca7e6f45de3bffeaf93995748a181b4d469b2d7936218bdcc4927fde78ce831")
	// 	// 			.unwrap(),
	// 	// 	)
	// 	// 	.await
	// 	// 	.unwrap()
	// 	// 	.unwrap();
	// 	// // let ev = client_a.yui.event_for_name("TransferInitiated").unwrap();
	// 	// // ev.filter.signature()
	// 	// dbg!(SendPacketFilter::signature());
	// 	// dbg!(TransferInitiatedFilter::signature());
	// 	// tx.logs.iter().for_each(|x| {
	// 	// 	// SendPacketFilter::
	// 	// 	// TransferInitiatedFilter::new
	// 	// 	println!("{:?}", x);
	// 	// });

	// 	// client_a.send_transfer()

	// 	// let block = client_a
	// 	// 	.client()
	// 	// 	.get_block(H256::from_str(
	// 	// 		"0xe44b85448b031c68a2e3b7377b895750bed23ea21bff086360443caeb82d8e62",
	// 	// 	)?)
	// 	// 	.await?
	// 	// 	.unwrap();
	// 	// dbg!(block.transactions.len());
	// 	//
	// 	// let tx = client_a
	// 	// 	.client()
	// 	// 	.get_transaction_receipt(H256::from_str(
	// 	// 		"0x9af4ef7c3c1c1f27d426480ee1348740023131d3eb06988a7fc62d92f173b5fc",
	// 	// 	)?)
	// 	// 	.await?
	// 	// 	.unwrap();
	// 	// tx.logs.iter().for_each(|x| {
	// 	// 	println!("{:?}", x);
	// 	// });
	// 	//
	// 	// let (height, _) = client_a.latest_height_and_timestamp().await.unwrap();
	// 	//
	// 	// let seqs = client_a.query_packet_commitments(height, channel_id, port_id.clone()).await?;
	// 	// seqs.iter().for_each(|x| {
	// 	// 	println!("{:?}", x);
	// 	// });
	// 	//
	// 	// let ps = client_a
	// 	// 	.query_send_packets(height, channel_id, port_id, vec![0, 1, 2, 3])
	// 	// 	.await
	// 	// 	.unwrap();
	// 	// dbg!(ps);

	// 	/*
	// 	Sender account: 0x73db010c3275eb7a92e5c38770316248f4c644ee
	// 	Diamond init address: 0x4d9654e1da9826361519be28c6db135e560f20a0
	// 	Deployed IBCClient on 0xb7198a3674e37433579be45aa9dd09f5ab4b314a
	// 	Deployed IBCConnection on 0xb26397cfa7e111e844086bdd3da5080f9de65cb7
	// 	Deployed IBCChannelHandshake on 0xfbf766071d0fdee42b78ab029b97194543b6d7a5
	// 	Deployed IBCPacket on 0x844d2447e6c00cf6a5fbe9ad5eebebe31e40368e
	// 	Deployed IBCQuerier on 0x992966599e81b9d4a3ef92172b9fa162d2e50d5b
	// 	Deployed DiamondCutFacet on 0x3bf46cf159422e1791d20d45683b21f34ecae4be
	// 	Deployed DiamondLoupeFacet on 0xb16af4cfc553ae0a8f43e812e22dc6caabdf5e63
	// 	Deployed OwnershipFacet on 0x4f6e145fbaf72be9ea283f5793e70a1c594d5ceb
	// 	Deployed update client delegate contract address: 0xe566a7e344f2aef783319a76233e54e7f8b47823
	// 	Deployed light client address: 0x56378f9b88f341b1913a2fc6ac2bcbaa1b9a9f9f
	// 	Deployed Bank module address: 0x0486ee42d89d569c4d8143e47a82c4b14545ae43
	// 	Deployed ICS-20 Transfer module address: 0x4976bb932815783f092dd0e3cca567d5502be46e
	// 	 */

	// 	// relay(client_a, client_b, None, None, None).await.unwrap();
	// 	Ok(())
	// }

	#[tokio::test]
	async fn send_tokens() {
		let config = toml::from_str::<EthereumClientConfig>(
			&std::fs::read_to_string("../../config/ethereum-testnet.toml").unwrap(),
		)
		.unwrap();
		let client = EthereumClient::new(config).await.unwrap();
		let abi = Ics20BankAbi::new(
			Address::from_str("0x0486ee42d89d569c4d8143e47a82c4b14545ae43").unwrap(),
			client.client(),
		);
		let from = Address::from_str("0x73db010c3275eb7a92e5c38770316248f4c644ee").unwrap();
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
