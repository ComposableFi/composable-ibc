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
use ics07_tendermint::{client_message::Header, client_state::ClientState, events::try_from_tx};
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState, AnyConsensusState};
use primitives::{Chain, IbcProvider, UpdateType};
use std::pin::Pin;
use tendermint::block::Height;
use tendermint::validator;
use tendermint_light_client::components::io::{AsyncIo, AtHeight};
use tendermint_proto::Protobuf;
use tendermint_proto::types::{SignedHeader, Validator};
use tendermint_rpc::{query::Query, Client, Order};
use ibc::core::ics02_client::msgs::update_client::MsgUpdateAnyClient;
use ibc::tx_msg::Msg;
use ibc_proto::ibc::core::client::v1::MsgUpdateClient;
use ics07_tendermint::client_def::TendermintClient;
use ics07_tendermint::client_message::ClientMessage;
use primitives::mock::LocalClientTypes;


pub enum FinalityEvent {
	Tendermint(Header),
}

pub struct TransactionId<Hash> {
	pub hash: Hash,
}

impl CosmosClient<H>
where
	H: Clone + Send + Sync + 'static
{
	async fn msg_update_client_header(&mut self, trusted_height: Height) -> Result<Header, anyhow::Error> {
		let latest_light_block = self.light_provider.fetch_light_block(AtHeight::Highest).await?;
		let trusted_light_block = self.light_provider.fetch_light_block(AtHeight(trusted_height)).await?;

		Ok(Header{
			signed_header: latest_light_block.signed_header,
			validator_set: latest_light_block.validators,
			trusted_height,
			trusted_validator_set: trusted_light_block.validators
		})
	}
}

#[async_trait::async_trait]
impl<H> IbcProvider for CosmosClient<H>
where
	H: Clone + Send + Sync + 'static,
{
	type FinalityEvent = FinalityEvent;
	type TransactionId = TransactionId<tendermint_rpc::abci::transaction::Hash>;
	type Error = Error;

	// Things to do
	// 1. query for ibc header
	// 2. query ibc events between last updated state on counterparty light client and current finalized block
	async fn query_latest_ibc_events<C>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &C,
	) -> Result<(Any, Vec<IbcEvent>, UpdateType), anyhow::Error>
	where
		C: Chain,
	{
		let client_id = counterparty.client_id();
		let latest_cp_height = counterparty.latest_height_and_timestamp().await?.0;
		let latest_cp_client_state = counterparty.query_client_state(latest_cp_height, client_id).await?;
		let client_state_response = latest_cp_client_state.client_state
			.ok_or_else(|| Error::Custom("counterparty returned empty client state".to_string()))?;

		let client_state = AnyClientState::try_from(client_state_response)
			.map_err(|err| Error::Custom("failed to decode client state response".to_string()))?;

		let cs = match client_state {
			AnyClientState::Tendermint(client_state) => client_state,
			c => Err(Error::Custom(format!("expected Tendermint::ClientState got {:?}", c)))?
		};

		let latest_cp_client_height = cs.latest_height.revision_height;
		let latest_height = self.rpc_client.latest_block().await?.block.header.height;

		let mut ibc_events: Vec<IbcEvent> = vec![];
		for height in latest_cp_client_height+1..latest_height.value()+1 {
			// todo()! maybe there's a more efficient way to query for blocks in batches?
			let block_results = self.rpc_client
				.block_results(height.into()).await
				.map_err(|e| Error::from(format!("Failed to query block result for height {:?}: {:?}", height, e)))?;

			let tx_results = block_results.txs_results.ok_or_else(|| Err(Error::Custom("empty transaction results".to_string())))?;
			for tx in tx_results.iter() {
				for event in tx.events {
					ibc_events.push(event.into());
				}
			}
		}

		let update_client_header = {
			let update_header = CosmosClient::msg_update_client_header(self, cs.latest_height).await?;
			let msg = MsgUpdateAnyClient::<LocalClientTypes> {
				client_id: self.client_id(),
				client_message: AnyClientMessage::Tendermint(ClientMessage::Header(update_header)),
				signer: counterparty.account_id(),
			};
			let value = msg.encode_vec();
			Any { value, type_url: msg.type_url() }
		};
		Ok((update_client_header, ibc_events, UpdateType::Mandatory))
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
			None =>
				return Err(Error::from(format!(
					"Failed to find tx hash: {:?}",
					tx_id.hash.to_string()
				))),
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
					IbcEvent::CreateClient(ref ev) => ev.client_id().clone(),
					IbcEvent::UpdateClient(ref ev) => ev.client_id().clone(),
					IbcEvent::UpgradeClient(ref ev) => ev.client_id().clone(),
					IbcEvent::ClientMisbehaviour(ref ev) => ev.client_id().clone(),
					_ => return Err(Error::from(format!("Unexpected event type"))),
				};
				Ok(client_id)
			}
		}
	}
}
