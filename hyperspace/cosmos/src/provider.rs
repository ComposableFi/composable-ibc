use super::{
	client::CosmosClient,
	events::{
		event_is_type_channel, event_is_type_client, event_is_type_connection,
		ibc_event_try_from_abci_event, IbcEventWithHeight,
	},
};
use crate::error::Error;
use futures::{
	stream::{self, select_all},
	Stream, StreamExt,
};
use ibc::{
	applications::transfer::{Amount, BaseDenom, PrefixedCoin, PrefixedDenom, TracePath},
	core::{
		ics02_client::{
			client_state::ClientType, events as ClientEvents,
			msgs::update_client::MsgUpdateAnyClient, trust_threshold::TrustThreshold,
		},
		ics04_channel::packet::Sequence,
		ics23_commitment::{commitment::CommitmentPrefix, specs::ProofSpecs},
		ics24_host::{
			identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
			path::{
				AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath,
				CommitmentsPath, ConnectionsPath, Path, ReceiptsPath, SeqRecvsPath,
			},
		},
	},
	events::IbcEvent,
	protobuf::Protobuf,
	timestamp::Timestamp,
	tx_msg::Msg,
	Height,
};
use ibc_primitives::PacketInfo as IbcPacketInfo;
use ibc_proto::{
	cosmos::bank::v1beta1::QueryBalanceRequest,
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{
			Channel, QueryChannelResponse, QueryChannelsRequest, QueryChannelsResponse,
			QueryConnectionChannelsRequest, QueryNextSequenceReceiveResponse,
			QueryPacketAcknowledgementResponse, QueryPacketAcknowledgementsRequest,
			QueryPacketCommitmentResponse, QueryPacketCommitmentsRequest,
			QueryPacketReceiptResponse, QueryUnreceivedAcksRequest, QueryUnreceivedPacketsRequest,
		},
		client::v1::{
			QueryClientStateResponse, QueryClientStatesRequest, QueryConsensusStateResponse,
		},
		connection::v1::{
			ConnectionEnd, IdentifiedConnection, QueryConnectionResponse, QueryConnectionsRequest,
		},
	},
};
use ibc_rpc::PacketInfo;
use ics07_tendermint::{
	client_message::ClientMessage, client_state::ClientState, consensus_state::ConsensusState,
};
use pallet_ibc::light_clients::{
	AnyClientMessage, AnyClientState, AnyConsensusState, HostFunctionsManager,
};
use primitives::{mock::LocalClientTypes, Chain, IbcProvider, UpdateType};
use prost::Message;
use std::{pin::Pin, str::FromStr, time::Duration};
use tendermint::block::Height as TmHeight;
pub use tendermint::Hash;
use tendermint_rpc::{
	endpoint::tx::Response,
	event::{Event, EventData},
	query::{EventType, Query},
	Client, Error as RpcError, Order, SubscriptionClient, WebSocketClient,
};
use tokio::time::sleep;

#[derive(Clone, Debug)]
pub enum FinalityEvent {
	Tendermint { from: TmHeight, to: TmHeight },
}

#[derive(Clone, Debug)]
pub struct TransactionId<Hash> {
	pub hash: Hash,
}

