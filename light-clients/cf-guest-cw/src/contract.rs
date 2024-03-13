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

use cosmwasm_std::{
	to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint64,
};
use prost::Message;

use crate::{context, context::log, crypto::Verifier, ibc, msg, state};

type Result<T = (), E = crate::error::Error> = core::result::Result<T, E>;


#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
fn instantiate(
	_deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	_msg: msg::InstantiateMsg,
) -> Result<Response> {
	Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
fn execute(
	deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	msg: msg::ExecuteMsg,
) -> Result<Response> {
	let mut ctx = context::new(deps, env);
	log!(ctx, "execute({msg:?})");
	let result = match msg {
		msg::ExecuteMsg::VerifyMembership(msg) => {
			verify_state_proof(ctx, msg.into())?;
			ContractResult::success()
		},
		msg::ExecuteMsg::VerifyNonMembership(msg) => {
			verify_state_proof(ctx, msg.into())?;
			ContractResult::success()
		},
		msg::ExecuteMsg::VerifyClientMessage(msg) => {
			verify_client_msg(ctx, msg)?;
			ContractResult::success()
		},
		msg::ExecuteMsg::CheckForMisbehaviour(msg) => {
			let found = check_for_misbehaviour(ctx, msg)?;
			ContractResult::success().found_misbehaviour(found)
		},
		msg::ExecuteMsg::UpdateStateOnMisbehaviour(_msg) => {
			let client_state = ctx.client_state()?.frozen();
			ctx.client_states_mut().set(client_state);
			ContractResult::success()
		},
		msg::SudoMsg::UpdateState(msg) => {
			process_update_state_msg(ctx, msg)?;
			ContractResult::success()
		}
	};
	to_json_binary(&result)
}

struct VerifyStateProof {
	pub proof: ibc::CommitmentProofBytes,
	pub path: ibc::path::Path,
	pub value: Option<Vec<u8>>,
	pub height: ibc::Height,
}

impl From<msg::VerifyMembershipMsg> for VerifyStateProof {
	fn from(msg: msg::VerifyMembershipMsg) -> Self {
		Self {
			proof: msg.proof,
			path: msg.path,
			value: Some(msg.value),
			height: msg.height,
		}
	}
}

impl From<msg::VerifyNonMembershipMsg> for VerifyStateProof {
	fn from(msg: msg::VerifyNonMembershipMsg) -> Self {
		Self {
			proof: msg.proof,
			path: msg.path,
			value: None,
			height: msg.height,
		}
	}
}

fn verify_state_proof(ctx: context::Context, msg: VerifyStateProof) -> Result<()> {
	let consensus_state = ctx.consensus_state(height)?;
	let result = cf_guest::proof::verify(
		&ibc::CommitmentPrefix::default(),
		&msg.proof,
		&consensus_state.block_hash,
		msg.path,
		msg.value.as_deref(),
	);
	match result {
		Ok(()) => Ok(msg::ContractResult::success()),
		Err(err) => Err(StdError::GenericErr { msg: err.to_string() })
	}
}

fn verify_client_msg(ctx: context::Context, msg: msg::VerifyClientMessageMsg) -> Result {
	let client_message =
		ibc::proto::google::protobuf::Any::decode(msg.client_message.as_slice()).map_err(crate::Error::from)?;
	ctx.client_state()?
		.verify_client_message(&Verifier, &ctx.client_id, client_message.try_into().unwrap())
		.map_err(crate::Error::from)
}

fn check_for_misbehaviour_msg(
	ctx: context::Context,
	msg: msg::CheckForMisbehaviourMsg,
) -> Result<bool> {
	let client_message =
	ibc::proto::google::protobuf::Any::decode(msg.client_message.as_slice()).map_err(crate::Error::from)?;
	ctx.client_state()?
		.check_for_misbehaviour(&Verifier, &ctx.client_id, client_message)
		.map_err(crate::Error::from)
}

fn process_update_state_msg(mut ctx: context::ContextMut, msg: msg::UpdateStateMsg) -> Result {
	let client_state = ctx.client_state()?;
	let now_ns = ctx.host_timestamp_ns;

	ctx.consensus_states_mut().prune_oldest_consensus_state(&client_state, now_ns)?;

	let new_consensus_state = state::ConsensusState::from(&msg.header);
	let new_client_state = client_state.with_header(&msg.header);

	let metadata = ctx.metadata;
	let height = ibc::Height::new(0, msg.header.block_header.block_height.into());
	ctx.client_states_mut().set(&new_client_state);
	ctx.consensus_states_mut().set(height, &new_consensus_state, metadata);
	Ok(())
}


#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
	let ctx = context::new_ro(deps, env);
	let response = match msg {
		msg::QueryMsg::ExportMetadata(_) => msg::QueryResponse::new(),
		msg::QueryMsg::Status(msg::StatusMsg {}) => query_status(ctx)?,
	};
	to_json_binary(&response)
}

fn query_status(ctx: context::Context) -> StdResult<msg::QueryResponse> {
	let client_state = ctx.client_state()?;
	if client_state.is_frozen {
		return Ok(msg::QueryResponse::frozen())
	}

	let height = client_state.latest_height;
	let height = ibc::Height::new(0, height.into());
	let consensus_state = ctx.consensus_state(height)?;

	let age = ctx.host_timestamp_ns.saturating_sub(consensus_state.timestamp_ns.get());
	Ok(if age >= client_state.trusting_period_ns {
		msg::QueryResponse::expired()
	} else {
		msg::QueryResponse::active()
	})
}
