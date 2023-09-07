use ethers::{
	abi::{
		encode, encode_packed, ethabi, Abi, AbiEncode, Detokenize, InvalidOutputType, ParamType,
		RawLog, Token, Tokenizable,
	},
	contract::{abigen, EthEvent},
	middleware::contract::Contract,
	prelude::{Block, Log},
	providers::Middleware,
	types::{
		BlockId, BlockNumber, EIP1186ProofResponse, Filter, StorageProof, Topic, ValueOrArray,
		H256, U256,
	},
	utils::keccak256,
};
use ibc::{
	core::{
		ics02_client::{
			client_state::ClientType,
			events::{Attributes, CreateClient},
		},
		ics04_channel::packet::Sequence,
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::{
			identifier::{ChannelId, ClientId, ConnectionId, PortId},
			path::{AcksPath, CommitmentsPath, ReceiptsPath},
			Path,
		},
	},
	timestamp::Timestamp,
	Height,
};
use ibc_proto::{
	google,
	ibc::core::{
		channel::v1::{
			Counterparty as ChannelCounterparty, QueryChannelResponse, QueryChannelsResponse,
			QueryNextSequenceReceiveResponse, QueryPacketCommitmentResponse,
			QueryPacketReceiptResponse,
		},
		client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
		connection::v1::{
			Counterparty as ConnectionCounterparty, IdentifiedConnection, QueryConnectionResponse,
		},
	},
};
use primitives::{IbcProvider, UpdateType};
use prost::Message;
use std::{
	collections::{HashMap, HashSet},
	future::Future,
	pin::Pin,
	str::FromStr,
	sync::Arc,
	time::Duration,
};

use crate::{
	client::{
		ClientError, EthereumClient, CHANNELS_STORAGE_INDEX, CLIENT_IMPLS_STORAGE_INDEX,
		COMMITMENTS_STORAGE_INDEX, CONNECTIONS_STORAGE_INDEX,
	},
	events::TryFromEvent,
	prove::prove,
};
use futures::{FutureExt, Stream, StreamExt};
use ssz_rs::Merkleized;
use thiserror::Error;

use crate::chain::{client_state_from_abi_token, tm_header_from_abi_token};
use ibc::{
	applications::transfer::PrefixedCoin,
	core::{
		ics02_client::{events::UpdateClient, msgs::update_client::MsgUpdateAnyClient},
		ics04_channel::{
			channel::{Order, State},
			events::SendPacket,
		},
		ics23_commitment::commitment::CommitmentRoot,
	},
	events::IbcEvent,
	protobuf::Protobuf,
	tx_msg::Msg,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{
		channel::v1::Channel,
		commitment::v1::MerklePrefix,
		connection::v1::{ConnectionEnd, Version},
	},
};
use ibc_rpc::{IbcApiClient, PacketInfo};
use icsxx_ethereum::{
	client_message::ClientMessage, client_state::ClientState, consensus_state::ConsensusState,
};
use pallet_ibc::light_clients::{
	AnyClientMessage, AnyClientState, AnyConsensusState, HostFunctionsManager,
};
use primitives::mock::LocalClientTypes;
use sync_committee_primitives::types::LightClientState;
use sync_committee_prover::SyncCommitteeProver;
use tracing::log;

abigen!(
	IbcClientAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/ibc-client-abi.json";

	IbcConnectionAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/ibc-connection-abi.json";

	IbcChannelAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/ibc-channel-abi.json";

	IbcPacketAbi,
	"/Users/vmark/work/centauri-private/hyperspace/ethereum/src/abi/ibc-packet-abi.json";
);

impl From<HeightData> for Height {
	fn from(value: HeightData) -> Self {
		Self {
			revision_number: value.revision_number.into(),
			revision_height: value.revision_height.into(),
		}
	}
}

impl From<HeightData> for ibc_proto::ibc::core::client::v1::Height {
	fn from(value: HeightData) -> Self {
		Self {
			revision_number: value.revision_number.into(),
			revision_height: value.revision_height.into(),
		}
	}
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlockHeight(pub(crate) BlockNumber);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FinalityEvent {
	Ethereum { hash: H256 },
}

async fn query_proof_then<Fut, F, T, Fut2>(query_proof: Fut, f: F) -> Result<T, ClientError>
where
	F: FnOnce(StorageProof) -> Fut2,
	Fut2: Future<Output = Result<T, ClientError>>,
	Fut: Future<Output = Result<EIP1186ProofResponse, ClientError>>,
{
	let proof = query_proof.await?;

	if let Some(storage_proof) = proof.storage_proof.last() {
		f(storage_proof.clone()).await
	} else {
		Err(ClientError::NoStorageProof)
	}
}

const NUMBER_OF_BLOCKS_TO_PROCESS_PER_ITER: u64 = 100;

pub async fn parse_ethereum_events(
	client: &EthereumClient,
	logs: Vec<Log>,
) -> Result<Vec<IbcEvent>, ClientError> {
	let mut events = vec![];

	for log in logs {
		let raw_log = RawLog::from(log.clone());
		let height = Height::new(0, log.block_number.unwrap().as_u64());
		let topic0 = log.topics[0];

		macro_rules! handle_events {
		    ($topic0:ident, $events:ident, $log:ident, $raw_log:ident, $height:ident, $($ty:ty),+) => {
				$(if $topic0 == <$ty>::signature() {
					 let event = <$ty>::decode_log(&$raw_log).expect("decode event");
					 $events.push(IbcEvent::try_from_event(client, event, $log, $height).await?)
				} else )+ {
					 log::error!(
						 target: "hyperspace_ethereum", "unknown event: {}",
						   log.log_type.unwrap_or(format!("{:?}", $topic0))
					 );
					 continue
				}
			};
		}

		handle_events!(
			topic0,
			events,
			log,
			raw_log,
			height,
			OpenInitConnectionFilter,
			OpenTryConnectionFilter
		);
	}

	Ok(events)
}

#[async_trait::async_trait]
impl IbcProvider for EthereumClient {
	type FinalityEvent = Block<H256>;

