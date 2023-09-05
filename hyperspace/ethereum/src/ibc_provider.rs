use ethers::{
	abi::{
		encode, encode_packed, ethabi, Abi, AbiEncode, Detokenize, InvalidOutputType, ParamType,
		RawLog, Token, Tokenizable, Topic,
	},
	contract::{abigen, EthEvent},
	middleware::contract::Contract,
	prelude::Block,
	providers::Middleware,
	types::{
		BlockId, BlockNumber, EIP1186ProofResponse, Filter, StorageProof, ValueOrArray, H256, U256,
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
use thiserror::Error;

use ibc::{
	applications::transfer::PrefixedCoin,
	core::{
		ics04_channel::{
			channel::{Order, State},
			events::SendPacket,
		},
		ics23_commitment::commitment::CommitmentRoot,
	},
	events::IbcEvent,
	protobuf::Protobuf,
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
use icsxx_ethereum::{client_state::ClientState, consensus_state::ConsensusState};
use pallet_ibc::light_clients::{AnyClientState, AnyConsensusState, HostFunctionsManager};
use sync_committee_primitives::types::LightClientState;
use sync_committee_prover::SyncCommitteeProver;

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

#[async_trait::async_trait]
impl IbcProvider for EthereumClient {
	type FinalityEvent = Block<H256>;

	type TransactionId = ();

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
		let client_state = ClientState::<HostFunctionsManager>::decode_vec(
			&client_state_response.value,
		)
		.map_err(|_| ClientError::Other("failed to decode client state response".to_string()))?;
		let latest_cp_client_height = client_state.latest_height().revision_height;
		let latest_height = self.latest_height_and_timestamp().await?.0;
		let latest_revision = latest_height.revision_number;

		// tracing::debug!(?finality_event, "querying latest ibc events");
		// tracing::warn!("TODO: implement query_latest_ibc_events");
		let from = latest_cp_client_height;
		let to = finality_event
			.number
			.unwrap()
			.as_u64()
			.min(latest_cp_client_height + NUMBER_OF_BLOCKS_TO_PROCESS_PER_ITER);
		println!("Getting blocks {}..{}", from, to);
		let filter =
			Filter::new().from_block(from).to_block(to).address(self.yui.diamond.address());
		let client = self.clone();
		let logs = self.http_rpc.get_logs(&filter).await.unwrap();
		let mut events = vec![];

		let update = prove(self, finality_event.hash.unwrap()).await?;

		println!(
			"proven: state root = {}, body root = {}, slot = {}",
			update.finalized_header.state_root,
			update.finalized_header.body_root,
			update.finalized_header.slot
		);
		// for log in logs {
		// 	let raw_log = RawLog::from(log.clone());
		// 	let height = Height::new(0, log.block_number.unwrap().as_u64());
		// 	let topic0 = log.topics[0];
		//
		// 	let mut maybe_ibc_event = if topic0 == CreateClientFilter::signature() {
		// 		let event = CreateClientFilter::decode_log(&raw_log).expect("decode event");
		// 		IbcEvent::try_from_event(&client, event, log, height).await
		// 	} else {
		// 		eprintln!("unknown event: {}", log.log_type.unwrap_or(format!("{topic0:?}")));
		// 		continue
		// 	};
		//
		// 	match maybe_ibc_event {
		// 		Ok(ev) => todo!(),
		// 		Err(err) => {
		// 			eprintln!("failed to decode event: {err}");
		// 			None
		// 		},
		// 	}
		// }

		Ok(events)
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

				let mut maybe_ibc_event = if topic0 == CreateClientFilter::signature() {
					let event = CreateClientFilter::decode_log(&raw_log).expect("decode event");
					IbcEvent::try_from_event(&client, event, log, height).await
				} else {
					eprintln!(
						"unknown event: {}",
						log.log_type.unwrap_or(format!("{topic0:?}"))
					);
					return None
				};

				match maybe_ibc_event {
					Ok(ev) => Some(ev),
					Err(err) => {
						eprintln!("failed to decode event: {err}");
						None
					},
				}
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
				eprintln!("error: {err}");
				err
			})
			.unwrap();

		let proof_height = Some(at.into());
		let consensus_state = google::protobuf::Any::decode(&*client_cons).ok();

		Ok(QueryConsensusStateResponse { consensus_state, proof: vec![], proof_height })
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		let (client_state, _): (Vec<u8>, bool) = self
			.yui
			.method("getClientState", (client_id.to_string(),))
			.expect("contract is missing getClientState")
			.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
			.call()
			.await
			.map_err(|err| todo!("query-client-state: error: {err:?}"))
			.unwrap();

		let proof_height = Some(at.into());
		let client_state = google::protobuf::Any::decode(&*client_state).ok();

		Ok(QueryClientStateResponse { client_state, proof: vec![], proof_height })
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

		Ok(QueryConnectionResponse { connection, proof: Vec::new(), proof_height: Some(at.into()) })
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

		let (channel_data, exists) = binding
			.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
			.call()
			.await
			.unwrap();
		if !exists {
			todo!("error: channel does not exist")
		}

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
			proof: vec![],
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
				self.http_rpc
					.get_block_number()
					.await
					.map_err(|err| ClientError::MiddlewareError(err))?,
			);
		// } else {
		// 	BlockNumber::Finalized
		// };

		let block = self
			.http_rpc
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

	#[cfg(test)]
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
			println!(
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
			.http_rpc
			.get_block(BlockId::Number(BlockNumber::Number(block_number.into())))
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?
			.ok_or_else(|| ClientError::MiddlewareError(todo!()))?;

		Ok(Duration::from_secs(block.timestamp.as_u64()).as_nanos() as u64)
	}

	// TODO
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
		todo!()
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
		let mut string = self.config.beacon_rpc_url.to_string();
		string.pop();
		let sync_committee_prover = SyncCommitteeProver::new(string);
		let block_id = "finalized";
		let block_header = sync_committee_prover.fetch_header(&block_id).await.unwrap();

		let state = sync_committee_prover
			.fetch_beacon_state(&block_header.slot.to_string())
			.await
			.unwrap();

		// TODO: query `at` block
		let finality_checkpoint = sync_committee_prover.fetch_finalized_checkpoint().await.unwrap();
		let client_state = LightClientState {
			finalized_header: block_header.clone(),
			latest_finalized_epoch: finality_checkpoint.finalized.epoch, // TODO: ????
			current_sync_committee: state.current_sync_committee,
			next_sync_committee: state.next_sync_committee,
		};

		let block = self
			.http_rpc
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
			latest_height: block_header.slot as _,
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
		tx_id: Self::TransactionId,
	) -> Result<ClientId, Self::Error> {
		todo!()
	}

	async fn query_connection_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ConnectionId, Self::Error> {
		todo!()
	}

	async fn query_channel_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<(ChannelId, PortId), Self::Error> {
		todo!()
	}

	async fn upload_wasm(&self, wasm: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
		unimplemented!("upload_wasm")
	}
}

fn u256_to_bytes(n: &U256) -> Vec<u8> {
	let mut bytes = vec![0u8; 256 / 8];
	n.to_big_endian(&mut bytes);
	bytes
}
