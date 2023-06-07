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

use crate::{assert_timeout_packet, timeout_future, StreamExt};
use futures::future;
use hyperspace_core::send_packet_relay::set_relay_status;
use hyperspace_primitives::{
	utils::{create_channel, create_connection},
	TestProvider,
};
use ibc::{
	core::{
		ics04_channel::channel::{ChannelEnd, Order, State},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
	},
	events::IbcEvent,
};
use pallet_ibc::Timeout;
use std::{str::FromStr, time::Duration};
use tokio::task::JoinHandle;

/// This will set up a connection and an ordered channel in-between the two chains with the provided
/// port and channel version
async fn setup_connection_and_channel<A, B>(
	chain_a: &mut A,
	chain_b: &mut B,
	connection_delay: Duration,
	port_id: PortId,
	version: String,
) -> (JoinHandle<()>, ChannelId, ChannelId, ConnectionId)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let client_a_clone = chain_a.clone();
	let client_b_clone = chain_b.clone();
	// Start relayer loop
	let handle = tokio::task::spawn(async move {
		hyperspace_core::relay(client_a_clone, client_b_clone, None, None, None)
			.await
			.unwrap()
	});
	// check if an open ping channel exists
	let (latest_height, ..) = chain_a.latest_height_and_timestamp().await.unwrap();
	let connections = chain_a
		.query_connection_using_client(
			latest_height.revision_height as u32,
			chain_b.client_id().to_string(),
		)
		.await
		.unwrap();

	for connection in connections {
		let connection_id = ConnectionId::from_str(&connection.id).unwrap();
		let connection_end = chain_a
			.query_connection_end(latest_height, connection_id.clone())
			.await
			.unwrap()
			.connection
			.unwrap();

		let delay_period = Duration::from_nanos(connection_end.delay_period);

		dbg!(&connection_delay);
		dbg!(&delay_period);

		if delay_period != connection_delay {
			continue
		}

		let channels = chain_a
			.query_connection_channels(latest_height, &connection_id)
			.await
			.unwrap()
			.channels;

		for channel in channels {
			let channel_id = ChannelId::from_str(&channel.channel_id).unwrap();
			let channel_end = chain_a
				.query_channel_end(latest_height, channel_id, port_id.clone())
				.await
				.unwrap()
				.channel
				.unwrap();
			let channel_end = ChannelEnd::try_from(channel_end).unwrap();

			if channel_end.state == State::Open && channel.port_id == port_id.to_string() {
				return (
					handle,
					channel_id,
					channel_end.counterparty().channel_id.unwrap().clone(),
					channel_end.connection_hops[0].clone(),
				)
			}
		}
	}

	let (connection_id_a, connection_id_b) =
		create_connection(chain_a, chain_b, connection_delay).await.unwrap();

	log::info!(target: "hyperspace", "============ Connection handshake completed: ConnectionId({connection_id_a}), ConnectionId({connection_id_b}) ============");
	log::info!(target: "hyperspace", "=========================== Starting channel handshake ===========================");

	let (channel_id_a, channel_id_b) =
		create_channel(chain_a, chain_b, connection_id_a.clone(), port_id, version, Order::Ordered)
			.await
			.unwrap();
	// channel handshake completed
	log::info!(target: "hyperspace", "============ Channel handshake completed: ChannelId({channel_id_a}) ============");

	(handle, channel_id_a, channel_id_b, connection_id_a)
}

/// Send a ordered packets and assert acknowledgement
async fn send_ordered_packet_and_assert_acknowledgement<A, B>(
	chain_a: &A,
	chain_b: &B,
	channel_id: ChannelId,
) where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	chain_a
		.send_ordered_packet(
			channel_id,
			Timeout::Offset { height: Some(100), timestamp: Some(60 * 60) },
		)
		.await
		.unwrap();

	chain_a
		.send_ordered_packet(
			channel_id,
			Timeout::Offset { height: Some(100), timestamp: Some(60 * 60) },
		)
		.await
		.unwrap();

	let future = chain_b
		.ibc_events()
		.await
		.skip_while(|ev| future::ready(!matches!(ev, IbcEvent::AcknowledgePacket(_))))
		.take(2)
		.collect::<Vec<_>>();
	timeout_future(
		future,
		20 * 60,
		format!("Didn't see Acknowledgement packet on {}", chain_b.name()),
	)
	.await;
}