	type TransactionId = (H256, H256);

	type AssetId = ();

	type Error = ClientError;

	async fn query_latest_ibc_events<T>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &T,
	) -> Result<Vec<(Any, Height, Vec<IbcEvent>, UpdateType)>, anyhow::Error>
	where
		T: primitives::Chain,
	{
		let client_id = self.client_id();
		let latest_cp_height = counterparty.latest_height_and_timestamp().await?.0;
		let latest_cp_client_state =
			counterparty.query_client_state(latest_cp_height, client_id.clone()).await?;
		let client_state_response = latest_cp_client_state.client_state.ok_or_else(|| {
			ClientError::Other("counterparty returned empty client state".to_string())
		})?;
		let AnyClientState::Ethereum(client_state) =
			AnyClientState::decode_recursive(client_state_response, |c| {
				matches!(c, AnyClientState::Ethereum(_))
			})
			.ok_or_else(|| ClientError::Other(format!("Could not decode client state")))?
		else {
			unreachable!()
		};
		let latest_cp_client_height = client_state.latest_height().revision_height;
		let latest_height = self.latest_height_and_timestamp().await?.0;
		let latest_revision = latest_height.revision_number;

		let from = latest_cp_client_height;
		let to = finality_event
			.number
			.unwrap()
			.as_u64()
			.min(latest_cp_client_height + NUMBER_OF_BLOCKS_TO_PROCESS_PER_ITER);
		log::info!(target: "hyperspace_ethereum", "Getting blocks {}..{}", from, to);
		let filter =
			Filter::new().from_block(from).to_block(to).address(self.yui.diamond.address());
		let logs = self.client().get_logs(&filter).await.unwrap();

		let update = prove(self, finality_event.number.unwrap().as_u64()).await?;

		log::info!(target: "hyperspace_ethereum",
			"proven: state root = {}, body root = {}, slot = {}, block number = {}",
			update.finalized_header.state_root,
			update.finalized_header.body_root,
			update.finalized_header.slot,
			update.execution_payload.block_number
		);
		// finality_checkpoint.finalized.epoch <= client_state.latest_finalized_epoch
		if update.finality_proof.epoch <= client_state.inner.latest_finalized_epoch {
			log::info!(target: "hyperspace_ethereum", "no new events");
			return Ok(vec![])
		}
		let update_height =
			Height::new(latest_revision, update.execution_payload.block_number.into());
		// let update_height = Height::new(latest_revision, update.finalized_header.slot.into());
		let events = parse_ethereum_events(&self, logs).await?;

		let update_client_header = {
			let msg = MsgUpdateAnyClient::<LocalClientTypes> {
				client_id: client_id.clone(),
				client_message: AnyClientMessage::Ethereum(ClientMessage::Header(update)),
				signer: counterparty.account_id(),
			};
			let value = msg.encode_vec().map_err(|e| {
				ClientError::from(format!("Failed to encode MsgUpdateClient {msg:?}: {e:?}"))
			})?;
			// log::info!(target: "hyperspace_ethereum", "update client header: {value:?}", value =
			// hex::encode(&value));
			Any { value, type_url: msg.type_url() }
		};

		Ok(vec![(update_client_header, update_height, events, UpdateType::Mandatory)])
	}

