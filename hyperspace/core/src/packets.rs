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

#[cfg(feature = "testing")]
use crate::send_packet_relay::packet_relay_status;
use rand::Rng;
use sp_runtime::Either::{Left, Right};
use std::{sync::Arc, time::Duration};
use tokio::{task::JoinSet, time::sleep};

use crate::packets::utils::{
	construct_ack_message, construct_recv_message, construct_timeout_message,
	get_timeout_proof_height, verify_delay_passed, VerifyDelayOn,
};
use ibc::{
	core::{
		ics02_client::client_state::ClientState as ClientStateT,
		ics03_connection::connection::ConnectionEnd,
		ics04_channel::channel::{ChannelEnd, State},
	},
	Height,
};
use ibc_proto::google::protobuf::Any;
use pallet_ibc::light_clients::AnyClientState;
use primitives::{
	error::Error, find_suitable_proof_height_for_client, packet_info_to_packet,
	query_undelivered_acks, query_undelivered_sequences, Chain,
};

pub mod connection_delay;
pub mod utils;

pub const PROCESS_PACKETS_BATCH_SIZE: usize = 100;
pub const MAX_PACKETS_TO_PROCESS: usize = 1000;

/// Returns a tuple of messages, with the first item being packets that are ready to be sent to the
/// sink chain. And the second item being packet timeouts that should be sent to the source.
pub async fn query_ready_and_timed_out_packets(
	source: &impl Chain,
	sink: &impl Chain,
) -> Result<(Vec<Any>, Vec<Any>), anyhow::Error> {
	let mut messages = vec![];
	let mut timeout_messages = vec![];
	let (source_height, source_timestamp) = source.latest_height_and_timestamp().await?;
	let (sink_height, sink_timestamp) = sink.latest_height_and_timestamp().await?;
	let channel_whitelist = source.channel_whitelist();

	// TODO: parallelize this
	for (channel_id, port_id) in channel_whitelist {
		let source_channel_response = match source
			.query_channel_end(source_height, channel_id, port_id.clone())
			.await
		{
			Ok(response) => response,
			Err(e) => {
				log::warn!(target: "hyperspace", "Failed to query channel end for {:?}/{:?}: {:?}", channel_id, port_id.clone(), e);
				continue
			},
		};
		let source_channel_end =
			ChannelEnd::try_from(source_channel_response.channel.ok_or_else(|| {
				Error::Custom(format!(
					"ChannelEnd not found for {:?}/{:?}",
					channel_id,
					port_id.clone()
				))
			})?)?;
		// we're only interested in open or closed channels
		if !matches!(source_channel_end.state, State::Open | State::Closed) {
			log::trace!(target: "hyperspace", "Skipping channel {:?}/{:?} because it is not open or closed", channel_id, port_id.clone());
			continue
		}
		let connection_id = source_channel_end
			.connection_hops
			.get(0)
			.ok_or_else(|| Error::Custom("Channel end missing connection id".to_string()))?
			.clone();
		let connection_response =
			source.query_connection_end(source_height, connection_id.clone()).await?;
		let source_connection_end =
			ConnectionEnd::try_from(connection_response.connection.ok_or_else(|| {
				Error::Custom(format!(
					"[query_ready_and_timed_out_packets] ConnectionEnd not found for {:?}",
					connection_id
				))
			})?)?;

		let sink_channel_id = source_channel_end
			.counterparty()
			.channel_id
			.ok_or_else(|| {
				Error::Custom(
					" An Open Channel End should have a valid counterparty channel id".to_string(),
				)
			})?
			.clone();
		let sink_port_id = source_channel_end.counterparty().port_id.clone();
		let sink_channel_response = sink
			.query_channel_end(sink_height, sink_channel_id, sink_port_id.clone())
			.await?;

		let sink_channel_end =
			ChannelEnd::try_from(sink_channel_response.channel.ok_or_else(|| {
				Error::Custom(format!(
					"Failed to convert to concrete channel end from raw channel end",
				))
			})?)?;

		let next_sequence_recv = sink
			.query_next_sequence_recv(sink_height, &sink_port_id, &sink_channel_id)
			.await?;

		let source_client_state_on_sink =
			sink.query_client_state(sink_height, source.client_id()).await?;
		let source_client_state_on_sink = AnyClientState::try_from(
			source_client_state_on_sink.client_state.ok_or_else(|| {
				Error::Custom(format!(
					"Client state for {} should exist on {}",
					source.name(),
					sink.name()
				))
			})?,
		)
		.map_err(|_| {
			Error::Custom(format!(
				"Invalid Client state for {} should found on {}",
				source.name(),
				sink.name()
			))
		})?;

		let sink_client_state_on_source =
			source.query_client_state(source_height, sink.client_id()).await?;
		let sink_client_state_on_source = AnyClientState::try_from(
			sink_client_state_on_source.client_state.ok_or_else(|| {
				Error::Custom(format!(
					"Client state for {} should exist on {}",
					source.name(),
					sink.name()
				))
			})?,
		)
		.map_err(|_| {
			Error::Custom(format!(
				"Invalid Client state for {} should found on {}",
				source.name(),
				sink.name()
			))
		})?;
		let latest_sink_height_on_source = sink_client_state_on_source.latest_height();
		let latest_source_height_on_sink = source_client_state_on_sink.latest_height();

		// query packets that are waiting for connection delay.
		let seqs = query_undelivered_sequences(
			source_height,
			sink_height,
			channel_id,
			port_id.clone(),
			source,
			sink,
		)
		.await?
		.into_iter()
		.take(MAX_PACKETS_TO_PROCESS)
		.collect::<Vec<_>>();

		log::trace!(target: "hyperspace", "Found {} undelivered packets for {:?}/{:?}", seqs.len(), channel_id, port_id.clone());

		let send_packets = source.query_send_packets(channel_id, port_id.clone(), seqs).await?;
		let mut timeout_packets_join_set: JoinSet<Result<_, anyhow::Error>> = JoinSet::new();
		let source = Arc::new(source.clone());
		let sink = Arc::new(sink.clone());
		for send_packets in send_packets.chunks(PROCESS_PACKETS_BATCH_SIZE) {
			for send_packet in send_packets.to_owned() {
				let source_connection_end = source_connection_end.clone();
				let sink_channel_end = sink_channel_end.clone();
				let source_connection_end = source_connection_end.clone();
				let source = source.clone();
				let sink = sink.clone();
				let duration = Duration::from_millis(
					rand::thread_rng().gen_range(1..source.rpc_call_delay().as_millis() as u64),
				);
				timeout_packets_join_set.spawn(async move {
					sleep(duration).await;
					let source = &source;
					let sink = &sink;
					let packet = packet_info_to_packet(&send_packet);
					// Check if packet has timed out
					let packet_height = send_packet.height.ok_or_else(|| {
				Error::Custom(format!("Packet height not found for packet {:?}", packet))
			})?;
			if packet.timed_out(&sink_timestamp, sink_height) {
				// so we know this packet has timed out on the sink, we need to find the maximum
				// consensus state height at which we can generate a non-membership proof of the
				// packet for the sink's client on the source.
				let proof_height = if let Some(proof_height) = get_timeout_proof_height(
					&**source,
					&**sink,
					source_height,
					sink_height,
					sink_timestamp,
					latest_sink_height_on_source,
					&packet,
					packet_height,
						)
							.await
						{
							proof_height
						} else {
							log::trace!(target: "hyperspace", "Skipping packet as no timeout proof height could be found: {:?}", packet);
							return Ok(None)
						};

						// given this maximum height, has the connection delay been satisfied?
						if !verify_delay_passed(
							&**source,
							&**sink,
							source_timestamp,
							source_height,
							sink_timestamp,
							sink_height,
							source_connection_end.delay_period(),
							proof_height,
							VerifyDelayOn::Source,
						)
							.await?
						{
							log::trace!(target: "hyperspace", "Skipping packet as connection delay has not passed {:?}", packet);
							return Ok(None)
						}

						// lets construct the timeout message to be sent to the source
						let msg = construct_timeout_message(
							&**source,
							&**sink,
							&sink_channel_end,
							packet,
							next_sequence_recv.next_sequence_receive,
							proof_height,
						)
							.await?;
						return Ok(Some(Left(msg)))
					} else {
						log::trace!(target: "hyperspace", "Skipping packet as it has not timed out: {:?}", packet);
					}

					// If packet has not timed out but channel is closed on sink we skip
					// Since we have no reference point for when this channel was closed so we can't
					// calculate connection delays yet
					if sink_channel_end.state == State::Closed {
						log::debug!(target: "hyperspace", "Skipping packet as channel is closed on sink: {:?}", packet);
						return Ok(None)
					}

					#[cfg(feature = "testing")]
					// If packet relay status is paused skip
					if !packet_relay_status() {
						return Ok(None)
					}

					// Check if packet is ready to be sent to sink
					// If sink does not have a client height that is equal to or greater than the packet
					// creation height, we can't send it yet, packet_info.height should represent the packet
					// creation height on source chain
					if packet_height > latest_source_height_on_sink.revision_height {
						// Sink does not have client update required to prove recv packet message
						log::debug!(target: "hyperspace", "Skipping packet {:?} as sink does not have client update required to prove recv packet message", packet);
						return Ok(None)
					}

					let proof_height = if let Some(proof_height) = find_suitable_proof_height_for_client(
						&**sink,
						sink_height,
						source.client_id(),
						Height::new(latest_source_height_on_sink.revision_number, packet_height),
						None,
						latest_source_height_on_sink,
					)
						.await
					{
						proof_height
					} else {
						log::trace!(target: "hyperspace", "Skipping packet {:?} as no proof height could be found", packet);
						return Ok(None)
					};

					if !verify_delay_passed(
						&**source,
						&**sink,
						source_timestamp,
						source_height,
						sink_timestamp,
						sink_height,
						source_connection_end.delay_period(),
						proof_height,
						VerifyDelayOn::Sink,
					)
						.await?
					{
						log::trace!(target: "hyperspace", "Skipping packet as connection delay has not passed {:?}", packet);
						return Ok(None)
					}

					if packet.timeout_height.is_zero() && packet.timeout_timestamp.nanoseconds() == 0 {
						log::warn!(target: "hyperspace", "Skipping packet as packet timeout is zero: {}", packet.sequence);
						return Ok(None)
					}

					let msg = construct_recv_message(&**source, &**sink, packet, proof_height).await?;
					Ok(Some(Right(msg)))
				});
			}
		}

		while let Some(result) = timeout_packets_join_set.join_next().await {
			let Some(either) = result?? else { continue };
			match either {
				Left(msg) => timeout_messages.push(msg),
				Right(msg) => messages.push(msg),
			}
		}

		// Get acknowledgement messages
		if source_channel_end.state == State::Closed {
			log::trace!(target: "hyperspace", "Skipping acknowledgements for channel {:?} as channel is closed on source", channel_id);
			continue
		}

		// query acknowledgements that are waiting for connection delay.
		let acks = query_undelivered_acks(
			source_height,
			sink_height,
			channel_id,
			port_id.clone(),
			&*source,
			&*sink,
		)
		.await?
		.into_iter()
		.take(MAX_PACKETS_TO_PROCESS)
		.collect::<Vec<_>>();

		let acknowledgements = source.query_recv_packets(channel_id, port_id.clone(), acks).await?;
		let mut acknowledgements_join_set: JoinSet<Result<_, anyhow::Error>> = JoinSet::new();
		for acknowledgements in acknowledgements.chunks(PROCESS_PACKETS_BATCH_SIZE) {
			for acknowledgement in acknowledgements.to_owned() {
				let source_connection_end = source_connection_end.clone();
				let source = source.clone();
				let sink = sink.clone();
				let duration1 = Duration::from_millis(
					rand::thread_rng().gen_range(1..source.rpc_call_delay().as_millis() as u64),
				);
				acknowledgements_join_set.spawn(async move {
					sleep(duration1).await;
					let source = &source;
					let sink = &sink;
					let packet = packet_info_to_packet(&acknowledgement);
					let ack = if let Some(ack) = acknowledgement.ack {
						ack
					} else {
						// Packet has no valid acknowledgement, skip
						log::trace!(target: "hyperspace", "Skipping acknowledgement for packet {:?} as packet has no valid acknowledgement", packet);
						return Ok(None)
					};

					// Check if ack is ready to be sent to sink
					// If sink does not have a client height that is equal to or greater than the packet
					// creation height, we can't send it yet packet_info.height should represent the
					// acknowledgement creation height on source chain
					let ack_height = acknowledgement.height.ok_or_else(|| {
				Error::Custom(format!("Packet height not found for packet {:?}", packet))
			})?;
			if ack_height > latest_source_height_on_sink.revision_height {
						// Sink does not have client update required to prove acknowledgement packet message
						log::trace!(target: "hyperspace", "Skipping acknowledgement for packet {:?} as sink does not have client update required to prove acknowledgement packet message", packet);
						return Ok(None)
					}

					log::trace!(target: "hyperspace", "sink_height: {:?}, latest_source_height_on_sink: {:?}, acknowledgement.height: {}", sink_height, latest_source_height_on_sink, ack_height);

					let proof_height = if let Some(proof_height) = find_suitable_proof_height_for_client(
						&**sink,
						sink_height,
						source.client_id(),
						Height::new(latest_source_height_on_sink.revision_number, ack_height),
						None,
						latest_source_height_on_sink,
					)
						.await
					{
						log::trace!(target: "hyperspace", "Using proof height: {}", proof_height);
						proof_height
					} else {
						log::trace!(target: "hyperspace", "Skipping acknowledgement for packet {:?} as no proof height could be found", packet);
						return Ok(None)
					};

					if !verify_delay_passed(
						&**source,
						&**sink,
						source_timestamp,
						source_height,
						sink_timestamp,
						sink_height,
						source_connection_end.delay_period(),
						proof_height,
						VerifyDelayOn::Sink,
					)
						.await?
					{
						log::trace!(target: "hyperspace", "Skipping acknowledgement for packet as connection delay has not passed {:?}", packet);
						return Ok(None)
					}

					let msg = construct_ack_message(&**source, &**sink, packet, ack, proof_height).await?;
					// messages.push(msg)
					Ok(Some(msg))
				});
			}
		}

		while let Some(result) = acknowledgements_join_set.join_next().await {
			let Some(msg) = result?? else { continue };
			messages.push(msg)
		}
	}

	Ok((messages, timeout_messages))
}
