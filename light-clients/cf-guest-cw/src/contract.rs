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

use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{context, context::log, ibc, msg, state};

type Result<T, E = crate::error::Error> = core::result::Result<T, E>;

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
	Ok(Response::default().set_data(process_sudo_msg(ctx, msg)?))
}

fn process_sudo_msg(mut ctx: context::ContextMut, msg: msg::SudoMsg) -> Result<Binary> {
	log!(ctx, "process_sudo_msg: {msg:?}");
	match msg {
		msg::SudoMsg::UpdateStateOnMisbehaviour(_msg) => {
			let state = ctx.client_state()?.frozen();
			ctx.client_states_mut().set(state);
		},
		msg::SudoMsg::UpdateState(msg) => process_update_state_msg(ctx, msg)?,
		// 	SudoMsg::MigrateClientStore(_msg) =>
		// 		check_substitute_and_update_state::<HostFunctions>(ctx)
		// 			.map_err(|e| ContractError::Tendermint(e.to_string()))
		// 			.and_then(|(cs, cu)| {
		// 				let height = cs.latest_height();
		// 				ctx.store_consensus_state_prefixed(height, cu, SUBJECT_PREFIX);
		// 				ctx.store_client_state_prefixed(cs, SUBJECT_PREFIX, client_id)
		// 					.map_err(|e| ContractError::Tendermint(e.to_string()))?;
		// 				Ok(to_binary(&ContractResult::success()))
		// 			}),
		// 	SudoMsg::VerifyMembership(msg) => {
		// 		let client_state = ctx
		// 			.client_state(&client_id)
		// 			.map_err(|e| ContractError::Tendermint(e.to_string()))?;
		// 		let msg = VerifyMembershipMsg::try_from(msg)?;
		// 		verify_delay_passed(&ctx, msg.height, msg.delay_time_period, msg.delay_block_period)
		// 			.map_err(|e| ContractError::Tendermint(e.to_string()))?;
		// 		let consensus_state = ctx
		// 			.consensus_state(&client_id, msg.height)
		// 			.map_err(|e| ContractError::Tendermint(e.to_string()))?;
		// 		verify_membership::<HostFunctions, _>(
		// 			&client_state,
		// 			&msg.prefix,
		// 			&msg.proof,
		// 			&consensus_state.root,
		// 			msg.path,
		// 			msg.value,
		// 		)
		// 		.map_err(|e| ContractError::Tendermint(e.to_string()))
		// 		.map(|_| to_binary(&ContractResult::success()))
		// 	},
		// 	SudoMsg::VerifyNonMembership(msg) => {
		// 		let client_state = ctx
		// 			.client_state(&client_id)
		// 			.map_err(|e| ContractError::Tendermint(e.to_string()))?;
		// 		let msg = VerifyNonMembershipMsg::try_from(msg)?;
		// 		verify_delay_passed(&ctx, msg.height, msg.delay_time_period, msg.delay_block_period)
		// 			.map_err(|e| ContractError::Tendermint(e.to_string()))?;
		// 		let consensus_state = ctx
		// 			.consensus_state(&client_id, msg.height)
		// 			.map_err(|e| ContractError::Tendermint(e.to_string()))?;

		// 		verify_non_membership::<HostFunctions, _>(
		// 			&client_state,
		// 			&msg.prefix,
		// 			&msg.proof,
		// 			&consensus_state.root,
		// 			msg.path,
		// 		)
		// 		.map_err(|e| ContractError::Tendermint(e.to_string()))
		// 		.map(|_| to_binary(&ContractResult::success()))
		// 	},
		// 	SudoMsg::VerifyUpgradeAndUpdateState(msg) => {
		// 		let old_client_state = ctx
		// 			.client_state(&client_id)
		// 			.map_err(|e| ContractError::Tendermint(e.to_string()))?;
		// 		let msg: VerifyUpgradeAndUpdateStateMsg<HostFunctions> =
		// 			VerifyUpgradeAndUpdateStateMsg::try_from(msg)?;
		// 		verify_upgrade_and_update_state::<HostFunctions>(
		// 			ctx,
		// 			client_id.clone(),
		// 			old_client_state,
		// 			msg.upgrade_client_state,
		// 			msg.upgrade_consensus_state,
		// 			msg.proof_upgrade_client,
		// 			msg.proof_upgrade_consensus_state,
		// 		)
		// 		.map_err(|e| ContractError::Tendermint(e.to_string()))
		// 		.and_then(|(cs, cu)| {
		// 			let height = cs.latest_height();
		// 			ctx.store_consensus_state(client_id.clone(), height, cu)
		// 				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
		// 			ctx.store_client_state(client_id, cs)
		// 				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
		// 			Ok(to_binary(&ContractResult::success()))
		// 		})
		// 	},
		_ => todo!(),
	}
	Ok(Vec::new().into())
}

