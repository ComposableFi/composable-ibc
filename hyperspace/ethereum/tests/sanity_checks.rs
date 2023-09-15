use std::{future::Future, ops::Deref, path::PathBuf, str::FromStr, sync::Arc, time::Duration};

use cast::revm::new;
use ecdsa::SigningKey;
use elliptic_curve::pkcs8::der::pem;
use ethers::{
	abi::{encode_packed, Token},
	contract::MultiAbigen,
	core::k256::sha2::{Digest, Sha256},
	prelude::{ContractFactory, ContractInstance, TransactionReceipt},
	providers::{Http, Provider},
	signers::Wallet,
	types::U256,
	utils::{keccak256, AnvilInstance},
};
use ethers_solc::{Artifact, ProjectCompileOutput, ProjectPathsConfig};
use futures::{Stream, TryStreamExt};
use hyperspace_cosmos::client::{CosmosClient, CosmosClientConfig};
use hyperspace_ethereum::{
	client::{EthRpcClient, EthereumClient},
	config::EthereumClientConfig,
	contract::{IbcHandler, UnwrapContractError},
	mock::{utils, utils::USE_GETH},
	utils::{DeployYuiIbc, ProviderImpl},
	yui_types::IntoToken,
};
use ibc::{
	core::{
		ics02_client::{
			height::Height,
			msgs::{create_client::MsgCreateAnyClient, update_client::MsgUpdateAnyClient},
			trust_threshold::TrustThreshold,
		},
		ics03_connection::{
			connection::Counterparty,
			msgs::{
				conn_open_ack::MsgConnectionOpenAck, conn_open_confirm::MsgConnectionOpenConfirm,
				conn_open_init::MsgConnectionOpenInit,
			},
		},
		ics04_channel::{
			channel::{ChannelEnd, Order, State},
			msgs::{
				acknowledgement::{Acknowledgement, MsgAcknowledgement},
				chan_close_confirm::MsgChannelCloseConfirm,
				chan_close_init::MsgChannelCloseInit,
				chan_open_ack::MsgChannelOpenAck,
				chan_open_confirm::MsgChannelOpenConfirm,
				chan_open_init::MsgChannelOpenInit,
				chan_open_try::MsgChannelOpenTry,
				recv_packet::MsgRecvPacket,
			},
			packet::{Packet, Sequence},
			Version,
		},
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes},
		ics24_host::identifier::{ChainId, ChannelId, ConnectionId, PortId},
	},
	events::IbcEvent,
	proofs::{ConsensusProof, Proofs},
	protobuf::Protobuf,
	signer::Signer,
	timestamp::Timestamp,
	tx_msg::Msg,
};
use ibc_proto::google::protobuf::Any;
use ibc_rpc::{IbcApiClient, PacketInfo};
use ics07_tendermint::client_state::ClientState;
use pallet_ibc::light_clients::{AnyClientState, AnyConsensusState, HostFunctionsManager};
use primitives::{mock::LocalClientTypes, Chain, IbcProvider};
use prost::Message;
use tokio::{
	task::LocalSet,
	time::{sleep, timeout, Interval},
};
use tokio_stream::{Elapsed, StreamExt as _};
use tracing::log;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};

