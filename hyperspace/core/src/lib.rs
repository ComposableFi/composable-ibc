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

#![warn(unused_variables)]

pub mod chain;
pub mod command;
pub mod events;
pub mod logging;
mod macros;
pub mod packets;
pub mod queue;
pub mod substrate;
mod utils;

use crate::utils::RecentStream;
use anyhow::anyhow;
use events::{has_packet_events, parse_events};
use futures::{future::ready, StreamExt, TryFutureExt};
use ibc::{events::IbcEvent, Height};
use ibc_proto::google::protobuf::Any;
use metrics::handler::MetricsHandler;
use primitives::{Chain, IbcProvider, UndeliveredType, UpdateType};
use std::collections::HashSet;

#[derive(Copy, Debug, Clone)]
pub enum Mode {
	/// Run without trying to relay packets or query channel state
	Light,
}

/// Core relayer loop, waits for new finality events and forwards any new [`ibc::IbcEvents`]
/// to the counter party chain.
pub async fn relay<A, B>(
	mut chain_a: A,
	mut chain_b: B,
	mut chain_a_metrics: Option<MetricsHandler>,
	mut chain_b_metrics: Option<MetricsHandler>,
	mode: Option<Mode>,
) -> Result<(), anyhow::Error>
where
	A: Chain,
	B: Chain,
{
	let stream_a = RecentStream::new(chain_a.finality_notifications().await?);
	let stream_b = RecentStream::new(chain_b.finality_notifications().await?);
	let (mut chain_a_finality, mut chain_b_finality) = (stream_a, stream_b);

	// Introduce altering between branches so that each branch gets a chance to execute first after
	// another one
	let mut first_executed = false;

	// loop forever
	loop {
		tokio::select! {
			// new finality event from chain A
			result = chain_a_finality.next(), if !first_executed => {
				first_executed = true;
				process_finality_event(&mut chain_a, &mut chain_b, &mut chain_a_metrics, mode, result, &mut chain_a_finality, &mut chain_b_finality).await?;
			}
			// new finality event from chain B
			result = chain_b_finality.next() => {
				first_executed = false;
				process_finality_event(&mut chain_b, &mut chain_a, &mut chain_b_metrics, mode, result, &mut chain_b_finality, &mut chain_a_finality).await?;
			}
			else => {
				first_executed = false;
			}
		}
	}
}

pub async fn fish<A, B>(chain_a: A, chain_b: B) -> Result<(), anyhow::Error>
where
	A: Chain,
	A::Error: From<B::Error>,
	B: Chain,
	B::Error: From<A::Error>,
{
	// we only care about events where the counterparty light client is updated.
	let (mut chain_a_client_updates, mut chain_b_client_updates) = (
		chain_a.ibc_events().await.filter_map(|ev| {
			ready(match ev {
				IbcEvent::UpdateClient(update) if chain_b.client_id() == *update.client_id() =>
					Some(update),
				_ => None,
			})
		}),
		chain_b.ibc_events().await.filter_map(|ev| {
			ready(match ev {
				IbcEvent::UpdateClient(update) if chain_a.client_id() == *update.client_id() =>
					Some(update),
				_ => None,
			})
		}),
	);

	// loop forever
	loop {
		tokio::select! {
			// new finality event from chain A
			update = chain_a_client_updates.next() => {
				let update = match update {
					Some(update) => update,
					None => break,
				};
				// The corresponding transaction on tendermint may not be indexed yet, so we wait for a bit
				if chain_a.client_type() == "07-tendermint" {
					tokio::time::sleep(chain_a.expected_block_time()).await;
				}
				let message = chain_a.query_client_message(update).await.map_err(|e| { log::info!("error: {}", e); e })?;
				chain_b.check_for_misbehaviour(&chain_a, message).await.map_err(|e| { log::info!("error: {}", e); e })?;
			}
			// new finality event from chain B
			update = chain_b_client_updates.next() => {
				let update = match update {
					Some(update) => update,
					None => break,
				};
				// The corresponding transaction on tendermint may not be indexed yet, so we wait for a bit
				if chain_a.client_type() == "07-tendermint" {
					tokio::time::sleep(chain_a.expected_block_time()).await;
				}
				let message = chain_b.query_client_message(update).await.map_err(|e| { log::info!("error: {}", e); e })?;
				chain_a.check_for_misbehaviour(&chain_b, message).await.map_err(|e| { log::info!("error: {}", e); e })?;
			}
		}
	}

	Ok(())
}