#[async_trait::async_trait]
impl<H> IbcProvider for CosmosClient<H>
where
	H: Clone + Send + Sync + 'static,
{
	type FinalityEvent = FinalityEvent;
	type TransactionId = TransactionId<Hash>;
	type AssetId = String;
	type Error = Error;

	async fn query_latest_ibc_events<C>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &C,
	) -> Result<(Vec<Any>, Vec<IbcEvent>, UpdateType), anyhow::Error>
	where
		C: Chain,
	{
		let finality_event_height = match finality_event {
			FinalityEvent::Tendermint { from: _, to } => to,
		};
		let client_id = self.client_id();
		let latest_cp_height = counterparty.latest_height_and_timestamp().await?.0;
		let latest_cp_client_state =
			counterparty.query_client_state(latest_cp_height, client_id.clone()).await?;
		let client_state_response = latest_cp_client_state
			.client_state
			.ok_or_else(|| Error::Custom("counterparty returned empty client state".to_string()))?;
		let client_state =
			ClientState::<HostFunctionsManager>::decode_vec(&client_state_response.value)
				.map_err(|_| Error::Custom("failed to decode client state response".to_string()))?;
		let latest_cp_client_height = client_state.latest_height().revision_height;
		let latest_height = self.latest_height_and_timestamp().await?.0;
		let latest_revision = latest_height.revision_number;

		let mut ibc_events: Vec<IbcEvent> = vec![];
		let from = TmHeight::try_from(latest_cp_client_height).unwrap();
		let to = finality_event_height;
		log::info!(target: "hyperspace_cosmos", "Getting blocks {}..{}", from, to);

		// query (exclusively) up to `to`, because the proof for the event at `to - 1` will be
		// contained at `to` and will be fetched below by `msg_update_client_header`
		for height in from.value()..to.value() {
			let block_results =
				self.rpc_client.block_results(TmHeight::try_from(height)?).await.map_err(|e| {
					Error::from(format!(
						"Failed to query block result for height {:?}: {:?}",
						height, e
					))
				})?;
			// TODO: handle block_results.begin_block_events, block_results.end_block_events?
			let tx_results = match block_results.txs_results {
				Some(tx) => tx,
				None => continue,
			};

			let ibc_height = Height::new(latest_revision, height);
			for tx in tx_results.iter() {
				for event in tx.clone().events {
					let ibc_event = ibc_event_try_from_abci_event(&event, ibc_height).ok();
					match ibc_event {
						Some(mut ev) => {
							ev.set_height(ibc_height);
							log::debug!(target: "hyperspace_cosmos", "Encountered event at {height}: {:?}", event.kind);
							ibc_events.push(ev);
						},
						None => {
							log::debug!(target: "hyperspace_cosmos", "Skipped event: {:?}", event.kind);
							continue
						},
					}
				}
			}
		}
		let updates = self.msg_update_client_header(from, to, client_state.latest_height).await?;
		let mut update_client_headers = Vec::new();
		let mut update_headers = Vec::new();
		for (update_header, update_type) in updates {
			log::info!(target: "hyperspace_cosmos", "Fetching block {}", update_header.height().revision_height);
			let update_client_header = {
				let msg = MsgUpdateAnyClient::<LocalClientTypes> {
					client_id: client_id.clone(),
					client_message: AnyClientMessage::Tendermint(ClientMessage::Header(
						update_header,
					)),
					signer: counterparty.account_id(),
				};
				let value = msg.encode_vec().map_err(|e| {
					Error::from(format!("Failed to encode MsgUpdateClient {:?}: {:?}", msg, e))
				})?;
				Any { value, type_url: msg.type_url() }
			};
			update_client_headers.push(update_client_header);
			update_headers.push(update_type);
		}
		Ok((update_client_headers, ibc_events, update_headers.remove(0)))
	}

	// TODO: Changed result: `Item =` from `IbcEvent` to `IbcEventWithHeight` to include the
	// necessary height field, as `height` is removed from `Attribute` from ibc-rs v0.22.0
	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>> {
		// Create websocket client. Like what `EventMonitor::subscribe()` does in `hermes`
		let (ws_client, ws_driver) = WebSocketClient::new(self.websocket_url.clone())
			.await
			.map_err(|e| Error::from(format!("Web Socket Client Error {:?}", e)))
			.unwrap();
		tokio::spawn(ws_driver.run());
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
				.map_err(|e| Error::from(format!("Web Socket Client Error {:?}", e)))
				.unwrap();
			subscriptions.push(subscription);
		}
		// Collect IBC events from each RPC event, Like what `stream_batches()` does in `hermes`
		let all_subs: Box<dyn Stream<Item = Result<Event, RpcError>> + Send + Sync + Unpin> =
			Box::new(select_all(subscriptions));
		let chain_id = self.chain_id.clone();
		let events = all_subs
			.map(move |event| {
				// Like what `get_all_events()` does in `hermes`
				let mut events_with_height: Vec<IbcEventWithHeight> = vec![];
				let Event { data, events: _, query } = event.unwrap();
				match data {
					EventData::NewBlock { block, .. }
						if query == Query::from(EventType::NewBlock).to_string() =>
					{
						let height = Height::new(
							ChainId::chain_version(chain_id.to_string().as_str()),
							u64::from(block.as_ref().ok_or("tx.height").unwrap().header.height),
						);
						events_with_height.push(IbcEventWithHeight::new(
							ClientEvents::NewBlock::new(height).into(),
							height,
						));
					},
					EventData::Tx { tx_result } => {
						let height = Height::new(
							ChainId::chain_version(chain_id.to_string().as_str()),
							tx_result.height as u64,
						);
						for abci_event in &tx_result.result.events {
							if let Ok(ibc_event) = ibc_event_try_from_abci_event(abci_event, height)
							{
								if query == Query::eq("message.module", "ibc_client").to_string() &&
									event_is_type_client(&ibc_event)
								{
									events_with_height
										.push(IbcEventWithHeight::new(ibc_event, height));
								} else if (query ==
									Query::eq("message.module", "ibc_connection").to_string() ||
									query ==
										Query::eq("message.module", "ibc_client").to_string()) &&
									event_is_type_connection(&ibc_event)
								{
									events_with_height
										.push(IbcEventWithHeight::new(ibc_event, height));
								} else if query ==
									Query::eq("message.module", "ibc_channel").to_string() &&
									event_is_type_channel(&ibc_event)
								{
									events_with_height
										.push(IbcEventWithHeight::new(ibc_event, height));
								}
							} else {
								log::debug!(target: "hyperspace_cosmos", "Failed to parse event {:?}", abci_event);
							}
						}
					},
					_ => {},
				}
				stream::iter(events_with_height)
			})
			.flatten()
			.map(|e| e.event)
			.boxed();
		events
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		let path_bytes = Path::ClientConsensusState(ClientConsensusStatePath {
			client_id: client_id.clone(),
			epoch: consensus_height.revision_number,
			height: consensus_height.revision_height,
		})
		.to_string()
		.into_bytes();
		let (query_result, proof) = self.query_path(path_bytes.clone(), at, true).await?;
		let consensus_state = Any::decode(&*query_result.value)?;
		Ok(QueryConsensusStateResponse {
			consensus_state: Some(consensus_state),
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		let path_bytes =
			Path::ClientState(ClientStatePath(client_id.clone())).to_string().into_bytes();
		let (q, proof) = self.query_path(path_bytes.clone(), at, true).await?;
		let client_state = Any::decode(&*q.value)?;
		Ok(QueryClientStateResponse {
			client_state: Some(client_state),
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		let path_bytes = Path::Connections(ConnectionsPath(connection_id.clone()))
			.to_string()
			.into_bytes();
		let (q, proof) = self.query_path(path_bytes.clone(), at, true).await?;
		let connection = ConnectionEnd::decode(&*q.value)?;
		Ok(QueryConnectionResponse {
			connection: Some(connection),
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		let path_bytes = Path::ChannelEnds(ChannelEndsPath(port_id.clone(), channel_id.clone()))
			.to_string()
			.into_bytes();
		let (q, proof) = self.query_path(path_bytes.clone(), at, true).await?;
		let channel = Channel::decode(&*q.value)?;
		Ok(QueryChannelResponse {
			channel: Some(channel),
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		let (_, proof) = self.query_path(keys[0].clone(), at, true).await?;
		Ok(proof)
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error> {
		let path_bytes = Path::Commitments(CommitmentsPath {
			port_id: port_id.clone(),
			channel_id: channel_id.clone(),
			sequence: Sequence::from(seq),
		})
		.to_string()
		.into_bytes();
		let (query_result, proof) = self.query_path(path_bytes.clone(), at, true).await?;
		Ok(QueryPacketCommitmentResponse {
			commitment: query_result.value,
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Self::Error> {
		let path_bytes = Path::Acks(AcksPath {
			port_id: port_id.clone(),
			channel_id: channel_id.clone(),
			sequence: Sequence::from(seq),
		})
		.to_string()
		.into_bytes();
		let (query_result, proof) = self.query_path(path_bytes, at, true).await?;
		Ok(QueryPacketAcknowledgementResponse {
			acknowledgement: query_result.value,
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		let path_bytes = Path::SeqRecvs(SeqRecvsPath(port_id.clone(), channel_id.clone()))
			.to_string()
			.into_bytes();
		let (query_result, proof) = self.query_path(path_bytes.clone(), at, true).await?;
		let next_sequence_receive = u64::from_be_bytes(
			query_result
				.value
				.try_into()
				.map_err(|_| Error::Custom("invalid next_sequence_receive value".to_owned()))?,
		);
		Ok(QueryNextSequenceReceiveResponse {
			next_sequence_receive,
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error> {
		let path_bytes = Path::Receipts(ReceiptsPath {
			port_id: port_id.clone(),
			channel_id: channel_id.clone(),
			sequence: Sequence::from(seq),
		})
		.to_string()
		.into_bytes();
		let (query_result, proof) = self.query_path(path_bytes, at, true).await?;
		let received = query_result.value[0] == 1;
		Ok(QueryPacketReceiptResponse {
			received,
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error> {
		// We cannot rely on `/status` endpoint to provide details about the latest block.
		// Instead, we need to pull block height via `/abci_info` and then fetch block
		// metadata at the given height via `/blockchain` endpoint.
		let abci_info = self
			.rpc_client
			.abci_info()
			.await
			.map_err(|e| Error::RpcError(format!("{:?}", e)))?;

		// Query `/blockchain` endpoint to pull the block metadata corresponding to
		// the latest block that the application committed.
		// TODO: Replace this query with `/header`, once it's available.
		//  https://github.com/informalsystems/tendermint-rs/pull/1101
		let blocks = self
			.rpc_client
			.blockchain(abci_info.last_block_height, abci_info.last_block_height)
			.await
			.map_err(|e| {
				Error::RpcError(format!(
					"failed to query /blockchain endpoint for latest app. block: {:?}",
					e
				))
			})?;

		let latest_app_block = blocks.block_metas.first().ok_or_else(|| {
			Error::Custom("/blockchain endpoint for latest app. block".to_owned())
		})?;

		let height = Height::new(
			ChainId::chain_version(latest_app_block.header.chain_id.as_str()),
			u64::from(abci_info.last_block_height),
		);
		let timestamp = latest_app_block.header.time.into();
		Ok((height, timestamp))
	}

	async fn query_packet_commitments(
		&self,
		_at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let mut grpc_client =
			ibc_proto::ibc::core::channel::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(e.to_string()))?;

		let request = QueryPacketCommitmentsRequest {
			port_id: port_id.to_string(),
			channel_id: channel_id.to_string(),
			pagination: None,
		};
		let request = tonic::Request::new(request);
		let response = grpc_client
			.packet_commitments(request)
			.await
			.map_err(|e| Error::from(e.to_string()))?
			.into_inner();

		let commitment_sequences: Vec<u64> =
			response.commitments.into_iter().map(|v| v.sequence.into()).collect();

		Ok(commitment_sequences)
	}

	async fn query_packet_acknowledgements(
		&self,
		_at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		log::debug!(
			target: "hyperspace_cosmos",
			"Querying packet acknowledgements for channel {:?} on port {:?}",
			channel_id,
			port_id
		);
		let mut grpc_client =
			ibc_proto::ibc::core::channel::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(e.to_string()))?;

		let request = QueryPacketAcknowledgementsRequest {
			port_id: port_id.to_string(),
			channel_id: channel_id.to_string(),
			packet_commitment_sequences: vec![],
			pagination: None,
		};
		let request = tonic::Request::new(request);
		let response = grpc_client
			.packet_acknowledgements(request)
			.await
			.map_err(|e| Error::from(e.to_string()))?
			.into_inner();

		let commitment_sequences: Vec<u64> =
			response.acknowledgements.into_iter().map(|v| v.sequence.into()).collect();

		Ok(commitment_sequences)
	}

	async fn query_unreceived_packets(
		&self,
		_at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let mut grpc_client =
			ibc_proto::ibc::core::channel::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(e.to_string()))?;

		let request = QueryUnreceivedPacketsRequest {
			port_id: port_id.to_string(),
			channel_id: channel_id.to_string(),
			packet_commitment_sequences: seqs.into(),
		};
		let request = tonic::Request::new(request);
		let response = grpc_client
			.unreceived_packets(request)
			.await
			.map_err(|e| Error::from(e.to_string()))?
			.into_inner();

		let commitment_sequences: Vec<u64> =
			response.sequences.into_iter().map(|v| v.into()).collect();

		Ok(commitment_sequences)
	}

	async fn query_unreceived_acknowledgements(
		&self,
		_at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let mut grpc_client =
			ibc_proto::ibc::core::channel::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(e.to_string()))?;

		let request = QueryUnreceivedAcksRequest {
			port_id: port_id.to_string(),
			channel_id: channel_id.to_string(),
			packet_ack_sequences: seqs.into(),
		};
		let request = tonic::Request::new(request);
		let response = grpc_client
			.unreceived_acks(request)
			.await
			.map_err(|e| Error::from(e.to_string()))?
			.into_inner();

		let commitment_sequences: Vec<u64> =
			response.sequences.into_iter().map(|v| v.into()).collect();

		Ok(commitment_sequences)
	}

	fn channel_whitelist(&self) -> Vec<(ChannelId, PortId)> {
		self.channel_whitelist.clone()
	}

	async fn query_connection_channels(
		&self,
		_at: Height,
		connection_id: &ConnectionId,
	) -> Result<QueryChannelsResponse, Self::Error> {
		let mut grpc_client =
			ibc_proto::ibc::core::channel::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(format!("{:?}", e)))?;
		let request = tonic::Request::new(QueryConnectionChannelsRequest {
			connection: connection_id.to_string(),
			pagination: None,
		});

		let response = grpc_client
			.connection_channels(request)
			.await
			.map_err(|e| Error::from(format!("{:?}", e)))?
			.into_inner();
		let channels = QueryChannelsResponse {
			channels: response.channels,
			pagination: response.pagination,
			height: response.height,
		};

		Ok(channels)
	}

	async fn query_send_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		log::debug!(
			target: "hyperspace_cosmos",
			"query_send_packets: channel_id: {}, port_id: {}, seqs: {:?}", channel_id, port_id, seqs
		);
		let mut block_events = vec![];

		for seq in seqs {
			let query_str =
				Query::eq(format!("{}.packet_src_channel", "send_packet"), channel_id.to_string())
					.and_eq(format!("{}.packet_src_port", "send_packet"), port_id.to_string())
					.and_eq(format!("{}.packet_sequence", "send_packet"), seq.to_string());

			let response = self
				.rpc_client
				.tx_search(
					query_str,
					true,
					1,
					1, // get only the first Tx matching the query
					Order::Ascending,
				)
				.await
				.map_err(|e| Error::RpcError(format!("{:?}", e)))?;

			for tx in response.txs {
				for ev in &tx.tx_result.events {
					let height = tx.height.value();
					let ev =
						ibc_event_try_from_abci_event(ev, Height::new(self.id().version(), height));

					match ev {
						Ok(IbcEvent::SendPacket(p)) => {
							let mut info = PacketInfo::try_from(IbcPacketInfo::from(p.packet))
								.map_err(|_| {
									Error::from(format!(
										"failed to convert packet info from IbcPacketInfo",
									))
								})?;
							info.height = p.height.revision_height;
							block_events.push(info)
						},
						_ => (),
					}
				}
			}
		}
		Ok(block_events)
	}

	async fn query_recv_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		log::debug!(
			target: "hyperspace_cosmos",
			"query_recv_packets: channel_id: {}, port_id: {}, seqs: {:?}", channel_id, port_id, seqs
		);
		let mut block_events = vec![];

		for seq in seqs {
			let query_str =
				Query::eq(format!("{}.packet_src_channel", "recv_packet"), channel_id.to_string())
					.and_eq(format!("{}.packet_src_port", "recv_packet"), port_id.to_string())
					.and_eq(format!("{}.packet_sequence", "recv_packet"), seq.to_string());

			let response = self
				.rpc_client
				.tx_search(
					query_str,
					true,
					1,
					1, // get only the first Tx matching the query
					Order::Ascending,
				)
				.await
				.map_err(|e| Error::RpcError(format!("{:?}", e)))?;

			for tx in response.txs {
				for ev in &tx.tx_result.events {
					let height = tx.height.value();
					let ev =
						ibc_event_try_from_abci_event(ev, Height::new(self.id().version(), height));
					match ev {
						Ok(IbcEvent::WriteAcknowledgement(p)) => {
							let mut info = PacketInfo::try_from(IbcPacketInfo::from(p.packet))
								.map_err(|_| {
									Error::from(format!(
										"failed to convert packet info from IbcPacketInfo",
									))
								})?;
							info.ack = Some(p.ack);
							info.height = p.height.revision_height;
							block_events.push(info)
						},
						_ => (),
					}
				}
			}
		}
		Ok(block_events)
	}

	fn expected_block_time(&self) -> Duration {
		Duration::from_secs(30)
	}

	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, Timestamp), Self::Error> {
		log::debug!(
			target: "hyperspace_cosmos",
			"Querying client update time and height for client {:?} at height {:?}",
			client_id,
			client_height
		);
		let query_str = Query::eq("update_client.client_id", client_id.to_string());

		let response = self
			.rpc_client
			.tx_search(
				query_str,
				true,
				1,
				1, // get only the first Tx matching the query
				Order::Ascending,
			)
			.await
			.map_err(|e| Error::RpcError(format!("{:?}", e)))?;

		for tx in response.txs {
			for ev in &tx.tx_result.events {
				let height = tx.height.value();
				let ev =
					ibc_event_try_from_abci_event(ev, Height::new(self.id().version(), height));
				let timestamp = self.query_timestamp_at(height).await?;
				match ev {
					Ok(IbcEvent::UpdateClient(_)) =>
						return Ok((
							Height::new(self.chain_id.version(), height),
							Timestamp::from_nanoseconds(timestamp)?,
						)),
					_ => (),
				}
			}
		}

		Err(Error::from("not found".to_string()))
	}

	async fn query_host_consensus_state_proof(
		&self,
		_height: Height,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		unimplemented!()
	}

	async fn query_ibc_balance(
		&self,
		asset_id: Self::AssetId,
	) -> Result<Vec<PrefixedCoin>, Self::Error> {
		let denom = &asset_id;
		let mut grpc_client = ibc_proto::cosmos::bank::v1beta1::query_client::QueryClient::connect(
			self.grpc_url.clone().to_string(),
		)
		.await
		.map_err(|e| Error::from(format!("{:?}", e)))?;

		let request = tonic::Request::new(QueryBalanceRequest {
			address: self.keybase.clone().account,
			denom: denom.to_string(),
		});

		let response = grpc_client
			.balance(request)
			.await
			.map(|r| r.into_inner())
			.map_err(|e| Error::from(format!("{:?}", e)))?;

		// Querying for a balance might fail, i.e. if the account doesn't actually exist
		let balance = response
			.balance
			.ok_or_else(|| Error::from(format!("No balance for denom {}", denom)))?;

		Ok(vec![PrefixedCoin {
			denom: PrefixedDenom {
				trace_path: TracePath::default(),
				base_denom: BaseDenom::from_str(denom)?,
			},
			amount: Amount::from_str(balance.amount.as_str())?,
		}])
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		CommitmentPrefix::try_from(self.commitment_prefix.clone()).expect("Should not fail")
	}

	fn client_id(&self) -> ClientId {
		self.client_id()
	}

	fn connection_id(&self) -> ConnectionId {
		self.connection_id.as_ref().expect("Connection id should be defined").clone()
	}

	fn client_type(&self) -> ClientType {
		ClientState::<()>::client_type()
	}

	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
		let height = TmHeight::try_from(block_number)
			.map_err(|e| Error::from(format!("Invalid block number: {}", e)))?;
		let response = self
			.rpc_client
			.block(height)
			.await
			.map_err(|e| Error::RpcError(e.to_string()))?;
		let time: Timestamp = response.block.header.time.into();
		Ok(time.nanoseconds())
	}

	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
		let request = tonic::Request::new(QueryClientStatesRequest { pagination: None });
		let grpc_client = ibc_proto::ibc::core::client::v1::query_client::QueryClient::connect(
			self.grpc_url.clone().to_string(),
		)
		.await
		.map_err(|e| Error::RpcError(format!("{:?}", e)))?;
		let response = grpc_client
			.clone()
			.client_states(request)
			.await
			.map_err(|e| {
				Error::from(format!("Failed to query client states from grpc client: {:?}", e))
			})?
			.into_inner();

		// Deserialize into domain type
		let clients: Vec<ClientId> = response
			.client_states
			.into_iter()
			.filter_map(|cs| {
				let id = ClientId::from_str(&cs.client_id).ok()?;
				Some(id)
			})
			.collect();
		Ok(clients)
	}

	async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
		let request = tonic::Request::new(QueryChannelsRequest { pagination: None });
		let mut grpc_client =
			ibc_proto::ibc::core::channel::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(format!("{:?}", e)))?;
		let response = grpc_client
			.channels(request)
			.await
			.map_err(|e| Error::from(format!("{:?}", e)))?
			.into_inner()
			.channels
			.into_iter()
			.filter_map(|c| {
				let id = ChannelId::from_str(&c.channel_id).ok()?;
				let port_id = PortId::from_str(&c.port_id).ok()?;
				Some((id, port_id))
			})
			.collect::<Vec<_>>();
		Ok(response)
	}

	async fn query_connection_using_client(
		&self,
		_height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>, Self::Error> {
		let mut grpc_client =
			ibc_proto::ibc::core::connection::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(format!("{:?}", e)))?;

		let request = tonic::Request::new(QueryConnectionsRequest { pagination: None });

		let response = grpc_client
			.connections(request)
			.await
			.map_err(|e| Error::from(format!("{:?}", e)))?
			.into_inner();

		let connections = response
			.connections
			.into_iter()
			.filter_map(|co| {
				IdentifiedConnection::try_from(co.clone())
					.map_err(|e| Error::from(format!("Failed to convert connection end: {:?}", e)))
					.ok()
			})
			.filter(|conn| {
				conn.client_id == client_id ||
					conn.counterparty.as_ref().map(|x| x.client_id == client_id).unwrap_or(false)
			})
			.collect();
		Ok(connections)
	}

	async fn is_update_required(
		&self,
		_latest_height: u64,
		_latest_client_height_on_counterparty: u64,
	) -> Result<bool, Self::Error> {
		// TODO: Implement is_update_required
		Ok(false)
	}

	async fn initialize_client_state(
		&self,
	) -> Result<(AnyClientState, AnyConsensusState), Self::Error> {
		let latest_height_timestamp = self.latest_height_and_timestamp().await?;
		let client_state = ClientState::new(
			self.chain_id.clone(),
			TrustThreshold::default(),
			Duration::from_secs(64000),
			Duration::from_secs(1814400),
			Duration::new(15, 0),
			latest_height_timestamp.0,
			ProofSpecs::default(),
			vec!["upgrade".to_string(), "upgradedIBCState".to_string()],
		)
		.map_err(|e| Error::from(format!("Invalid client state {}", e)))?;
		let light_block = self
			.light_client
			.verify(latest_height_timestamp.0, latest_height_timestamp.0, &client_state)
			.await
			.map_err(|e| Error::from(format!("Invalid light block {}", e)))?;
		let consensus_state = ConsensusState::from(light_block.clone().signed_header.header);
		Ok((
			AnyClientState::Tendermint(client_state),
			AnyConsensusState::Tendermint(consensus_state),
		))
	}

	async fn query_client_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ClientId, Self::Error> {
		const WAIT_BACKOFF: Duration = Duration::from_millis(300);
		const TIME_OUT: Duration = Duration::from_millis(30000);
		let start_time = std::time::Instant::now();

		let response: Response = loop {
			let response = self
				.rpc_client
				.tx_search(
					Query::eq("tx.hash", tx_id.hash.to_string()),
					false,
					1,
					1, // get only the first Tx matching the query
					Order::Ascending,
				)
				.await
				.map_err(|e| Error::from(format!("Failed to query tx hash: {}", e)))?;
			match response.txs.into_iter().next() {
				None => {
					let elapsed = start_time.elapsed();
					if &elapsed > &TIME_OUT {
						return Err(Error::from(format!(
							"Timeout waiting for tx {:?} to be included in a block",
							tx_id.hash
						)))
					} else {
						std::thread::sleep(WAIT_BACKOFF);
					}
				},
				Some(resp) => break resp,
			}
		};

		let height = Height::new(
			ChainId::chain_version(self.chain_id.to_string().as_str()),
			response.height.value(),
		);
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
				.flat_map(|e| ibc_event_try_from_abci_event(e, height).ok().into_iter())
				.filter(|e| matches!(e, IbcEvent::CreateClient(_)))
				.collect::<Vec<_>>();
			if result.clone().len() != 1 {
				Err(Error::from(format!(
					"Expected exactly one CreateClient event, found {}",
					result.len()
				)))
			} else {
				Ok(match result[0] {
					IbcEvent::CreateClient(ref e) => e.client_id().clone(),
					_ => unreachable!(),
				})
			}
		}
	}
}

impl<H: Clone + Send + Sync + 'static> CosmosClient<H> {
	#[allow(unused)]
	async fn wait_for_tx_result(
		&self,
		tx_id: <Self as IbcProvider>::TransactionId,
	) -> Result<Response, <Self as IbcProvider>::Error> {
		const WAIT_BACKOFF: Duration = Duration::from_millis(300);
		const TIME_OUT: Duration = Duration::from_millis(30000);
		let start_time = std::time::Instant::now();

		let response: Response = loop {
			let response = self
				.rpc_client
				.tx_search(
					Query::eq("tx.hash", tx_id.hash.to_string()),
					false,
					1,
					1, // get only the first Tx matching the query
					Order::Ascending,
				)
				.await
				.map_err(|e| Error::from(format!("Failed to query tx hash: {}", e)))?;
			match response.txs.into_iter().next() {
				None => {
					let elapsed = start_time.elapsed();
					if &elapsed > &TIME_OUT {
						return Err(Error::from(format!(
							"Timeout waiting for tx {:?} to be included in a block",
							tx_id.hash
						)))
					} else {
						sleep(WAIT_BACKOFF).await;
					}
				},
				Some(resp) => break resp,
			}
		};

		let deliver_tx_result = &response.tx_result;
		if deliver_tx_result.code.is_err() {
			Err(Error::from(format!(
				"Transaction failed with code {:?} and log {:?}",
				deliver_tx_result.code, deliver_tx_result.log
			)))
		} else {
			Ok(response)
		}
	}
}

fn increment_proof_height(
	height: Option<ibc_proto::ibc::core::client::v1::Height>,
) -> Option<ibc_proto::ibc::core::client::v1::Height> {
	match height {
		Some(height) => Some(ibc_proto::ibc::core::client::v1::Height {
			revision_height: height.revision_height + 1,
			..height
		}),
		None => None,
	}
}