#[allow(dead_code)]
pub struct DeployYuiIbcMockClient {
	pub path: PathBuf,
	pub project_output: ProjectCompileOutput,
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

pub async fn deploy_yui_ibc_and_mock_client_fixture() -> DeployYuiIbcMockClient {
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
		let connection_id = deploy.yui_ibc.connection_open_init_mock(&self.0).await;
		let () = deploy
			.yui_ibc
			.connection_open_ack_mock(&connection_id, utils::mock::client_state_bytes())
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

pub fn get_dummy_proof() -> Vec<u8> {
	"Y29uc2Vuc3VzU3RhdGUvaWJjb25lY2xpZW50LzIy".as_bytes().to_vec()
}

#[tokio::test]
async fn test_deploy_yui_ibc_and_create_eth_client() {
	let start = std::time::Instant::now();
	let path = utils::yui_ibc_solidity_path();

	/* ______________________________________________________________________________ */
	//compile and deploy yui ibc contracts
	let project_output = utils::compile_yui(&path, "contracts/core");
	let project_output1 = utils::compile_yui(&path, "contracts/clients");
	let diamond_project_output = utils::compile_yui(&path, "contracts/diamond");

	let (anvil, client) = utils::spawn_anvil();

	let mut yui_ibc =
		utils::deploy_yui_ibc(&project_output, &diamond_project_output, client.clone()).await;

	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//create ethereum hyperspace client
	let mut hyperspace =
		EthereumClient::new(hyperspace_ethereum_client_fixture(&anvil, yui_ibc.clone()).await)
			.await
			.unwrap();
	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//find and deploy tendermint client
	let upd = project_output1.find_first("DelegateTendermintUpdate").unwrap();
	let (abi, bytecode, _) = upd.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let update_client_delegate_contract = factory.deploy(()).unwrap().send().await.unwrap();

	let contract = project_output1.find_first("TendermintLightClientSimple").unwrap();
	// dbg!(&contract);
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

	//replace the tendermint client address in hyperspace config with a real one
	hyperspace.config.ibc_handler_address = yui_ibc.diamond.address();
	hyperspace.config.tendermint_client_address = tendermint_light_client.address();
	// yui_ibc.tendermint_client = tendermint_light_client;
	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//register client
	let _ = yui_ibc
		.register_client(
			&hyperspace.config.client_type,
			hyperspace.config.tendermint_client_address,
		)
		.await;
	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//create client
	let signer = Signer::from_str("0CDA3F47EF3C4906693B170EF650EB968C5F4B2C").unwrap();

	let tm = serde_json::from_str::<tendermint::block::signed_header::SignedHeader>(include_str!(
		"../../../light-clients/ics07-tendermint/src/mock/signed_header.json"
	))
	.unwrap();

	let tm_header = tm.header.clone();

	let client_state = ClientState::<HostFunctionsManager>::new(
		ChainId::from(tm_header.chain_id.clone()),
		Default::default(),
		Duration::from_secs(64000),
		Duration::from_secs(128000),
		Duration::from_millis(3000),
		Height::new(
			ChainId::chain_version(tm_header.chain_id.as_str()),
			u64::from(tm_header.height),
		),
		Default::default(),
		vec!["".to_string()],
	)
	.unwrap();

	let msg = MsgCreateAnyClient::<LocalClientTypes>::new(
		AnyClientState::Tendermint(client_state.clone()),
		AnyConsensusState::Tendermint(tm_header.try_into().unwrap()),
		signer.clone(),
	)
	.unwrap();

	use ibc::{protobuf::Protobuf, tx_msg::Msg};
	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };

	let result = hyperspace.submit(vec![msg]).await.unwrap();
	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//update client
	let mut set = vec![];
	let header = ics07_tendermint::client_message::Header {
		signed_header: tm,
		validator_set: tendermint::validator::Set::new(set.clone(), None),
		trusted_height: Height::new(
			client_state.latest_height.revision_number,
			client_state.latest_height.revision_height,
		),
		trusted_validator_set: tendermint::validator::Set::new(set, None),
	};
	let msg = MsgUpdateAnyClient::<LocalClientTypes> {
		client_id: hyperspace.config.clone().client_id.unwrap(),
		client_message: pallet_ibc::light_clients::AnyClientMessage::Tendermint(
			ics07_tendermint::client_message::ClientMessage::Header(header.clone()),
		),
		signer,
	};

	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };

