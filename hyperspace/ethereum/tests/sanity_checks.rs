use std::{future::Future, ops::Deref, path::PathBuf, sync::Arc};

use ethers::{
	abi::Token, prelude::ContractInstance, solc::ProjectPathsConfig, utils::AnvilInstance,
};
use hyperspace_ethereum::config::Config;

use primitives::IbcProvider;
use prost::Message;

mod utils;

async fn hyperspace_ethereum_client_fixture<M>(
	anvil: &ethers::utils::AnvilInstance,
	utils::DeployYuiIbc {
		ibc_client,
		ibc_connection,
		ibc_channel_handshake,
		ibc_packet,
		ibc_handler,
	}: &utils::DeployYuiIbc<Arc<M>, M>,
) -> hyperspace_ethereum::client::Client {
	hyperspace_ethereum::client::Client::new(Config {
		http_rpc_url: anvil.endpoint().parse().unwrap(),
		ws_rpc_url: Default::default(),
		ibc_handler_address: ibc_handler.address(),
		ibc_packet_address: ibc_packet.address(),
		ibc_client_address: ibc_client.address(),
		ibc_connection_address: ibc_connection.address(),
		ibc_channel_handshake_address: ibc_channel_handshake.address(),
		mnemonic: None,
		max_block_weight: 1,
		private_key: Some(
			anvil.keys()[0]
				.to_sec1_pem(elliptic_curve::pkcs8::LineEnding::LF)
				.unwrap()
				.deref()
				.clone(),
		),
		name: "foo".into(),
		client_id: None,
		connection_id: None,
		channel_whitelist: vec![],
		commitment_prefix: "".into(),
	})
	.await
	.unwrap()
}

type ProviderImpl = ethers::prelude::SignerMiddleware<
	ethers::providers::Provider<ethers::providers::Http>,
	ethers::signers::LocalWallet,
>;

#[allow(dead_code)]
struct DeployYuiIbcMockClient {
	pub path: PathBuf,
	pub project_output: ethers::solc::ProjectCompileOutput,
	pub anvil: AnvilInstance,
	pub client: Arc<ProviderImpl>,
	pub ibc_mock_client: ContractInstance<Arc<ProviderImpl>, ProviderImpl>,
	pub yui_ibc: utils::DeployYuiIbc<Arc<ProviderImpl>, ProviderImpl>,
}

async fn deploy_yui_ibc_and_mock_client_fixture() -> DeployYuiIbcMockClient {
	let path = utils::yui_ibc_solidity_path();
	let project_output = utils::compile_yui(&path, "contracts/core");
	let (anvil, client) = utils::spawn_anvil();
	let yui_ibc = utils::deploy_yui_ibc(&project_output, client.clone()).await;

	let ibc_mock_client = utils::compile_solc({
		let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

		ProjectPathsConfig::builder()
			.root(manifest_dir.join("tests/contracts"))
			.sources(manifest_dir.join("tests/contracts/clients"))
			.build()
			.unwrap()
	});

	let ibc_mock_client = utils::deploy_contract(
		ibc_mock_client.find_first("MockClient").unwrap(),
		(Token::Address(yui_ibc.ibc_handler.address()),),
		client.clone(),
	)
	.await;

	// todo: some interactions between the mock client and the ibc handler to verify behaviour.

	DeployYuiIbcMockClient { path, project_output, anvil, client, yui_ibc, ibc_mock_client }
}

async fn deploy_mock_client_fixture(deploy: &DeployYuiIbcMockClient) -> String {
	deploy
		.yui_ibc
		.register_client("mock-client", deploy.ibc_mock_client.address())
		.await;

	deploy.yui_ibc.create_client(utils::mock::create_client_msg()).await
}

#[track_caller]
fn deploy_mock_module_fixture(
	deploy: &DeployYuiIbcMockClient,
) -> impl Future<Output = ContractInstance<Arc<ProviderImpl>, ProviderImpl>> + '_ {
	async move {
		let clients = utils::compile_solc({
			let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

			ProjectPathsConfig::builder()
				.root(manifest_dir.join("tests/contracts"))
				.sources(manifest_dir.join("tests/contracts/clients"))
				.build()
				.unwrap()
		});

		let contract = clients.find_first("MockModule").expect("no MockModule in project output");
		let constructor_args = (Token::Address(deploy.yui_ibc.ibc_handler.address()),);
		utils::deploy_contract(contract, constructor_args, deploy.client.clone()).await
	}
}

#[tokio::test]
async fn test_deploy_yui_ibc_and_mock_client() {
	deploy_yui_ibc_and_mock_client_fixture().await;
}

#[tokio::test]
async fn test_hyperspace_ethereum_client() {
	let DeployYuiIbcMockClient { anvil, yui_ibc, .. } =
		deploy_yui_ibc_and_mock_client_fixture().await;
	let _hyperspace = hyperspace_ethereum_client_fixture(&anvil, &yui_ibc).await;
}

#[tokio::test]
async fn test_ibc_client() {
	let deploy = deploy_yui_ibc_and_mock_client_fixture().await;
	let hyperspace = hyperspace_ethereum_client_fixture(&deploy.anvil, &deploy.yui_ibc).await;
	let client_id = deploy_mock_client_fixture(&deploy).await;

	let r = hyperspace
		.query_client_state(
			ibc::Height { revision_number: 0, revision_height: 1 },
			client_id.parse().unwrap(),
		)
		.await
		.unwrap();

	assert_eq!(
		r.client_state,
		Some(ibc_proto::google::protobuf::Any {
			type_url: "/ibc.lightclients.mock.v1.ClientState".into(),
			value: utils::mock::ClientState {
				height: ibc_proto::ibc::core::client::v1::Height {
					revision_number: 0,
					revision_height: 1,
				},
			}
			.encode_to_vec(),
		})
	);

	let r = hyperspace
		.query_client_consensus(
			ibc::Height { revision_number: 0, revision_height: 1 },
			client_id.parse().unwrap(),
			ibc::Height { revision_number: 0, revision_height: 1 },
		)
		.await
		.unwrap();

	assert_eq!(
		r.consensus_state,
		Some(ibc_proto::google::protobuf::Any {
			type_url: "/ibc.lightclients.mock.v1.ConsensusState".into(),
			value: utils::mock::ConsensusState { timestamp: 1 }.encode_to_vec(),
		})
	);
}

	// query_client_state
	// query_client_consensus
}