	// TODO: this function is mostly used in tests and in 'fishing' mode.
	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>> {
		let ibc_address = self.config.ibc_handler_address;
		let client = self.clone();

		let ws = self.websocket_provider().await.unwrap();
		(async_stream::stream! {
			let mut events_stream = ws.subscribe_logs(
				 &Filter::new().from_block(BlockNumber::Latest).address(ibc_address),
			)
			.await
			.unwrap()
			.filter_map(|log| async {
				let raw_log = RawLog::from(log.clone());
				let height = Height::new(0, log.block_number.unwrap().as_u64());
				let topic0 = log.topics[0];

				// let mut maybe_ibc_event = if topic0 == CreateClientFilter::signature() {
				// 	let event = CreateClientFilter::decode_log(&raw_log).expect("decode event");
				// 	IbcEvent::try_from_event(&client, event, log, height).await
				// } else {
					log::error!(target: "hyperspace_ethereum",
						"unknown event: {}",
						log.log_type.unwrap_or(format!("{topic0:?}"))
					);
					return None
				// };

				// match maybe_ibc_event {
				// 	Ok(ev) => Some(ev),
				// 	Err(err) => {
				// 		log::error!(target: "hyperspace_ethereum", "failed to decode event: {err}");
				// 		None
				// 	},
				// }
			}).boxed();

			while let Some(ev) = events_stream.next().await {
				yield ev
			}
		})
		.boxed()
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		let binding = self
			.yui
			.method(
				"getConsensusState",
				(
					Token::String(client_id.as_str().to_owned()),
					Token::Tuple(vec![
						Token::Uint(consensus_height.revision_number.into()),
						Token::Uint(consensus_height.revision_height.into()),
					]),
				),
			)
			.expect("contract is missing getConsensusState");

		let (client_cons, _): (Vec<u8>, bool) = binding
			.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
			.call()
			.await
			.map_err(|err| {
				log::error!(target: "hyperspace_ethereum", "error: {err}");
				err
			})
			.unwrap();

		let proof_height = Some(at.into());
		let consensus_state = google::protobuf::Any::decode(&*client_cons).ok();

		Ok(QueryConsensusStateResponse { consensus_state, proof: vec![0], proof_height })
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		// First, we try to find an `UpdateClient` event at the given height...
		log::info!(target: "hyperspace_ethereum", "qcs {}", line!());
		let mut client_state;
		let mut event_filter = self
			.yui
			.event_for_name::<UpdateClientFilter>("UpdateClient")
			.expect("contract is missing UpdateClient event")
			.from_block(BlockNumber::Earliest)
			.to_block(at.revision_height);
		event_filter.filter = event_filter.filter.topic1({
			let hash = H256::from_slice(&encode(&[Token::FixedBytes(
				keccak256(client_id.to_string().into_bytes()).to_vec(),
			)]));
			ValueOrArray::Value(hash)
		});
		log::info!(target: "hyperspace_ethereum", "qcs {}", line!());
		let maybe_log = self
			.yui
			.diamond
			.client()
			.get_logs(&event_filter.filter)
			.await
			.unwrap()
			.pop() // get only the last event
		;
		log::info!(target: "hyperspace_ethereum", "qcs {}", line!());
		match maybe_log {
			Some(log) => {
				let tx_hash = log.transaction_hash.expect("tx hash should exist");
				let func = self.yui.function("updateClient")?;
				let tx =
					self.client().get_transaction(tx_hash).await.unwrap().ok_or_else(|| {
						ClientError::Other(format!("transaction not found: {}", tx_hash))
					})?;
				let calldata = func.decode_input(&tx.input[4..])?.pop().unwrap();
				let Token::Tuple(toks) = calldata else { panic!() };
				let header = tm_header_from_abi_token(toks[1].clone())?;
				let client_state_token = toks[2].clone();
				client_state = client_state_from_abi_token::<LocalClientTypes>(client_state_token)?;
				client_state.latest_height = Height::new(
					client_state.latest_height.revision_number,
					header.signed_header.header.height.into(),
				);
				// TODO: handle frozen height
			},
			None => {
				log::trace!(target: "hyperspace_ethereum", "no update client event found for blocks ..{at}, looking for a create client event...");

				// ...otherwise, try to get the `CreateClient` event
				let mut event_filter = self
					.yui
					.event_for_name::<CreateClientFilter>("CreateClient")
					.expect("contract is missing CreateClient event")
					.from_block(BlockNumber::Earliest)
					.to_block(at.revision_height);
				event_filter.filter = event_filter.filter.topic1({
					let hash = H256::from_slice(&encode(&[Token::FixedBytes(
						keccak256(client_id.to_string().into_bytes()).to_vec(),
					)]));
					ValueOrArray::Value(hash)
				});
				let log = self
					.yui
					.diamond
					.client()
					.get_logs(&event_filter.filter)
					.await
					.unwrap()
					.pop() // get only the last event
					.ok_or_else(|| ClientError::Other("no events found".to_string()))?;

				let tx_hash = log.transaction_hash.expect("tx hash should exist");
				let func = self.yui.function("createClient")?;
				let tx =
					self.client().get_transaction(tx_hash).await.unwrap().ok_or_else(|| {
						ClientError::Other(format!("transaction not found: {}", tx_hash))
					})?;
				let calldata = func.decode_input(&tx.input[4..])?.pop().unwrap();
				let Token::Tuple(toks) = calldata else { panic!() };
				let client_state_token = toks[1].clone();
				client_state = client_state_from_abi_token::<LocalClientTypes>(client_state_token)?;
			},
		}

		// let decoded_log = UpdateClientFilter::decode_log(&log.clone().into()).unwrap();
		// decoded_log.
		// let block_number = self
		// 	.client()
		// 	.get_block(BlockId::Hash(block_hash))
		// 	.await
		// 	.unwrap()
		// 	.unwrap()
		// 	.number
		// 	.unwrap();
		// let event_filter = self
		// 	.yui
		// 	.event_for_name::<GeneratedClientIdentifierFilter>("GeneratedClientIdentifier")
		// 	.expect("contract is missing GeneratedClientIdentifier event")
		// 	.from_block(block_number)
		// 	.to_block(block_number);
		// let log = self
		// 	.yui
		// 	.diamond
		// 	.client()
		// 	.get_logs(&event_filter.filter)
		// 	.await
		// 	.unwrap()
		// 	.into_iter()
		// 	.find(|log| log.transaction_hash.expect("tx hash should exist") == tx_hash)
		// 	.unwrap();

		// let decoded_log =
		// GeneratedClientIdentifierFilter::decode_log(&log.clone().into()).unwrap();

		// emit CreateClient(clientId, msg_.clientType);
		// emit UpdateClient(msg_.clientId, keccak256(msg_.clientMessage));
		// let (client_state, _): (Vec<u8>, bool) = self
		// 	.yui
		// 	.method("getClientState", (client_id.to_string(),))
		// 	.expect("contract is missing getClientState")
		// 	.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
		// 	.call()
		// 	.await
		// 	.map_err(|err| todo!("query-client-state: error: {err:?}"))
		// 	.unwrap();

		let proof_height = Some(at.into());
		// let client_state = google::protobuf::Any::decode(&*client_state).ok();

		Ok(QueryClientStateResponse {
			client_state: Some(client_state.to_any()),
			proof_height,
			proof: vec![0],
		})
		// log::error!(target: "hyperspace_ethereum", "TODO: implement query_client_state");
		// todo!()
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		let (connection_end, exists): (ConnectionEndData, bool) = self
			.yui
			.method("getConnection", (connection_id.to_string(),))
			.expect("contract is missing getConnectionEnd")
			.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
			.call()
			.await
			.map_err(|err| todo!("query_connection_end: error: {err:?}"))
			.unwrap();

		let connection = if exists {
			let prefix = if connection_end.counterparty.prefix.key_prefix.0.is_empty() {
				None
			} else {
				Some(MerklePrefix {
					key_prefix: connection_end.counterparty.prefix.key_prefix.to_vec(),
				})
			};

			Some(ConnectionEnd {
				client_id: connection_end.client_id,
				versions: connection_end
					.versions
					.into_iter()
					.map(|v| Version { identifier: v.identifier, features: v.features })
					.collect(),
				state: connection_end.state as _,
				counterparty: Some(ConnectionCounterparty {
					client_id: connection_end.counterparty.client_id,
					connection_id: connection_end.counterparty.connection_id,
					prefix,
				}),
				delay_period: connection_end.delay_period,
			})
		} else {
			None
		};

		Ok(QueryConnectionResponse { connection, proof: vec![0], proof_height: Some(at.into()) })
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		let binding = self
			.yui
			.method::<_, ChannelData>(
				"getChannel",
				(port_id.as_str().to_owned(), channel_id.to_string()),
			)
			.expect("contract is missing getChannel");

		let channel_data = binding
			.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
			.call()
			.await
			.unwrap();

		let state = State::from_i32(channel_data.state as _).expect("invalid channel state");
		let counterparty = match state {
			State::Init | State::TryOpen => None,
			State::Open | State::Closed => Some(ChannelCounterparty {
				port_id: channel_data.counterparty.port_id,
				channel_id: channel_data.counterparty.channel_id,
			}),
		};
		Ok(QueryChannelResponse {
			channel: Some(Channel {
				state: channel_data.state as _,
				ordering: channel_data.ordering as _,
				counterparty,
				connection_hops: channel_data.connection_hops,
				version: channel_data.version,
			}),
			proof: vec![0],
			proof_height: None,
		})
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		let key = String::from_utf8(keys[0].clone()).unwrap();

		let proof_result = self
			.eth_query_proof(&key, Some(at.revision_height), COMMITMENTS_STORAGE_INDEX)
			.await?;

		let bytes = proof_result
			.storage_proof
			.first()
			.map(|p| p.proof.first())
			.flatten()
			.map(|b| b.to_vec())
			.unwrap_or_default();

		// Ok(bytes)
		todo!("query-proof: redo")
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error> {
		let path = Path::Commitments(CommitmentsPath {
			port_id: port_id.clone(),
			channel_id: channel_id.clone(),
			sequence: Sequence::from(seq),
		})
		.to_string();

		let proof = self
			.eth_query_proof(&path, Some(at.revision_height), COMMITMENTS_STORAGE_INDEX)
			.await?;
		let storage = proof.storage_proof.first().unwrap();
		let bytes = u256_to_bytes(&storage.value);

		Ok(QueryPacketCommitmentResponse {
			commitment: bytes,
			proof: encode(&[Token::Array(
				storage.proof.clone().into_iter().map(|p| Token::Bytes(p.to_vec())).collect(),
			)]),
			proof_height: Some(at.into()),
		})
	}

	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<ibc_proto::ibc::core::channel::v1::QueryPacketAcknowledgementResponse, Self::Error>
	{
		let path = Path::Acks(AcksPath {
			port_id: port_id.clone(),
			channel_id: channel_id.clone(),
			sequence: Sequence::from(seq),
		})
		.to_string();

		let proof = self
			.eth_query_proof(&path, Some(at.revision_height), COMMITMENTS_STORAGE_INDEX)
			.await?;
		let storage = proof.storage_proof.first().unwrap();

		let bytes = u256_to_bytes(&storage.value);

		Ok(ibc_proto::ibc::core::channel::v1::QueryPacketAcknowledgementResponse {
			acknowledgement: bytes,
			proof: encode(&[Token::Array(
				storage.proof.clone().into_iter().map(|p| Token::Bytes(p.to_vec())).collect(),
			)]),
			proof_height: Some(at.into()),
		})
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		let binding = self
			.yui
			.method::<_, u64>(
				"getNextSequenceRecv",
				(channel_id.to_string(), port_id.as_str().to_owned()),
			)
			.expect("contract is missing getNextSequenceRecv");

		let channel_data = binding
			.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
			.call()
			.await
			.unwrap();

		Ok(QueryNextSequenceReceiveResponse {
			next_sequence_receive: todo!(),
			proof: todo!(),
			proof_height: todo!(),
		})
	}

	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		sequence: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error> {
		let path = Path::Receipts(ReceiptsPath {
			port_id: port_id.clone(),
			channel_id: channel_id.clone(),
			sequence: Sequence::from(sequence),
		})
		.to_string();

		let proof = self
			.eth_query_proof(&path, Some(at.revision_height), COMMITMENTS_STORAGE_INDEX)
			.await?;
		let storage = proof.storage_proof.first().unwrap();

		let received = self
			.has_packet_receipt(at, port_id.as_str().to_owned(), format!("{channel_id}"), sequence)
			.await?;

		Ok(QueryPacketReceiptResponse {
			received,
			proof: encode(&[Token::Array(
				storage.proof.clone().into_iter().map(|p| Token::Bytes(p.to_vec())).collect(),
			)]),
			proof_height: Some(at.into()),
		})
	}

	async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error> {
		// TODO: fix latest_height_and_timestamp in basic builds
		let block_number =// if dbg!(cfg!(feature = "test")) {
			BlockNumber::from(
				self
					.client()
					.get_block_number()
					.await
					.map_err(|err| ClientError::MiddlewareError(err))?,
			);
		// } else {
		// 	BlockNumber::Finalized
		// };

		let block = self
			.client()
			.get_block(BlockId::Number(block_number))
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?
			.ok_or_else(|| ClientError::MiddlewareError(todo!()))?;

		let nanoseconds = Duration::from_secs(block.timestamp.as_u64()).as_nanos() as u64;
		let timestamp = Timestamp::from_nanoseconds(nanoseconds).expect("timestamp error");

		Ok((Height::new(0, block.number.expect("expected block number").as_u64()), timestamp))
	}

	async fn query_packet_commitments(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let start_seq = 0u64;
		let end_seq = 255u64;
		let binding = self
			.yui
			.method(
				"hasCommitments",
				(port_id.as_str().to_owned(), channel_id.to_string(), start_seq, end_seq),
			)
			.expect("contract is missing getConnectionEnd");

		let bitmap: U256 = binding
			.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
			.call()
			.await
			.unwrap();
		let mut seqs = vec![];
		for i in 0..256u64 {
			if bitmap.bit(i as _).into() {
				println!("bit {} is set", i);
				seqs.push(start_seq + i);
			}
		}

		// next_ack is the sequence number used when acknowledging packets.
		// the value of next_ack is the sequence number of the next packet to be acknowledged yet.
		// aka the last acknowledged packet was next_ack - 1.

		// this function is called to calculate which acknowledgements have not yet been
		// relayed from this chain to the counterparty chain.
		Ok(seqs)
	}

	async fn query_packet_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let start_seq = 0u64;
		let end_seq = 255u64;
		let binding = self
			.yui
			.method(
				"hasAcknowledgements",
				(port_id.as_str().to_owned(), channel_id.to_string(), start_seq, end_seq),
			)
			.expect("contract is missing getConnectionEnd");

		let bitmap: U256 = binding
			.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
			.call()
			.await
			.unwrap();
		let mut seqs = vec![];
		for i in 0..256u64 {
			if bitmap.bit(i as _).into() {
				println!("bit {} is set", i);
				seqs.push(start_seq + i);
			}
		}

		// next_ack is the sequence number used when acknowledging packets.
		// the value of next_ack is the sequence number of the next packet to be acknowledged yet.
		// aka the last acknowledged packet was next_ack - 1.

		// this function is called to calculate which acknowledgements have not yet been
		// relayed from this chain to the counterparty chain.
		Ok(seqs)
	}

