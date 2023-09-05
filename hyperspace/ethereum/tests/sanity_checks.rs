use std::{future::Future, ops::Deref, path::PathBuf, str::FromStr, sync::Arc, time::Duration};

use crate::utils::USE_GETH;
use ethers::{
	abi::{encode_packed, Token},
	core::k256::sha2::{Digest, Sha256},
	prelude::{ContractInstance, TransactionReceipt},
	types::U256,
	utils::{keccak256, AnvilInstance},
};
use ethers_solc::{ProjectCompileOutput, ProjectPathsConfig};
use futures::{stream::StreamExt, Stream, TryStreamExt};
use hyperspace_ethereum::{
	config::EthereumClientConfig,
	contract::UnwrapContractError,
	utils::{DeployYuiIbc, ProviderImpl},
};
use ibc::{
	core::{
		ics02_client::height::Height,
		ics04_channel::{
			channel::Order,
			packet::{Packet, Sequence},
		},
		ics24_host::identifier::{ChannelId, PortId},
	},
	events::IbcEvent,
	timestamp::Timestamp,
};
use ibc_rpc::PacketInfo;
use primitives::{Chain, IbcProvider};
use prost::Message;
use tokio::time::timeout;
use tracing::log;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};

mod utils;

async fn hyperspace_ethereum_client_fixture(
	anvil: &ethers::utils::AnvilInstance,
	yui_ibc: DeployYuiIbc<Arc<ProviderImpl>, ProviderImpl>,
) -> hyperspace_ethereum::client::EthereumClient {
	let endpoint = if USE_GETH { "http://localhost:6001".to_string() } else { anvil.endpoint() };
	let wallet_path = if USE_GETH {
		Some("keys/0x73db010c3275eb7a92e5c38770316248f4c644ee".to_string())
	} else {
		None
	};
	let wallet = if !USE_GETH { Some(anvil.endpoint().parse().unwrap()) } else { None };

	hyperspace_ethereum::client::EthereumClient::new(
		EthereumClientConfig {
			http_rpc_url: endpoint.parse().unwrap(),
			ws_rpc_url: "ws://localhost:5001".parse().unwrap(),
			ibc_handler_address: yui_ibc.diamond.address(),
			mnemonic: None,
			max_block_weight: 1,
			private_key: wallet,
			private_key_path: wallet_path,
			name: "mock-ethereum-client".into(),
			client_id: None,
			connection_id: None,
			channel_whitelist: vec![],
			commitment_prefix: "".into(),
		},
		yui_ibc,
	)
	.await
	.unwrap()
}

#[allow(dead_code)]
struct DeployYuiIbcMockClient {
	pub path: PathBuf,
	pub project_output: ProjectCompileOutput,
	// pub project_output: ethers::solc::ProjectCompileOutput,
	pub anvil: AnvilInstance,
	pub client: Arc<ProviderImpl>,
	pub ibc_mock_client: ContractInstance<Arc<ProviderImpl>, ProviderImpl>,
	pub ibc_mock_module: Option<ContractInstance<Arc<ProviderImpl>, ProviderImpl>>,
	pub yui_ibc: DeployYuiIbc<Arc<ProviderImpl>, ProviderImpl>,
}

impl DeployYuiIbcMockClient {
	pub async fn send_mock_packet(
		&self,
		source_port: PortId,
		source_channel: ChannelId,
		timeout_height: Height,
		data: Vec<u8>,
	) -> TransactionReceipt {
		let fut = self
			.mock_module_ref()
			.method::<_, ()>(
				"sendMockPacket",
				(
					Token::Bytes(data),                        // data
					Token::String(source_port.to_string()),    // port-id
					Token::String(source_channel.to_string()), // channel-id
					// timeout-height
					Token::Uint(timeout_height.revision_height.into()),
				),
			)
			.unwrap();

		let () = fut.call().await.unwrap_contract_error();
		// let trace = self
		// 	.ibc_handler
		// 	.client()
		// 	.borrow()
		// 	.debug_trace_call(fut.tx.clone(), None, GethDebugTracingCallOptions::default())
		// 	.await
		// 	.unwrap();
		// std::fs::write("trace.txt", format!("{:#?}", trace)).unwrap();
		// println!("trace: {:?}", trace);
		let tx = fut.send().await.unwrap_contract_error().await.unwrap().unwrap();
		// dbg!(tx.logs);
		let status = tx.status.expect("status not found");

		if status == 0.into() {
			panic!("status is 0");
		}
		tx
	}

	fn mock_module_ref(&self) -> &ContractInstance<Arc<ProviderImpl>, ProviderImpl> {
		self.ibc_mock_module.as_ref().expect("no mock module set")
	}
}

