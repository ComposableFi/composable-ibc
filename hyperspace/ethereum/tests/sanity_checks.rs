use std::{future::Future, ops::Deref, path::PathBuf, str::FromStr, sync::Arc};

use ethers::{
	abi::Token,
	prelude::ContractInstance,
	providers::{FilterKind, Middleware},
	solc::ProjectPathsConfig,
	types::Filter,
	utils::AnvilInstance,
};
use hyperspace_ethereum::config::Config;

use ibc::core::ics24_host::identifier::{ChannelId, PortId};
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct ClientId(pub String);

impl Deref for ClientId {
	type Target = String;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ClientId {
	pub async fn open_connection(&self, deploy: &DeployYuiIbcMockClient) -> String {
		let connection_id = deploy.yui_ibc.connection_open_init(&self.0).await;
		let () = deploy
			.yui_ibc
			.connection_open_ack(&connection_id, utils::mock::client_state_bytes())
			.await;
		connection_id
	}
}

async fn deploy_mock_client_fixture(deploy: &DeployYuiIbcMockClient) -> ClientId {
	deploy
		.yui_ibc
		.register_client("mock-client", deploy.ibc_mock_client.address())
		.await;

	ClientId(
		deploy
			.yui_ibc
			.create_client(utils::mock::create_client_msg("mock-client"))
			.await,
	)
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

#[tokio::test]
async fn test_ibc_connection() {
	let deploy = deploy_yui_ibc_and_mock_client_fixture().await;
	let hyperspace = hyperspace_ethereum_client_fixture(&deploy.anvil, &deploy.yui_ibc).await;
	let client_id = deploy_mock_client_fixture(&deploy).await;

	let connection_id = deploy.yui_ibc.connection_open_init(&client_id).await;
	let () = deploy
		.yui_ibc
		.connection_open_ack(&connection_id, utils::mock::client_state_bytes())
		.await;

	// hyperspace.query_connection_channels(at, connection_id)
	let channels = hyperspace
		.query_connection_channels(
			ibc::Height { revision_number: 0, revision_height: 1 },
			&connection_id.parse().unwrap(),
		)
		.await
		.unwrap();

	assert!(channels.channels.is_empty());

	// hyperspace.query_connection_end(at, connection_id)
	let connection_end = hyperspace
		.query_connection_end(
			ibc::Height { revision_number: 0, revision_height: 1 },
			connection_id.parse().unwrap(),
		)
		.await
		.unwrap();

	assert_eq!(
		connection_end.connection,
		Some(ibc_proto::ibc::core::connection::v1::ConnectionEnd {
			client_id: client_id.parse().unwrap(),
			counterparty: Some(ibc_proto::ibc::core::connection::v1::Counterparty {
				client_id: client_id.parse().unwrap(),
				connection_id: connection_id.parse().unwrap(),
				prefix: Some(ibc_proto::ibc::core::commitment::v1::MerklePrefix {
					key_prefix: vec![],
				}),
			}),
			state: ibc_proto::ibc::core::connection::v1::State::Init as i32,
			delay_period: 0,
			versions: vec![ibc_proto::ibc::core::connection::v1::Version {
				identifier: "1".into(),
				features: vec![],
			}],
		})
	);
}

#[tokio::test]
async fn test_ibc_channel() {
	let deploy = deploy_yui_ibc_and_mock_client_fixture().await;
	let hyperspace = hyperspace_ethereum_client_fixture(&deploy.anvil, &deploy.yui_ibc).await;
	let client_id = deploy_mock_client_fixture(&deploy).await;

	let mock_module = deploy_mock_module_fixture(&deploy).await;
	deploy.yui_ibc.bind_port("port-0", mock_module.address()).await;

	let connection_id = client_id.open_connection(&deploy).await;
	let channel_id = deploy.yui_ibc.channel_open_init("port-0", &connection_id).await;
	deploy.yui_ibc.channel_open_ack(&channel_id, "port-0").await;

	// hyperspace.query_channels(at, channel_id)
	let channels = hyperspace.query_channels().await.unwrap();

	assert!(!channels.is_empty());
	assert_eq!(channels[0].0, channel_id.parse().unwrap());

	// hyperspace.query_channel_end(at, channel_id)
	let channel_end = hyperspace
		.query_channel_end(
			ibc::Height { revision_number: 0, revision_height: 1 },
			channel_id.parse().unwrap(),
			"port-0".parse().unwrap(),
		)
		.await
		.unwrap();

	assert_eq!(
		channel_end.channel,
		Some(ibc_proto::ibc::core::channel::v1::Channel {
			state: ibc_proto::ibc::core::channel::v1::State::Init as i32,
			ordering: ibc_proto::ibc::core::channel::v1::Order::Unordered as i32,
			counterparty: Some(ibc_proto::ibc::core::channel::v1::Counterparty {
				port_id: "port-0".into(),
				channel_id: channel_id.parse().unwrap(),
			}),
			connection_hops: vec![connection_id.parse().unwrap()],
			version: "1".into(),
		})
	);
}

#[tokio::test]
async fn test_ibc_packet() {
	let deploy = deploy_yui_ibc_and_mock_client_fixture().await;
	let hyperspace = hyperspace_ethereum_client_fixture(&deploy.anvil, &deploy.yui_ibc).await;
	let client_id = deploy_mock_client_fixture(&deploy).await;

	let mock_module = deploy_mock_module_fixture(&deploy).await;
	deploy.yui_ibc.bind_port("port-0", mock_module.address()).await;

	let connection_id = client_id.open_connection(&deploy).await;
	let channel_id = deploy.yui_ibc.channel_open_init("port-0", &connection_id).await;
	deploy.yui_ibc.channel_open_ack(&channel_id, "port-0").await;

	// query_packet_acknowledgement
	let ack = hyperspace
		.query_packet_acknowledgement(
			ibc::Height { revision_number: 0, revision_height: 1 },
			&PortId::from_str("port-0").unwrap(),
			&ChannelId::from_str("channel-0").unwrap(),
			1, // sequence
		)
		.await
		.unwrap();

	assert_eq!(ack.acknowledgement, Vec::<u8>::new());

	// query_packet_commitment
	let commitment = hyperspace
		.query_packet_commitments(
			ibc::Height { revision_number: 0, revision_height: 1 },
			ChannelId::from_str("channel-0").unwrap(),
			PortId::from_str("port-0").unwrap(),
		)
		.await
		.unwrap();

	// query_packet_commitments
	let commitments = hyperspace
		.query_packet_commitments(
			ibc::Height { revision_number: 0, revision_height: 1 },
			ChannelId::from_str("channel-0").unwrap(),
			PortId::from_str("port-0").unwrap(),
		)
		.await
		.unwrap();

	// query_packet_receipt
	let receipt = hyperspace
		.query_packet_receipt(
			ibc::Height { revision_number: 0, revision_height: 1 },
			&PortId::from_str("port-0").unwrap(),
			&ChannelId::from_str("channel-0").unwrap(),
			1, // sequence
		)
		.await
		.unwrap();

	// query_packet_acknowledgements
	let acks = hyperspace
		.query_packet_acknowledgements(
			ibc::Height { revision_number: 0, revision_height: 1 },
			ChannelId::from_str("channel-0").unwrap(),
			PortId::from_str("port-0").unwrap(),
		)
		.await
		.unwrap();

	// query_unreceived_packets
	let unreceived = hyperspace
		.query_unreceived_packets(
			ibc::Height { revision_number: 0, revision_height: 1 },
			ChannelId::from_str("channel-0").unwrap(),
			PortId::from_str("port-0").unwrap(),
			vec![],
		)
		.await
		.unwrap();

	// query_unreceived_acknowledgements
	let unreceived = hyperspace
		.query_unreceived_acknowledgements(
			ibc::Height { revision_number: 0, revision_height: 1 },
			ChannelId::from_str("channel-0").unwrap(),
			PortId::from_str("port-0").unwrap(),
			vec![],
		)
		.await
		.unwrap();
}
