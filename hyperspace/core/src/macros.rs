// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_export]
macro_rules! process_finality_event {
	($source:ident, $sink:ident, $metrics:expr, $mode:ident, $result:ident, $stream_source:ident, $stream_sink:ident) => {
		match $result {
			// stream closed
			None => {
				log::warn!("Stream closed for {}", $source.name());
				$stream_source = loop {
					match $source.finality_notifications().await {
						Ok(stream) => break stream,
						Err(e) => {
							log::error!("Failed to get finality notifications for {} {:?}. Trying again in 30 seconds...", $source.name(), e);
							tokio::time::sleep(std::time::Duration::from_secs(30)).await;
						},
					};
				};
				$stream_sink = loop {
					match $sink.finality_notifications().await {
						Ok(stream) => break stream,
						Err(e) => {
							log::error!("Failed to get finality notifications for {} {:?}. Trying again in 30 seconds...", $sink.name(), e);
							tokio::time::sleep(std::time::Duration::from_secs(30)).await;
						},
					};
				};
			},
			Some(finality_event) => {
				log::info!("=======================================================");
				log::info!("Received finality notification from {}", $source.name());
				let sink_initial_rpc_call_delay = $sink.rpc_call_delay();
				let source_initial_rpc_call_delay = $source.rpc_call_delay();
				// TODO: make better error handling
				let mut had_error = false;

				let updates = match $source.query_latest_ibc_events(finality_event, &$sink).await {
					Ok(resp) => resp,
					Err(err) => {
						log::error!(
							"Failed to fetch IBC events for finality event for {} {:?}",
							$source.name(),
							err
						);
						match $sink.handle_error(&err).and_then(|_| $source.handle_error(&err)).await {
							Ok(_) => {},
							Err(e) => log::error!("Failed to handle error for {} {:?}", $sink.name(), e),
						}
						continue
					},
				};
				log::trace!(target: "hyperspace", "Received updates count: {}", updates.len());
				// query packets that can now be sent, at this sink height because of connection
				// delay.
				let (ready_packets, timeout_msgs) =
					crate::packets::query_ready_and_timed_out_packets(&$source, &$sink).await?;

				let mut msgs = Vec::new();

				for (msg_update_client, events, update_type) in updates {
					if let Some(metrics) = $metrics.as_mut() {
						if let Err(e) = metrics.handle_events(events.as_slice()).await {
							log::error!("Failed to handle metrics for {} {:?}", $source.name(), e);
						}
					}
					let event_types = events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>();
					let mut messages =
						match parse_events(&mut $source, &mut $sink, events, $mode).await {
							Ok(msgs) => msgs,
							Err(e) => {
								log::error!("Failed to parse events for {} {:?}", $source.name(), e);
								match $sink.handle_error(&e).and_then(|_| $source.handle_error(&e)).await {
									Ok(_) => {},
									Err(e) => log::error!("Failed to handle error for {} {:?}", $sink.name(), e),
								}
								had_error = true;
								continue
							},
					};
					log::trace!(target: "hyperspace", "Received messages count: {}, timeouts count: {}, is the update optional: {}, has undelivered packets: {}", messages.len(), timeout_msgs.len(), update_type.is_optional(), $source.has_undelivered_sequences());

					// We want to send client update if packet messages exist but where not sent due
					// to a connection delay even if client update message is optional
					match (
						// TODO: we actually man send only when timeout of some packet has reached,
						// not when we have *any* undelivered packets. But this requires rewriting
						// `find_suitable_proof_height_for_client` function, that uses binary
						// search, which won't work in this case
						update_type.is_optional() && !$source.has_undelivered_sequences(),
						has_packet_events(&event_types),
						messages.is_empty(),
					) {
						(true, false, true) => {
							// skip sending ibc messages if no new events
							log::info!("Skipping finality notification for {}", $sink.name());
							continue
						},
						(false, _, true) => log::info!(
							"Sending mandatory client update message for {}",
							$sink.name()
						),
						_ => log::info!(
							"Received finalized events from: {} {event_types:#?}",
							$source.name()
						),
					};
					msgs.push(msg_update_client);
					msgs.append(&mut messages);
				}
				msgs.extend(ready_packets);

				if !msgs.is_empty() {
					if let Some(metrics) = $metrics.as_ref() {
						metrics.handle_messages(msgs.as_slice()).await;
					}
					let type_urls =
						msgs.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
					log::info!("Submitting messages to {}: {type_urls:#?}", $sink.name());
					match queue::flush_message_batch(msgs, $metrics.as_ref(), &$sink).await {
						Ok(_) => {
							log::trace!(target: "hyperspace", "Successfully submitted messages to {}", $sink.name());
						},
						Err(e) => {
							log::error!(
								target:"hyperspace",
								"Failed to submit messages to {} {:?}",
								$sink.name(),
								e
							);
							match $sink.handle_error(&e).and_then(|_| $source.handle_error(&e)).await {
								Ok(_) => {},
								Err(e) => log::error!("Failed to handle error for {} {:?}", $sink.name(), e),
							}
							had_error = true;
						},
					}
				}

				if !timeout_msgs.is_empty() {
					if let Some(metrics) = $metrics.as_ref() {
						metrics.handle_timeouts(timeout_msgs.as_slice()).await;
					}
					let type_urls =
						timeout_msgs.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
					log::info!(
						"Submitting timeout messages to {}: {type_urls:#?}",
						$source.name()
					);
					match queue::flush_message_batch(timeout_msgs, $metrics.as_ref(), &$source).await {
						Ok(_) => {
							log::trace!(target: "hyperspace", "Successfully submitted timeout messages to {}", $source.name());
						},
						Err(e) => {
							log::error!(
								target:"hyperspace",
								"Failed to submit timeout messages to {} {:?}",
								$source.name(),
								e
							);
							match $sink.handle_error(&e).and_then(|_| $source.handle_error(&e)).await {
								Ok(_) => {},
								Err(e) => log::error!("Failed to handle error for {} {:?}", $sink.name(), e),
							}
							had_error = true;
						},
					}
				}
				if !had_error {
					$sink.set_rpc_call_delay(sink_initial_rpc_call_delay);
					$source.set_rpc_call_delay(source_initial_rpc_call_delay);
				}
			},
		}
	};
}

