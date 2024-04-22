// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
	context::Context,
	ics23::{ConsensusStates, FakeInner, ProcessedStates},
	msg::ExecuteMsg,
};
use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};
use ibc::{
	core::{
		ics02_client::{
			client_state::ClientState as _, context::ClientReader, error::Error as Ics02Error,
			height::Height,
		},
		ics23_commitment::commitment::CommitmentProofBytes,
		ics24_host::identifier::ClientId,
	},
	protobuf::Protobuf,
};
use ibc_proto::google::protobuf::Any;
use prost::Message;

use cf_guest::{ClientState, ConsensusState};

use ics08_wasm::{
	client_state::ClientState as WasmClientState,
	consensus_state::ConsensusState as WasmConsensusState, SUBJECT_PREFIX, SUBSTITUTE_PREFIX,
};

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
	pub fn addr(&self) -> Addr {
		self.0.clone()
	}

	pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
		let msg = to_binary(&msg.into())?;
		Ok(WasmMsg::Execute { contract_addr: self.addr().into(), msg, funds: vec![] }.into())
	}
}

pub fn check_substitute_and_update_state(
	ctx: &mut Context,
) -> Result<(ClientState<crate::crypto::PubKey>, ConsensusState), Ics02Error> {
	let mut subject_client_state = ctx.client_state_prefixed(SUBJECT_PREFIX).map_err(|_| {
		Ics02Error::implementation_specific("subject client state not found".to_string())
	})?;
	let substitute_client_state = ctx.client_state_prefixed(SUBSTITUTE_PREFIX).map_err(|_| {
		Ics02Error::implementation_specific("substitute client state not found".to_string())
	})?;

	if subject_client_state.0.genesis_hash != subject_client_state.0.genesis_hash {
		return Err(Ics02Error::implementation_specific("Clients do not match".to_string()))
	}

	let height = substitute_client_state.latest_height();
	let substitute_consensus_state =
		ctx.consensus_state_prefixed(height, SUBSTITUTE_PREFIX).map_err(|_| {
			Ics02Error::implementation_specific("substitute consensus state not found".to_string())
		})?;

	let mut process_states = ProcessedStates::new(ctx.storage_mut());
	let substitute_processed_time = process_states
		.get_processed_time(height, &mut SUBSTITUTE_PREFIX.to_vec())
		.unwrap();
	let substitute_processed_height = process_states
		.get_processed_height(height, &mut SUBSTITUTE_PREFIX.to_vec())
		.unwrap();
	let substitute_iteration_key = process_states
		.get_iteration_key(height, &mut SUBSTITUTE_PREFIX.to_vec())
		.unwrap();
	process_states.set_processed_time(
		height,
		substitute_processed_time,
		&mut SUBJECT_PREFIX.to_vec(),
	);
	process_states.set_processed_height(
		height,
		substitute_processed_height,
		&mut SUBJECT_PREFIX.to_vec(),
	);
	process_states.set_iteration_key(substitute_iteration_key, &mut SUBJECT_PREFIX.to_vec());

	subject_client_state.0.latest_height = substitute_client_state.0.latest_height;
	subject_client_state.0.trusting_period_ns = substitute_client_state.0.trusting_period_ns;
	subject_client_state.0.epoch_commitment = substitute_client_state.0.epoch_commitment;
	subject_client_state.0.prev_epoch_commitment = substitute_client_state.0.prev_epoch_commitment;
	subject_client_state.0.is_frozen = substitute_client_state.0.is_frozen;

	Ok((subject_client_state, substitute_consensus_state))
}

pub fn verify_upgrade_proof(
	is_client: bool,
	height: u64,
	commitment_root: &[u8],
	proof: CommitmentProofBytes,
	state: Any,
) -> Result<(), Ics02Error> {
	use ibc::core::ics24_host::ClientUpgradePath;
	let path = if is_client {
		ClientUpgradePath::UpgradedClientState(height)
	} else {
		ClientUpgradePath::UpgradedClientConsensusState(height)
	};
	cf_guest::proof::verify_bytes(
		&[],
		proof.as_bytes(),
		commitment_root,
		path.into(),
		Some(state.encode_to_vec().as_slice()),
	)
	.map_err(|err| {
		Ics02Error::implementation_specific(format!("upgrade state verification failed: {err}"))
	})
}

pub fn verify_upgrade_and_update_state(
	ctx: &mut Context,
	client_id: ClientId,
	old_client_state: ClientState<crate::crypto::PubKey>,
	upgrade_client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
	upgrade_consensus_state: WasmConsensusState<FakeInner>,
	proof_upgrade_client: CommitmentProofBytes,
	proof_upgrade_consensus_state: CommitmentProofBytes,
) -> Result<(ClientState<crate::crypto::PubKey>, ConsensusState), Ics02Error> {
	let latest_height = old_client_state.latest_height();
	if upgrade_client_state.latest_height.lt(&latest_height) {
		return Err(Ics02Error::implementation_specific(
			"upgrade cs is less than current height".to_string(),
		))
	}

	let consensus_state = ctx.consensus_state(&client_id, latest_height)?;
	let commitment_root = consensus_state.0.block_hash.as_bytes();
	let latest_height = latest_height.revision_height;

	verify_upgrade_proof(
		true,
		latest_height,
		commitment_root,
		proof_upgrade_client,
		upgrade_client_state.to_any(),
	)?;

	verify_upgrade_proof(
		false,
		latest_height,
		commitment_root,
		proof_upgrade_consensus_state,
		upgrade_consensus_state.to_any(),
	)?;

	let any = Any::decode(&mut upgrade_client_state.data.as_slice()).unwrap();
	let upgrade_client_state_inner =
		ClientState::<crate::crypto::PubKey>::decode_vec(&any.value).unwrap();
	let new_client_state = old_client_state.upgrade(
		upgrade_client_state_inner.latest_height(),
		cf_guest::client::UpgradeOptions {},
		upgrade_client_state_inner.chain_id(),
	);

	let any = Any::decode(&mut upgrade_consensus_state.data.as_slice()).unwrap();
	let upgrade_consensus_state_inner = ConsensusState::decode_vec(&any.value).unwrap();
	Ok((new_client_state, upgrade_consensus_state_inner))
}

pub fn prune_oldest_consensus_state(
	ctx: &mut Context,
	client_state: &ClientState<crate::crypto::PubKey>,
	current_time: u64,
) {
	let mut processed_states = ProcessedStates::new(ctx.storage_mut());
	let latest_height = client_state.latest_height();
	if let Some(earliest_height) = processed_states.get_earliest_height(latest_height) {
		let processed_time =
			processed_states.get_processed_time(earliest_height, &mut Vec::new()).unwrap();
		let elapsed = current_time.saturating_sub(processed_time);
		let expired = elapsed > client_state.0.trusting_period_ns;
		if expired {
			processed_states.remove_states_at_height(earliest_height);
			let mut consensus_states = ConsensusStates::new(ctx.storage_mut());
			consensus_states.remove(earliest_height);
		}
	}
}

pub fn verify_delay_passed(
	ctx: &Context,
	height: Height,
	delay_period_time: u64,
	delay_period_height: u64,
) -> Result<(), Ics02Error> {
	let current_timestamp = ctx.host_timestamp();
	let current_height = ctx.host_height();

	let processed_time = ctx.processed_timestamp(height)?;
	let processed_height = ctx.processed_height(height)?;

	ClientState::<crate::crypto::PubKey>::verify_delay_passed(
		current_timestamp,
		current_height,
		processed_time,
		processed_height,
		delay_period_time,
		delay_period_height,
	)
	.map_err(|e| e.into())
}
