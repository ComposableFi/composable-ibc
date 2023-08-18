mod utils;
use ethers::{prelude::ContractFactory, abi::Token};
use ethers_solc::Artifact;
use futures::StreamExt;
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

	let c = hyperspace_core::command::Cmd{ config_a: "../../config-a.toml".to_string(), config_b: "../../config-b.toml".to_string(), config_core: "../../config-core.toml".to_string(), port_id: None, delay_period: None, order: None, version: None, out_config_a: None, out_config_b: None };
	let config = c.parse_config().await.unwrap();
	let mut chain_a = config.chain_b.into_client().await.unwrap();
	let mut f = chain_a.finality_notifications().await.unwrap();
	let n = f.next().await.unwrap();

	


	let path = utils::yui_ibc_solidity_path();
	let project_output = utils::compile_yui(&path, "contracts/core");
	let project_output1 = utils::compile_yui(&path, "contracts/clients");

	let (anvil, client) = utils::spawn_anvil();

	let yui_ibc = utils::deploy_yui_ibc(&project_output, client.clone()).await;

	let mut hyperspace = utils::hyperspace_ethereum_client_fixture(&anvil, &yui_ibc, Some(client.clone())).await;

	let result = chain_a.query_latest_ibc_events(n, &mut hyperspace).await.unwrap();

	let upd = project_output1.find_first("DelegateTendermintUpdate").unwrap();
	let (abi, bytecode, _) = upd.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let update_client_delegate_contract = factory.deploy(()).unwrap().send().await.unwrap();

	let contract = project_output1.find_first("TendermintLightClientSimple").unwrap();
	// dbg!(&contract);
	let r = contract.clone();
	let (abi, bytecode, _) = r.into_parts();

	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let tendermint_light_client = factory.deploy(
		(Token::Address(yui_ibc.ibc_handler.address()), Token::Address(update_client_delegate_contract.address()))
	).unwrap().send().await.unwrap();

	//replace the tendermint client address in hyperspace config with a real one
	hyperspace.config.tendermint_client_address = tendermint_light_client.address();

	let result = hyperspace.submit(vec![]).await.unwrap();

	//call submit to create a new client
}