async fn deploy_yui_ibc_and_mock_client_fixture() -> DeployYuiIbcMockClient {
	let path = utils::yui_ibc_solidity_path();
	let project_output = utils::compile_yui(&path, "contracts/core");
	let diamond_project_output = utils::compile_yui(&path, "contracts/diamond");
	let (anvil, client) = utils::spawn_anvil();
	log::warn!("{}", anvil.endpoint());
	let yui_ibc =
		utils::deploy_yui_ibc(&project_output, &diamond_project_output, client.clone()).await;

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
		(Token::Address(yui_ibc.diamond.address()),),
		client.clone(),
	)
	.await;

	println!("Mock client address: {:?}", ibc_mock_client.address());

	// todo: some interactions between the mock client and the ibc handler to verify behaviour.

	DeployYuiIbcMockClient {
		path,
		project_output,
		anvil,
		client,
		yui_ibc,
		ibc_mock_client,
		ibc_mock_module: None,
	}
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

	let string = deploy
		.yui_ibc
		.create_client(utils::mock::create_client_msg("mock-client"))
		.await;
	println!("client id: {}", string);
	println!("Diamond contract addr: {:?}", deploy.yui_ibc.diamond.address());
	ClientId(string)
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
		let constructor_args = (Token::Address(deploy.yui_ibc.diamond.address()),);
		let contract =
			utils::deploy_contract(contract, constructor_args, deploy.client.clone()).await;
		println!("Mock module address: {:?}", contract.address());
		contract
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
	let _hyperspace = hyperspace_ethereum_client_fixture(&anvil, yui_ibc.clone()).await;
}

#[tokio::test]
async fn test_ibc_client() {
	let deploy = deploy_yui_ibc_and_mock_client_fixture().await;
	let hyperspace =
		hyperspace_ethereum_client_fixture(&deploy.anvil, deploy.yui_ibc.clone()).await;
	let events = hyperspace.ibc_events().await;
	let client_id = deploy_mock_client_fixture(&deploy).await;
	let height = hyperspace.latest_height_and_timestamp().await.unwrap().0;
	let r = hyperspace.query_client_state(height, client_id.parse().unwrap()).await.unwrap();

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
			height,
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

	let (_host_height, _host_timestamp) = hyperspace
		.query_client_update_time_and_height(
			client_id.parse().unwrap(),
			ibc::Height { revision_number: 0, revision_height: 1 },
		)
		.await
		.unwrap();

	let events = timeout(Duration::from_secs(5), events.take(1).collect::<Vec<_>>())
		.await
		.unwrap();
	dbg!(events);
}

#[tokio::test]
async fn test_ibc_connection() {
	let stdout_log = tracing_subscriber::fmt::layer().pretty();
	tracing_subscriber::registry()
		.with(stdout_log.with_filter(filter::LevelFilter::INFO).with_filter(filter::filter_fn(
			|metadata| metadata.file().map(|x| x.contains("ethers")).unwrap_or_default(),
		)))
		.init();
	let deploy = deploy_yui_ibc_and_mock_client_fixture().await;
	let hyperspace =
		hyperspace_ethereum_client_fixture(&deploy.anvil, deploy.yui_ibc.clone()).await;
	let client_id = deploy_mock_client_fixture(&deploy).await;

	let connection_id = deploy.yui_ibc.connection_open_init(&client_id).await;
	let () = deploy
		.yui_ibc
		.connection_open_ack(&connection_id, utils::mock::client_state_bytes())
		.await;
	let height = hyperspace.latest_height_and_timestamp().await.unwrap().0;

	// hyperspace.query_connection_end(at, connection_id)
	let connection_end = hyperspace
		.query_connection_end(dbg!(height), connection_id.parse().unwrap())
		.await
		.unwrap();

	assert_eq!(
		connection_end.connection,
		Some(ibc_proto::ibc::core::connection::v1::ConnectionEnd {
			client_id: client_id.parse().unwrap(),
			counterparty: Some(ibc_proto::ibc::core::connection::v1::Counterparty {
				client_id: client_id.parse().unwrap(),
				connection_id: "counterparty-connection-id".parse().unwrap(),
				prefix: None,
			}),
			state: ibc_proto::ibc::core::connection::v1::State::Open as i32,
			delay_period: 0,
			versions: vec![ibc_proto::ibc::core::connection::v1::Version {
				identifier: "counterparty-version".into(),
				features: vec!["ORDER_ORDERED".to_string(), "ORDER_UNORDERED".to_string()],
			}],
		})
	);
}