	//before to call update you need to be sure that the prev revision_height was stored on the
	// chain
	assert_eq!(client_state.latest_height.revision_height, header.trusted_height.revision_height);
	let result = hyperspace.submit(vec![msg]).await.unwrap();
	/* ______________________________________________________________________________ */
	/* ______________________________________________________________________________ */
	//create conenction:
	let msg = MsgConnectionOpenInit {
		client_id: ibc::core::ics24_host::identifier::ClientId::new("07-tendermint", 0).unwrap(),
		counterparty: Counterparty::new(
			ibc::core::ics24_host::identifier::ClientId::new("07-ethereum", 0).unwrap(),
			None,
			CommitmentPrefix::try_from(vec![1, 2, 3]).unwrap(),
		),
		version: Some(Default::default()), // not nessesary
		delay_period: Duration::from_secs(1000000),
		signer: Signer::from_str("s").unwrap(), //not nessesary
	};

	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };
	let result = hyperspace.submit(vec![msg]).await.unwrap();

	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//open ack
	let x = CommitmentProofBytes::try_from(get_dummy_proof()).unwrap();
	let proofs = Proofs::new(
		x.clone(),
		Some(x.clone()),
		Some(
			ConsensusProof::new(x.clone(), Height { revision_number: 1, revision_height: 1 })
				.unwrap(),
		),
		Some(x.clone()),
		Height::new(1, 1),
	)
	.unwrap();

	let msg = MsgConnectionOpenAck::<LocalClientTypes> {
		connection_id: ConnectionId::new(0),
		counterparty_connection_id: ConnectionId::new(0),
		client_state: Some(AnyClientState::Tendermint(client_state.clone())),
		proofs: proofs.clone(),
		host_consensus_state_proof: get_dummy_proof(),
		version: ibc::core::ics03_connection::version::Version::default(),
		signer: Signer::from_str("s").unwrap(),
	};
	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };

	//connectionOpenAck test
	let result = hyperspace.submit(vec![msg]).await.unwrap();
	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//open try

	use hyperspace_ethereum::yui_types::ics03_connection::conn_open_try::*;
	let yui_conn_try = YuiMsgConnectionOpenTry {
		counterparty: YuiCounterparty {
			client_id: "07-tendermint-0".to_string(),
			connection_id: "0".to_string(),
			prefix: YuiCommitmentPrefix { key_prefix: vec![1, 2, 3] },
		},
		delay_period: 1000000,
		client_id: "07-tendermint-0".to_string(),
		client_state_bytes: vec![1, 2, 3],
		counterparty_versions: vec![YuiVersion {
			identifier: "07-tendermint-0".to_string(),
			features: vec![],
		}],
		proof_init: get_dummy_proof(),
		proof_client: get_dummy_proof(),
		proof_consensus: get_dummy_proof(),
		proof_height: YuiHeight { revision_number: 1, revision_height: 1 },
		consensus_height: YuiHeight { revision_number: 1, revision_height: 1 },
	};

	//this test will pass only when this functions on yui solidity side return true
	/*
		require(
			verifyConnectionState(
				connection, msg_.proofHeight, msg_.proofTry, msg_.counterpartyConnectionID, expectedConnection
			),
			"failed to verify connection state"
		);
		require(
			verifyClientState(
				connection,
				msg_.proofHeight,
				IBCCommitment.clientStatePath(connection.counterparty.client_id),
				msg_.proofClient,
				msg_.clientStateBytes
			),
			"failed to verify clientState"
		);
	*/
	yui_ibc.connection_open_try(yui_conn_try.into_token()).await;
	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//open confirm
	let msg = MsgConnectionOpenConfirm {
		connection_id: ConnectionId::new(0),
		proofs,
		signer: Signer::from_str("s").unwrap(),
	};

	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };
	//connectionOpenConfirm test works only in that case when connection is "connection state is
	// TRYOPEN status" before to uncomment next line be sure that you call open try before
	// let result = hyperspace.submit(vec![msg]).await.unwrap();
	/* ______________________________________________________________________________ */

	//all tests related to channels is bellow this test: --- > (fn relayer_channel_tests)
	//basiclay that tests is compatibiliy with solidity side

	let duration = std::time::Instant::now().duration_since(start);
	println!("Time elapsed: {:.2} seconds", duration.as_secs());

	//call submit to create a new client
}

