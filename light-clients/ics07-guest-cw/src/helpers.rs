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

use std::time::Duration;

use guestchain::PubKey;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
	context::Context,
	ics23::{ConsensusStates, FakeInner, ProcessedStates},
	msg::ExecuteMsg,
};
use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};
use ibc::core::{
	ics02_client::{
		client_consensus::ConsensusState as _, context::ClientReader, error::Error as Ics02Error,
		height::Height,
	},
	ics23_commitment::{commitment::CommitmentProofBytes, merkle::MerkleProof},
	ics24_host::identifier::ClientId,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::commitment::v1::{MerklePath, MerkleProof as RawMerkleProof},
};
use prost::Message;

use cf_guest::{error::Error, ClientState, ConsensusState};

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

// pub fn check_substitute_and_update_state<PK: PubKey + 'static>(
// 	ctx: &mut Context<PK>,
// ) -> Result<(ClientState<PK>, ConsensusState), Ics02Error> {
// 	let mut subject_client_state = ctx.client_state_prefixed(SUBJECT_PREFIX).map_err(|_| {
// 		Ics02Error::implementation_specific("subject client state not found".to_string())
// 	})?;
// 	let substitute_client_state = ctx.client_state_prefixed(SUBSTITUTE_PREFIX).map_err(|_| {
// 		Ics02Error::implementation_specific("substitute client state not found".to_string())
// 	})?;

// 	if subject_client_state.trust_level != substitute_client_state.trust_level ||
// 		subject_client_state.unbonding_period != substitute_client_state.unbonding_period ||
// 		subject_client_state.max_clock_drift != substitute_client_state.max_clock_drift ||
// 		subject_client_state.proof_specs != substitute_client_state.proof_specs ||
// 		subject_client_state.upgrade_path != substitute_client_state.upgrade_path
// 	{
// 		return Err(Ics02Error::implementation_specific("Clients do not match".to_string()))
// 	}

// 	let height = substitute_client_state.latest_height();
// 	let substitute_consensus_state =
// 		ctx.consensus_state_prefixed(height, SUBSTITUTE_PREFIX).map_err(|_| {
// 			Ics02Error::implementation_specific("substitute consensus state not found".to_string())
// 		})?;

// 	let mut process_states = ProcessedStates::new(ctx.storage_mut());
// 	let substitute_processed_time = process_states
// 		.get_processed_time(height, &mut SUBSTITUTE_PREFIX.to_vec())
// 		.unwrap();
// 	let substitute_processed_height = process_states
// 		.get_processed_height(height, &mut SUBSTITUTE_PREFIX.to_vec())
// 		.unwrap();
// 	let substitute_iteration_key = process_states
// 		.get_iteration_key(height, &mut SUBSTITUTE_PREFIX.to_vec())
// 		.unwrap();
// 	process_states.set_processed_time(
// 		height,
// 		substitute_processed_time,
// 		&mut SUBJECT_PREFIX.to_vec(),
// 	);
// 	process_states.set_processed_height(
// 		height,
// 		substitute_processed_height,
// 		&mut SUBJECT_PREFIX.to_vec(),
// 	);
// 	process_states.set_iteration_key(substitute_iteration_key, &mut SUBJECT_PREFIX.to_vec());

// 	subject_client_state.latest_height = substitute_client_state.latest_height;
// 	subject_client_state.chain_id = substitute_client_state.chain_id;
// 	subject_client_state.trusting_period = substitute_client_state.trusting_period;
// 	subject_client_state.frozen_height = substitute_client_state.frozen_height;

// 	Ok((subject_client_state, substitute_consensus_state))
// }

pub fn prune_oldest_consensus_state(
	ctx: &mut Context,
	client_state: &ClientState<crate::crypto::PubKey>,
	current_time: u64,
) {
	let mut processed_states = ProcessedStates::new(ctx.storage_mut());
	if let Some(earliest_height) = processed_states.get_earliest_height(client_state.latest_height)
	{
		let processed_time =
			processed_states.get_processed_time(earliest_height, &mut Vec::new()).unwrap();
		let elapsed = Duration::from_nanos(current_time - processed_time);
		if client_state.expired(elapsed) {
			processed_states.remove_states_at_height(earliest_height);
			let mut consensus_states = ConsensusStates::new(ctx.storage_mut());
			consensus_states.remove(earliest_height);
		}
	}
}
