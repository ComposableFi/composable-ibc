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

use crate::{context, context::log, ibc, msg, state};

type Result<T = (), E = crate::error::Error> = core::result::Result<T, E>;

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn instantiate(
	deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	msg: msg::InstantiateMessage,
) -> Result<Response> {
	let mut ctx = context::new(deps, env);
	log!(ctx, "instantiate: {msg:?}");

	ctx.client_states_mut().set(&msg.client_state);

	let height = ibc::Height::new(0, msg.client_state.latest_height.into()).unwrap();
	let metadata = ctx.metadata;
	ctx.consensus_states_mut().set(height, &msg.consensus_state, metadata);

	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: msg::SudoMsg) -> Result<Response> {
	let ctx = context::new(deps, env);
	log!(ctx, "sudo({msg:?})");
	process_sudo_msg(ctx, msg)?;
	Ok(Response::default())
}

fn process_sudo_msg(mut ctx: context::ContextMut, msg: msg::SudoMsg) -> Result {
	log!(ctx, "process_sudo_msg: {msg:?}");
	match msg {
		msg::SudoMsg::UpdateStateOnMisbehaviour(_msg) => {
			let client_state = ctx.client_state()?.frozen();
			ctx.client_states_mut().set(client_state);
		},
		msg::SudoMsg::UpdateState(msg) => process_update_state_msg(ctx, msg)?,
		// msg::SudoMsg::MigrateClientStore(_msg) => todo!(),
		// msg::SudoMsg::VerifyUpgradeAndUpdateState(msg) => todo!(),
	}
	Ok(())
}

fn process_update_state_msg(mut ctx: context::ContextMut, msg: msg::UpdateStateMsg) -> Result {
	let client_state = ctx.client_state()?;
	let now_ns = ctx.host_timestamp_ns;

	ctx.consensus_states_mut().prune_oldest_consensus_state(&client_state, now_ns)?;

	let new_consensus_state = state::ConsensusState::from(&msg.header);
	let new_client_state = client_state.with_header(&msg.header);

	let metadata = ctx.metadata;
	let height = ibc::Height::new(0, msg.header.block_header.block_height.into())?;
	ctx.client_states_mut().set(&new_client_state);
	ctx.consensus_states_mut().set(height, &new_consensus_state, metadata);
	Ok(())
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
	let ctx = context::new_ro(deps, env);
	match msg {
		msg::QueryMsg::VerifyClientMessage(msg) => {
			query_verify_client_msg(ctx, msg)?;
			Ok(Binary::default())
		},
		msg::QueryMsg::CheckForMisbehaviour(msg) => {
			let res = query_check_for_misbehaviour_msg(ctx, msg)?;
			Ok(if res { "true" } else { "false" }.as_bytes().into())
		},
		msg::QueryMsg::VerifyStateProof(msg) => query_verify_state_proof(ctx, msg),

		msg::QueryMsg::Status(msg::StatusMsg {}) => to_json_binary(&query_status(ctx)?),

		msg::QueryMsg::GetLatestHeights(msg::GetLatestHeightsMsg {}) => todo!(),

		msg::QueryMsg::TimestampAtHeight(msg) => {
			let state = ctx.consensus_state(msg.height)?;
			let ts = Uint64::from(state.timestamp_ns.get());
			to_json_binary(&ts)
		},

		msg::QueryMsg::ExportMetadata(msg::ExportMetadataMsg {}) => {
			let meta = ctx.consensus_states().get_all_metadata()?;
			to_json_binary(&meta)
		},
	}
}

fn query_verify_state_proof(
	ctx: context::Context,
	msg: msg::VerifyStateProofMsg,
) -> StdResult<Binary> {
	let client_state = ctx.client_state()?;
	let consensus_state = ctx.consensus_state(msg.height)?;
	cf_guest::proof::verify(
		&ibc::CommitmentPrefix::default(),
		&msg.proof,
		&consensus_state.block_hash,
		msg.path,
		msg.value.as_deref(),
	)
	.map_err(|err| StdError::GenericErr { msg: err.to_string() })?;
	Ok(Binary::default())
}

fn query_verify_client_msg(ctx: context::Context, msg: msg::VerifyClientMessageMsg) -> Result {
	let client_state = ctx.client_state()?;
	let verifier = crate::pubkey::NullVerifier;
	match msg.client_message {
		state::ClientMessage::Header(header) =>
			client_state.verify_header(&verifier, &ctx.client_id, header),
		state::ClientMessage::Misbehaviour(misbehaviour) =>
			client_state.verify_misbehaviour(&verifier, &ctx.client_id, misbehaviour),
	}
	.map_err(crate::Error::from)
}

fn query_check_for_misbehaviour_msg(
	ctx: context::Context,
	msg: msg::CheckForMisbehaviourMsg,
) -> Result<bool> {
	let client_state = ctx.client_state()?;
	let verifier = crate::pubkey::NullVerifier;
	match msg.client_message {
		state::ClientMessage::Header(header) =>
			client_state.check_for_misbehaviour_header(&verifier, &ctx.client_id, header),
		state::ClientMessage::Misbehaviour(misbehaviour) =>
			client_state.check_for_misbehaviour_misbehavior(&verifier, &ctx.client_id, misbehaviour),
	}
	.map_err(crate::Error::from)
}

fn query_status(ctx: context::Context) -> StdResult<msg::StatusResponse> {
	let client_state = ctx.client_state()?;
	if client_state.is_frozen {
		return Ok(msg::StatusResponse::Frozen);
	}

	let height = client_state.latest_height;
	let height = ibc::Height::new(0, height.into()).unwrap();
	let consensus_state = ctx.consensus_state(height)?;

	let age = ctx.host_timestamp_ns.saturating_sub(consensus_state.timestamp_ns.get());
	Ok(if age >= client_state.trusting_period_ns {
		msg::StatusResponse::Expired
	} else {
		msg::StatusResponse::Active
	})
}