async fn process_finality_event<A: Chain, B: Chain>(
	source: &mut A,
	sink: &mut B,
	metrics: &mut Option<MetricsHandler>,
	mode: Option<Mode>,
	result: Option<A::FinalityEvent>,
	stream_source: &mut RecentStream<A::FinalityEvent>,
	stream_sink: &mut RecentStream<B::FinalityEvent>,
) -> anyhow::Result<()> {
	match result {
		// stream closed
		None => {
			log::warn!("Stream closed for {}", source.name());
			*stream_source = loop {
				match source.finality_notifications().await {
					Ok(stream) => break RecentStream::new(stream),
					Err(e) => {
						log::error!("Failed to get finality notifications for {} {:?}. Trying again in 30 seconds...", source.name(), e);
						tokio::time::sleep(std::time::Duration::from_secs(30)).await;
						let _ = source.reconnect().await;
					},
				};
			};
			*stream_sink = loop {
				match sink.finality_notifications().await {
					Ok(stream) => break RecentStream::new(stream),
					Err(e) => {
						log::error!("Failed to get finality notifications for {} {:?}. Trying again in 30 seconds...", sink.name(), e);
						tokio::time::sleep(std::time::Duration::from_secs(30)).await;
						let _ = sink.reconnect().await;
					},
				};
			};
		},
		Some(finality_event) => {
			log::info!("=======================================================");
			log::info!("Received finality notification from {}", source.name(),);

			let result =
				process_some_finality_event(source, sink, metrics, mode, finality_event).await;

			match result {
				Ok(()) => {
					let sink_initial_rpc_call_delay = sink.initial_rpc_call_delay();
					let source_initial_rpc_call_delay = source.initial_rpc_call_delay();
					sink.set_rpc_call_delay(sink_initial_rpc_call_delay);
					source.set_rpc_call_delay(source_initial_rpc_call_delay);
				},
				Err(e) => {
					log::error!("{}", e);
					match sink.handle_error(&e).and_then(|_| source.handle_error(&e)).await {
						Ok(_) => (),
						Err(e) => {
							log::error!("Failed to handle error {:?}", e)
						},
					}
				},
			}
		},
	}
	Ok(())
}

async fn process_some_finality_event<A: Chain, B: Chain>(
	source: &mut A,
	sink: &mut B,
	metrics: &mut Option<MetricsHandler>,
	mode: Option<Mode>,
	finality_event: <A as IbcProvider>::FinalityEvent,
) -> anyhow::Result<()> {
	let updates = source
		.query_latest_ibc_events(finality_event, &*sink)
		.await
		.map_err(|e| anyhow!("Failed to fetch IBC events for finality event {e}"))?;
	log::trace!(target: "hyperspace", "Received updates count: {}", updates.len());
	// query packets that can now be sent, at this sink height because of connection
	// delay.
	let (ready_packets, timeout_msgs) =
		packets::query_ready_and_timed_out_packets(&*source, &*sink)
			.await
			.map_err(|e| anyhow!("Failed to parse events: {:?}", e))?;

	let mut msgs = Vec::new();

	log::trace!(
		target: "hyperspace",
		"{} has undelivered seqs: [{:?}:{}, {:?}:{}, {:?}:{}]", source.name(),
		UndeliveredType::Acks, source.has_undelivered_sequences(UndeliveredType::Acks),
		UndeliveredType::Timeouts, source.has_undelivered_sequences(UndeliveredType::Timeouts),
		UndeliveredType::Recvs, source.has_undelivered_sequences(UndeliveredType::Recvs),
	);

	log::trace!(
		target: "hyperspace",
		"{} has undelivered seqs: [{:?}:{}, {:?}:{}, {:?}:{}]", sink.name(),
		UndeliveredType::Acks, sink.has_undelivered_sequences(UndeliveredType::Acks),
		UndeliveredType::Timeouts, sink.has_undelivered_sequences(UndeliveredType::Timeouts),
		UndeliveredType::Recvs, sink.has_undelivered_sequences(UndeliveredType::Recvs),
	);

	log::trace!(
		target: "hyperspace",
		"Received timeouts count: {}",
		timeout_msgs.len()
	);

	process_updates(source, sink, metrics, mode, updates, &mut msgs).await?;

	msgs.extend(ready_packets);

	process_messages(sink, metrics, msgs).await?;
	process_timeouts(source, metrics, timeout_msgs).await?;
	Ok(())
}

