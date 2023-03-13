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

use crate::Error;
use beefy_light_client_primitives::{ClientState, MmrUpdateProof};
use beefy_primitives::known_payloads::MMR_ROOT_ID;
use beefy_prover::helpers::unsafe_arc_cast;
use codec::Decode;
use frame_support::pallet_prelude::{DispatchClass, Weight};
use frame_system::limits::BlockWeights;
use sp_core::H256;
use std::sync::Arc;

pub fn get_updated_client_state(
	mut client_state: ClientState,
	mmr_update: &MmrUpdateProof,
) -> ClientState {
	if mmr_update.signed_commitment.commitment.validator_set_id == client_state.next_authorities.id
	{
		client_state.current_authorities = client_state.next_authorities.clone();
		client_state.next_authorities = mmr_update.latest_mmr_leaf.beefy_next_authority_set.clone();
	}

	client_state.latest_beefy_height = mmr_update.signed_commitment.commitment.block_number;
	if let Some(mmr_root_hash) =
		mmr_update.signed_commitment.commitment.payload.get_raw(&MMR_ROOT_ID)
	{
		let mmr_root_hash = H256::from_slice(&*mmr_root_hash);
		client_state.mmr_root_hash = mmr_root_hash;
	}

	client_state
}

/// Fetch the maximum allowed extrinsic weight from a substrate node with the given client.
pub async fn fetch_max_extrinsic_weight<T: light_client_common::config::Config>(
	client: &subxt::OnlineClient<T>,
) -> Result<u64, Error> {
	let metadata = client.rpc().metadata(None).await?;
	let block_weights = metadata.pallet("System")?.constant("BlockWeights")?;
	let weights = BlockWeights::decode(&mut &block_weights.value[..])?;
	let extrinsic_weights = weights.per_class.get(DispatchClass::Normal);
	let max_extrinsic_weight = extrinsic_weights
		.max_extrinsic
		.or(extrinsic_weights.max_total)
		.unwrap_or(Weight::from_ref_time(u64::MAX));
	Ok(max_extrinsic_weight.ref_time())
}

pub unsafe fn unsafe_cast_to_jsonrpsee_client(
	client: &Arc<jsonrpsee_ws_client::WsClient>,
) -> Arc<jsonrpsee::core::client::Client> {
	unsafe_arc_cast::<_, _>(client.clone())
}
