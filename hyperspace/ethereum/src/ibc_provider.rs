use std::sync::Arc;

use ethers::{
	abi::{Abi, Detokenize},
	middleware::contract::Contract,
	providers::Middleware,
	types::H256,
};
use ibc::{
	core::{
		ics02_client::client_state::ClientType,
		ics04_channel::packet::Sequence,
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::{
			identifier::{ChannelId, ClientId, ConnectionId, PortId},
			path::{AcksPath, CommitmentsPath, ReceiptsPath, SeqRecvsPath},
			Path,
		},
	},
	Height,
};
use ibc_proto::{
	google,
	ibc::core::{
		channel::v1::{QueryNextSequenceReceiveResponse, QueryPacketReceiptResponse, QueryChannelResponse},
		client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
		connection::v1::{ConnectionEnd, Counterparty, QueryConnectionResponse, Version},
	},
};
use primitives::IbcProvider;
use prost::Message;

use futures::{FutureExt, Stream};
use thiserror::Error;

use crate::client::{
	Client, ClientError, CLIENT_IMPLS_STORAGE_INDEX, COMMITMENTS_STORAGE_INDEX,
	CONNECTIONS_STORAGE_INDEX,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Height(pub(crate) ethers::types::BlockNumber);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FinalityEvent {
	Ethereum { hash: H256 },
}

#[async_trait::async_trait]
impl IbcProvider for Client {
	type FinalityEvent = FinalityEvent;

	type TransactionId = ();

	type AssetId = ();

	type Error = ClientError;

	async fn query_latest_ibc_events<T>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &T,
	) -> Result<
		Vec<(google::protobuf::Any, Vec<ibc::events::IbcEvent>, primitives::UpdateType)>,
		anyhow::Error,
	>
	where
		T: primitives::Chain,
	{
		tracing::debug!(?finality_event, "querying latest ibc events");
		tracing::warn!("TODO: implement query_latest_ibc_events");
		Ok(vec![])
	}

	fn ibc_events<'life0, 'async_trait>(
		&'life0 self,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = std::pin::Pin<
						Box<dyn Stream<Item = ibc::events::IbcEvent> + Send + 'static>,
					>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		let fut = self.eth_query_proof(
			client_id.as_str(),
			Some(at.revision_height),
			CLIENT_IMPLS_STORAGE_INDEX,
		);

		let contract = crate::contract::light_client_contract(
			self.config.address.clone().parse().unwrap(),
			Arc::clone(&self.http_rpc),
		);

		let proof = fut.await?;

		if let Some(storage_proof) = proof.storage_proof.last() {
			if !storage_proof.value.is_zero() {
				let binding = contract
					.method(
						"getConsensusState",
						(client_id.as_str().to_owned(), (at.revision_number, at.revision_height)),
					)
					.expect("contract is missing getConsensusState");

				let get_consensus_state_fut = binding.call();
				let (consensus_state, _): (Vec<u8>, bool) =
					get_consensus_state_fut.await.map_err(|err| todo!()).unwrap();

				let proof_height = Some(at.into());
				let consensus_state = google::protobuf::Any::decode(&*consensus_state).ok();
				let proof = storage_proof.proof.first().map(|b| b.to_vec()).unwrap_or_default();

				Ok(QueryConsensusStateResponse { consensus_state, proof, proof_height })
			} else {
				todo!("error: client address is zero")
			}
		} else {
			todo!("error: no storage proof")
		}
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		let fut = self.eth_query_proof(
			client_id.as_str(),
			Some(at.revision_height),
			CLIENT_IMPLS_STORAGE_INDEX,
		);

		let contract = crate::contract::light_client_contract(
			self.config.address.clone().parse().unwrap(),
			Arc::clone(&self.http_rpc),
		);

		let proof = fut.await?;

		if let Some(storage_proof) = proof.storage_proof.last() {
			if !storage_proof.value.is_zero() {
				let binding = contract
					.method("getClientState", (client_id.as_str().to_owned(),))
					.expect("contract is missing getClientState");

				let get_client_state_fut = binding.call();
				let (client_state, _): (Vec<u8>, bool) =
					get_client_state_fut.await.map_err(|err| todo!()).unwrap();

				let proof_height = Some(at.into());
				let client_state = google::protobuf::Any::decode(&*client_state).ok();
				let proof = storage_proof.proof.first().map(|b| b.to_vec()).unwrap_or_default();

				Ok(QueryClientStateResponse { client_state, proof, proof_height })
			} else {
				todo!("error: client address is zero")
			}
		} else {
			todo!("error: no storage proof")
		}
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

		let proof = fut.await?;

		if let Some(storage_proof) = proof.storage_proof.last() {
			if !storage_proof.value.is_zero() {
				let contract = crate::contract::contract(
					self.config.address.clone().parse().unwrap(),
					Arc::clone(&self.http_rpc),
				);

				let binding = contract
					.method::<_, crate::contract::ConnectionEnd>(
						"ConnectionEnd.Data",
						(connection_id.as_str().to_owned(),),
					)
					.expect("contract is missing ConnectionEnd.Data");

				let connection_end = binding.call().await.unwrap();

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
						versions: connection_end.versions.into_iter().map(|v| {
							ibc_proto::ibc::core::connection::v1::Version {
								identifier: v.identifier,
								features: v.features,
							}
						}).collect(),
						delay_period: connection_end.delay_period,
					}),
					proof,
					proof_height,
				})
			} else {
				todo!("error: client address is zero")
			}
		} else {
			todo!("error: no storage proof")
		}
	}

	fn query_channel_end<'life0, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						ibc_proto::ibc::core::channel::v1::QueryChannelResponse,
						Self::Error,
					>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		use ibc::core::ics23_commitment::{error::Error, merkle::MerkleProof};
		use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;

		let rpc = self.http_rpc.clone();

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

		Ok(bytes)
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<ibc_proto::ibc::core::channel::v1::QueryPacketCommitmentResponse, Self::Error> {
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

		Ok(ibc_proto::ibc::core::channel::v1::QueryPacketCommitmentResponse {
			commitment: storage.value.as_u128().to_be_bytes().to_vec(),
			proof: storage.proof.last().map(|p| p.to_vec()).unwrap_or_default(),
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

		Ok(ibc_proto::ibc::core::channel::v1::QueryPacketAcknowledgementResponse {
			acknowledgement: storage.value.as_u128().to_be_bytes().to_vec(),
			proof: storage.proof.last().map(|p| p.to_vec()).unwrap_or_default(),
			proof_height: Some(at.into()),
		})
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		todo!()
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
			.has_packet_receipt(port_id.as_str().to_owned(), format!("{channel_id}"), sequence)
			.await?;

		Ok(QueryPacketReceiptResponse {
			received,
			proof: storage.proof.last().map(|p| p.to_vec()).unwrap_or_default(),
			proof_height: Some(at.into()),
		})
	}

	fn latest_height_and_timestamp<'life0, 'async_trait>(
		&'life0 self,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<(Height, ibc::timestamp::Timestamp), Self::Error>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_packet_commitments<'life0, 'async_trait>(
		&'life0 self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<u64>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_packet_acknowledgements<'life0, 'async_trait>(
		&'life0 self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<u64>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_unreceived_packets<'life0, 'async_trait>(
		&'life0 self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<u64>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_unreceived_acknowledgements<'life0, 'async_trait>(
		&'life0 self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<u64>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn channel_whitelist(&self) -> Vec<(ChannelId, PortId)> {
		self.config.channel_whitelist.clone()
	}

	fn query_connection_channels<'life0, 'life1, 'async_trait>(
		&'life0 self,
		at: Height,
		connection_id: &'life1 ConnectionId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						ibc_proto::ibc::core::channel::v1::QueryChannelsResponse,
						Self::Error,
					>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		'life1: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_send_packets<'life0, 'async_trait>(
		&'life0 self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<ibc_rpc::PacketInfo>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_recv_packets<'life0, 'async_trait>(
		&'life0 self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<ibc_rpc::PacketInfo>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn expected_block_time(&self) -> std::time::Duration {
		todo!()
	}

	fn query_client_update_time_and_height<'life0, 'async_trait>(
		&'life0 self,
		client_id: ClientId,
		client_height: Height,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<(Height, ibc::timestamp::Timestamp), Self::Error>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_host_consensus_state_proof<'life0, 'async_trait>(
		&'life0 self,
		height: Height,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Option<Vec<u8>>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_ibc_balance<'life0, 'async_trait>(
		&'life0 self,
		asset_id: Self::AssetId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<Vec<ibc::applications::transfer::PrefixedCoin>, Self::Error>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
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

	fn set_channel_whitelist(&mut self, channel_whitelist: Vec<(ChannelId, PortId)>) {
		self.config.channel_whitelist = channel_whitelist;
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

	fn query_timestamp_at<'life0, 'async_trait>(
		&'life0 self,
		block_number: u64,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<u64, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_clients<'life0, 'async_trait>(
		&'life0 self,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<ClientId>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_channels<'life0, 'async_trait>(
		&'life0 self,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<(ChannelId, PortId)>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_connection_using_client<'life0, 'async_trait>(
		&'life0 self,
		height: u32,
		client_id: String,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						Vec<ibc_proto::ibc::core::connection::v1::IdentifiedConnection>,
						Self::Error,
					>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
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

	fn initialize_client_state<'life0, 'async_trait>(
		&'life0 self,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						(
							pallet_ibc::light_clients::AnyClientState,
							pallet_ibc::light_clients::AnyConsensusState,
						),
						Self::Error,
					>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_client_id_from_tx_hash<'life0, 'async_trait>(
		&'life0 self,
		tx_id: Self::TransactionId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<ClientId, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_connection_id_from_tx_hash<'life0, 'async_trait>(
		&'life0 self,
		tx_id: Self::TransactionId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<ConnectionId, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_channel_id_from_tx_hash<'life0, 'async_trait>(
		&'life0 self,
		tx_id: Self::TransactionId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<(ChannelId, PortId), Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}
}
