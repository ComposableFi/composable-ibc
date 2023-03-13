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
	($source:ident, $sink:ident, $metrics:expr, $mode:ident, $result:ident) => {
		match $result {
			// stream closed
			None => break,
			Some(finality_event) => {
				log::info!("=======================================================");
				log::info!("Received finality notification from {}", $source.name());
				let updates = match $source.query_latest_ibc_events(finality_event, &$sink).await {
					Ok(resp) => resp,
					Err(err) => {
						log::error!(
							"Failed to fetch IBC events for finality event for {} {:?}",
							$source.name(),
							err
						);
						continue
					},
				};

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
						parse_events(&mut $source, &mut $sink, events, $mode).await?;
					log::debug!(
						"Has packets {}: {}",
						$source.name(),
						$source.has_undelivered_sequences()
					);
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
					queue::flush_message_batch(msgs, $metrics.as_ref(), &$sink).await?;
				}

				if !timeout_msgs.is_empty() {
					if let Some(metrics) = $metrics.as_ref() {
						metrics.handle_timeouts(timeout_msgs.as_slice()).await;
					}
					let type_urls =
						timeout_msgs.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
					log::info!("Submitting timeout messages to {}: {type_urls:#?}", $source.name());
					queue::flush_message_batch(timeout_msgs, $metrics.as_ref(), &$source).await?;
				}
			},
		}
	};
}