#[tokio::test]
async fn relayer_channel_tests() {
	let path = utils::yui_ibc_solidity_path();

	let project_output1 = utils::compile_yui(&path, "contracts/clients");
	let (anvil, client) = utils::spawn_anvil();

	let signer = Signer::from_str("0CDA3F47EF3C4906693B170EF650EB968C5F4B2C").unwrap();
	/* ______________________________________________________________________________ */
	//channel open init
	let port_id = PortId::transfer();
	let version = "1.0".to_string();
	let order = Order::default();
	let conenction_id = ConnectionId::new(0);
	let mut channel = ChannelEnd::new(
		State::Init,
		order,
		ibc::core::ics04_channel::channel::Counterparty::new(port_id.clone(), None),
		vec![conenction_id],
		Version::new(version.clone()),
	);

	let msg =
		MsgChannelOpenInit::new(port_id.clone(), channel.clone(), Signer::from_str("s").unwrap());
	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };
	// let _ = hyperspace.submit(vec![msg]).await.unwrap();

	let upd = project_output1.find_first("IBCChannelHandlerMock").unwrap();
	let (abi, bytecode, _) = upd.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let contract_mock = factory.deploy(()).unwrap().send().await.unwrap();

	let msg = MsgChannelOpenInit::decode_vec(&msg.value).unwrap();
	let ibc_handler_mock = IbcHandler::new(contract_mock);
	let channel_id = ibc_handler_mock.send::<String>(msg.into_token(), "channelOpenInit").await;
	assert!(channel_id.len() > 0);
	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//MsgChannelOpenTry
	channel.remote.channel_id = Some(ChannelId::new(27));
	let x = CommitmentProofBytes::try_from(get_dummy_proof()).unwrap();
	let proofs = Proofs::new(
		x.clone(),
		Some(x.clone()),
		Some(
			ConsensusProof::new(x.clone(), Height { revision_number: 1, revision_height: 1 })
				.unwrap(),
		),
		Some(x.clone()),
		Height::new(1, 1),
	)
	.unwrap();
	let msg = MsgChannelOpenTry::new(
		port_id.clone(),
		channel,
		Version::new(version.clone()),
		proofs.clone(),
		signer.clone(),
	);
	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };
	let msg = MsgChannelOpenTry::decode_vec(&msg.value).unwrap();
	let channel_id = ibc_handler_mock.send::<String>(msg.into_token(), "channelOpenTry").await;
	assert!(channel_id.len() > 0);
	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//MsgChannelOpenAck
	let msg = MsgChannelOpenAck::new(
		port_id.clone(),
		ChannelId::new(0),
		ChannelId::new(27),
		Version::new(version.clone()),
		proofs.clone(),
		signer.clone(),
	);
	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };
	let msg = MsgChannelOpenAck::decode_vec(&msg.value).unwrap();
	let _ = ibc_handler_mock.send_and_get_tuple(msg.into_token(), "channelOpenAck").await;

	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//channelOpenConfirm
	let msg = MsgChannelOpenConfirm::new(
		port_id.clone(),
		ChannelId::new(0),
		proofs.clone(),
		signer.clone(),
	);
	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };
	let msg = MsgChannelOpenConfirm::decode_vec(&msg.value).unwrap();
	let _ = ibc_handler_mock
		.send_and_get_tuple(msg.into_token(), "channelOpenConfirm")
		.await;

	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//channelCloseInit
	let msg = MsgChannelCloseInit::new(port_id.clone(), ChannelId::new(0), signer.clone());
	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };
	let msg = MsgChannelCloseInit::decode_vec(&msg.value).unwrap();
	let _ = ibc_handler_mock.send_and_get_tuple(msg.into_token(), "channelCloseInit").await;

	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//channelCloseConfirm
	let msg = MsgChannelCloseConfirm::new(
		port_id.clone(),
		ChannelId::new(0),
		proofs.clone(),
		signer.clone(),
	);
	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };
	let msg = MsgChannelCloseConfirm::decode_vec(&msg.value).unwrap();
	let _ = ibc_handler_mock
		.send_and_get_tuple(msg.into_token(), "channelCloseConfirm")
		.await;
	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//MsgRecvPacket
	let packet = Packet {
		sequence: Sequence(1),
		source_port: port_id.clone(),
		source_channel: ChannelId::new(0),
		destination_port: port_id,
		destination_channel: ChannelId::new(0),
		data: vec![1, 2, 3],
		timeout_height: Height { revision_number: 1, revision_height: 1 },
		timeout_timestamp: Timestamp::now(),
	};
	let msg = MsgRecvPacket::new(packet.clone(), proofs.clone(), signer.clone());
	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };
	let msg = MsgRecvPacket::decode_vec(&msg.value).unwrap();
	let _ = ibc_handler_mock.send_and_get_tuple(msg.into_token(), "recvPacket").await;
	/* ______________________________________________________________________________ */

	/* ______________________________________________________________________________ */
	//MsgAcknowledgement
	let msg = MsgAcknowledgement::new(packet, vec![1, 2, 3].into(), proofs, signer);
	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };
	let msg = MsgAcknowledgement::decode_vec(&msg.value).unwrap();
	let _ = ibc_handler_mock.send_and_get_tuple(msg.into_token(), "acknowledgePacket").await;
	/* ______________________________________________________________________________ */
}

