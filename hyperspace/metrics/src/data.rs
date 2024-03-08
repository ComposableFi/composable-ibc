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

use super::*;
use crate::register;
use ibc::{core::ics24_host::identifier::ClientId, Height};
use std::collections::HashMap;

/// Optional shareable link to basic metrics.
#[derive(Clone, Default)]
pub struct MetricsLink(Option<Metrics>);

impl MetricsLink {
	pub fn new(prefix: &str, registry: Option<&Registry>) -> Self {
		Self(registry.and_then(|registry| {
			Metrics::register(prefix, registry)
				.map_err(|err| {
					log::warn!("Failed to register proposer prometheus metrics: {}", err)
				})
				.ok()
		}))
	}

	pub fn report<O>(&self, do_this: impl FnOnce(&Metrics) -> O) -> Option<O> {
		self.0.as_ref().map(do_this)
	}
}

#[derive(Clone)]
pub struct LightClientMetrics {
	/// The height of the last trusted state.
	pub height: Counter<U64>,
	/// The revision of the last trusted state.
	pub revision: Gauge<U64>,
	/// Total number of header updates received.
	pub number_of_received_header_updates: Counter<U64>,
}

impl LightClientMetrics {
	pub fn register(
		client_id: &ClientId,
		prefix: &str,
		registry: &Registry,
	) -> Result<Self, PrometheusError> {
		Ok(Self {
			height: register(
				Counter::with_opts(
					Opts::new(
						"hyperspace_light_client_revision_height",
						"The height of the last trusted state",
					)
					.const_label("client_id", client_id.to_string())
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			revision: register(
				Gauge::with_opts(
					Opts::new(
						"hyperspace_light_client_revision",
						"The revision of the last trusted state",
					)
					.const_label("client_id", client_id.to_string())
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			number_of_received_header_updates: register(
				Counter::with_opts(
					Opts::new(
						"hyperspace_number_of_received_header_updates",
						"Total number of header updates received",
					)
					.const_label("client_id", client_id.to_string())
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
		})
	}
}

#[derive(Clone)]
pub struct Metrics {
	/// Total number of "send packet" events received.
	pub number_of_received_send_packets: Counter<U64>,
	/// Total number of "receive packet" events received.
	pub number_of_received_receive_packets: Counter<U64>,
	/// Total number of "acknowledge packet" events received.
	pub number_of_received_acknowledge_packets: Counter<U64>,
	/// Total number of "timeout packet" events received.
	pub number_of_received_timeouts: Counter<U64>,

	/// Total number of received packets on the counterparty's side.
	pub counterparty_number_of_received_packets: Option<Counter<U64>>,
	/// Total number of received acknowledgments on the counterparty's side.
	pub counterparty_number_of_received_acknowledgments: Option<Counter<U64>>,

	/// Total number of sent packets.
	pub number_of_sent_packets: Counter<U64>,
	/// Total number of sent acknowledgments.
	pub number_of_sent_acknowledgments: Counter<U64>,
	/// Total number of timed out packets.
	pub number_of_sent_timeout_packets: Counter<U64>,

	/// Number of undelivered packets over time.
	pub number_of_undelivered_packets: Gauge<U64>,
	/// Number of undelivered acknowledgements over time.
	pub number_of_undelivered_acknowledgements: Gauge<U64>,
	/// Gas cost for every sent tx bundle.
	pub gas_cost_for_sent_tx_bundle: Histogram,
	/// Transaction length (in bytes) for every sent tx bundle.
	pub transaction_length_for_sent_tx_bundle: Histogram,

	/// Light client height.
	pub light_client_height: HashMap<ClientId, LightClientMetrics>,

	/// Average time between "send packet" events.
	pub send_packet_event_time: Histogram,
	/// Average time between "receive packet" events.
	pub receive_packet_event_time: Histogram,
	/// Average time between "acknowledge packet" events.
	pub acknowledge_packet_event_time: Histogram,
	/// Average time between sending and receiving packets.
	pub sent_packet_time: Histogram,
	/// Average time between sending and receiving acknowledgments.
	pub sent_acknowledgment_time: Histogram,
	/// Average time between sending and receiving timeout packets.
	pub sent_timeout_packet_time: Histogram,
	/// Average time between client updates.
	pub sent_update_client_time: Histogram,

	/// Latest processed height - helpful to prevent pushing the same event twice
	pub latest_processed_height: Gauge<U64>,

	/// Metrics prefix.
	pub prefix: String,
}

impl Metrics {
	pub fn register(prefix: &str, registry: &Registry) -> Result<Self, PrometheusError> {
		Ok(Self {
			number_of_received_send_packets: register(
				Counter::with_opts(
					Opts::new(
						format!("hyperspace_{prefix}_number_of_send_packet_events"),
						"Total number of 'send packet' events.",
					)
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			number_of_received_receive_packets: register(
				Counter::with_opts(
					Opts::new(
						format!("hyperspace_{prefix}_number_of_receive_packet_events"),
						"Total number of 'receive packet' events.",
					)
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			number_of_received_acknowledge_packets: register(
				Counter::with_opts(
					Opts::new(
						"hyperspace_number_of_acknowledge_packet_events".to_string(),
						"Total number of 'acknowledge packet' events.",
					)
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			number_of_received_timeouts: register(
				Counter::with_opts(
					Opts::new(
						"hyperspace_number_of_timeout_packet_events".to_string(),
						"Total number of 'timeout packet' events.",
					)
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			counterparty_number_of_received_packets: None,
			counterparty_number_of_received_acknowledgments: None,
			number_of_sent_packets: register(
				Counter::with_opts(
					Opts::new(
						"hyperspace_number_of_sent_packets".to_string(),
						"Total number of sent packets",
					)
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			number_of_sent_acknowledgments: register(
				Counter::with_opts(
					Opts::new(
						"hyperspace_number_of_sent_acknowledgments".to_string(),
						"Total number of sent acknowledgments",
					)
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			number_of_sent_timeout_packets: register(
				Counter::with_opts(
					Opts::new(
						"hyperspace_number_of_timed_out_packets".to_string(),
						"Total number of timed out packets",
					)
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			number_of_undelivered_packets: register(
				Gauge::with_opts(
					Opts::new(
						"hyperspace_number_of_undelivered_packets".to_string(),
						"Number of undelivered packets over time",
					)
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			number_of_undelivered_acknowledgements: register(
				Gauge::with_opts(
					Opts::new(
						"hyperspace_number_of_undelivered_acknowledgements".to_string(),
						"Number of undelivered acknowledgements over time",
					)
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			gas_cost_for_sent_tx_bundle: register(
				Histogram::with_opts(
					HistogramOpts::new(
						"hyperspace_gas_cost_for_sent_tx_bundle".to_string(),
						"Gas cost for every sent tx bundle",
					)
					.buckets(vec![1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0])
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			transaction_length_for_sent_tx_bundle: register(
				Histogram::with_opts(
					HistogramOpts::new(
						"hyperspace_transaction_length_for_sent_tx_bundle".to_string(),
						"Transaction length for every sent tx bundle",
					)
					.buckets(vec![1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0])
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			light_client_height: HashMap::new(),
			send_packet_event_time: register(
				Histogram::with_opts(
					HistogramOpts::new(
						"hyperspace_send_packet_event_time".to_string(),
						"Time it takes to process a 'send packet' event",
					)
					.buckets(vec![1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0])
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			receive_packet_event_time: register(
				Histogram::with_opts(
					HistogramOpts::new(
						"hyperspace_receive_packet_event_time".to_string(),
						"Time it takes to process a 'receive packet' event",
					)
					.buckets(vec![1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0])
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			acknowledge_packet_event_time: register(
				Histogram::with_opts(
					HistogramOpts::new(
						"hyperspace_acknowledge_packet_event_time".to_string(),
						"Time it takes to process a 'acknowledge packet' event",
					)
					.buckets(vec![1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0])
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			sent_packet_time: register(
				Histogram::with_opts(
					HistogramOpts::new(
						"hyperspace_sent_packet_time".to_string(),
						"Time it takes to send and receive a packet",
					)
					.buckets(vec![1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0])
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			sent_acknowledgment_time: register(
				Histogram::with_opts(
					HistogramOpts::new(
						"hyperspace_sent_acknowledgment_time".to_string(),
						"Time it takes to send and receive an acknowledgment",
					)
					.buckets(vec![1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0])
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			sent_timeout_packet_time: register(
				Histogram::with_opts(
					HistogramOpts::new(
						"hyperspace_sent_timeout_packet_time".to_string(),
						"Time it takes to send and receive a timeout packet",
					)
					.buckets(vec![1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0])
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			sent_update_client_time: register(
				Histogram::with_opts(
					HistogramOpts::new(
						"hyperspace_sent_update_client_time".to_string(),
						"Average time between client updates",
					)
					.buckets(vec![1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0])
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			latest_processed_height: register(
				Gauge::with_opts(
					Opts::new(
						"hyperspace_latest_processed_height".to_string(),
						"Latest processed finalized height",
					)
					.const_label("name", prefix.to_string()),
				)?,
				registry,
			)?,
			prefix: prefix.to_string(),
		})
	}

	pub fn link_with_counterparty_metrics(&mut self, other: &mut Metrics) {
		self.counterparty_number_of_received_packets =
			Some(other.number_of_received_receive_packets.clone());
		self.counterparty_number_of_received_acknowledgments =
			Some(other.number_of_received_acknowledge_packets.clone());

		other.counterparty_number_of_received_packets =
			Some(self.number_of_received_receive_packets.clone());
		other.counterparty_number_of_received_acknowledgments =
			Some(self.number_of_received_acknowledge_packets.clone());
	}

	pub fn counterparty_number_of_received_packets(&self) -> &Counter<U64> {
		self.counterparty_number_of_received_packets
            .as_ref()
            .expect("counterparty_number_of_received_packets is not set. Perhaps you forgot to call `link_with_counterparty_metrics`?")
	}

	pub fn counterparty_number_of_received_acknowledgments(&self) -> &Counter<U64> {
		self.counterparty_number_of_received_acknowledgments
            .as_ref()
            .expect("counterparty_number_of_received_acknowledgments is not set. Perhaps you forgot to call `link_with_counterparty_metrics`?")
	}

	pub fn update_light_client_height(
		&mut self,
		client_id: &ClientId,
		height: Height,
		registry: &Registry,
	) -> anyhow::Result<()> {
		match self.light_client_height.get(client_id) {
			Some(metrics) => {
				let current = metrics.height.get();
				metrics.height.inc_by(height.revision_height.saturating_sub(current));
				metrics.revision.set(height.revision_number);
				metrics.number_of_received_header_updates.inc();
				Ok(())
			},
			None => {
				let light_client_metrics =
					LightClientMetrics::register(client_id, &self.prefix, registry)?;
				self.light_client_height.insert(client_id.clone(), light_client_metrics);
				Ok(())
			},
		}
	}

	pub fn update_latest_processed_height(&mut self, revision_height: u64) -> anyhow::Result<()> {
		self.latest_processed_height.set(revision_height);
		Ok(())
	}
}
