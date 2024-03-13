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
	to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

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
			verify_state_proof(ctx, msg.try_into()?)?;
			msg::ContractResult::success()
		},
		msg::ExecuteMsg::VerifyNonMembership(msg) => {
			verify_state_proof(ctx, msg.try_into()?)?;
			msg::ContractResult::success()
		},
		msg::ExecuteMsg::VerifyClientMessage(msg) => {
			verify_client_message(ctx, msg.try_into()?)?;
			msg::ContractResult::success()
		},
		msg::ExecuteMsg::CheckForMisbehaviour(msg) => {
			let found = check_for_misbehaviour(ctx, msg.try_into()?)?;
			msg::ContractResult::success().misbehaviour(found)
		},
		msg::ExecuteMsg::UpdateStateOnMisbehaviour(_msg) => {
			let client_state = ctx.client_state()?.frozen();
			ctx.client_states_mut().set(client_state);
			msg::ContractResult::success()
		},
		msg::ExecuteMsg::UpdateState(msg) => {
			process_update_state_msg(ctx, msg.try_into()?)?;
			msg::ContractResult::success()
		}

		msg::ExecuteMsg::CheckSubstituteAndUpdateState(_) => unimplemented!(),
		msg::ExecuteMsg::VerifyUpgradeAndUpdateState(_) => unimplemented!(),
	};
	Ok(Response::default().set_data(to_json_binary(&result)?))
}

fn verify_state_proof(ctx: context::ContextMut, msg: msg::VerifyStateProof) -> Result<()> {
	let consensus_state = ctx.consensus_state(msg.height)?;
	cf_guest::proof::verify(
		&ibc::CommitmentPrefix::default(),
		&msg.proof,
		&consensus_state.block_hash,
		msg.path,
		msg.value.as_deref(),
	).map_err(|err| StdError::GenericErr { msg: err.to_string() }.into())
}

fn verify_client_message(ctx: context::ContextMut, msg: msg::VerifyClientMessageMsg) -> Result {
	ctx.client_state()?
		.verify_client_message(&Verifier, &ctx.client_id, msg.client_message)
		.map_err(crate::Error::from)
}

fn check_for_misbehaviour(
	ctx: context::ContextMut,
	msg: msg::CheckForMisbehaviourMsg,
) -> Result<bool> {
	ctx.client_state()?
		.check_for_misbehaviour(&Verifier, &ctx.client_id, msg.client_message.into())
		.map_err(crate::Error::from)
}

fn process_update_state_msg(mut ctx: context::ContextMut, msg: msg::UpdateStateMsg) -> Result {
	let client_state = ctx.client_state()?;

	let header = crate::state::Header::try_from(msg.client_message).unwrap();
	let header_height =
		ibc::Height::new(0, header.block_header.block_height.into());

	if ctx.consensus_states().get::<state::ConsensusState, crate::Error>(header_height)?.is_some() {
		return Ok(());
	}

	let metadata = ctx.metadata;
	ctx.consensus_states_mut()
		.prune_oldest_consensus_state(&client_state, metadata.host_timestamp_ns)?;

	ctx.client_states_mut().set(client_state.with_header(&header));
	ctx.consensus_states_mut().set(
		header_height,
		state::ConsensusState::from(&header),
		metadata);

	Ok(())
}


#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
	let ctx = context::new_ro(deps, env);
	let response = match msg {
		msg::QueryMsg::ClientTypeMsg(_) => unimplemented!(),
		msg::QueryMsg::GetLatestHeightsMsg(_) => unimplemented!(),
		msg::QueryMsg::ExportMetadata(_) => msg::QueryResponse::new(""),
		msg::QueryMsg::Status(msg::StatusMsg {}) => query_status(ctx)?,
	};
	to_json_binary(&response)
}

fn query_status(ctx: context::Context) -> StdResult<msg::QueryResponse> {
	let client_state = ctx.client_state()?;
	if client_state.is_frozen {
		return Ok(msg::QueryResponse::new("Frozen"))
	}

	let height = client_state.latest_height;
	let height = ibc::Height::new(0, height.into());
	let consensus_state = ctx.consensus_state(height)?;

	let age = ctx.host_timestamp_ns.saturating_sub(consensus_state.timestamp_ns.get());
	Ok(if age >= client_state.trusting_period_ns {
		msg::QueryResponse::new("Expired")
	} else {
		msg::QueryResponse::new("Active")
	})
}