#[tokio::test]
async fn test_ibc_channel() {
	let deploy = deploy_yui_ibc_and_mock_client_fixture().await;
	let hyperspace =
		hyperspace_ethereum_client_fixture(&deploy.anvil, deploy.yui_ibc.clone()).await;
	let client_id = deploy_mock_client_fixture(&deploy).await;

	let mock_module = deploy_mock_module_fixture(&deploy).await;
	deploy.yui_ibc.bind_port("port-0", mock_module.address()).await;

	let connection_id = client_id.open_connection(&deploy).await;
	let channel_id = deploy.yui_ibc.channel_open_init("port-0", &connection_id).await;
	deploy.yui_ibc.channel_open_ack(&channel_id, "port-0").await;

	let channels = hyperspace.query_channels().await.unwrap();
	assert!(!channels.is_empty());
	assert_eq!(channels[0].0, channel_id.parse().unwrap());

	let height = hyperspace.latest_height_and_timestamp().await.unwrap().0;

	let channel_end = hyperspace
		.query_channel_end(height, channel_id.parse().unwrap(), "port-0".parse().unwrap())
		.await
		.unwrap();

	let expected_channel = ibc_proto::ibc::core::channel::v1::Channel {
		state: ibc_proto::ibc::core::channel::v1::State::Open as i32,
		ordering: ibc_proto::ibc::core::channel::v1::Order::Unordered as i32,
		counterparty: Some(ibc_proto::ibc::core::channel::v1::Counterparty {
			port_id: "port-0".into(),
			channel_id: channel_id.parse().unwrap(),
		}),
		connection_hops: vec![connection_id.parse().unwrap()],
		version: "1".into(),
	};
	assert_eq!(channel_end.channel, Some(expected_channel.clone()));

	// TODO: only used in integration tests. Should we really test that?
	// let channels = hyperspace
	// 	.query_connection_channels(height, &connection_id.parse().unwrap())
	// 	.await
	// 	.unwrap();
	// assert_eq!(
	// 	channels,
	// 	vec![ibc_proto::ibc::core::channel::v1::IdentifiedChannel {
	// 		state: expected_channel.state,
	// 		ordering: expected_channel.ordering,
	// 		counterparty: expected_channel.counterparty,
	// 		connection_hops: expected_channel.connection_hops,
	// 		version: expected_channel.version,
	// 		port_id: "port-0".into(),
	// 		channel_id,
	// 	}]
	// );
}

