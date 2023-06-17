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
use std::{
	sync::{
		atomic::{AtomicUsize, Ordering},
		Arc,
	},
	time::Duration,
};
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
	query_undelivered_acks, query_undelivered_sequences, Chain, UndeliveredType,
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
			// this can happen in case the channel is not yet created
			Err(e) => {
				log::warn!(target: "hyperspace", "Failed to query channel end for chain {}, channel {}/{}: {:?}", source.name(), channel_id, port_id, e);
				continue
			},
		};
		let source_channel_end = match source_channel_response.channel.map(ChannelEnd::try_from) {
			Some(Ok(source_channel)) => source_channel,
			_ => {
				log::warn!(target: "hyperspace", "ChannelEnd not found for {:?}/{:?}", channel_id, port_id.clone());
				continue
			},
		};
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
		let sink_channel_response = match sink
			.query_channel_end(sink_height, sink_channel_id, sink_port_id.clone())
			.await
		{
			Ok(response) => response,
			Err(e) => {
				// this can happen in case the channel is not yet created
				log::warn!(target: "hyperspace", "Failed to query channel end for chain {}, channel {}/{}: {:?}", sink.name(), channel_id, port_id, e);
				continue
			},
		};

		let sink_channel_end = match sink_channel_response.channel.map(ChannelEnd::try_from) {
			Some(Ok(sink_channel)) => sink_channel,
			_ => {
				log::warn!(target: "hyperspace", "ChannelEnd not found for {:?}/{:?}", channel_id, port_id.clone());
				continue
			},
		};

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

		log::trace!(target: "hyperspace", "Found {} undelivered packets for {:?}/{:?} for {seqs:?}", seqs.len(), channel_id, port_id.clone());

		let send_packets = source.query_send_packets(channel_id, port_id.clone(), seqs).await?;
		log::trace!(target: "hyperspace", "SendPackets count: {}", send_packets.len());
		let mut timeout_packets_join_set: JoinSet<Result<_, anyhow::Error>> = JoinSet::new();
		let source = Arc::new(source.clone());
		let sink = Arc::new(sink.clone());
		let mut timeout_packets_count = Arc::new(AtomicUsize::new(0));
		let mut send_packets_count = Arc::new(AtomicUsize::new(0));
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
				let timeout_packets_count = timeout_packets_count.clone();
				let send_packets_count = send_packets_count.clone();
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
						timeout_packets_count.fetch_add(1, Ordering::SeqCst);
						// so we know this packet has timed out on the sink, we need to find the maximum
						// consensus state height at which we can generate a non-membership proof of the
						// packet for the sink's client on the source.
						let proof_height =
							if let Some(proof_height) = get_timeout_proof_height(
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
						send_packets_count.fetch_add(1, Ordering::SeqCst);
						return Ok(None)
					}

					let proof_height = if let Some(proof_height) = find_suitable_proof_height_for_client(
						&**source,
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

		let timeouts_count = timeout_packets_count.load(Ordering::SeqCst);
		log::debug!(target: "hyperspace", "Found {timeouts_count} packets that have timed out");
		source
			.on_undelivered_sequences(timeouts_count == 0, UndeliveredType::Timeouts)
			.await?;

		let sends_count = send_packets_count.load(Ordering::SeqCst);
		log::debug!(target: "hyperspace", "Found {sends_count} sent packets");
		sink.on_undelivered_sequences(sends_count == 0, UndeliveredType::Sends).await?;

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

		/*
		[2023-05-16T21:42:27Z DEBUG hyperspace_cosmos] Simulated transaction: events: [
			Event { r#type: "coin_spent", attributes: [EventAttribute { key: "spender", value: "banksy1dd076q46xe0vs92vdq0e3m0dg95ahxhwzew66y", index: false }, EventAttribute { key: "amount", value: "250000stake", index: false }] },
			Event { r#type: "coin_received", attributes: [EventAttribute { key: "receiver", value: "banksy17xpfvakm2amg962yls6f84z3kell8c5l67u43k", index: false }, EventAttribute { key: "amount", value: "250000stake", index: false }] },
			Event { r#type: "transfer", attributes: [EventAttribute { key: "recipient", value: "banksy17xpfvakm2amg962yls6f84z3kell8c5l67u43k", index: false }, EventAttribute { key: "sender", value: "banksy1dd076q46xe0vs92vdq0e3m0dg95ahxhwzew66y", index: false }, EventAttribute { key: "amount", value: "250000stake", index: false }] },
			Event { r#type: "message", attributes: [EventAttribute { key: "sender", value: "banksy1dd076q46xe0vs92vdq0e3m0dg95ahxhwzew66y", index: false }] },
			Event { r#type: "tx", attributes: [EventAttribute { key: "fee", value: "250000stake", index: false }, EventAttribute { key: "fee_payer", value: "banksy1dd076q46xe0vs92vdq0e3m0dg95ahxhwzew66y", index: false }] },
			Event { r#type: "tx", attributes: [EventAttribute { key: "acc_seq", value: "banksy1dd076q46xe0vs92vdq0e3m0dg95ahxhwzew66y/425", index: false }] },
			Event { r#type: "tx", attributes: [EventAttribute { key: "signature", value: "43Hx6M2Fl6AK3j3WJjIoU+9RGAfRIBOgm4wpowlLK9tmhWYSv3yUyE3SLTX4xL+5pjsNsH8rKGUOCmeRX2SP+A==", index: false }] },
			Event { r#type: "message", attributes: [EventAttribute { key: "action", value: "/ibc.core.client.v1.MsgUpdateClient", index: false }, EventAttribute { key: "sender", value: "banksy1dd076q46xe0vs92vdq0e3m0dg95ahxhwzew66y", index: false }] },
			Event { r#type: "update_client", attributes: [EventAttribute { key: "client_id", value: "08-wasm-5", index: false }, EventAttribute { key: "client_type", value: "08-wasm", index: false }, EventAttribute { key: "consensus_height", value: "2087-613", index: false }, EventAttribute { key: "consensus_heights", value: "2087-613", index: false }, EventAttribute { key: "header", value: "0a202f6962632e6c69676874636c69656e74732e7761736d2e76312e48656164657212a0110a95110a232f6962632e6c69676874636c69656e74732e6772616e6470612e76312e48656164657212ed100ae4100a20e31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc1912c2050100000000000000e31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc19da04000014e31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc19da04000082a44426ac25c5cc71dbd3ec4d2e544ef4b8835081212410f18d9ef765ef12ad9d12db208856e701e02bfeb0edcaff68f1c658a65fafe665eff096c1afd2060c1dfe3e22cc0d45c70779c1095f7489a8ef3cf52d62fbd8c2fa38c9f1723502b5e31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc19da040000eab2b10032f980d3845d2ef5809a53513efcb5021bd26cfd383f3e283864843a214bedefd4a256ec33b06207c2102d56b142667aa380afdd8c5b3d91ac55f607568cb4a574c6d178feb39c27dfc8b3f789e5f5423e19c71633c748b9acf086b5e31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc19da0400007c816c048bb767ea52e7a85269a792f11e37d0a7236b8ec07674fc610f9644bc91f87bbfe5a65102a73f28b2d668f79deb0ed7adc223c421b9c953ac92e52a0b5e639b43e0052c47447dac87d6fd2b6ec50bdd4d0f614e4299c665249bbd09d9e31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc19da040000a2bb146f83a4e8265e25ae8e099a67a19c1e4b8fc315d9a921a2a1d931194a03e20c57e420dc4b9f7260774db92405b3291f0c121fea136192b74cf1e606fb0488dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0eee31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc19da04000041ee3d462ecf1f7fb09ffe0774938540139d820bde9e34271e7053d392fda269cda9350477b094245db8994e19c29c8eb0e441ac8f7774db179b4f6b9de03004d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69001ab20869ffe602fe6c705860dc401e9c3b42107c47346de36145ec7fc83320488bd88165132c4c28dfe1edc8b5d4241906da1335dc4ca1daa622c77720450e22bb0ef23cd937268de3b0b352d21e7544b4dde3cc36989aa1682f2121a521ca00bb470ef455180642414245b50103040000001c54bb1000000000b8c02455891caf9fe3f0a71e90cdbe5dba7f42774edce2223139b004e9af257a05e1cda9ca38442dcd4efefa964940dc4af4e523ce2b93855c397b2527b95c00ec5f4e43b65b56366309522ea1ca48ea0474960f4e10a368b34976331d34e60e044245454684033f3d52c2d6539e98f0c022df71a5bd6ec90f93f885027b61715795342f14ec84044241424549040118d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01000000000000008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48010000000000000090b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe220100000000000000306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc200100000000000000e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e01000000000000001cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0100000000000000d4d21735a12258b4a207c6b30443a7c026f8d4e8912da829bb28b066aa427246044245454641030118020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a10390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f270389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb03bc9d0ca094bd5b8b3225d7651eac5d18c1c04bf8ae8f8b263eebca4e1410ed0c031d10105e323c4afce225208f71a6441ee327a65b9e646e772500c74d31f669aa0291f1217d5a04cb83312ee3d88a6e6b33284e053e6ccfc3a90339a0299d12967c7c000000000000000446524e4bd903011888dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee0100000000000000d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae690100000000000000439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f01000000000000005e639b43e0052c47447dac87d6fd2b6ec50bdd4d0f614e4299c665249bbd09d901000000000000001dfe3e22cc0d45c70779c1095f7489a8ef3cf52d62fbd8c2fa38c9f1723502b50100000000000000568cb4a574c6d178feb39c27dfc8b3f789e5f5423e19c71633c748b9acf086b5010000000000000000000000054241424501016a37deaae3971a248a13b99b06efac4a820249f477a73e125d8ad3057d78580d12b7aab91766d88d9e28a7ebb94d0c2789332fcc5ea8c8e8f7b746263912b7891ac502feaa873104489f12ac9a0744beced16952b1b4f64821c347819ab96d5ad8cf9169138d40cecadc92b7268c9ebcd37b4af9f2a50b150fcffdd3dd761ca771046b134aeaaf7f651333bd6ee56206fb01f9f8782e2c01343a269d1e05d0a471ed3c80120c0642414245b50103020000001d54bb100000000014e56b56b99372893d47ac7addba5a3a3f829a8f90d31d2dc2a92ca922fcdf19d71c2aa8b83ad4efb1596615b8259b53b4bb72936eba8dfe5930e465b276160f4d954845d83e475cb5314f9747a67d6154ea6e4cdb1c1372fc69be36103e7403044245454684037823f9c3f130b12e4d0a59126c5999dcf08ef8554c2863470a89e24b60426d4a0542414245010132549774ee0cc297ad7455e543d05fba54c71edde67d7bebcf809023181b5551f2c5a1e53b41a470a9a120fde03b877f6e1b87cda5b5262b177e117c3d481c8d18a71020e504120608a71010e504", index: false }] },
			Event { r#type: "message", attributes: [EventAttribute { key: "module", value: "ibc_client", index: false }] },
			Event { r#type: "message", attributes: [EventAttribute { key: "action", value: "/ibc.core.channel.v1.MsgRecvPacket", index: false }, EventAttribute { key: "sender", value: "banksy1dd076q46xe0vs92vdq0e3m0dg95ahxhwzew66y", index: false }] },
			Event { r#type: "recv_packet", attributes: [EventAttribute { key: "packet_data", value: "{\"denom\":\"1\",\"amount\":\"2000000000000\",\"sender\":\"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\",\"receiver\":\"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48\",\"memo\":\"\"}", index: false }, EventAttribute { key: "packet_data_hex", value: "7b2264656e6f6d223a2231222c22616d6f756e74223a2232303030303030303030303030222c2273656e646572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764222c227265636569766572223a22307838656166303431353136383737333633323663396665613137653235666335323837363133363933633931323930396362323236616134373934663236613438222c226d656d6f223a22227d", index: false }, EventAttribute { key: "packet_timeout_height", value: "1-123631", index: false }, EventAttribute { key: "packet_timeout_timestamp", value: "1684373166165272000", index: false }, EventAttribute { key: "packet_sequence", value: "2", index: false }, EventAttribute { key: "packet_src_port", value: "transfer", index: false }, EventAttribute { key: "packet_src_channel", value: "channel-0", index: false }, EventAttribute { key: "packet_dst_port", value: "transfer", index: false }, EventAttribute { key: "packet_dst_channel", value: "channel-2", index: false }, EventAttribute { key: "packet_channel_ordering", value: "ORDER_UNORDERED", index: false }, EventAttribute { key: "packet_connection", value: "connection-6", index: false }, EventAttribute { key: "connection_id", value: "connection-6", index: false }] }, Event { r#type: "message", attributes: [EventAttribute { key: "module", value: "ibc_channel", index: false }] }, Event { r#type: "fungible_token_packet", attributes: [EventAttribute { key: "module", value: "transfer", index: false }, EventAttribute { key: "sender", value: "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d", index: false }, EventAttribute { key: "receiver", value: "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48", index: false }, EventAttribute { key: "denom", value: "1", index: false }, EventAttribute { key: "amount", value: "2000000000000", index: false }, EventAttribute { key: "memo", value: "", index: false }, EventAttribute { key: "success", value: "false", index: false }, EventAttribute { key: "error", value: "decoding bech32 failed: invalid character not part of charset: 98", index: false }] }, Event { r#type: "write_acknowledgement", attributes: [EventAttribute { key: "packet_data", value: "{\"denom\":\"1\",\"amount\":\"2000000000000\",\"sender\":\"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\",\"receiver\":\"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48\",\"memo\":\"\"}", index: false }, EventAttribute { key: "packet_data_hex", value: "7b2264656e6f6d223a2231222c22616d6f756e74223a2232303030303030303030303030222c2273656e646572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764222c227265636569766572223a22307838656166303431353136383737333633323663396665613137653235666335323837363133363933633931323930396362323236616134373934663236613438222c226d656d6f223a22227d", index: false }, EventAttribute { key: "packet_timeout_height", value: "1-123631", index: false }, EventAttribute { key: "packet_timeout_timestamp", value: "1684373166165272000", index: false }, EventAttribute { key: "packet_sequence", value: "2", index: false }, EventAttribute { key: "packet_src_port", value: "transfer", index: false }, EventAttribute { key: "packet_src_channel", value: "channel-0", index: false }, EventAttribute { key: "packet_dst_port", value: "transfer", index: false }, EventAttribute { key: "packet_dst_channel", value: "channel-2", index: false }, EventAttribute { key: "packet_ack", value: "{\"error\":\"ABCI code: 1: error handling packet: see events for details\"}", index: false }, EventAttribute { key: "packet_ack_hex", value: "7b226572726f72223a224142434920636f64653a20313a206572726f722068616e646c696e67207061636b65743a20736565206576656e747320666f722064657461696c73227d", index: false }, EventAttribute { key: "packet_connection", value: "connection-6", index: false }, EventAttribute { key: "connection_id", value: "connection-6", index: false }] }, Event { r#type: "message", attributes: [EventAttribute { key: "module", value: "ibc_channel", index: false }] }]
		logs: [{"msg_index":0,"events":[{"type":"message","attributes":[{"key":"action","value":"/ibc.core.client.v1.MsgUpdateClient"},{"key":"sender","value":"banksy1dd076q46xe0vs92vdq0e3m0dg95ahxhwzew66y"}]},{"type":"update_client","attributes":[{"key":"client_id","value":"08-wasm-5"},{"key":"client_type","value":"08-wasm"},{"key":"consensus_height","value":"2087-613"},{"key":"consensus_heights","value":"2087-613"},{"key":"header","value":"0a202f6962632e6c69676874636c69656e74732e7761736d2e76312e48656164657212a0110a95110a232f6962632e6c69676874636c69656e74732e6772616e6470612e76312e48656164657212ed100ae4100a20e31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc1912c2050100000000000000e31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc19da04000014e31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc19da04000082a44426ac25c5cc71dbd3ec4d2e544ef4b8835081212410f18d9ef765ef12ad9d12db208856e701e02bfeb0edcaff68f1c658a65fafe665eff096c1afd2060c1dfe3e22cc0d45c70779c1095f7489a8ef3cf52d62fbd8c2fa38c9f1723502b5e31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc19da040000eab2b10032f980d3845d2ef5809a53513efcb5021bd26cfd383f3e283864843a214bedefd4a256ec33b06207c2102d56b142667aa380afdd8c5b3d91ac55f607568cb4a574c6d178feb39c27dfc8b3f789e5f5423e19c71633c748b9acf086b5e31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc19da0400007c816c048bb767ea52e7a85269a792f11e37d0a7236b8ec07674fc610f9644bc91f87bbfe5a65102a73f28b2d668f79deb0ed7adc223c421b9c953ac92e52a0b5e639b43e0052c47447dac87d6fd2b6ec50bdd4d0f614e4299c665249bbd09d9e31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc19da040000a2bb146f83a4e8265e25ae8e099a67a19c1e4b8fc315d9a921a2a1d931194a03e20c57e420dc4b9f7260774db92405b3291f0c121fea136192b74cf1e606fb0488dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0eee31f91cc4aea923519c17967651d0aaa05ead7f68407ead72a892122079dbc19da04000041ee3d462ecf1f7fb09ffe0774938540139d820bde9e34271e7053d392fda269cda9350477b094245db8994e19c29c8eb0e441ac8f7774db179b4f6b9de03004d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69001ab20869ffe602fe6c705860dc401e9c3b42107c47346de36145ec7fc83320488bd88165132c4c28dfe1edc8b5d4241906da1335dc4ca1daa622c77720450e22bb0ef23cd937268de3b0b352d21e7544b4dde3cc36989aa1682f2121a521ca00bb470ef455180642414245b50103040000001c54bb1000000000b8c02455891caf9fe3f0a71e90cdbe5dba7f42774edce2223139b004e9af257a05e1cda9ca38442dcd4efefa964940dc4af4e523ce2b93855c397b2527b95c00ec5f4e43b65b56366309522ea1ca48ea0474960f4e10a368b34976331d34e60e044245454684033f3d52c2d6539e98f0c022df71a5bd6ec90f93f885027b61715795342f14ec84044241424549040118d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01000000000000008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48010000000000000090b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe220100000000000000306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc200100000000000000e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e01000000000000001cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0100000000000000d4d21735a12258b4a207c6b30443a7c026f8d4e8912da829bb28b066aa427246044245454641030118020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a10390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f270389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb03bc9d0ca094bd5b8b3225d7651eac5d18c1c04bf8ae8f8b263eebca4e1410ed0c031d10105e323c4afce225208f71a6441ee327a65b9e646e772500c74d31f669aa0291f1217d5a04cb83312ee3d88a6e6b33284e053e6ccfc3a90339a0299d12967c7c000000000000000446524e4bd903011888dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee0100000000000000d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae690100000000000000439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f01000000000000005e639b43e0052c47447dac87d6fd2b6ec50bdd4d0f614e4299c665249bbd09d901000000000000001dfe3e22cc0d45c70779c1095f7489a8ef3cf52d62fbd8c2fa38c9f1723502b50100000000000000568cb4a574c6d178feb39c27dfc8b3f789e5f5423e19c71633c748b9acf086b5010000000000000000000000054241424501016a37deaae3971a248a13b99b06efac4a820249f477a73e125d8ad3057d78580d12b7aab91766d88d9e28a7ebb94d0c2789332fcc5ea8c8e8f7b746263912b7891ac502feaa873104489f12ac9a0744beced16952b1b4f64821c347819ab96d5ad8cf9169138d40cecadc92b7268c9ebcd37b4af9f2a50b150fcffdd3dd761ca771046b134aeaaf7f651333bd6ee56206fb01f9f8782e2c01343a269d1e05d0a471ed3c80120c0642414245b50103020000001d54bb100000000014e56b56b99372893d47ac7addba5a3a3f829a8f90d31d2dc2a92ca922fcdf19d71c2aa8b83ad4efb1596615b8259b53b4bb72936eba8dfe5930e465b276160f4d954845d83e475cb5314f9747a67d6154ea6e4cdb1c1372fc69be36103e7403044245454684037823f9c3f130b12e4d0a59126c5999dcf08ef8554c2863470a89e24b60426d4a0542414245010132549774ee0cc297ad7455e543d05fba54c71edde67d7bebcf809023181b5551f2c5a1e53b41a470a9a120fde03b877f6e1b87cda5b5262b177e117c3d481c8d18a71020e504120608a71010e504"}]},{"type":"message","attributes":[{"key":"module","value":"ibc_client"}]}]},{"msg_index":1,"events":[{"type":"message","attributes":[{"key":"action","value":"/ibc.core.channel.v1.MsgRecvPacket"},{"key":"sender","value":"banksy1dd076q46xe0vs92vdq0e3m0dg95ahxhwzew66y"}]},{"type":"recv_packet","attributes":[{"key":"packet_data","value":"{\"denom\":\"1\",\"amount\":\"2000000000000\",\"sender\":\"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\",\"receiver\":\"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48\",\"memo\":\"\"}"},{"key":"packet_data_hex","value":"7b2264656e6f6d223a2231222c22616d6f756e74223a2232303030303030303030303030222c2273656e646572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764222c227265636569766572223a22307838656166303431353136383737333633323663396665613137653235666335323837363133363933633931323930396362323236616134373934663236613438222c226d656d6f223a22227d"},{"key":"packet_timeout_height","value":"1-123631"},{"key":"packet_timeout_timestamp","value":"1684373166165272000"},{"key":"packet_sequence","value":"2"},{"key":"packet_src_port","value":"transfer"},{"key":"packet_src_channel","value":"channel-0"},{"key":"packet_dst_port","value":"transfer"},{"key":"packet_dst_channel","value":"channel-2"},{"key":"packet_channel_ordering","value":"ORDER_UNORDERED"},{"key":"packet_connection","value":"connection-6"},{"key":"connection_id","value":"connection-6"}]},{"type":"message","attributes":[{"key":"module","value":"ibc_channel"}]},{"type":"fungible_token_packet","attributes":[{"key":"module","value":"transfer"},{"key":"sender","value":"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"},{"key":"receiver","value":"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"},{"key":"denom","value":"1"},{"key":"amount","value":"2000000000000"},{"key":"memo"},{"key":"success","value":"false"},{"key":"error","value":"decoding bech32 failed: invalid character not part of charset: 98"}]},{"type":"write_acknowledgement","attributes":[{"key":"packet_data","value":"{\"denom\":\"1\",\"amount\":\"2000000000000\",\"sender\":\"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\",\"receiver\":\"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48\",\"memo\":\"\"}"},{"key":"packet_data_hex","value":"7b2264656e6f6d223a2231222c22616d6f756e74223a2232303030303030303030303030222c2273656e646572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764222c227265636569766572223a22307838656166303431353136383737333633323663396665613137653235666335323837363133363933633931323930396362323236616134373934663236613438222c226d656d6f223a22227d"},{"key":"packet_timeout_height","value":"1-123631"},{"key":"packet_timeout_timestamp","value":"1684373166165272000"},{"key":"packet_sequence","value":"2"},{"key":"packet_src_port","value":"transfer"},{"key":"packet_src_channel","value":"channel-0"},{"key":"packet_dst_port","value":"transfer"},{"key":"packet_dst_channel","value":"channel-2"},{"key":"packet_ack","value":"{\"error\":\"ABCI code: 1: error handling packet: see events for details\"}"},{"key":"packet_ack_hex","value":"7b226572726f72223a224142434920636f64653a20313a206572726f722068616e646c696e67207061636b65743a20736565206576656e747320666f722064657461696c73227d"},{"key":"packet_connection","value":"connection-6"},{"key":"connection_id","value":"connection-6"}]},{"type":"message","attributes":[{"key":"module","value":"ibc_channel"}]}]}]

		 */
		let acknowledgements = source.query_recv_packets(channel_id, port_id.clone(), acks).await?;
		log::trace!(target: "hyperspace", "Got acknowledgements for channel {:?}: {:?}", channel_id, acknowledgements);
		let mut acknowledgements_join_set: JoinSet<Result<_, anyhow::Error>> = JoinSet::new();
		sink.on_undelivered_sequences(acknowledgements.is_empty(), UndeliveredType::Acks)
			.await?;
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

					/*
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client: [host_height]
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [store_client_state] >> client_id: ClientId("07-tendermint-2"), client_state = Tendermint(ClientState { chain_id: ChainId { id: "banksy-testnet-1", version: 1 }, trust_level: TrustThreshold { numerator: 1, denominator: 3 }, trusting_period: 64000s, unbonding_period: 1814400s, max_clock_drift: 15s, latest_height: Height { revision: 1, height: 8783 }, proof_specs: ProofSpecs([ProofSpec(ProofSpec { leaf_spec: Some(LeafOp { hash: Sha256, prehash_key: NoHash, prehash_value: Sha256, length: VarProto, prefix: [0] }), inner_spec: Some(InnerSpec { child_order: [0, 1], child_size: 33, min_prefix_length: 4, max_prefix_length: 12, empty_child: [], hash: Sha256 }), max_depth: 0, min_depth: 0, prehash_key_before_comparison: false }), ProofSpec(ProofSpec { leaf_spec: Some(LeafOp { hash: Sha256, prehash_key: NoHash, prehash_value: Sha256, length: VarProto, prefix: [0] }), inner_spec: Some(InnerSpec { child_order: [0, 1], child_size: 32, min_prefix_length: 1, max_prefix_length: 1, empty_child: [], hash: Sha256 }), max_depth: 0, min_depth: 0, prehash_key_before_comparison: false })]), upgrade_path: ["upgrade", "upgradedIBCState"], frozen_height: None, _phantom: PhantomData<pallet_ibc::light_clients::HostFunctionsManager> })
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [store_consensus_state] >> client_id: ClientId("07-tendermint-2"), height = Height { revision: 1, height: 8783 }, consensus_state = Tendermint(ConsensusState { timestamp: Time(2023-05-18 2:02:19.270883), root: CommitmentRoot("7A5B7D71D7505B128231ECBDBE0BC395FE6A476565397BE49F9C26816E8A90CE"), next_validators_hash: Hash::Sha256(66B967B86055A5D71F330A68C4E2C07C2A72026EE868F983938BD85231C127F6) })
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client: [store_update_time] >> Client Height Height { revision: 1, height: 8783 }, Host Timestamp Timestamp { time: Some(Time(2023-05-18 2:06:00.024)) }
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client: [store_update_height] >> Client Height Height { revision: 1, height: 8783 }, Host Height Height { revision: 2000, height: 1081 }
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [client_type] >> client_id = ClientId("07-tendermint-2")
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [client_state] >> client_id = ClientId("07-tendermint-2")
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [client_state] >> any client_state: Tendermint(ClientState { chain_id: ChainId { id: "banksy-testnet-1", version: 1 }, trust_level: TrustThreshold { numerator: 1, denominator: 3 }, trusting_period: 64000s, unbonding_period: 1814400s, max_clock_drift: 15s, latest_height: Height { revision: 1, height: 8783 }, proof_specs: ProofSpecs([ProofSpec(ProofSpec { leaf_spec: Some(LeafOp { hash: Sha256, prehash_key: NoHash, prehash_value: Sha256, length: VarProto, prefix: [0] }), inner_spec: Some(InnerSpec { child_order: [0, 1], child_size: 33, min_prefix_length: 4, max_prefix_length: 12, empty_child: [], hash: Sha256 }), max_depth: 0, min_depth: 0, prehash_key_before_comparison: false }), ProofSpec(ProofSpec { leaf_spec: Some(LeafOp { hash: Sha256, prehash_key: NoHash, prehash_value: Sha256, length: VarProto, prefix: [0] }), inner_spec: Some(InnerSpec { child_order: [0, 1], child_size: 32, min_prefix_length: 1, max_prefix_length: 1, empty_child: [], hash: Sha256 }), max_depth: 0, min_depth: 0, prehash_key_before_comparison: false })]), upgrade_path: ["upgrade", "upgradedIBCState"], frozen_height: None, _phantom: PhantomData<pallet_ibc::light_clients::HostFunctionsManager> })
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [consensus_state] >> client_id = ClientId("07-tendermint-2"), height = Height { revision: 1, height: 8783 }
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [consensus_state] >> any consensus state = Tendermint(ConsensusState { timestamp: Time(2023-05-18 2:02:19.270883), root: CommitmentRoot("7A5B7D71D7505B128231ECBDBE0BC395FE6A476565397BE49F9C26816E8A90CE"), next_validators_hash: Hash::Sha256(66B967B86055A5D71F330A68C4E2C07C2A72026EE868F983938BD85231C127F6) })
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in channel: [client_update_time] >> height = Height { revision: 1, height: 8783 }, timestamp = 1684375560024000000
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [consensus_state] >> client_id = ClientId("07-tendermint-2"), height = Height { revision: 1, height: 8784 }
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [consensus_state] >> client_id = ClientId("07-tendermint-2"), height = Height { revision: 1, height: 8783 }
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [consensus_state] >> any consensus state = Tendermint(ConsensusState { timestamp: Time(2023-05-18 2:02:19.270883), root: CommitmentRoot("7A5B7D71D7505B128231ECBDBE0BC395FE6A476565397BE49F9C26816E8A90CE"), next_validators_hash: Hash::Sha256(66B967B86055A5D71F330A68C4E2C07C2A72026EE868F983938BD85231C127F6) })
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [consensus_state] >> client_id = ClientId("07-tendermint-2"), height = Height { revision: 1, height: 8784 }
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [consensus_state] >> client_id = ClientId("07-tendermint-2"), height = Height { revision: 1, height: 8783 }
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [consensus_state] >> any consensus state = Tendermint(ConsensusState { timestamp: Time(2023-05-18 2:02:19.270883), root: CommitmentRoot("7A5B7D71D7505B128231ECBDBE0BC395FE6A476565397BE49F9C26816E8A90CE"), next_validators_hash: Hash::Sha256(66B967B86055A5D71F330A68C4E2C07C2A72026EE868F983938BD85231C127F6) })
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client: [host_height]
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client: [host_height]
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [store_client_state] >> client_id: ClientId("07-tendermint-2"), client_state = Tendermint(ClientState { chain_id: ChainId { id: "banksy-testnet-1", version: 1 }, trust_level: TrustThreshold { numerator: 1, denominator: 3 }, trusting_period: 64000s, unbonding_period: 1814400s, max_clock_drift: 15s, latest_height: Height { revision: 1, height: 8784 }, proof_specs: ProofSpecs([ProofSpec(ProofSpec { leaf_spec: Some(LeafOp { hash: Sha256, prehash_key: NoHash, prehash_value: Sha256, length: VarProto, prefix: [0] }), inner_spec: Some(InnerSpec { child_order: [0, 1], child_size: 33, min_prefix_length: 4, max_prefix_length: 12, empty_child: [], hash: Sha256 }), max_depth: 0, min_depth: 0, prehash_key_before_comparison: false }), ProofSpec(ProofSpec { leaf_spec: Some(LeafOp { hash: Sha256, prehash_key: NoHash, prehash_value: Sha256, length: VarProto, prefix: [0] }), inner_spec: Some(InnerSpec { child_order: [0, 1], child_size: 32, min_prefix_length: 1, max_prefix_length: 1, empty_child: [], hash: Sha256 }), max_depth: 0, min_depth: 0, prehash_key_before_comparison: false })]), upgrade_path: ["upgrade", "upgradedIBCState"], frozen_height: None, _phantom: PhantomData<pallet_ibc::light_clients::HostFunctionsManager> })
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [store_consensus_state] >> client_id: ClientId("07-tendermint-2"), height = Height { revision: 1, height: 8784 }, consensus_state = Tendermint(ConsensusState { timestamp: Time(2023-05-18 2:02:24.304381), root: CommitmentRoot("7725B4ED676FAFB4E06F8F595F0B411B400A69AE922C4C75DD28763DD5A47B64"), next_validators_hash: Hash::Sha256(66B967B86055A5D71F330A68C4E2C07C2A72026EE868F983938BD85231C127F6) })
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client: [store_update_time] >> Client Height Height { revision: 1, height: 8784 }, Host Timestamp Timestamp { time: Some(Time(2023-05-18 2:06:00.024)) }
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client: [store_update_height] >> Client Height Height { revision: 1, height: 8784 }, Host Height Height { revision: 2000, height: 1081 }

2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in channel : [channel_end] >> port_id = PortId("transfer"), channel_id = ChannelId("channel-1")
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in channel : [channel_end] >> channel_end = ChannelEnd { state: Open, ordering: Unordered, remote: Counterparty { port_id: PortId("transfer"), channel_id: Some(ChannelId("channel-3")) }, connection_hops: [ConnectionId("connection-1")], version: Version("ics20-1") }
2023-05-17 23:06:00.027 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in connection : [connection_end] >> connection_id = ConnectionId("connection-1")
2023-05-17 23:06:00.028 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in connection : [connection_end] >>  connection_end = ConnectionEnd { state: Open, client_id: ClientId("07-tendermint-2"), counterparty: Counterparty { client_id: ClientId("08-wasm-6"), connection_id: Some(ConnectionId("connection-4")), prefix: ibc }, versions: [Version { identifier: "1", features: ["ORDER_ORDERED", "ORDER_UNORDERED"] }], delay_period: 10s }
2023-05-17 23:06:00.028 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in channel : [get_packet_commitment] >> packet_commitment = [41, 185, 185, 38, 107, 175, 207, 1, 159, 193, 150, 100, 254, 74, 174, 18, 31, 158, 221, 10, 208, 178, 209, 41, 131, 204, 226, 135, 174, 200, 134, 244]
2023-05-17 23:06:00.028 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [client_state] >> client_id = ClientId("07-tendermint-2")
2023-05-17 23:06:00.028 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [client_state] >> any client_state: Tendermint(ClientState { chain_id: ChainId { id: "banksy-testnet-1", version: 1 }, trust_level: TrustThreshold { numerator: 1, denominator: 3 }, trusting_period: 64000s, unbonding_period: 1814400s, max_clock_drift: 15s, latest_height: Height { revision: 1, height: 8784 }, proof_specs: ProofSpecs([ProofSpec(ProofSpec { leaf_spec: Some(LeafOp { hash: Sha256, prehash_key: NoHash, prehash_value: Sha256, length: VarProto, prefix: [0] }), inner_spec: Some(InnerSpec { child_order: [0, 1], child_size: 33, min_prefix_length: 4, max_prefix_length: 12, empty_child: [], hash: Sha256 }), max_depth: 0, min_depth: 0, prehash_key_before_comparison: false }), ProofSpec(ProofSpec { leaf_spec: Some(LeafOp { hash: Sha256, prehash_key: NoHash, prehash_value: Sha256, length: VarProto, prefix: [0] }), inner_spec: Some(InnerSpec { child_order: [0, 1], child_size: 32, min_prefix_length: 1, max_prefix_length: 1, empty_child: [], hash: Sha256 }), max_depth: 0, min_depth: 0, prehash_key_before_comparison: false })]), upgrade_path: ["upgrade", "upgradedIBCState"], frozen_height: None, _phantom: PhantomData<pallet_ibc::light_clients::HostFunctionsManager> })
2023-05-17 23:06:00.028 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [consensus_state] >> client_id = ClientId("07-tendermint-2"), height = Height { revision: 1, height: 8780 }
2023-05-17 23:06:00.028 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client : [consensus_state] >> any consensus state = Tendermint(ConsensusState { timestamp: Time(2023-05-18 2:02:04.173242), root: CommitmentRoot("07B49DF5EA9C2D9EE2B3A67DAC9830E411434E61625016A4512A368C1F39597A"), next_validators_hash: Hash::Sha256(66B967B86055A5D71F330A68C4E2C07C2A72026EE868F983938BD85231C127F6) })
2023-05-17 23:06:00.028 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in client: [host_height]
2023-05-17 23:06:00.028 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in channel: [client_update_time] >> height = Height { revision: 1, height: 8780 }, timestamp = 1684375464047000000
2023-05-17 23:06:00.028 TRACE tokio-runtime-worker pallet_ibc: [Parachain] in channel: [client_update_height] >> height = Height { revision: 1, height: 8780 }, host height [8, 208, 15, 16, 182, 8]
2023-05-17 23:06:00.028 DEBUG tokio-runtime-worker pallet_ibc: [Parachain] verify_membership:
prefix=696263
proof=0aef040aec040a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d312f73657175656e6365732f311220439dd0ea54f168850977fce615993ac34f8e2d238c76c0c9328a744e89a2230d1a0c0801180120012a040002c028222d080112060406948901201a212018b8b1cd4e5487dd06c9d630e90e4db05476e5c379cff0c346173b70cfa0983e222b08011227060a94890120505f0e6b3039e050189315bd9a868a00274aacabd9ae85abbd58d857a8f5ec5620222d08011206081a948901201a21208261cb18738d315a27474946d85c0ae81715858dee97e6abda2892ccafa18776222d080112060a2a948901201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222d080112060c56948901201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222d080112060e74948901201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222e0801120712bc02948901201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222e0801120714d604948901201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222e0801120716b609948901201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222e0801120718ee13948901201a2120763fe0dc26765a9069f0198cb4dc24ed387a710ba0bf4f75ae6e6ab3a51d80ec222e080112071aac22968901201a2120000544f6164eb772b5e4d861e753b4ad813d7b2e26489d28e22414d04c24ba670afe010afb010a03696263122002d1c5b1ce8dc9ffe45249153d97fd23379cfdb48cc4863004a9d746062fa3121a090801180120012a0100222708011201011a2052532327b15377f3e4610721f7e69778833f471f6af4d8f3f9f06c81c9ab70ec222508011221013d62060c06c86ae55dc892cbefc1d5a63c65a638f8b613ceecc55250d0af20c5222708011201011a2044ba93e2aa5de099a7e0a4cc79a5c0911956d3557cf22ba26b16117da845bdb8222508011221014ccbbe24a16228b2bc74e21d028b42b03753c679ca0dab083a621c561ba27b11222708011201011a2014a66aa0ada7b629a21909b3f812c0dfc91bb73da10c6bc9ac9eddf3f05efd31
root=07b49df5ea9c2d9ee2b3a67dac9830e411434e61625016a4512a368c1f39597a
path=acks/ports/transfer/channels/channel-3/sequences/1
value=08f7557ed51826fe18d84512bf24ec75001edbaf2123a477df72a0a9f3640a7c
2023-05-17 23:06:00.028 TRACE tokio-runtime-worker pallet_ibc: [Parachain] execution error: StringTracer: ICS04 channel error: Verification fails for the packet with the sequence number 1: client '07-tendermint' error: StringTracer: ics23 commitment error: proof verification failed


[2023-05-18T02:04:36Z TRACE hyperspace] Got acknowledgements for channel ChannelId("channel-3"): [PacketInfo { height: Some(8778), sequence: 1, source_port: "transfer", source_channel: "channel-1", destination_port: "transfer", destination_channel: "channel-3", channel_order: "0", data: [123, 34, 100, 101, 110, 111, 109, 34, 58, 34, 85, 78, 73, 84, 34, 44, 34, 97, 109, 111, 117, 110, 116, 34, 58, 34, 50, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 34, 44, 34, 115, 101, 110, 100, 101, 114, 34, 58, 34, 48, 120, 100, 52, 51, 53, 57, 51, 99, 55, 49, 53, 102, 100, 100, 51, 49, 99, 54, 49, 49, 52, 49, 97, 98, 100, 48, 52, 97, 57, 57, 102, 100, 54, 56, 50, 50, 99, 56, 53, 53, 56, 56, 53, 52, 99, 99, 100, 101, 51, 57, 97, 53, 54, 56, 52, 101, 55, 97, 53, 54, 100, 97, 50, 55, 100, 34, 44, 34, 114, 101, 99, 101, 105, 118, 101, 114, 34, 58, 34, 98, 97, 110, 107, 115, 121, 49, 103, 53, 114, 50, 118, 109, 110, 112, 54, 108, 116, 97, 57, 99, 112, 115, 116, 52, 108, 122, 99, 52, 115, 121, 121, 51, 107, 99, 106, 50, 108, 106, 57, 53, 114, 122, 112, 48, 34, 44, 34, 109, 101, 109, 111, 34, 58, 34, 34, 125], timeout_height: Height { revision_number: 1, revision_height: 1008634 }, timeout_timestamp: 1685374589403068000, ack: Some([123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 34, 65, 81, 61, 61, 34, 125]) }]
[2023-05-18T02:04:36Z TRACE hyperspace] sink_height: Height { revision: 2000, height: 1078 }, latest_source_height_on_sink: Height { revision: 1, height: 8781 }, acknowledgement.height: 8778
[2023-05-18T02:04:36Z TRACE hyperspace] Using proof height: 1-8779
[2023-05-18T02:04:36Z TRACE hyperspace] Verifying delay passed for source: 1-8809, Timestamp(2023-05-18T02:04:30.133314Z), sink: 2000-1078, Timestamp(2023-05-18T02:04:24.047Z), connection delay: 10, proof height: 1-8779, verify delay on: Sink
[2023-05-18T02:04:36Z TRACE hyperspace_parachain] Querying client update time and height for client ClientId("07-tendermint-2") at height Height { revision: 1, height: 8780 }
[2023-05-18T02:04:36Z TRACE hyperspace] Checking if delay has elapsed: current_time: Timestamp(2023-05-18T02:04:24.047Z), current_height: 2000-1078, client_update_time: Timestamp(2023-05-18T02:04:24.047Z), client_update_height: 2000-1078, delay_period_time: 10s, delay_period_blocks: 1
[2023-05-18T02:04:36Z TRACE hyperspace] Skipping acknowledgement for packet as connection delay has not passed Packet { sequence: Sequence(1), source_port: PortId("transfer"), source_channel: ChannelId("channel-1"), destination_port: PortId("transfer"), destination_channel: ChannelId("channel-3"), data: [123, 34, 100, 101, 110, 111, 109, 34, 58, 34, 85, 78, 73, 84, 34, 44, 34, 97, 109, 111, 117, 110, 116, 34, 58, 34, 50, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 34, 44, 34, 115, 101, 110, 100, 101, 114, 34, 58, 34, 48, 120, 100, 52, 51, 53, 57, 51, 99, 55, 49, 53, 102, 100, 100, 51, 49, 99, 54, 49, 49, 52, 49, 97, 98, 100, 48, 52, 97, 57, 57, 102, 100, 54, 56, 50, 50, 99, 56, 53, 53, 56, 56, 53, 52, 99, 99, 100, 101, 51, 57, 97, 53, 54, 56, 52, 101, 55, 97, 53, 54, 100, 97, 50, 55, 100, 34, 44, 34, 114, 101, 99, 101, 105, 118, 101, 114, 34, 58, 34, 98, 97, 110, 107, 115, 121, 49, 103, 53, 114, 50, 118, 109, 110, 112, 54, 108, 116, 97, 57, 99, 112, 115, 116, 52, 108, 122, 99, 52, 115, 121, 121, 51, 107, 99, 106, 50, 108, 106, 57, 53, 114, 122, 112, 48, 34, 44, 34, 109, 101, 109, 111, 34, 58, 34, 34, 125], timeout_height: Height { revision: 1, height: 1008634 }, timeout_timestamp: Timestamp { time: Some(Time(2023-05-29 15:36:29.403068)) } }

					 */
					let proof_height = if let Some(proof_height) = find_suitable_proof_height_for_client(
						&**source,
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
