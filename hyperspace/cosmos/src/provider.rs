use super::{error::Error, CosmosClient};
use core::time::Duration;
use futures::Stream;
use ibc::{
	applications::transfer::PrefixedCoin,
	core::{
		ics02_client::client_state::ClientType,
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	events::IbcEvent,
	timestamp::Timestamp,
	Height,
};

use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{
			QueryChannelResponse, QueryChannelsResponse, QueryNextSequenceReceiveResponse,
			QueryPacketAcknowledgementResponse, QueryPacketCommitmentResponse,
			QueryPacketReceiptResponse,
		},
		client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
		connection::v1::{IdentifiedConnection, QueryConnectionResponse},
	},
};
use ibc_rpc::PacketInfo;
use ics07_tendermint::{client_message::Header, events::try_from_tx};
use pallet_ibc::light_clients::{AnyClientState, AnyConsensusState};
use primitives::{Chain, IbcProvider, UpdateType};
use std::pin::Pin;
use tendermint_rpc::{query::Query, Client, Order};

pub enum FinalityEvent {
	Tendermint(Header),
}

pub struct TransactionId<Hash> {
	pub hash: Hash,
}

#[async_trait::async_trait]
impl<H> IbcProvider for CosmosClient<H>
where
	H: Clone + Send + Sync + 'static,
{
	type FinalityEvent = FinalityEvent;
	type TransactionId = TransactionId<tendermint_rpc::abci::transaction::Hash>;
	type Error = Error;

	async fn query_latest_ibc_events<C>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &C,
	) -> Result<(Any, Vec<IbcEvent>, UpdateType), anyhow::Error>
	where
		C: Chain,
	{
		todo!()
	}

	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent>>> {
		todo!()
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		todo!()
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		todo!()
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		todo!()
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		todo!()
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		todo!()
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error> {
		todo!()
	}

	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Self::Error> {
		todo!()
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
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error> {
		todo!()
	}

	async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error> {
		todo!()
	}

	async fn query_packet_commitments(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		todo!()
	}

	async fn query_packet_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		todo!()
	}

	async fn query_unreceived_packets(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		todo!()
	}

	async fn query_unreceived_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		todo!()
	}

	fn channel_whitelist(&self) -> Vec<(ChannelId, PortId)> {
		todo!()
	}

	async fn query_connection_channels(
		&self,
		at: Height,
		connection_id: &ConnectionId,
	) -> Result<QueryChannelsResponse, Self::Error> {
		todo!()
	}

	async fn query_send_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		todo!()
	}

	async fn query_recv_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		todo!()
	}

	fn expected_block_time(&self) -> Duration {
		todo!()
	}

	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, Timestamp), Self::Error> {
		todo!()
	}

	async fn query_host_consensus_state_proof(
		&self,
		height: Height,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		todo!()
	}

	async fn query_ibc_balance(&self) -> Result<Vec<PrefixedCoin>, Self::Error> {
		todo!()
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		todo!()
	}

	fn client_id(&self) -> ClientId {
		self.client_id()
	}

	fn client_type(&self) -> ClientType {
		todo!()
	}

	fn connection_id(&self) -> ConnectionId {
		todo!()
	}

	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
		todo!()
	}

	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
		todo!()
	}

	async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
		todo!()
	}

	async fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>, Self::Error> {
		todo!()
	}

	fn is_update_required(
		&self,
		latest_height: u64,
		latest_client_height_on_counterparty: u64,
	) -> bool {
		todo!()
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
		let response_vec = self
			.rpc_client
			.tx_search(
				Query::eq("tx.hash", tx_id.hash.to_string()),
				false,
				1,
				1, // get only the first Tx matching the query
				Order::Ascending,
			)
			.await
			.map_err(|e| Error::from(format!("Failed to query tx hash: {:?}", e)))?;
		let response = match response_vec.txs.into_iter().next() {
			None => {
				return Err(Error::from(format!(
					"Failed to find tx hash: {:?}",
					tx_id.hash.to_string()
				)))
			},
			Some(resp) => resp,
		};

		// let height =
		// 	ICSHeight::new(self.chain_id.version(), u64::from(response.clone().height));
		let deliver_tx_result = response.tx_result;
		if deliver_tx_result.code.is_err() {
			Err(Error::from(format!(
				"Transaction failed with code {:?} and log {:?}",
				deliver_tx_result.code, deliver_tx_result.log
			)))
		} else {
			let result = deliver_tx_result
				.events
				.iter()
				.flat_map(|event| try_from_tx(event))
				.collect::<Vec<_>>();
			if result.clone().len() != 1 {
				Err(Error::from(format!(
					"Expected exactly one CreateClient event, found {}",
					result.len()
				)))
			} else {
				let client_id = match result[0] {
					IbcEvent::CreateClient(ref ev) => ev.client_id(),
					IbcEvent::UpdateClient(ref ev) => ev.client_id(),
					IbcEvent::UpgradeClient(ref ev) => ev.client_id(),
					IbcEvent::ClientMisbehaviour(ref ev) => ev.client_id(),
					_ => return Err(Error::from(format!("Unexpected event type"))),
				};
				Ok(*client_id)
			}
		}
	}
}
