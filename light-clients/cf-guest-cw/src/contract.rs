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

use crate::{
	context::Context,
	error::ContractError,
	helpers,
	ics23::ReadonlyProcessedStates,
	msg::{
		CheckForMisbehaviourMsg, ContractResult, ExecuteMsg, ExportMetadataMsg, InstantiateMsg,
		QueryMsg, QueryResponse, StatusMsg, UpdateStateMsg, UpdateStateOnMisbehaviourMsg,
		VerifyClientMessage, VerifyMembershipMsg, VerifyNonMembershipMsg,
		VerifyUpgradeAndUpdateStateMsg,
	},
	state::{get_client_state, get_consensus_state},
};
use cf_guest::{client_def::GuestClient, proof::verify};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use ibc::core::{
	ics02_client::{
		client_consensus::ConsensusState as _,
		client_def::{ClientDef, ConsensusUpdateResult},
		client_state::ClientState as _,
		context::{ClientKeeper, ClientReader},
	},
	ics23_commitment::commitment::CommitmentPrefix,
	ics24_host::identifier::ClientId,
};
use ics08_wasm::SUBJECT_PREFIX;
use std::str::FromStr;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	_msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	let _client = GuestClient::<crate::crypto::PubKey>::default();
	let mut ctx = Context::new(deps, env);
	let client_id = ClientId::from_str("08-wasm-0").expect("client id is valid");
	let client_state = ctx
		.client_state(&client_id)
		.map_err(|e| ContractError::Tendermint(e.to_string()))?;
	let latest_height = client_state.latest_height();
	ctx.store_update_height(client_id.clone(), latest_height, ctx.host_height())
		.map_err(|e| ContractError::Tendermint(e.to_string()))?;
	ctx.store_update_time(client_id, latest_height, ctx.host_timestamp())
		.map_err(|e| ContractError::Tendermint(e.to_string()))?;

	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	let client = GuestClient::<crate::crypto::PubKey>::default();
	let mut ctx = Context::new(deps, env);
	let client_id = ClientId::from_str("08-wasm-0").expect("client id is valid");
	let data = process_message(msg, client, &mut ctx, client_id)?;
	let mut response = Response::default();
	response.data = Some(data);
	Ok(response)
}