/// Send a packet on an ordered channel and assert timeout
async fn send_ordered_packet_and_assert_timeout<A, B>(
	chain_a: &A,
	chain_b: &B,
	channel_id: ChannelId,
) where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	log::info!(target: "hyperspace", "Suspending send packet relay");
	set_relay_status(false);

	let timestamp = 60 * 2;
	chain_a
		.send_ordered_packet(
			channel_id,
			Timeout::Offset { height: Some(200), timestamp: Some(timestamp) },
		)
		.await
		.unwrap();
	let timeout_timestamp = Duration::from_secs(timestamp).as_nanos() as u64;

	// Wait timeout timestamp to elapse, then
	let future = chain_b
		.subscribe_blocks()
		.await
		.skip_while(|block_number| {
			let block_number = *block_number;
			let chain_clone = chain_b.clone();
			async move {
				let timestamp = chain_clone.query_timestamp_at(block_number).await.unwrap();
				timestamp < timeout_timestamp
			}
		})
		.take(1)
		.collect::<Vec<_>>();

	log::info!(target: "hyperspace", "Waiting for packet timeout to elapse on counterparty");
	timeout_future(
		future,
		10 * 60,
		format!("Timeout timestamp was not reached on {}", chain_b.name()),
	)
	.await;

	set_relay_status(true);

	assert_timeout_packet(chain_a, 130).await;
	log::info!(target: "hyperspace", "🚀🚀 Timeout packet successfully processed for ordered channel");
}

///
pub async fn ibc_messaging_ordered_packet_with_connection_delay<A, B>(
	chain_a: &mut A,
	chain_b: &mut B,
	port_id: PortId,
	version: String,
) where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (handle, channel_id, channel_b, _connection_id) = setup_connection_and_channel(
		chain_a,
		chain_b,
		Duration::from_secs(60 * 2),
		port_id.clone(),
		version,
	)
	.await;
	handle.abort();
	// Set channel whitelist and restart relayer loop
	chain_a.set_channel_whitelist(vec![(channel_id, port_id.clone())].into_iter().collect());
	chain_b.set_channel_whitelist(vec![(channel_b, port_id)].into_iter().collect());
	let client_a_clone = chain_a.clone();
	let client_b_clone = chain_b.clone();
	let handle = tokio::task::spawn(async move {
		hyperspace_core::relay(client_a_clone, client_b_clone, None, None, None)
			.await
			.unwrap()
	});
	send_ordered_packet_and_assert_acknowledgement(chain_a, chain_b, channel_id).await;
	handle.abort()
}

///
pub async fn ibc_messaging_ordered_packet_timeout<A, B>(
	chain_a: &mut A,
	chain_b: &mut B,
	port_id: PortId,
	version: String,
) where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (handle, channel_id, channel_b, _connection_id) = setup_connection_and_channel(
		chain_a,
		chain_b,
		Duration::from_secs(60 * 2),
		port_id.clone(),
		version,
	)
	.await;
	// Set channel whitelist and restart relayer loop
	handle.abort();
	chain_a.set_channel_whitelist(vec![(channel_id, port_id.clone())].into_iter().collect());
	chain_b.set_channel_whitelist(vec![(channel_b, port_id)].into_iter().collect());
	let client_a_clone = chain_a.clone();
	let client_b_clone = chain_b.clone();
	let handle = tokio::task::spawn(async move {
		hyperspace_core::relay(client_a_clone, client_b_clone, None, None, None)
			.await
			.unwrap()
	});
	send_ordered_packet_and_assert_timeout(chain_a, chain_b, channel_id).await;
	handle.abort()
}