fn process_update_state_msg(mut ctx: context::ContextMut, msg: msg::UpdateStateMsg) -> Result<()> {
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
	let ret = match msg {
		// msg::QueryMsg::CheckForMisbehaviour(msg) => {
		// 	let client_state = ctx.client_state(&client_id)?;
		// 	client
		// 		.check_for_misbehaviour(&ctx, client_id, client_state, msg.client_message)
		// 		.map_err(|e| ContractError::Tendermint(e.to_string()))
		// 		.map(|result| to_binary(&QueryResponse::success().misbehaviour(result)))?
		// },
		// msg::QueryMsg::ExportMetadata(ExportMetadataMsg {}) => {
		// 	let ro_proceeded_state = ReadonlyProcessedStates::new(deps.storage);
		// 	to_binary(&QueryResponse::success().genesis_metadata(ro_proceeded_state.get_metadata()))
		// },
		// msg::QueryMsg::Status(StatusMsg {}) => {
		// 	let client_state = match get_client_state::<HostFunctions>(deps, &client_id) {
		// 		Ok(client_state) => client_state,
		// 		Err(_) => return to_binary(&QueryResponse::success().status("Unknown".to_string())),
		// 	};

		// 	if client_state.frozen_height().is_some() {
		// 		to_binary(&QueryResponse::success().status("Frozen".to_string()))
		// 	} else {
		// 		let height = client_state.latest_height();
		// 		match get_consensus_state(deps, &client_id, height) {
		// 			Ok(consensus_state) => {
		// 				let last_update = consensus_state.timestamp.unix_timestamp().unsigned_abs();
		// 				let tp = client_state.trusting_period.as_secs();
		// 				let now = env.block.time.seconds();
		// 				if (last_update + tp) < now {
		// 					return to_binary(
		// 						&QueryResponse::success().status("Expired".to_string()),
		// 					)
		// 				}
		// 				to_binary(&QueryResponse::success().status("Active".to_string()))
		// 			},
		// 			Err(_) => to_binary(&QueryResponse::success().status("Expired".to_string())),
		// 		}
		// 	}
		// },
		msg::QueryMsg::TimestampAtHeight(msg) => {
			let state = ctx.consensus_state(msg.height)?;
			state.timestamp_ns.get().to_be_bytes().to_vec()
		},
		// msg::QueryMsg::VerifyClientMessage(msg) => {
		// 	let ctx = Context::<HostFunctions>::new_ro(deps, env);
		// 	let client = TendermintClient::<HostFunctions>::default();
		// 	let client_state = ctx
		// 		.client_state(&client_id)
		// 		.map_err(|e| ContractError::Tendermint(e.to_string()))?;
		// 	let msg = VerifyClientMessage::try_from(msg)?;
		// 	client
		// 		.verify_client_message(&ctx, client_id, client_state, msg.client_message)
		// 		.map_err(|e| ContractError::Tendermint(format!("{e:?}")))
		// 		.map(|_| to_binary(&QueryResponse::success()))?
		// },
		_ => todo!(),
	};
	Ok(ret.into())
}
