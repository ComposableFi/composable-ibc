use super::{error::Error, CosmosClient};
use core::time::Duration;
use futures::{
	stream::{self, select_all},
	Stream,
};
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
use primitives::{Chain, IbcProvider, UpdateType, mock::LocalClientTypes};
use std::pin::Pin;
use tendermint::block::{Height as BlockHeight};
use tendermint_light_client::components::io::{AsyncIo, AtHeight};
use tendermint_proto::Protobuf;
use tendermint_rpc::{
	query::{EventType, Query},
	event::{Event, EventData},
	Client, Order, SubscriptionClient, WebSocketClient,
};
use ibc::core::ics02_client::msgs::update_client::MsgUpdateAnyClient;
use ibc::tx_msg::Msg;
use ics07_tendermint::client_message::ClientMessage;
use crate::utils::{
	event_is_type_channel, event_is_type_client,
	event_is_type_connection, ibc_event_try_from_abci_event,
};

pub enum FinalityEvent {
	Tendermint(Header),
}

pub struct TransactionId<Hash> {
	pub hash: Hash,
}

impl<H> CosmosClient<H>
where
	H: Clone + Send + Sync + 'static
{
	async fn msg_update_client_header(&mut self, trusted_height: Height) -> Result<(Header, UpdateType), anyhow::Error> {
		let latest_light_block = self.light_provider.fetch_light_block(AtHeight::Highest).await?;
		let height = BlockHeight::try_from(trusted_height.revision_height)?;
		let trusted_light_block = self.light_provider.fetch_light_block(AtHeight::At(height)).await?;

		let update_type = match latest_light_block.validators == latest_light_block.next_validators {
			true => UpdateType::Optional,
			false => UpdateType::Mandatory
		};

		Ok((Header{
			signed_header: latest_light_block.signed_header,
			validator_set: latest_light_block.validators,
			trusted_height,
			trusted_validator_set: trusted_light_block.validators
		}, update_type))
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
					let ibc_event = ibc_event_try_from_abci_event(&event)?;
					ibc_events.push(ibc_event);
				}
			}
		}

		let update_header = CosmosClient::msg_update_client_header(self, cs.latest_height).await?;
		let update_client_header = {
			let msg = MsgUpdateAnyClient::<LocalClientTypes> {
				client_id: self.client_id(),
				client_message: AnyClientMessage::Tendermint(ClientMessage::Header(update_header.0)),
				signer: counterparty.account_id(),
			};
			let value = msg.encode_vec();
			Any { value, type_url: msg.type_url() }
		};
		Ok((update_client_header, ibc_events, update_header.1))
	}

	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent>>> {
		let (ws_client, ws_driver) = WebSocketClient::new(self.websocket_url.clone())
			.await
			.map_err(|e| Error::from(format!("Web Socket Client Error {:?}", e)))
			.unwrap();
		let driver_handle = std::thread::spawn(|| ws_driver.run());

		// ----
		let query_all = vec![
			Query::from(EventType::NewBlock),
			Query::eq("message.module", "ibc_client"),
			Query::eq("message.module", "ibc_connection"),
			Query::eq("message.module", "ibc_channel"),
		];

		let mut subscriptions = vec![];
		for query in &query_all {
			let subscription = ws_client
				.subscribe(query.clone())
				.await
				.map_err(|e| Error::from(format!("Web Socket Client Error {:?}", e)));
			subscriptions.push(subscription);
		}

		let all_subscribtions = Box::new(select_all(subscriptions));
		// Collect IBC events from each RPC event
		let events = all_subscribtions
			.map_ok(move |event| {
				let mut events: Vec<IbcEvent> = vec![];
				let Event { data, events, query } = event;
				match data {
					EventData::NewBlock { block, .. }
					if query == Query::from(EventType::NewBlock).to_string() =>
						{
							events.push(ClientEvents::NewBlock::new(height).into());
							// events_with_height.append(&mut extract_block_events(height, &events));
						},
					EventData::Tx { tx_result } => {
						for abci_event in &tx_result.result.events {
							if let Ok(ibc_event) = ibc_event_try_from_abci_event(abci_event) {
								if query == Query::eq("message.module", "ibc_client").to_string()
									&& event_is_type_client(&ibc_event)
								{
									events.push(ibc_event);
								} else if query
									== Query::eq("message.module", "ibc_connection").to_string()
									&& event_is_type_connection(&ibc_event)
								{
									events.push(ibc_event);
								} else if query
									== Query::eq("message.module", "ibc_channel").to_string()
									&& event_is_type_channel(&ibc_event)
								{
									events.push(ibc_event);
								}
							}
						}
					},
					_ => {},
				}
				stream::iter(events).map(Ok)
			})
			.map_err(|e| Error::from(format!("Web Socket Client Error {:?}", e)))
			.try_flatten();

		Pin::new(events)
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