fn process_message(
	msg: ExecuteMsg,
	client: GuestClient<crate::crypto::PubKey>,
	ctx: &mut Context,
	client_id: ClientId,
) -> Result<Binary, ContractError> {
	//log!(ctx, "process_message: {:?}", msg);
	let result = match msg {
		ExecuteMsg::VerifyMembership(msg) => {
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let msg = VerifyMembershipMsg::try_from(msg)?;
			crate::helpers::verify_delay_passed(
				ctx,
				msg.height,
				msg.delay_time_period,
				msg.delay_block_period,
			)
			.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let consensus_state = ctx
				.consensus_state(&client_id, msg.height)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			// TODO(blas)
			verify(
				&CommitmentPrefix::default(),
				&msg.proof,
				&consensus_state.root(),
				msg.path,
				Some(msg.value.as_ref()),
			)
			.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			Ok(()).map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::VerifyNonMembership(msg) => {
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let msg = VerifyNonMembershipMsg::try_from(msg)?;
			crate::helpers::verify_delay_passed(
				ctx,
				msg.height,
				msg.delay_time_period,
				msg.delay_block_period,
			)
			.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let consensus_state = ctx
				.consensus_state(&client_id, msg.height)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;

			verify(
				&CommitmentPrefix::default(),
				&msg.proof,
				&consensus_state.root(),
				msg.path,
				None,
			)
			.map_err(|e| ContractError::Tendermint(e.to_string()))
			.map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::VerifyClientMessage(msg) => {
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let msg = VerifyClientMessage::try_from(msg)?;
			client
				.verify_client_message(ctx, client_id, client_state, msg.client_message)
				.map_err(|e| ContractError::Tendermint(format!("{e:?}")))
				.map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::CheckForMisbehaviour(msg) => {
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let msg = CheckForMisbehaviourMsg::try_from(msg)?;
			client
				.check_for_misbehaviour(ctx, client_id, client_state, msg.client_message)
				.map_err(|e| ContractError::Tendermint(e.to_string()))
				.map(|result| to_binary(&ContractResult::success().misbehaviour(result)))
		},
		ExecuteMsg::UpdateStateOnMisbehaviour(msg_raw) => {
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let msg = UpdateStateOnMisbehaviourMsg::try_from(msg_raw)?;
			client
				.update_state_on_misbehaviour(client_state, msg.client_message)
				.map_err(|e| ContractError::Tendermint(e.to_string()))
				.and_then(|cs| {
					ctx.store_client_state(client_id, cs)
						.map_err(|e| ContractError::Tendermint(e.to_string()))?;
					Ok(to_binary(&ContractResult::success()))
				})
		},
		ExecuteMsg::UpdateState(msg_raw) => {
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let msg = UpdateStateMsg::try_from(msg_raw)?;
			let latest_revision_height = client_state.latest_height().revision_height;
			helpers::prune_oldest_consensus_state(
				ctx,
				&client_state,
				ctx.host_timestamp().nanoseconds(),
			);
			client
				.update_state(ctx, client_id.clone(), client_state, msg.client_message)
				.map_err(|e| ContractError::Tendermint(e.to_string()))
				.and_then(|(cs, cu)| {
					let height = cs.latest_height();
					match cu {
						ConsensusUpdateResult::Single(cs) => {
							ctx.store_consensus_state(client_id.clone(), height, cs)
								.map_err(|e| ContractError::Tendermint(e.to_string()))?;
						},
						ConsensusUpdateResult::Batch(css) =>
							for (height, cs) in css {
								ctx.store_consensus_state(client_id.clone(), height, cs)
									.map_err(|e| ContractError::Tendermint(e.to_string()))?;
							},
					}
					if u64::from(cs.0.latest_height) > latest_revision_height {
						ctx.store_client_state(client_id, cs)
							.map_err(|e| ContractError::Tendermint(e.to_string()))?;
					}
					Ok(to_binary(&ContractResult::success()))
				})
		},
		ExecuteMsg::CheckSubstituteAndUpdateState(
			crate::msg::CheckSubstituteAndUpdateStateMsg {},
		) => helpers::check_substitute_and_update_state(ctx)
			.map_err(|e| ContractError::Tendermint(e.to_string()))
			.and_then(|(cs, cu)| {
				let height = cs.latest_height();
				ctx.store_consensus_state_prefixed(height, cu, SUBJECT_PREFIX);
				ctx.store_client_state_prefixed(cs, SUBJECT_PREFIX)
					.map_err(|e| ContractError::Tendermint(e.to_string()))?;
				Ok(to_binary(&ContractResult::success()))
			}),
		ExecuteMsg::VerifyUpgradeAndUpdateState(msg) => {
			let old_client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let msg: VerifyUpgradeAndUpdateStateMsg =
				VerifyUpgradeAndUpdateStateMsg::try_from(msg)?;
			helpers::verify_upgrade_and_update_state(
				ctx,
				client_id.clone(),
				old_client_state,
				msg.upgrade_client_state,
				msg.upgrade_consensus_state,
				msg.proof_upgrade_client,
				msg.proof_upgrade_consensus_state,
			)
			.map_err(|e| ContractError::Tendermint(e.to_string()))
			.and_then(|(cs, cu)| {
				let height = cs.latest_height();
				ctx.store_consensus_state(client_id.clone(), height, cu)
					.map_err(|e| ContractError::Tendermint(e.to_string()))?;
				ctx.store_client_state(client_id, cs)
					.map_err(|e| ContractError::Tendermint(e.to_string()))?;
				Ok(to_binary(&ContractResult::success()))
			})
		},
		_ => unimplemented!("none of the other messages are implemented at the moment"),
	};
	Ok(result??)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
	let client_id = ClientId::from_str("08-wasm-0").expect("client id is valid");
	//deps.api.debug("In query");
	match msg {
		QueryMsg::ClientTypeMsg(_) => unimplemented!("ClientTypeMsg"),
		QueryMsg::GetLatestHeightsMsg(_) => unimplemented!("GetLatestHeightsMsg"),
		QueryMsg::ExportMetadata(ExportMetadataMsg {}) => {
			let ro_proceeded_state = ReadonlyProcessedStates::new(deps.storage);
			to_binary(&QueryResponse::genesis_metadata(ro_proceeded_state.get_metadata()))
		},
		QueryMsg::Status(StatusMsg {}) => {
			let client_state = match get_client_state::<crate::crypto::PubKey>(deps) {
				Ok(state) => state,
				Err(_) => return to_binary(&QueryResponse::status("Unknown".to_string())),
			};

			if client_state.frozen_height().is_some() {
				return to_binary(&QueryResponse::status("Frozen".to_string()));
			}

			let height = client_state.latest_height();
			let consensus_state = match get_consensus_state(deps, &client_id, height) {
				Ok(state) => state,
				Err(_) => return to_binary(&QueryResponse::status("Expired".to_string())),
			};

			let last_update = consensus_state.0.timestamp_ns.get();
			let trusting_period = client_state.0.trusting_period_ns;
			let now = env.block.time.nanos();
			if last_update + trusting_period < now {
				return to_binary(&QueryResponse::status("Expired".to_string()))
			}
			to_binary(&QueryResponse::status("Active".to_string()))
		},
	}
}
