use ethereum::client::ClientError;
use ibc::{
	core::{ics02_client::client_state::ClientState, ics24_host::identifier::ClientId},
	timestamp::Timestamp,
	Height,
};
use pallet_ibc::light_clients::AnyClientState;
use primitives::Chain;

pub async fn query_latest_update_time_and_height<A: Chain, B: Chain>(
	source: &A,
	sink: &B,
) -> anyhow::Result<(Timestamp, Height)> {
	let (client_id, client_state) = query_latest_client_state(source, sink).await?;
	let (update_height, update_time) = sink
		.query_client_update_time_and_height(client_id.clone(), client_state.latest_height())
		.await?;

	Ok((update_time, update_height))
}

async fn query_latest_client_state<A: Chain, B: Chain>(
	source: &A,
	sink: &B,
) -> anyhow::Result<(ClientId, AnyClientState)> {
	let client_id = source.client_id();
	let latest_cp_height = sink.latest_height_and_timestamp().await?.0;
	let latest_cp_client_state =
		sink.query_client_state(latest_cp_height, client_id.clone()).await?;
	let client_state_response = latest_cp_client_state.client_state.ok_or_else(|| {
		ClientError::Other("counterparty returned empty client state".to_string())
	})?;
	let client_state = AnyClientState::try_from(client_state_response)?;
	Ok((client_id, client_state))
}
