#![warn(unused_variables)]

use futures::StreamExt;
use primitives::Chain;

pub mod chain;
pub mod command;
pub mod events;
pub mod logging;
mod macros;
pub mod packets;
pub mod queue;

use events::{has_packet_events, parse_events};
use ibc::{core::ics24_host::identifier::ClientId, events::IbcEvent};
use metrics::handler::MetricsHandler;

/// Core relayer loop, waits for new finality events and forwards any new [`ibc::IbcEvents`]
/// to the counter party chain.
pub async fn relay<A, B>(
	mut chain_a: A,
	mut chain_b: B,
	mut chain_a_metrics: Option<MetricsHandler>,
	mut chain_b_metrics: Option<MetricsHandler>,
) -> Result<(), anyhow::Error>
where
	A: Chain,
	B: Chain,
{
	let (mut chain_a_finality, mut chain_b_finality) =
		(chain_a.finality_notifications().await, chain_b.finality_notifications().await);
	// loop forever
	loop {
		tokio::select! {
			// new finality event from chain A
			result = chain_a_finality.next() => {
				process_finality_event!(chain_a, chain_b, chain_a_metrics, result)
			}
			// new finality event from chain B
			result = chain_b_finality.next() => {
				process_finality_event!(chain_b, chain_a, chain_b_metrics, result)
			}
		}
	}

	Ok(())
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
		chain_a.ibc_events().await.filter(|ev| match ev {
			IbcEvent::UpdateClient(update) => chain_b.client_id() == update.client_id(),
			_ => false,
		}),
		chain_b.ibc_events().await.filter(|ev| match ev {
			IbcEvent::UpdateClient(update) => chain_a.client_id() == update.client_id(),
			_ => false,
		}),
	);

	// loop forever
	loop {
		tokio::select! {
			// new finality event from chain A
			update = chain_a_client_updates.next() => {
				let message = chain_a.query_client_message(update).await?;
				chain_b.check_for_misbehaviour(&chain_a, message).await?;
			}
			// new finality event from chain B
			update = chain_b_client_updates.next() => {
				let message = chain_b.query_client_message(update).await?;
				chain_a.check_for_misbehaviour(&chain_b, message).await?;
			}
		}
	}

	Ok(())
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