	async fn query_unreceived_packets(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let mut pending = vec![];

		for seq in seqs {
			let received = self
				.has_packet_receipt(at, port_id.as_str().to_owned(), format!("{channel_id}"), seq)
				.await?;

			if !received {
				pending.push(seq);
			}
		}

		Ok(pending)
	}

	async fn query_unreceived_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let mut pending = vec![];

		for seq in seqs {
			let received = self
				.has_acknowledgement(at, port_id.as_str().to_owned(), format!("{channel_id}"), seq)
				.await?;

			if !received {
				pending.push(seq);
			}
		}

		Ok(pending)
	}

	fn channel_whitelist(&self) -> HashSet<(ChannelId, PortId)> {
		self.config.channel_whitelist.clone().into_iter().collect()
	}

	async fn query_connection_channels(
		&self,
		at: Height,
		connection_id: &ConnectionId,
	) -> Result<QueryChannelsResponse, Self::Error> {
		unimplemented!("query_connection_channels")
	}

	async fn query_send_packets(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		let source_port = port_id.to_string();
		let source_channel = channel_id.to_string();
		let event_filter = self
			.yui
			.event_for_name::<SendPacketFilter>("SendPacket")
			.expect("contract is missing SendPacket event")
			.from_block(BlockNumber::Earliest) // TODO: use contract creation height
			.to_block(BlockNumber::Latest)
			.topic1(ValueOrArray::Array(
				seqs.into_iter()
					.map(|seq| {
						let bytes = encode(&[Token::Uint(seq.into())]);
						H256::from_slice(bytes.as_slice())
					})
					.collect(),
			))
			.topic2({
				let hash = H256::from_slice(&encode(&[Token::FixedBytes(
					keccak256(source_port.clone().into_bytes()).to_vec(),
				)]));
				ValueOrArray::Value(hash)
			})
			.topic3({
				let hash = H256::from_slice(&encode(&[Token::FixedBytes(
					keccak256(source_channel.clone().into_bytes()).to_vec(),
				)]));
				ValueOrArray::Value(hash)
			});

		for i in 0..4 {
			let Some(topic) = &event_filter.filter.topics[i] else { continue };
			let data = match topic {
				Topic::Value(v) => v.iter().map(|v| &v.0[..]).collect::<Vec<_>>(),
				Topic::Array(vs) => vs.iter().flatten().map(|v| &v.0[..]).collect(),
			};
			log::debug!(target: "hyperspace_ethereum",
				"Looking for topic{i}: {}",
				data.into_iter().map(hex::encode).collect::<Vec<_>>().join(", ")
			);
		}
		let events = event_filter.query().await.unwrap();
		let channel = self.query_channel_end(at, channel_id, port_id).await?;

		let channel = channel.channel.expect("channel is none");
		let counterparty = channel.counterparty.expect("counterparty is none");
		Ok(events
			.into_iter()
			.map(move |value| PacketInfo {
				height: None,
				source_port: source_port.clone(),
				source_channel: source_channel.clone(),
				destination_port: counterparty.port_id.clone(),
				destination_channel: counterparty.channel_id.clone(),
				sequence: value.sequence,
				timeout_height: value.timeout_height.into(),
				timeout_timestamp: value.timeout_timestamp,
				data: value.data.to_vec(),
				channel_order: Order::from_i32(channel.ordering)
					.map_err(|_| Self::Error::Other("invalid channel order".to_owned()))
					.unwrap()
					.to_string(),
				ack: None,
			})
			.collect())
	}

	async fn query_received_packets(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		let destination_port = port_id.to_string();
		let destination_channel = channel_id.to_string();
		let event_filter = self
			.yui
			.event_for_name::<RecvPacketFilter>("RecvPacket")
			.expect("contract is missing RecvPacket event")
			.from_block(BlockNumber::Earliest) // TODO: use contract creation height
			.to_block(BlockNumber::Latest)
			.topic1(ValueOrArray::Array(
				seqs.clone()
					.into_iter()
					.map(|seq| {
						let bytes = encode(&[Token::Uint(seq.into())]);
						H256::from_slice(bytes.as_slice())
					})
					.collect(),
			))
			.topic2({
				ValueOrArray::Value(H256::from_slice(&encode(&[Token::FixedBytes(
					keccak256(destination_port.clone().into_bytes()).to_vec(),
				)])))
			})
			.topic3({
				ValueOrArray::Value(H256::from_slice(&encode(&[Token::FixedBytes(
					keccak256(destination_channel.clone().into_bytes()).to_vec(),
				)])))
			});

		let events = event_filter.query().await.unwrap();
		let channel = self.query_channel_end(at, channel_id, port_id).await?;
		let channel = channel.channel.expect("channel is none");

		let acks_filter = self
			.yui
			.event_for_name::<WriteAcknowledgementFilter>("WriteAcknowledgement")
			.expect("contract is missing WriteAcknowledgement event")
			.from_block(BlockNumber::Earliest) // TODO: use contract creation height
			.to_block(BlockNumber::Latest)
			.topic3(ValueOrArray::Array(
				seqs.into_iter()
					.map(|seq| {
						let bytes = encode(&[Token::Uint(seq.into())]);
						H256::from_slice(bytes.as_slice())
					})
					.collect(),
			))
			.topic1({
				ValueOrArray::Value(H256::from_slice(&encode(&[Token::FixedBytes(
					keccak256(destination_port.clone().into_bytes()).to_vec(),
				)])))
			})
			.topic2({
				ValueOrArray::Value(H256::from_slice(&encode(&[Token::FixedBytes(
					keccak256(destination_channel.clone().into_bytes()).to_vec(),
				)])))
			});

		let mut acks_map = acks_filter
			.query()
			.await
			.unwrap()
			.into_iter()
			.map(|ack| (ack.sequence, ack.acknowledgement.to_vec()))
			.collect::<HashMap<_, _>>();

		Ok(events
			.into_iter()
			.map(move |value| PacketInfo {
				height: None,
				source_port: value.source_port.clone(),
				source_channel: value.source_channel.clone(),
				destination_port: destination_port.clone(),
				destination_channel: destination_channel.clone(),
				sequence: value.sequence,
				timeout_height: value.timeout_height.into(),
				timeout_timestamp: value.timeout_timestamp,
				data: value.data.to_vec(),
				channel_order: Order::from_i32(channel.ordering)
					.map_err(|_| Self::Error::Other("invalid channel order".to_owned()))
					.unwrap()
					.to_string(),
				ack: acks_map.get(&value.sequence).cloned(),
			})
			.collect())
	}

	fn expected_block_time(&self) -> Duration {
		Duration::from_secs(14)
	}

	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, Timestamp), Self::Error> {
		let event_filter = self
			.yui
			.event_for_name::<UpdateClientHeightFilter>("UpdateClientHeight")
			.expect("contract is missing UpdateClientHeight event")
			.from_block(BlockNumber::Earliest) // TODO: use contract creation height
			.to_block(BlockNumber::Latest)
			.topic1({
				ValueOrArray::Value(H256::from_slice(&encode(&[Token::FixedBytes(
					keccak256(client_id.to_string()).to_vec(),
				)])))
			})
			.topic2({
				let height_bytes = encode(&[Token::Tuple(vec![
					Token::Uint(client_height.revision_number.into()),
					Token::Uint(client_height.revision_height.into()),
				])]);
				ValueOrArray::Value(H256::from_slice(&encode(&[Token::FixedBytes(
					keccak256(&height_bytes).to_vec(),
				)])))
			});

		let log = self
			.yui
			.diamond
			.client()
			.get_logs(&event_filter.filter)
			.await
			.unwrap()
			.pop()
			.unwrap();

		let height = Height::new(0, log.block_number.expect("block number is none").as_u64());

		let timestamp =
			Timestamp::from_nanoseconds(self.query_timestamp_at(height.revision_height).await?)
				.unwrap();

		Ok((height, timestamp))
	}

	async fn query_host_consensus_state_proof(
		&self,
		_client_state: &AnyClientState,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		Ok(Some(vec![]))
	}

	async fn query_ibc_balance(
		&self,
		asset_id: Self::AssetId,
	) -> Result<Vec<PrefixedCoin>, Self::Error> {
		todo!()
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		CommitmentPrefix::try_from(self.config.commitment_prefix()).unwrap()
	}

	#[track_caller]
	fn client_id(&self) -> ClientId {
		self.config.client_id.clone().expect("no client id set")
	}

	fn set_client_id(&mut self, client_id: ClientId) {
		self.config.client_id = Some(client_id);
	}

	fn connection_id(&self) -> Option<ConnectionId> {
		self.config.connection_id.clone()
	}

	fn set_channel_whitelist(&mut self, channel_whitelist: HashSet<(ChannelId, PortId)>) {
		self.config.channel_whitelist = channel_whitelist.into_iter().collect();
	}

	fn add_channel_to_whitelist(&mut self, channel: (ChannelId, PortId)) {
		self.config.channel_whitelist.push(channel)
	}

	fn set_connection_id(&mut self, connection_id: ConnectionId) {
		self.config.connection_id = Some(connection_id);
	}

	fn client_type(&self) -> ClientType {
		"xx-ethereum".to_string()
	}

	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
		let block = self
			.client()
			.get_block(BlockId::Number(BlockNumber::Number(block_number.into())))
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?
			.ok_or_else(|| ClientError::MiddlewareError(todo!()))?;

		Ok(Duration::from_secs(block.timestamp.as_u64()).as_nanos() as u64)
	}

	// TODO: query_clients (ethereum)
	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
		Ok(vec![])
	}

	async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
		let ids = self.generated_channel_identifiers(0.into()).await?;
		dbg!(&ids);
		ids.into_iter()
			.map(|id| Ok((id.1.parse().unwrap(), id.0.parse().unwrap())))
			.collect()
	}

	async fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>, Self::Error> {
		Ok(vec![]) // TODO: query_connection_using_client (ethereum)
	}

	async fn is_update_required(
		&self,
		latest_height: u64,
		latest_client_height_on_counterparty: u64,
	) -> Result<bool, Self::Error> {
		Ok(false)
	}

	async fn initialize_client_state(
		&self,
	) -> Result<(AnyClientState, AnyConsensusState), Self::Error> {
		let sync_committee_prover = self.prover();
		let block_id = "finalized";
		let block_header = sync_committee_prover.fetch_header(&block_id).await.unwrap();

		let state = sync_committee_prover
			.fetch_beacon_state(&block_header.slot.to_string())
			.await
			.unwrap();

		// TODO: query `at` block
		let finality_checkpoint = sync_committee_prover.fetch_finalized_checkpoint().await.unwrap();

		log::info!(target: "hyperspace_ethereum", "{:?}, {}", state.clone().finalized_checkpoint.clone(), state.slot.clone());
		log::info!(target: "hyperspace_ethereum", "{:?}, {}", state.clone().hash_tree_root(), block_header.slot);
		log::info!(target: "hyperspace_ethereum", "Using init epoch: {}, also have {}", finality_checkpoint.finalized.epoch, state.finalized_checkpoint.epoch);

		let client_state = LightClientState {
			finalized_header: block_header.clone(),
			latest_finalized_epoch: finality_checkpoint.finalized.epoch, // TODO: ????
			current_sync_committee: state.current_sync_committee,
			next_sync_committee: state.next_sync_committee,
		};

		let block = self
			.client()
			.get_block(BlockId::Number(BlockNumber::Number(dbg!(block_header.slot).into())))
			.await
			.unwrap()
			.unwrap();

		dbg!(&block.state_root);
		dbg!(&block.hash.unwrap());
		dbg!(&state.state_roots.iter().take(10).collect::<Vec<_>>());
		dbg!(&state.block_roots.iter().take(10).collect::<Vec<_>>());
		dbg!(&block_header.state_root);
		dbg!(&block_header.body_root);

		let client_state = AnyClientState::Ethereum(ClientState {
			inner: client_state,
			frozen_height: None,
			latest_height: state.latest_execution_payload_header.block_number as _,
			// latest_height: block_header.slot as _
			_phantom: Default::default(),
		});

		let consensus_state = AnyConsensusState::Ethereum(ConsensusState {
			timestamp: tendermint::time::Time::from_unix_timestamp(
				block.timestamp.as_u64() as i64,
				0,
			)
			.unwrap(),
			root: CommitmentRoot { bytes: block.state_root.0.to_vec() },
		});

		Ok((client_state, consensus_state))
	}

	async fn query_client_id_from_tx_hash(
		&self,
		(block_hash, tx_hash): Self::TransactionId,
	) -> Result<ClientId, Self::Error> {
		let block_number = self
			.client()
			.get_block(BlockId::Hash(block_hash))
			.await
			.unwrap()
			.unwrap()
			.number
			.unwrap();
		let event_filter = self
			.yui
			.event_for_name::<GeneratedClientIdentifierFilter>("GeneratedClientIdentifier")
			.expect("contract is missing GeneratedClientIdentifier event")
			.from_block(block_number)
			.to_block(block_number);
		let log = self
			.yui
			.diamond
			.client()
			.get_logs(&event_filter.filter)
			.await
			.unwrap()
			.into_iter()
			.find(|log| log.transaction_hash.expect("tx hash should exist") == tx_hash)
			.unwrap();

		let decoded_log = GeneratedClientIdentifierFilter::decode_log(&log.clone().into()).unwrap();
		Ok(decoded_log.0.parse()?)
	}

	async fn query_connection_id_from_tx_hash(
		&self,
		(block_hash, tx_hash): Self::TransactionId,
	) -> Result<ConnectionId, Self::Error> {
		let block_number = self
			.client()
			.get_block(BlockId::Hash(block_hash))
			.await
			.unwrap()
			.unwrap()
			.number
			.unwrap();
		let event_filter = self
			.yui
			.event_for_name::<OpenInitConnectionFilter>("OpenInitConnection")
			.expect("contract is missing OpenInitConnection event")
			.from_block(block_number)
			.to_block(block_number);
		let log = self
			.yui
			.diamond
			.client()
			.get_logs(&event_filter.filter)
			.await
			.unwrap()
			.into_iter()
			.find(|log| log.transaction_hash.expect("tx hash should exist") == tx_hash)
			.unwrap();

		let decoded_log = OpenInitConnectionFilter::decode_log(&log.clone().into()).unwrap();
		Ok(decoded_log.connection_id.parse()?)
	}

	async fn query_channel_id_from_tx_hash(
		&self,
		(block_hash, tx_hash): Self::TransactionId,
	) -> Result<(ChannelId, PortId), Self::Error> {
		let block_number = self
			.client()
			.get_block(BlockId::Hash(block_hash))
			.await
			.unwrap()
			.unwrap()
			.number
			.unwrap();
		let event_filter = self
			.yui
			.event_for_name::<OpenInitChannelFilter>("OpenInitChannel")
			.expect("contract is missing OpenInitChannel event")
			.from_block(block_number)
			.to_block(block_number);
		let log = self
			.yui
			.diamond
			.client()
			.get_logs(&event_filter.filter)
			.await
			.unwrap()
			.into_iter()
			.find(|log| log.transaction_hash.expect("tx hash should exist") == tx_hash)
			.unwrap();

		let decoded_log = OpenInitChannelFilter::decode_log(&log.clone().into()).unwrap();
		Ok((decoded_log.channel_id.parse()?, decoded_log.port_id.parse()?))
	}

	async fn upload_wasm(&self, _wasm: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
		unimplemented!("upload_wasm")
	}
}

fn u256_to_bytes(n: &U256) -> Vec<u8> {
	let mut bytes = vec![0u8; 256 / 8];
	n.to_big_endian(&mut bytes);
	bytes
}
