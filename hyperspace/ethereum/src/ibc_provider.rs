use std::{
	collections::HashSet, future::Future, pin::Pin, str::FromStr, sync::Arc, time::Duration,
};

use cast::executor::Output;
use ethers::{
	abi::{encode, Abi, AbiEncode, Detokenize, InvalidOutputType, ParamType, Token, Tokenizable},
	middleware::contract::Contract,
	providers::Middleware,
	types::{BlockId, BlockNumber, EIP1186ProofResponse, Filter, StorageProof, H256, I256, U256},
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
			path::{AcksPath, CommitmentsPath, ReceiptsPath, SeqRecvsPath},
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
			QueryChannelResponse, QueryChannelsResponse, QueryNextSequenceReceiveResponse,
			QueryPacketCommitmentResponse, QueryPacketReceiptResponse,
		},
		client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
		connection::v1::{
			ConnectionEnd, Counterparty, IdentifiedConnection, QueryConnectionResponse, Version,
		},
	},
};
use primitives::{IbcProvider, UpdateType};
use prost::Message;

use futures::{FutureExt, Stream, StreamExt};
use ibc::{applications::transfer::PrefixedCoin, events::IbcEvent};
use ibc_proto::google::protobuf::Any;
use ibc_rpc::PacketInfo;
use pallet_ibc::light_clients::{AnyClientState, AnyConsensusState};
use thiserror::Error;