async fn process_updates<A: Chain, B: Chain>(
	source: &mut A,
	sink: &mut B,
	metrics: &mut Option<MetricsHandler>,
	mode: Option<Mode>,
	updates: Vec<(Any, Height, Vec<IbcEvent>, UpdateType)>,
	msgs: &mut Vec<Any>,
) -> anyhow::Result<()> {
	// for timeouts we need both chains to be up to date
	let sink_has_undelivered_acks = sink.has_undelivered_sequences(UndeliveredType::Recvs) ||
		sink.has_undelivered_sequences(UndeliveredType::Acks) ||
		sink.has_undelivered_sequences(UndeliveredType::Timeouts);
	let source_has_undelivered_acks = source.has_undelivered_sequences(UndeliveredType::Timeouts);

	let mandatory_heights_for_undelivered_seqs =
		if (sink_has_undelivered_acks || source_has_undelivered_acks) && !updates.is_empty() {
			find_mandatory_heights_for_undelivered_sequences(source, &updates).await
		} else {
			HashSet::new()
		};

	for (msg_update_client, height, events, update_type) in updates {
		if let Some(metrics) = metrics.as_mut() {
			if let Err(e) = metrics.handle_events(events.as_slice()).await {
				log::error!("Failed to handle metrics for {} {:?}", source.name(), e);
			}
		}

		let event_types = events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>();
		let mut messages = parse_events(source, sink, events, mode)
			.await
			.map_err(|e| anyhow!("Failed to parse events: {:?}", e))?;

		log::trace!(
			target: "hyperspace",
			"Received messages count: {}, is the update optional: {}",
			messages.len(), update_type.is_optional(),
		);

		let need_to_send_proofs_for_sequences = (sink_has_undelivered_acks ||
			source_has_undelivered_acks) &&
			mandatory_heights_for_undelivered_seqs.contains(&height.revision_height);
		let common_state = source.common_state();
		let skip_optional_updates = common_state.skip_optional_client_updates;

		// We want to send client update if packet messages exist but where not sent due
		// to a connection delay even if client update message is optional
		match (
			// TODO: we actually may send only when timeout of some packet has reached,
			// not when we have *any* undelivered packets. But this requires rewriting
			// `find_suitable_proof_height_for_client` function, that uses binary
			// search, which won't work in this case
			skip_optional_updates &&
				update_type.is_optional() &&
				!need_to_send_proofs_for_sequences,
			has_packet_events(&event_types),
			messages.is_empty(),
		) {
			(true, false, true) => {
				// skip sending ibc messages if no new events
				log::info!("Skipping finality notification for {}", sink.name());
				continue
			},
			(false, _, true) =>
				if update_type.is_optional() && need_to_send_proofs_for_sequences {
					log::info!("Sending an optional update because source ({}) chain has undelivered sequences", sink.name());
				} else {
					log::info!("Sending mandatory client update message for {}", sink.name())
				},
			_ => log::info!("Received finalized events from: {} {event_types:#?}", source.name()),
		};
		msgs.push(msg_update_client);
		msgs.append(&mut messages);
	}
	Ok(())
}

async fn process_messages<B: Chain>(
	sink: &mut B,
	metrics: &mut Option<MetricsHandler>,
	msgs: Vec<Any>,
) -> anyhow::Result<()> {
	if !msgs.is_empty() {
		if let Some(metrics) = metrics.as_ref() {
			metrics.handle_messages(msgs.as_slice()).await;
		}
		let type_urls = msgs.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
		log::info!("Submitting messages to {}: {type_urls:#?}", sink.name());

		queue::flush_message_batch(msgs, metrics.as_ref(), &*sink)
			.await
			.map_err(|e| anyhow!("Failed to submit messages: {:?}", e))?;
		log::debug!(target: "hyperspace", "Successfully submitted messages to {}", sink.name());
	}
	Ok(())
}

async fn process_timeouts<A: Chain>(
	source: &mut A,
	metrics: &mut Option<MetricsHandler>,
	timeout_msgs: Vec<Any>,
) -> anyhow::Result<()> {
	if !timeout_msgs.is_empty() {
		if let Some(metrics) = metrics.as_ref() {
			metrics.handle_timeouts(timeout_msgs.as_slice()).await;
		}
		let type_urls = timeout_msgs.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
		log::info!("Submitting timeout messages to {}: {type_urls:#?}", source.name());
		queue::flush_message_batch(timeout_msgs, metrics.as_ref(), &*source)
			.await
			.map_err(|e| anyhow!("Failed to submit timeout messages: {:?}", e))?;
		log::debug!(target: "hyperspace", "Successfully submitted timeout messages to {}", source.name());
	}
	Ok(())
}

async fn find_mandatory_heights_for_undelivered_sequences<A: Chain>(
	source: &mut A,
	updates: &Vec<(Any, Height, Vec<IbcEvent>, UpdateType)>,
) -> HashSet<u64> {
	let mut mandatory_updates_for_undelivered_seqs = HashSet::new();
	let update_heights = updates
		.iter()
		.map(|(_, height, ..)| height.revision_height)
		.collect::<HashSet<_>>();
	let (_, height, ..) = updates.first().unwrap().clone();
	let proof_height = source.get_proof_height(*height).await;
	let block_proof_height_difference = proof_height
		.revision_height
		.checked_sub(height.revision_height)
		.expect("proof height is less than update height");
	let needed_updates_num = if block_proof_height_difference > 0 { 2 } else { 1 };
	for (_, height, ..) in updates.iter().rev() {
		if let Some(prev_height) = height.revision_height.checked_sub(block_proof_height_difference)
		{
			if update_heights.contains(&prev_height) {
				mandatory_updates_for_undelivered_seqs.insert(height.revision_height);
				mandatory_updates_for_undelivered_seqs.insert(prev_height);
			}
		}
		if mandatory_updates_for_undelivered_seqs.len() == needed_updates_num {
			break
		}
	}
	mandatory_updates_for_undelivered_seqs
}

#[cfg(feature = "testing")]
pub mod send_packet_relay {
	use std::sync::atomic::{AtomicBool, Ordering};
	static RELAY_PACKETS: AtomicBool = AtomicBool::new(true);

	/// Returns status of send packet relay
	pub fn packet_relay_status() -> bool {
		RELAY_PACKETS.load(Ordering::SeqCst)
	}

	/// Sets packet relay status
	pub fn set_relay_status(status: bool) {
		RELAY_PACKETS.store(status, Ordering::SeqCst);
	}
}