#[tokio::test]
async fn test_ibc_packet() {
	let _ = env_logger::try_init();
	let mut deploy = deploy_yui_ibc_and_mock_client_fixture().await;
	let hyperspace =
		hyperspace_ethereum_client_fixture(&deploy.anvil, deploy.yui_ibc.clone()).await;
	let events = hyperspace.ibc_events().await;

	let client_id = deploy_mock_client_fixture(&deploy).await;

	let mock_module = deploy_mock_module_fixture(&deploy).await;
	deploy.yui_ibc.bind_port("port-0", mock_module.address()).await;
	deploy.ibc_mock_module = Some(mock_module);

	let connection_id = client_id.open_connection(&deploy).await;
	let channel_id = deploy.yui_ibc.channel_open_init("port-0", &connection_id).await;
	deploy.yui_ibc.channel_open_ack(&channel_id, "port-0").await;

	let data = "hello_send".as_bytes().to_vec();
	let tx = deploy
		.send_mock_packet(
			PortId::from_str("port-0").unwrap(),
			ChannelId::from_str("channel-0").unwrap(),
			Height::new(0, 1000000),
			data.clone(),
		)
		.await;
	let height = tx.block_number.unwrap().as_u64();

	tx.logs.iter().for_each(|log| {
		println!("send_packet log: {:#?}", log);
	});

	let send_packet = hyperspace
		.query_send_packets(
			ibc::Height { revision_number: 0, revision_height: height },
			ChannelId::from_str("channel-0").unwrap(),
			PortId::from_str("port-0").unwrap(),
			vec![1],
		)
		.await
		.unwrap()
		.pop()
		.unwrap();

	let info = PacketInfo {
		height: None,
		sequence: 1,
		source_port: "port-0".to_string(),
		source_channel: "channel-0".to_string(),
		destination_port: "port-0".to_string(),
		destination_channel: "channel-0".to_string(),
		channel_order: Order::Unordered.to_string(),
		data: "hello_send".as_bytes().to_vec(),
		timeout_height: Height::new(0, 1000000).into(),
		timeout_timestamp: 0,
		ack: None,
	};
	assert_eq!(send_packet, info);

	// query_packet_commitment
	let commitment = hyperspace
		.query_packet_commitment(
			ibc::Height { revision_number: 0, revision_height: height },
			&PortId::from_str("port-0").unwrap(),
			&ChannelId::from_str("channel-0").unwrap(),
			1,
		)
		.await
		.unwrap();

	assert_eq!(
		commitment.commitment,
		keccak256(&Sha256::digest(
			&encode_packed(&[
				u64_to_token(0),
				u64_to_token(0),
				u64_to_token(1000000),
				Token::Bytes(Sha256::digest(&data).to_vec())
			])
			.unwrap()
		))
		.to_vec()
	);

	// query_packet_commitments
	let commitments = hyperspace
		.query_packet_commitments(
			ibc::Height { revision_number: 0, revision_height: height },
			ChannelId::from_str("channel-0").unwrap(),
			PortId::from_str("port-0").unwrap(),
		)
		.await
		.unwrap();

	assert_eq!(commitments, vec![1]);

	let sequence = Sequence(0);
	let data = "hello_recv".as_bytes().to_vec();
	let ack_hash = keccak256(&Sha256::digest(&data));
	let tx = deploy
		.yui_ibc
		.recv_packet(Packet {
			sequence,
			source_port: PortId::from_str("port-0").unwrap(),
			source_channel: ChannelId::from_str("channel-0").unwrap(),
			destination_port: PortId::from_str("port-0").unwrap(),
			destination_channel: ChannelId::from_str("channel-0").unwrap(),
			data,
			timeout_height: Height::new(0, 1000000),
			timeout_timestamp: Timestamp::default(),
		})
		.await;
	let height = tx.block_number.unwrap().as_u64();

	let ack = hyperspace
		.query_packet_acknowledgement(
			ibc::Height { revision_number: 0, revision_height: height },
			&PortId::from_str("port-0").unwrap(),
			&ChannelId::from_str("channel-0").unwrap(),
			sequence.0, // sequence
		)
		.await
		.unwrap();

	println!("{}", hex::encode(&ack.acknowledgement));
	assert_eq!(ack.acknowledgement, ack_hash.to_vec());

	let received_packets = hyperspace
		.query_received_packets(
			ibc::Height { revision_number: 0, revision_height: height },
			ChannelId::from_str("channel-0").unwrap(),
			PortId::from_str("port-0").unwrap(),
			vec![sequence.0],
		)
		.await
		.unwrap();
	assert_eq!(
		received_packets,
		vec![PacketInfo {
			height: None,
			sequence: sequence.0,
			source_port: "port-0".to_string(),
			source_channel: "channel-0".to_string(),
			destination_port: "port-0".to_string(),
			destination_channel: "channel-0".to_string(),
			channel_order: Order::Unordered.to_string(),
			data: "hello_recv".as_bytes().to_vec(),
			timeout_height: Height::new(0, 1000000).into(),
			timeout_timestamp: 0,
			ack: None,
		}]
	);

	// TODO: query_packet_receipt
	let receipt = hyperspace
		.query_packet_receipt(
			ibc::Height { revision_number: 0, revision_height: height },
			&PortId::from_str("port-0").unwrap(),
			&ChannelId::from_str("channel-0").unwrap(),
			0, // sequence
		)
		.await
		.unwrap();
	assert!(receipt.received);

	// query_packet_acknowledgements
	let acks = hyperspace
		.query_packet_acknowledgements(
			ibc::Height { revision_number: 0, revision_height: height },
			ChannelId::from_str("channel-0").unwrap(),
			PortId::from_str("port-0").unwrap(),
		)
		.await
		.unwrap();

	assert_eq!(acks, vec![0]);

	// query_unreceived_packets
	let unreceived = hyperspace
		.query_unreceived_packets(
			ibc::Height { revision_number: 0, revision_height: height },
			ChannelId::from_str("channel-0").unwrap(),
			PortId::from_str("port-0").unwrap(),
			vec![0, 1, 2],
		)
		.await
		.unwrap();
	assert_eq!(unreceived, vec![1, 2]);

	// query_unreceived_acknowledgements
	let unreceived = hyperspace
		.query_unreceived_acknowledgements(
			ibc::Height { revision_number: 0, revision_height: height },
			ChannelId::from_str("channel-0").unwrap(),
			PortId::from_str("port-0").unwrap(),
			vec![0, 1, 2],
		)
		.await
		.unwrap();
	assert_eq!(unreceived, vec![1, 2]);

	let events = events.collect::<Vec<_>>().await;
	dbg!(events);
}

fn u64_to_token(x: u64) -> Token {
	let n = U256::from(x);
	let mut buf = [0; 32];
	n.to_big_endian(&mut buf);
	let start = (32 - u64::BITS / 8) as usize;
	Token::Bytes(buf[start..32].to_vec())
}