use crate::client::{
	ClientError, EthereumClient, CLIENT_IMPLS_STORAGE_INDEX, COMMITMENTS_STORAGE_INDEX,
	CONNECTIONS_STORAGE_INDEX,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlockHeight(pub(crate) ethers::types::BlockNumber);

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

#[async_trait::async_trait]
impl IbcProvider for EthereumClient {
	type FinalityEvent = FinalityEvent;

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
		tracing::debug!(?finality_event, "querying latest ibc events");
		tracing::warn!("TODO: implement query_latest_ibc_events");
		Ok(vec![])
	}

	async fn ibc_events(
		&self,
	) -> Pin<Box<dyn Stream<Item = ibc::events::IbcEvent> + Send + 'static>> {
		fn decode_string(bytes: &[u8]) -> String {
			ethers::abi::decode(&[ParamType::String], &bytes)
				.unwrap()
				.into_iter()
				.next()
				.unwrap()
				.to_string()
		}

		fn decode_client_id_log(log: ethers::types::Log) -> ibc::events::IbcEvent {
			let client_id = decode_string(&log.data.0);
			ibc::events::IbcEvent::CreateClient(CreateClient(Attributes {
				height: Height::default(),
				client_id: ClientId::from_str(&client_id).unwrap(),
				client_type: "00-uninitialized".to_owned(),
				consensus_height: Height::default(),
			}))
		}

		let ibc_handler_address = self.config.ibc_handler_address;

		match self.websocket_provider().await {
			Ok(ws) => async_stream::stream! {
				let channel_id_stream = ws
					.subscribe_logs(
						&Filter::new()
							.from_block(BlockNumber::Latest)
							.address(ibc_handler_address)
							.event("GeneratedChannelIdentifier(string)"),
					)
					.await
					.expect("failed to subscribe to GeneratedChannelIdentifier event")
					.map(decode_client_id_log);

				let client_id_stream = ws
					.subscribe_logs(
						&Filter::new()
							.from_block(BlockNumber::Latest)
							.address(ibc_handler_address)
							.event("GeneratedClientIdentifier(string)"),
					)
					.await
					.expect("failed to subscribe to GeneratedClientId event")
					.map(decode_client_id_log);

				let connection_id_stream = ws
					.subscribe_logs(
						&Filter::new()
							.from_block(BlockNumber::Latest)
							.address(ibc_handler_address)
							.event("GeneratedConnectionIdentifier(string)"),
					)
					.await
					.expect("failed to subscribe to GeneratedConnectionIdentifier event")
					.map(decode_client_id_log);

				let recv_packet_stream = ws
					.subscribe_logs(
						&Filter::new()
							.from_block(BlockNumber::Latest)
							.address(ibc_handler_address)
							.event("RecvPacket((uint64,string,string,string,string,bytes,(uint64,uint64),uint64))"),
					)
					.await
					.expect("failed to subscribe to RecvPacket event")
					.map(decode_client_id_log);

				let send_packet_stream = ws
					.subscribe_logs(
						&Filter::new()
							.from_block(BlockNumber::Latest)
							.address(ibc_handler_address)
							.event("SendPacket(uint64,string,string,(uint64,uint64),uint64,bytes)"),
					)
					.await
					.expect("failed to subscribe to SendPacket event")
					.map(decode_client_id_log);

				let inner = futures::stream::select_all([
					channel_id_stream,
					client_id_stream,
					connection_id_stream,
					recv_packet_stream,
					send_packet_stream
				]);
				futures::pin_mut!(inner);

				while let Some(ev) = inner.next().await {
					yield ev
				}
			}
			.left_stream(),
			Err(_) => futures::stream::empty().right_stream(),
		}
		.boxed()
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		let contract = crate::contract::ibc_handler(
			self.config.ibc_handler_address.clone(),
			Arc::clone(&self.http_rpc),
		);
		println!("the address again: {:?}, {client_id}", self.config.ibc_handler_address);

		let binding = contract
			.method(
				"getConsensusState",
				(
					Token::String(client_id.as_str().to_owned()),
					// Token::Uint(consensus_height.revision_height.into()),
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
				eprintln!("{err}");
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
		let contract = crate::contract::ibc_handler(
			self.config.ibc_handler_address.clone(),
			Arc::clone(&self.http_rpc),
		);

		let (client_state, _): (Vec<u8>, bool) = contract
			.method("getClientState", (client_id.as_str().to_owned(),))
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
		let fut = self.eth_query_proof(
			connection_id.as_str(),
			Some(at.revision_height),
			CONNECTIONS_STORAGE_INDEX,
		);

		query_proof_then(fut, move |storage_proof| async move {
			if !storage_proof.value.is_zero() {
				let contract = crate::contract::ibc_handler(
					self.config.ibc_handler_address.clone(),
					Arc::clone(&self.http_rpc),
				);

				let binding = contract
					.method("getConnectionEnd", (connection_id.as_str().to_owned(),))
					.expect("contract is missing getConnectionEnd");

				let connection_end: crate::contract::ConnectionEnd = binding
					.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
					.call()
					.await
					.unwrap();

				let proof_height = Some(at.into());
				let proof = storage_proof.proof.first().map(|b| b.to_vec()).unwrap_or_default();

				Ok(QueryConnectionResponse {
					connection: Some(ConnectionEnd {
						state: connection_end.state as i32,
						client_id: connection_end.client_id,
						counterparty: Some(Counterparty {
							client_id: connection_end.counterparty.client_id,
							connection_id: connection_end.counterparty.connection_id,
							prefix: Some(ibc_proto::ibc::core::commitment::v1::MerklePrefix {
								key_prefix: connection_end.counterparty.prefix,
							}),
						}),
						versions: connection_end
							.versions
							.into_iter()
							.map(|v| ibc_proto::ibc::core::connection::v1::Version {
								identifier: v.identifier,
								features: v.features,
							})
							.collect(),
						delay_period: connection_end.delay_period,
					}),
					proof,
					proof_height,
				})
			} else {
				todo!("error: client address is zero")
			}
		})
		.await
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		let contract = crate::contract::ibc_handler(
			self.config.ibc_handler_address.clone(),
			Arc::clone(&self.http_rpc),
		);

		let binding = contract
			.method::<_, crate::contract::ChannelEnd>(
				"getChannel",
				(channel_id.to_string(), port_id.as_str().to_owned()),
			)
			.expect("contract is missing getChannel");

		let channel_data = binding
			.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
			.call()
			.await
			.unwrap();

		Ok(QueryChannelResponse { channel: None, proof: todo!(), proof_height: todo!() })
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
		let contract = crate::contract::ibc_handler(
			self.config.ibc_handler_address.clone(),
			Arc::clone(&self.http_rpc),
		);

		let binding = contract
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
		let latest_block = self
			.http_rpc
			.get_block_number()
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?;

		let block = self
			.http_rpc
			.get_block(latest_block)
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?
			.ok_or_else(|| ClientError::MiddlewareError(todo!()))?;

		let nanoseconds = Duration::from_secs(block.timestamp.as_u64()).as_nanos() as u64;
		let timestamp = Timestamp::from_nanoseconds(nanoseconds).expect("timestamp error");

		Ok((Height::new(0, latest_block.as_u64()), timestamp))
	}

	async fn query_packet_commitments(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let contract = crate::contract::ibc_handler(
			self.config.ibc_handler_address.clone(),
			Arc::clone(&self.http_rpc),
		);
		let start_seq = 0u64;
		let end_seq = 255u64;
		let binding = contract
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
		let contract = crate::contract::ibc_handler(
			self.config.ibc_handler_address.clone(),
			Arc::clone(&self.http_rpc),
		);

		let start_seq = 0u64;
		let end_seq = 255u64;
		let binding = contract
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
		todo!("query_connection_channels")
	}

	async fn query_send_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<ibc_rpc::PacketInfo>, Self::Error> {
		todo!()
	}

	async fn query_received_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		todo!()
	}

	fn expected_block_time(&self) -> std::time::Duration {
		todo!()
	}

	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, Timestamp), Self::Error> {
		let ibc_handler =
			crate::contract::ibc_handler(self.config.ibc_client_address, self.http_rpc.clone());

		todo!();
	}

	async fn query_host_consensus_state_proof(
		&self,
		client_state: &AnyClientState,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		todo!()
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
		todo!()
	}

	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
		todo!()
	}

	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
		todo!()
	}

	async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
		let ids = self.generated_channel_identifiers(0.into()).await?;

		todo!("query_channels")
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
		// not implemented for the moment.
		Ok(false)
	}

	async fn initialize_client_state(
		&self,
	) -> Result<(AnyClientState, AnyConsensusState), Self::Error> {
		todo!()
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
		todo!()
	}
}

fn u256_to_bytes(n: &U256) -> Vec<u8> {
	let mut bytes = vec![0u8; 256 / 8];
	n.to_big_endian(&mut bytes);
	bytes
}