#[macro_export]
macro_rules! chains {
	($(
        $(#[$($meta:meta)*])*
		$name:ident($config:path, $client:path),
	)*) => {
		#[derive(Debug, Serialize, Deserialize, Clone)]
		#[serde(tag = "type", rename_all = "snake_case")]
		pub enum AnyConfig {
			$(
				$(#[$($meta)*])*
				$name($config),
			)*
		}

		#[derive(Clone)]
		pub enum AnyChain {
			$(
				$(#[$($meta)*])*
				$name($client),
			)*
			Wasm(WasmChain),
		}

		#[derive(Debug)]
		pub enum AnyFinalityEvent {
			$(
				$(#[$($meta)*])*
				$name(<$client as IbcProvider>::FinalityEvent),
			)*
		}

		#[derive(Clone)]
		pub enum AnyAssetId {
			$(
				$(#[$($meta)*])*
				$name(<$client as IbcProvider>::AssetId),
			)*
		}

		#[derive(Debug)]
		pub enum AnyTransactionId {
			$(
				$(#[$($meta)*])*
				$name(<$client as IbcProvider>::TransactionId),
			)*
		}

		#[derive(Error, Debug)]
		pub enum AnyError {
			$(
				$(#[$($meta)*])*
				#[error("{0}")]
				$name(<$client as IbcProvider>::Error),
			)*
			#[error("{0}")]
			Other(String),
		}

		impl From<anyhow::Error> for AnyError {
			fn from(e: anyhow::Error) -> Self {
				Self::Other(e.to_string())
			}
		}

		#[async_trait]
		impl IbcProvider for AnyChain {
			type FinalityEvent = AnyFinalityEvent;
			type TransactionId = AnyTransactionId;
			type Error = AnyError;
			type AssetId = AnyAssetId;

			async fn query_latest_ibc_events<T>(
				&mut self,
				finality_event: Self::FinalityEvent,
				counterparty: &T,
			) -> Result<Vec<(Any, Vec<IbcEvent>, UpdateType)>, anyhow::Error>
			where
				T: Chain,
			{
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => {
							let finality_event = downcast!(finality_event => AnyFinalityEvent::$name)
								.ok_or_else(|| AnyError::Other("Invalid finality event type".to_owned()))?;
							chain.query_latest_ibc_events(finality_event, counterparty).await
						}
					)*
					AnyChain::Wasm(c) =>
						c.inner.query_latest_ibc_events(finality_event, counterparty).await,
				}
			}

			async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.ibc_events().await,
					)*
					Self::Wasm(c) => c.inner.ibc_events().await,
				}
			}

			async fn query_client_consensus(
				&self,
				at: Height,
				client_id: ClientId,
				consensus_height: Height,
			) -> Result<QueryConsensusStateResponse, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_client_consensus(at, client_id, consensus_height)
							.await
							.map_err(AnyError::$name),
					)*
					AnyChain::Wasm(c) =>
						c.inner.query_client_consensus(at, client_id, consensus_height).await,
				}
			}

			async fn query_client_state(
				&self,
				at: Height,
				client_id: ClientId,
			) -> Result<QueryClientStateResponse, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_client_state(at, client_id)
							.await
							.map_err(AnyError::$name),
					)*
					AnyChain::Wasm(c) => c.inner.query_client_state(at, client_id).await,
				}
			}

			async fn query_connection_end(
				&self,
				at: Height,
				connection_id: ConnectionId,
			) -> Result<QueryConnectionResponse, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_connection_end(at, connection_id)
							.await
							.map_err(AnyError::$name),
					)*
					AnyChain::Wasm(c) => c.inner.query_connection_end(at, connection_id).await,
				}
			}

			async fn query_channel_end(
				&self,
				at: Height,
				channel_id: ChannelId,
				port_id: PortId,
			) -> Result<QueryChannelResponse, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_channel_end(at, channel_id, port_id)
							.await
							.map_err(AnyError::$name),
					)*
					AnyChain::Wasm(c) => c.inner.query_channel_end(at, channel_id, port_id).await,
				}
			}

			async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_proof(at, keys)
							.await
							.map_err(AnyError::$name),
					)*
					AnyChain::Wasm(c) => c.inner.query_proof(at, keys).await,
				}
			}

			async fn query_packet_commitment(
				&self,
				at: Height,
				port_id: &PortId,
				channel_id: &ChannelId,
				seq: u64,
			) -> Result<QueryPacketCommitmentResponse, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_packet_commitment(at, port_id, channel_id, seq)
							.await
							.map_err(AnyError::$name),
					)*
					AnyChain::Wasm(c) =>
						c.inner.query_packet_commitment(at, port_id, channel_id, seq).await,
				}
			}

			async fn query_packet_acknowledgement(
				&self,
				at: Height,
				port_id: &PortId,
				channel_id: &ChannelId,
				seq: u64,
			) -> Result<QueryPacketAcknowledgementResponse, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_packet_acknowledgement(at, port_id, channel_id, seq)
							.await
							.map_err(AnyError::$name),
					)*
					AnyChain::Wasm(c) =>
						c.inner.query_packet_acknowledgement(at, port_id, channel_id, seq).await,
				}
			}

			async fn query_next_sequence_recv(
				&self,
				at: Height,
				port_id: &PortId,
				channel_id: &ChannelId,
			) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_next_sequence_recv(at, port_id, channel_id)
							.await
							.map_err(AnyError::$name),
					)*
					AnyChain::Wasm(c) => c.inner.query_next_sequence_recv(at, port_id, channel_id).await,
				}
			}

			async fn query_packet_receipt(
				&self,
				at: Height,
				port_id: &PortId,
				channel_id: &ChannelId,
				seq: u64,
			) -> Result<QueryPacketReceiptResponse, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_packet_receipt(at, port_id, channel_id, seq)
							.await
							.map_err(AnyError::$name),
					)*
					AnyChain::Wasm(c) => c.inner.query_packet_receipt(at, port_id, channel_id, seq).await,
				}
			}

			async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.latest_height_and_timestamp()
							.await
							.map_err(AnyError::$name),
					)*
					AnyChain::Wasm(c) => c.inner.latest_height_and_timestamp().await,
				}
			}

			async fn query_packet_commitments(
				&self,
				at: Height,
				channel_id: ChannelId,
				port_id: PortId,
			) -> Result<Vec<u64>, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_packet_commitments(at, channel_id, port_id)
							.await
							.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_packet_commitments(at, channel_id, port_id).await,
				}
			}

			async fn query_packet_acknowledgements(
				&self,
				at: Height,
				channel_id: ChannelId,
				port_id: PortId,
			) -> Result<Vec<u64>, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_packet_acknowledgements(at, channel_id, port_id)
							.await
							.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_packet_acknowledgements(at, channel_id, port_id).await,
				}
			}

			async fn query_unreceived_packets(
				&self,
				at: Height,
				channel_id: ChannelId,
				port_id: PortId,
				seqs: Vec<u64>,
			) -> Result<Vec<u64>, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_unreceived_packets(at, channel_id, port_id, seqs)
							.await
							.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_unreceived_packets(at, channel_id, port_id, seqs).await,
				}
			}

			async fn query_unreceived_acknowledgements(
				&self,
				at: Height,
				channel_id: ChannelId,
				port_id: PortId,
				seqs: Vec<u64>,
			) -> Result<Vec<u64>, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_unreceived_acknowledgements(at, channel_id, port_id, seqs)
							.await
							.map_err(AnyError::$name),
					)*
					Self::Wasm(c) =>
						c.inner.query_unreceived_acknowledgements(at, channel_id, port_id, seqs).await,
				}
			}

			fn channel_whitelist(&self) -> std::collections::HashSet<(ChannelId, PortId)> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.channel_whitelist(),
					)*
					Self::Wasm(c) => c.inner.channel_whitelist(),
				}
			}

			async fn query_connection_channels(
				&self,
				at: Height,
				connection_id: &ConnectionId,
			) -> Result<QueryChannelsResponse, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_connection_channels(at, connection_id)
							.await
							.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_connection_channels(at, connection_id).await,
				}
			}

			async fn query_send_packets(
				&self,
				channel_id: ChannelId,
				port_id: PortId,
				seqs: Vec<u64>,
			) -> Result<Vec<ibc_rpc::PacketInfo>, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_send_packets(channel_id, port_id, seqs)
							.await
							.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_send_packets(channel_id, port_id, seqs).await,
				}
			}

			async fn query_recv_packets(
				&self,
				channel_id: ChannelId,
				port_id: PortId,
				seqs: Vec<u64>,
			) -> Result<Vec<ibc_rpc::PacketInfo>, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_recv_packets(channel_id, port_id, seqs)
							.await
							.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_recv_packets(channel_id, port_id, seqs).await,
				}
			}

			fn expected_block_time(&self) -> Duration {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.expected_block_time(),
					)*
					Self::Wasm(c) => c.inner.expected_block_time(),
				}
			}

			async fn query_client_update_time_and_height(
				&self,
				client_id: ClientId,
				client_height: Height,
			) -> Result<(Height, Timestamp), Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_client_update_time_and_height(client_id, client_height)
							.await
							.map_err(AnyError::$name),
					)*
					Self::Wasm(c) =>
						c.inner.query_client_update_time_and_height(client_id, client_height).await,
				}
			}

			async fn query_host_consensus_state_proof(
				&self,
				client_state: &AnyClientState,
			) -> Result<Option<Vec<u8>>, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_host_consensus_state_proof(client_state)
							.await
							.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_host_consensus_state_proof(client_state).await,
				}
			}

			async fn query_ibc_balance(
				&self,
				asset_id: AnyAssetId,
			) -> Result<Vec<PrefixedCoin>, Self::Error> {
				match (self, asset_id) {
					$(
						$(#[$($meta)*])*
						(Self::$name(chain), AnyAssetId::$name(asset_id)) =>
							chain.query_ibc_balance(asset_id.into()).await.map_err(AnyError::$name),
					)*
					(Self::Wasm(c), asset_id) => c.inner.query_ibc_balance(asset_id).await,
					(chain, _) => panic!("query_ibc_balance is not implemented for {}", chain.name()),
				}
			}

			fn connection_prefix(&self) -> CommitmentPrefix {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.connection_prefix(),
					)*
					AnyChain::Wasm(c) => c.inner.connection_prefix(),
				}
			}

			fn client_id(&self) -> ClientId {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.client_id(),
					)*
					AnyChain::Wasm(c) => c.inner.client_id(),
				}
			}

			fn set_client_id(&mut self, client_id: ClientId) {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.set_client_id(client_id),
					)*
					Self::Wasm(c) => c.inner.set_client_id(client_id),
				}
			}

			fn connection_id(&self) -> Option<ConnectionId> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.connection_id(),
					)*
					AnyChain::Wasm(c) => c.inner.connection_id(),
				}
			}

			fn client_type(&self) -> ClientType {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.client_type(),
					)*
					AnyChain::Wasm(c) => c.inner.client_type(),
				}
			}

			async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.query_timestamp_at(block_number).await.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_timestamp_at(block_number).await,
				}
			}

			async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.query_clients().await.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_clients().await,
				}
			}

			async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.query_channels().await.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_channels().await,
				}
			}

			async fn query_connection_using_client(
				&self,
				height: u32,
				client_id: String,
			) -> Result<Vec<IdentifiedConnection>, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) =>
							chain.query_connection_using_client(height, client_id).await.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_connection_using_client(height, client_id).await,
				}
			}

			async fn is_update_required(
				&self,
				latest_height: u64,
				latest_client_height_on_counterparty: u64,
			) -> Result<bool, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.is_update_required(latest_height, latest_client_height_on_counterparty)
							.await
							.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c
						.inner
						.is_update_required(latest_height, latest_client_height_on_counterparty)
						.await
						.map_err(Into::into),
				}
			}

			async fn initialize_client_state(
				&self,
			) -> Result<(AnyClientState, AnyConsensusState), Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.initialize_client_state().await.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.initialize_client_state().await,
				}
			}

			async fn query_client_id_from_tx_hash(
				&self,
				tx_id: Self::TransactionId,
			) -> Result<ClientId, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_client_id_from_tx_hash(
								downcast!(tx_id => AnyTransactionId::$name)
									.expect("Should be $name transaction id"),
							)
							.await
							.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_client_id_from_tx_hash(tx_id).await,
				}
			}

			async fn upload_wasm(&self, wasm: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.upload_wasm(wasm).await.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.upload_wasm(wasm).await,
				}
			}

			async fn on_undelivered_sequences(&self, seqs: &[u64]) -> Result<(), Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.on_undelivered_sequences(seqs).await.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.on_undelivered_sequences(seqs).await,
				}
			}

			fn has_undelivered_sequences(&self) -> bool {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.has_undelivered_sequences(),
					)*
					Self::Wasm(c) => c.inner.has_undelivered_sequences(),
				}
			}

			async fn query_connection_id_from_tx_hash(
				&self,
				tx_id: Self::TransactionId,
			) -> Result<ConnectionId, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_connection_id_from_tx_hash(
								downcast!(tx_id => AnyTransactionId::$name)
									.expect("Should be $name transaction id"),
							)
							.await
							.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_connection_id_from_tx_hash(tx_id).await,
				}
			}

			async fn query_channel_id_from_tx_hash(
				&self,
				tx_id: Self::TransactionId,
			) -> Result<(ChannelId, PortId), Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.query_channel_id_from_tx_hash(
								downcast!(tx_id => AnyTransactionId::$name)
									.expect("Should be $name transaction id"),
							)
							.await
							.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_channel_id_from_tx_hash(tx_id).await,
				}
			}

			fn set_channel_whitelist(&mut self, channel_whitelist: std::collections::HashSet<(ChannelId, PortId)>) {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.set_channel_whitelist(channel_whitelist),
					)*
					Self::Wasm(c) => c.inner.set_channel_whitelist(channel_whitelist),
				}
			}

			fn add_channel_to_whitelist(&mut self, channel: (ChannelId, PortId)) {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.add_channel_to_whitelist(channel),
					)*
					Self::Wasm(c) => c.inner.add_channel_to_whitelist(channel),
				}
			}

			fn set_connection_id(&mut self, connection_id: ConnectionId) {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.set_connection_id(connection_id),
					)*
					Self::Wasm(c) => c.inner.set_connection_id(connection_id),
				}
			}
		}

		#[async_trait]
		impl MisbehaviourHandler for AnyChain {
			async fn check_for_misbehaviour<C: Chain>(
				&self,
				counterparty: &C,
				client_message: AnyClientMessage,
			) -> Result<(), anyhow::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) =>
							chain.check_for_misbehaviour(counterparty, client_message).await,
					)*
					AnyChain::Wasm(c) => c.inner.check_for_misbehaviour(counterparty, client_message).await,
				}
			}
		}

		impl KeyProvider for AnyChain {
			fn account_id(&self) -> Signer {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.account_id(),
					)*
					AnyChain::Wasm(c) => c.inner.account_id(),
				}
			}
		}

		#[async_trait]
		impl Chain for AnyChain {

			fn name(&self) -> &str {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.name(),
					)*
					Self::Wasm(c) => c.inner.name(),
				}
			}

			fn block_max_weight(&self) -> u64 {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.block_max_weight(),
					)*
					Self::Wasm(c) => c.inner.block_max_weight(),
				}
			}

			async fn estimate_weight(&self, msg: Vec<Any>) -> Result<u64, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.estimate_weight(msg).await.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.estimate_weight(msg).await,
				}
			}

			async fn finality_notifications(
				&self,
			) -> Result<Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send + Sync>>, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => {
							use futures::StreamExt;
							Ok(
								Box::pin(chain.finality_notifications().await
									.map_err(AnyError::$name)?
									.map(AnyFinalityEvent::$name))
							)
						},
					)*
					Self::Wasm(c) => c.inner.finality_notifications().await,
				}
			}

			async fn submit(&self, messages: Vec<Any>) -> Result<Self::TransactionId, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain
							.submit(messages)
							.await
							.map_err(AnyError::$name)
							.map(|id| AnyTransactionId::$name(id)),
					)*
					Self::Wasm(chain) => {
						let messages = messages
							.into_iter()
							.map(|msg| wrap_any_msg_into_wasm(msg, chain.code_id.clone()))
							.collect::<Result<Vec<_>, _>>()?;
						chain.inner.submit(messages).await.map_err(AnyError::into)
					},
				}
			}

			async fn query_client_message(
				&self,
				update: UpdateClient,
			) -> Result<AnyClientMessage, Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.query_client_message(update).await.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.query_client_message(update).await,
				}
			}

			async fn get_proof_height(&self, block_height: Height) -> Height {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.get_proof_height(block_height).await,
					)*
					Self::Wasm(c) => c.inner.get_proof_height(block_height).await,
				}
			}

			async fn handle_error(&mut self, e: &anyhow::Error) -> std::result::Result<(), anyhow::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.handle_error(e).await,
					)*
					Self::Wasm(c) => c.inner.handle_error(e).await,
				}
			}

			fn rpc_call_delay(&self) -> std::time::Duration {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.rpc_call_delay(),
					)*
					Self::Wasm(c) => c.inner.rpc_call_delay(),
				}
			}

			fn set_rpc_call_delay(&mut self, d: std::time::Duration) {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.set_rpc_call_delay(d),
					)*
					Self::Wasm(c) => c.inner.set_rpc_call_delay(d),
				}
			}
		}

		#[async_trait]
		impl LightClientSync for AnyChain {
			async fn is_synced<C: Chain>(&self, counterparty: &C) -> Result<bool, anyhow::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.is_synced(counterparty).await.map_err(Into::into),
					)*
					Self::Wasm(c) => c.inner.is_synced(counterparty).await,
				}
			}

			async fn fetch_mandatory_updates<C: Chain>(
				&self,
				counterparty: &C,
			) -> Result<(Vec<Any>, Vec<IbcEvent>), anyhow::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) =>
							chain.fetch_mandatory_updates(counterparty).await.map_err(Into::into),
					)*
					Self::Wasm(c) => c.inner.fetch_mandatory_updates(counterparty).await,
				}
			}
		}

		#[cfg(any(test, feature = "testing"))]
		impl AnyChain {
			pub fn set_client_id(&mut self, client_id: ClientId) {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.set_client_id(client_id),
					)*
					Self::Wasm(chain) => chain.inner.set_client_id(client_id),
				}
			}
		}

		#[cfg(any(test, feature = "testing"))]
		#[async_trait]
		impl primitives::TestProvider for AnyChain {
			async fn send_transfer(&self, params: MsgTransfer<PrefixedCoin>) -> Result<(), Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.send_transfer(params).await.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.send_transfer(params).await,
				}
			}

			async fn send_ordered_packet(
				&self,
				channel_id: ChannelId,
				timeout: Timeout,
			) -> Result<(), Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.send_ordered_packet(channel_id, timeout).await.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.send_ordered_packet(channel_id, timeout).await,
				}
			}

			async fn subscribe_blocks(&self) -> Pin<Box<dyn Stream<Item = u64> + Send + Sync>> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.subscribe_blocks().await,
					)*
					Self::Wasm(c) => c.inner.subscribe_blocks().await,
				}
			}

			async fn increase_counters(&mut self) -> Result<(), Self::Error> {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.increase_counters().await.map_err(AnyError::$name),
					)*
					Self::Wasm(c) => c.inner.increase_counters().await,
				}
			}
		}

		impl AnyConfig {
			pub async fn into_client(self) -> anyhow::Result<AnyChain> {
				let maybe_wasm_code_id = self.wasm_code_id();
				let chain = match self {
					$(
						$(#[$($meta)*])*
						AnyConfig::$name(config) => AnyChain::$name(<$client>::new(config).await?),
					)*
				};
				if let Some(code_id) = maybe_wasm_code_id {
					Ok(AnyChain::Wasm(WasmChain { inner: Box::new(chain), code_id }))
				} else {
					Ok(chain)
				}
			}

			pub fn set_client_id(&mut self, client_id: ClientId) {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => {
							chain.client_id.replace(client_id);
						},
					)*
				}
			}

			pub fn set_connection_id(&mut self, connection_id: ConnectionId) {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => {
							chain.connection_id.replace(connection_id);
						},
					)*
				}
			}

			pub fn set_channel_whitelist(&mut self, channel_id: ChannelId, port_id: PortId) {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => {
							chain.channel_whitelist.push((channel_id, port_id));
						},
					)*
				}
			}

			pub fn wasm_code_id(&self) -> Option<CodeId> {
				let maybe_code_id = match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => chain.wasm_code_id.as_ref(),
					)*
				};
				let maybe_code_id =
					maybe_code_id.map(|s| hex::decode(s).expect("Wasm code id is hex-encoded"));

				maybe_code_id
			}

			pub fn set_wasm_code_id(&mut self, code_id: String) {
				match self {
					$(
						$(#[$($meta)*])*
						Self::$name(chain) => {
							chain.wasm_code_id = Some(code_id);
						},
					)*
				}
			}
		}
	};
}
