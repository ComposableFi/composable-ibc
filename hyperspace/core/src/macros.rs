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
				for (msg_update_client, events, update_type) in updates {
					let mut msgs_update_client = vec![msg_update_client];
					if let Some(metrics) = $metrics.as_mut() {
						if let Err(e) = metrics.handle_events(events.as_slice()).await {
							log::error!("Failed to handle metrics for {} {:?}", $source.name(), e);
						}
					}
					let event_types = events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>();
					let (mut messages, timeouts) =
						parse_events(&mut $source, &mut $sink, events, $mode).await?;
					if !timeouts.is_empty() {
						if let Some(metrics) = $metrics.as_ref() {
							metrics.handle_timeouts(timeouts.as_slice()).await;
						}
						let type_urls =
							timeouts.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
						log::info!(
							"Submitting timeout messages to {}: {type_urls:#?}",
							$source.name()
						);
						queue::flush_message_batch(timeouts, $metrics.as_ref(), &$source).await?;
					}
					// We want to send client update if packet messages exist but where not sent due
					// to a connection delay even if client update message is optional
					match (
						update_type.is_optional(),
						has_packet_events(&event_types),
						messages.is_empty(),
					) {
						(true, false, true) => {
							// skip sending ibc messages if no new events
							log::info!(
								"Skipping finality notification for {}, No new events",
								$source.name()
							);
							continue
						},
						(false, _, true) => log::info!(
							"Sending mandatory client update message for {}",
							$source.name()
						),
						_ => log::info!(
							"Received finalized events from: {} {event_types:#?}",
							$source.name()
						),
					};
					// todo: we should be able skip update clients that are optional even when
					// messages is not empty. insert client update at first position.
					msgs_update_client.append(&mut messages);
					if let Some(metrics) = $metrics.as_ref() {
						metrics.handle_messages(msgs_update_client.as_slice()).await;
					}
					let type_urls = msgs_update_client
						.iter()
						.map(|msg| msg.type_url.as_str())
						.collect::<Vec<_>>();
					log::info!("Submitting messages to {}: {type_urls:#?}", $sink.name());
					queue::flush_message_batch(msgs_update_client, $metrics.as_ref(), &$sink)
						.await?;
				}
			},
		}
	};
}
