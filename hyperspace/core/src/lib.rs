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
#[cfg(feature = "parachain")]
pub mod substrate;
mod utils;

use crate::utils::query_latest_update_time_and_height;
use anyhow::anyhow;
use events::parse_events;
use futures::{future::ready, StreamExt, TryFutureExt};
use ibc::{
	core::ics02_client::msgs::update_client::MsgUpdateAnyClient,
	events::{IbcEvent, IbcEventType},
	timestamp::Timestamp,
	Height,
};
use ibc_proto::google::protobuf::Any;
use itertools::Itertools;
use metrics::handler::MetricsHandler;
use primitives::{utils::RecentStream, Chain, IbcProvider, UndeliveredType, UpdateType};
use std::{collections::HashSet, time::Duration};
use tendermint_proto::Protobuf;

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
	chain_a.set_client_id_ref(chain_b.get_counterparty_client_id_ref());
	chain_b.set_client_id_ref(chain_a.get_counterparty_client_id_ref());

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
	let name_n = source.name().to_string();
	let updates = source
		.query_latest_ibc_events(finality_event, &*sink)
		.await
		.map_err(|e| anyhow!("Failed to fetch IBC events from {name_n} for finality event {e}"))?;
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
		"Received timeouts count: {}, packets count: {}",
		timeout_msgs.len(),
		ready_packets.len(),
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

	let need_to_update = sink_has_undelivered_acks;
	log::info!(target: "hyperspace", "Received {} client updates from {}", updates.len(), source.name(),);

	let common_state = source.common_state();
	let skip_optional_updates = common_state.skip_optional_client_updates;
	let has_events = updates.iter().any(|(_, _, events, ..)| !events.is_empty());
	let mut mandatory_updates = updates
		.iter()
		.filter(|(_, h, _, update_type)| {
			log::info!(
				"Received updated from {} at {}, is_mandatory: {}",
				source.name(),
				h.revision_height,
				!update_type.is_optional()
			);
			!skip_optional_updates || !update_type.is_optional()
		})
		.map(|(upd, height, ..)| (upd.clone(), *height))
		.collect::<Vec<_>>();

	let mut update_delay_passed = true;
	let (update_time, _update_height) = query_latest_update_time_and_height(source, sink)
		.await
		.map_err(|e| anyhow!("Failed to fetch latest update time and height {e}"))?;
	log::debug!(
		target: "hyperspace",
		"update_time: {}, interval: {}",
		update_time,
		sink.common_state().client_update_interval.as_secs()
	);
	if let Some(not_updated_during) = Timestamp::now().duration_since(&update_time) {
		if not_updated_during < source.common_state().client_update_interval {
			log::debug!(target: "hyperspace", "Sending only mandatory updates, not updated during {} seconds, need {}", not_updated_during.as_secs(), sink.common_state().client_update_interval.as_secs());
			update_delay_passed = false;
		}
	} else {
		log::warn!(target: "hyperspace", "Update time is from the future: {}", update_time);
	}

	log::debug!(target: "hyperspace", "Received' {} client updates from {}", mandatory_updates.len(), source.name(),);

	let ibc_events = updates
		.iter()
		.map(|(_, _, events, ..)| events.clone())
		.flatten()
		.collect::<Vec<_>>();

	let event_types = ibc_events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>();

	let has_instant_events = event_types.iter().any(|event_type| {
		matches!(
			event_type,
			&IbcEventType::OpenInitConnection |
				&IbcEventType::OpenInitChannel |
				&IbcEventType::OpenTryConnection |
				&IbcEventType::OpenTryChannel |
				&IbcEventType::OpenAckChannel |
				&IbcEventType::OpenAckConnection |
				&IbcEventType::CloseInitChannel
		)
	});

	//filter of instance event . filter(match from prev statement^ | ) 
	//filter iclude any of the following:
	/*
		SendPacket,
		ReceivePacket,
		WriteAck,
		Timeout,
		TimeoutOnClose,
	 */
	//max_event_height
	
	
	log::debug!(target: "hyperspace", "Received'' {}, has_instant_events = {has_instant_events}, update_delay_passed = {update_delay_passed}, need_to_update = {need_to_update}", mandatory_updates.len(), );

	if !updates.is_empty() &&
		(mandatory_updates.is_empty() && update_delay_passed && need_to_update) ||
		has_instant_events
	{	
		//replace updates.last() -> max_event_height
		//replace mandatory_updates.last() -> max_event_height
		// let max_event_height =
    	// ibc_events.iter().map(|ev| ev.height()).max().unwrap_or_else(|| Height::zero());
		//max_event_height never should be ZERO!!!
		//panic unreachable

		// let max_event_height = ibc_events.iter().filter(|ev| {
		// 	matches!(
		// 		&ev.event_type(),
		// 		&IbcEventType::OpenInitConnection |
		// 			&IbcEventType::OpenInitChannel |
		// 			&IbcEventType::OpenTryConnection |
		// 			&IbcEventType::OpenTryChannel |
		// 			&IbcEventType::OpenAckChannel |
		// 			&IbcEventType::OpenAckConnection |
		// 			&IbcEventType::CloseInitChannel | 

		// 			&IbcEventType::SendPacket |
		// 			&IbcEventType::ReceivePacket |
		// 			&IbcEventType::WriteAck |
		// 			&IbcEventType::Timeout | 
		// 			&IbcEventType::TimeoutOnClose
		// 	)
		// }).map(|ev| ev.height()).max().unwrap_or_else(|| Height::zero());
		// log::error!(target: "hyperspace", "max_event_height is : {:#?}", max_event_height);

		// if max_event_height == Height::zero() {
		// 	log::error!(target: "hyperspace", "max_event_height is ZERO");
		// 	return Ok(())
		// }

		// .filter(|e| e.1 == max_event_height)
		let (forced_update, height) = updates.last().map(|(msg, h, ..)| (msg.clone(), h)).unwrap();
		if !mandatory_updates.is_empty() {
			let (_, last_height) =
				mandatory_updates.last().map(|(msg, h)| (msg.clone(), h)).unwrap();
			if last_height != height {
				mandatory_updates.push((forced_update, *height));
			}
		} else {
			mandatory_updates.push((forced_update, *height));
		}
	}
	if mandatory_updates.is_empty() && !has_events {
		log::info!(target: "hyperspace", "No messages to send");
		return Ok(())
	}

	let latest_update_height = if !mandatory_updates.is_empty() {
		mandatory_updates.last().map(|(_, height)| *height).unwrap()
	} else {
		log::warn!(target: "hyperspace", "Expected at least one update");
		return Ok(())
	};

	log::info!(target: "hyperspace", "Received finalized events from: {} {event_types:#?}", source.name());
	let mut new_height = source.get_proof_height(latest_update_height).await;
	if new_height != latest_update_height {
		new_height.revision_height -=
			(new_height.revision_height - latest_update_height.revision_height) * 2;
	}

	let mut ibc_events_messages =
		parse_events(source, sink, ibc_events.clone(), mode, Some(new_height))
			.await
			.map_err(|e| anyhow!("Failed to parse events: {:?}", e))?;
	let (latest_height, _) = source.latest_height_and_timestamp().await?;
	for (msg_update_client, h) in mandatory_updates {
		log::debug!(target: "hyperspace", "Received client update message for {}: {}, latest height: {latest_height}", source.name(), h);
		// log::error!(target: "hyperspace", "___________________________________________________________________________________________");
		// log::error!(target: "hyperspace", "Received client update message for {}: {}, latest height: {latest_height}", source.name(), h);
		msgs.push(msg_update_client);
	}

	msgs.append(&mut ibc_events_messages);

	if let Some(metrics) = metrics.as_mut() {
		if let Err(e) = metrics.handle_events(ibc_events.as_slice()).await {
			log::error!(target: "hyperspace", "Failed to handle metrics for {} {:?}", source.name(), e);
		}
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
	updates: &[(Any, Height, Vec<IbcEvent>, UpdateType)],
) -> HashSet<u64> {
	let mut mandatory_updates_for_undelivered_seqs = HashSet::new();
	let update_heights = updates
		.iter()
		.map(|(_, height, ..)| height.revision_height)
		.collect::<HashSet<_>>();
	let (_, height, ..) = updates.first().unwrap();
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