//all tests bellow is from Chris probably something usefull can be there (something that related to
// channel)

#[tokio::test]
async fn test_deploy_yui_ibc_and_mock_client() {
	deploy_yui_ibc_and_mock_client_fixture().await;
}

#[tokio::test]
async fn test_hyperspace_ethereum_client() {
	let DeployYuiIbcMockClient { anvil, yui_ibc, .. } =
		deploy_yui_ibc_and_mock_client_fixture().await;
	let _hyperspace =
		EthereumClient::new(hyperspace_ethereum_client_fixture(&anvil, yui_ibc.clone()).await)
			.await
			.unwrap();
}

#[tokio::test]
async fn test_ibc_client() {
	let deploy = deploy_yui_ibc_and_mock_client_fixture().await;
	let hyperspace = EthereumClient::new(
		hyperspace_ethereum_client_fixture(&deploy.anvil, deploy.yui_ibc.clone()).await,
	)
	.await
	.unwrap();
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

	let events = events
		.timeout(Duration::from_secs(5))
		.take_while(|x| x.is_ok())
		.filter_map(|x| x.ok())
		.collect::<Vec<_>>()
		.await;
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
	let hyperspace = EthereumClient::new(
		hyperspace_ethereum_client_fixture(&deploy.anvil, deploy.yui_ibc.clone()).await,
	)
	.await
	.unwrap();
	let client_id = deploy_mock_client_fixture(&deploy).await;

	let connection_id = deploy.yui_ibc.connection_open_init_mock(&client_id).await;
	let () = deploy
		.yui_ibc
		.connection_open_ack_mock(&connection_id, utils::mock::client_state_bytes())
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
	let hyperspace = EthereumClient::new(
		hyperspace_ethereum_client_fixture(&deploy.anvil, deploy.yui_ibc.clone()).await,
	)
	.await
	.unwrap();
	let client_id = deploy_mock_client_fixture(&deploy).await;

	let mock_module = deploy_mock_module_fixture(&deploy).await;
	deploy.yui_ibc.bind_port("port-0", mock_module.address()).await;

	let connection_id = client_id.open_connection(&deploy).await;
	let channel_id = deploy.yui_ibc.channel_open_init_mock("port-0", &connection_id).await;
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
	let hyperspace = EthereumClient::new(
		hyperspace_ethereum_client_fixture(&deploy.anvil, deploy.yui_ibc.clone()).await,
	)
	.await
	.unwrap();
	let events = hyperspace.ibc_events().await;
	let client_id = deploy_mock_client_fixture(&deploy).await;

	let mock_module = deploy_mock_module_fixture(&deploy).await;
	deploy.yui_ibc.bind_port("port-0", mock_module.address()).await;
	deploy.ibc_mock_module = Some(mock_module);

	let connection_id = client_id.open_connection(&deploy).await;
	let channel_id = deploy.yui_ibc.channel_open_init_mock("port-0", &connection_id).await;
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
			data: data.clone(),
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
			ack: Some(data.clone()),
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

	log::warn!("waiting for events");

	let events = events
		.timeout(Duration::from_secs(5))
		.take_while(|x| x.is_ok())
		.filter_map(|x| x.ok())
		.collect::<Vec<_>>()
		.await;
	dbg!(events);
}

fn u64_to_token(x: u64) -> Token {
	let n = U256::from(x);
	let mut buf = [0; 32];
	n.to_big_endian(&mut buf);
	let start = (32 - u64::BITS / 8) as usize;
	Token::Bytes(buf[start..32].to_vec())
}
