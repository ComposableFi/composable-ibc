mod utils;
use ethers::{prelude::ContractFactory, abi::Token};
use ethers_solc::Artifact;
use hyperspace_core::substrate::DefaultConfig;
use hyperspace_cosmos::client::{CosmosClientConfig, CosmosClient};
use ibc::core::ics24_host::identifier::{ConnectionId, ChannelId, PortId};
use primitives::{CommonClientConfig, Chain, IbcProvider};
#[tokio::main]
async fn main() {

}

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


#[tokio::test]
async fn test_deploy_yui_ibc_and_create_eth_client() {
	// let path = utils::yui_ibc_solidity_path();
	// let project_output = utils::compile_yui(&path, "contracts/core");
	// let project_output1 = utils::compile_yui(&path, "contracts/clients");

	// let (anvil, client) = utils::spawn_anvil();

	// let yui_ibc = utils::deploy_yui_ibc(&project_output, client.clone()).await;

	// let mut hyperspace = utils::hyperspace_ethereum_client_fixture(&anvil, &yui_ibc, Some(client.clone())).await;

	// let upd = project_output1.find_first("DelegateTendermintUpdate").unwrap();
	// let (abi, bytecode, _) = upd.clone().into_parts();
	// let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	// let update_client_delegate_contract = factory.deploy(()).unwrap().send().await.unwrap();

	// let contract = project_output1.find_first("TendermintLightClientSimple").unwrap();
	// // dbg!(&contract);
	// let r = contract.clone();
	// let (abi, bytecode, _) = r.into_parts();

	// let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	// let tendermint_light_client = factory.deploy(
	// 	(Token::Address(yui_ibc.ibc_handler.address()), Token::Address(update_client_delegate_contract.address()))
	// ).unwrap().send().await.unwrap();

	// //replace the tendermint client address in hyperspace config with a real one
	// hyperspace.config.tendermint_client_address = tendermint_light_client.address();

	// let result = hyperspace.submit(vec![]).await.unwrap();


	let args = Args::default();

	// let mut config_b = CosmosClientConfig {
	// 	name: "cosmos".to_string(),
	// 	rpc_url: "https://rpc-composable-ia.cosmosia.notional.ventures".to_string().parse().unwrap(),
	// 	grpc_url: "https://grpc-composable-ia.cosmosia.notional.ventures".to_string().parse().unwrap(),
	// 	websocket_url: "ws://rpc-composable-ia.cosmosia.notional.ventures/websocket".to_string().parse().unwrap(),
	// 	chain_id: "centauri-1".to_string(),
	// 	client_id: Some(ibc::core::ics24_host::identifier::ClientId::new("07-tendermint", 30).unwrap()),
	// 	connection_id: Some(ConnectionId::new(2)),
	// 	account_prefix: "centauri".to_string(),
	// 	fee_denom: "ppica".to_string(),
	// 	fee_amount: "1500000".to_string(),
	// 	gas_limit: 10000000000 as u64,
	// 	store_prefix: "ibc".to_string(),
	// 	max_tx_size: 20000000,
	// 	mnemonic:
	// 		"bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort"
	// 			.to_string(),
	// 	wasm_code_id: Some("58c7623a3ab78f4cb2e4c5d02876ac36c3b38bb472118173a7ec7faa688a66d2".to_owned()),
	// 	channel_whitelist: vec![(ChannelId::new(1), PortId::transfer())],
	// 	common: CommonClientConfig {
	// 		skip_optional_client_updates: false,
	// 		max_packets_to_process: 200,
	// 	},
	// };

	// let chain_b = CosmosClient::<DefaultConfig>::new(config_b.clone()).await.unwrap();
	// config_b.wasm_code_id = None;

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
		common: CommonClientConfig {
			skip_optional_client_updates: true,
			max_packets_to_process: 200,
		},
	};

	let chain_b = CosmosClient::<DefaultConfig>::new(config_b.clone()).await.unwrap();

	let chain_state = chain_b.initialize_client_state().await.unwrap();

	//call submit to create a new client
}