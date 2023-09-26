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
	log,
	msg::{
		CheckForMisbehaviourMsg, CheckSubstituteAndUpdateStateMsg, ContractResult, ExecuteMsg,
		ExportMetadataMsg, InstantiateMsg, QueryMsg, QueryResponse, StatusMsg, UpdateStateMsg,
		UpdateStateOnMisbehaviourMsg, VerifyClientMessage, VerifyMembershipMsg,
		VerifyNonMembershipMsg, VerifyUpgradeAndUpdateStateMsg,
	},
	state::{get_client_state, get_consensus_state},
	Bytes,
};
use byteorder::{ByteOrder, LittleEndian};
use core::{fmt::Debug, hash::Hasher};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_storage_plus::{Item, Map};
use digest::Digest;
use ibc::core::{
	ics02_client::{
		client_def::{ClientDef, ConsensusUpdateResult},
		context::{ClientKeeper, ClientReader},
		height::Height,
	},
	ics24_host::identifier::ClientId,
};
use ics08_wasm::{SUBJECT_PREFIX, SUBSTITUTE_PREFIX};
use icsxx_ethereum::{
	client_def::EthereumClient, client_message::ClientMessage, client_state::ClientState,
	consensus_state::ConsensusState,
};
// use light_client_common::{verify_membership, verify_non_membership};
use std::{collections::BTreeSet, str::FromStr};
use sync_committee_verifier::LightClientState;
/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ics10-grandpa-cw";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

pub const CHANNELS_CONNECTION: Map<Bytes, Vec<(Bytes, Bytes)>> = Map::new("channels_connection");
pub const CLIENT_UPDATE_TIME: Map<(Bytes, Bytes), u64> = Map::new("client_update_time");
pub const CLIENT_UPDATE_HEIGHT: Map<(Bytes, Bytes), Bytes> = Map::new("client_update_height");
pub const CHANNEL_COUNTER: Item<u32> = Item::new("channel_counter");
pub const EXPECTED_BLOCK_TIME: Item<u64> = Item::new("expected_block_time");
pub const CONNECTION_PREFIX: Item<Vec<u8>> = Item::new("connection_prefix");
pub const CONNECTION_COUNTER: Item<u32> = Item::new("connection_counter");
pub const CLIENT_COUNTER: Item<u32> = Item::new("client_counter");
pub const CODE_ID: Item<Vec<u8>> = Item::new("code_id");
pub const HOST_CONSENSUS_STATE: Map<u64, ConsensusState> = Map::new("host_consensus_state");
pub const CONSENSUS_STATES_HEIGHTS: Map<Bytes, BTreeSet<Height>> =
	Map::new("consensus_states_heights");

#[derive(Clone, Copy, Debug, PartialEq, Default, Eq)]
pub struct HostFunctions;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	_deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	_msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	let client = EthereumClient::<HostFunctions>::default();
	let mut ctx = Context::<HostFunctions>::new(deps, env);
	let client_id = ClientId::from_str("08-wasm-0").expect("client id is valid");
	let data = process_message(msg, client, &mut ctx, client_id)?;
	let mut response = Response::default();
	response.data = Some(data);
	Ok(response)
}

fn process_message(
	msg: ExecuteMsg,
	client: EthereumClient<HostFunctions>,
	ctx: &mut Context<HostFunctions>,
	client_id: ClientId,
) -> Result<Binary, ContractError> {
	// log!(ctx, "process_message: {:?}", msg);
	let result = match msg {
		ExecuteMsg::VerifyMembership(msg) => {
			let msg = VerifyMembershipMsg::try_from(msg)?;
			let consensus_state = ctx
				.consensus_state(&client_id, msg.height)
				.map_err(|e| ContractError::Client(e.to_string()))?;
			// verify_membership::<BlakeTwo256, _>(
			// 	&msg.prefix,
			// 	&msg.proof,
			// 	&consensus_state.root,
			// 	msg.path,
			// 	msg.value,
			// )
			// .map_err(|e| ContractError::Ethereum(e.to_string()))?;
			Ok(()).map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::VerifyNonMembership(msg) => {
			let msg = VerifyNonMembershipMsg::try_from(msg)?;
			let consensus_state = ctx
				.consensus_state(&client_id, msg.height)
				.map_err(|e| ContractError::Client(e.to_string()))?;

			// verify_non_membership::<BlakeTwo256, _>(
			// 	&msg.prefix,
			// 	&msg.proof,
			// 	&consensus_state.root,
			// 	msg.path,
			// )
			// .map_err(|e| ContractError::Ethereum(e.to_string()))
			// .map(|_| to_binary(&ContractResult::success()))
			Ok(()).map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::VerifyClientMessage(msg) => {
			let client_state =
				ctx.client_state(&client_id).map_err(|e| ContractError::Client(e.to_string()))?;
			let msg = VerifyClientMessage::try_from(msg)?;

			// client
			// 	.verify_client_message(ctx, client_id, client_state, msg.client_message)
			// 	.map_err(|e| ContractError::Client(format!("{e:?}")))
			// 	.map(|_| to_binary(&ContractResult::success()))
			Ok(()).map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::CheckForMisbehaviour(msg) => {
			let client_state =
				ctx.client_state(&client_id).map_err(|e| ContractError::Client(e.to_string()))?;
			let msg = CheckForMisbehaviourMsg::try_from(msg)?;
			client
				.check_for_misbehaviour(ctx, client_id, client_state, msg.client_message)
				.map_err(|e| ContractError::Client(e.to_string()))
				.map(|result| to_binary(&ContractResult::success().misbehaviour(result)))
		},
		ExecuteMsg::UpdateStateOnMisbehaviour(msg_raw) => {
			let client_state =
				ctx.client_state(&client_id).map_err(|e| ContractError::Client(e.to_string()))?;
			let msg = UpdateStateOnMisbehaviourMsg::try_from(msg_raw)?;
			client
				.update_state_on_misbehaviour(client_state, msg.client_message)
				.map_err(|e| ContractError::Client(e.to_string()))
				.and_then(|cs| {
					ctx.store_client_state(client_id, cs)
						.map_err(|e| ContractError::Client(e.to_string()))?;
					Ok(to_binary(&ContractResult::success()))
				})
		},
		ExecuteMsg::UpdateState(msg_raw) => {
			let client_state =
				ctx.client_state(&client_id).map_err(|e| ContractError::Client(e.to_string()))?;
			let msg = UpdateStateMsg::try_from(msg_raw)?;

			client
				.update_state(ctx, client_id.clone(), client_state, msg.client_message)
				.map_err(|e| ContractError::Client(e.to_string()))
				.and_then(|(cs, cu)| {
					store_client_and_consensus_states(ctx, client_id.clone(), cs, cu)
				})
		},
		ExecuteMsg::CheckSubstituteAndUpdateState(msg) => {
			let _msg = CheckSubstituteAndUpdateStateMsg::try_from(msg)?;
			// manually load both states from the combined storage using the appropriate prefixes
			let old_client_state = ctx
				.client_state_prefixed(SUBJECT_PREFIX)
				.map_err(|e| ContractError::Client(e.to_string()))?;
			let substitute_client_state = ctx
				.client_state_prefixed(SUBSTITUTE_PREFIX)
				.map_err(|e| ContractError::Client(e.to_string()))?;

			// Check that the substitute client state is valid:
			// all fields should be the same as in the old state, except for the `relay_chain`,
			// `para_id`, `latest_para_height`, `latest_relay_height`, `frozen_height`,
			// `current_authorities`, `current_set_id`
			// old_client_state.relay_chain = substitute_client_state.relay_chain;
			// old_client_state.para_id = substitute_client_state.para_id;
			// old_client_state.latest_para_height = substitute_client_state.latest_para_height;
			// old_client_state.latest_relay_height = substitute_client_state.latest_relay_height;
			// old_client_state.frozen_height = substitute_client_state.frozen_height;
			// old_client_state.current_authorities =
			// 	substitute_client_state.current_authorities.clone();
			// old_client_state.current_set_id = substitute_client_state.current_set_id;

			if old_client_state != substitute_client_state {
				return Err(ContractError::Client(
					"subject client state does not match substitute client state".to_string(),
				))
			}
			let substitute_client_state = old_client_state;
			let height = substitute_client_state.latest_height();
			// consensus state should be replaced as well
			let substitute_consensus_state =
				ctx.consensus_state_prefixed(height, SUBSTITUTE_PREFIX)?;
			ctx.store_consensus_state_prefixed(height, substitute_consensus_state, SUBJECT_PREFIX);
			ctx.store_client_state_prefixed(substitute_client_state, SUBJECT_PREFIX)
				.map_err(|e| ContractError::Client(e.to_string()))?;

			Ok(()).map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::VerifyUpgradeAndUpdateState(msg) => {
			let old_client_state =
				ctx.client_state(&client_id).map_err(|e| ContractError::Client(e.to_string()))?;
			let msg: VerifyUpgradeAndUpdateStateMsg<HostFunctions> =
				VerifyUpgradeAndUpdateStateMsg::try_from(msg)?;
			client
				.verify_upgrade_and_update_state(
					ctx,
					client_id.clone(),
					&old_client_state,
					&msg.upgrade_client_state,
					&msg.upgrade_consensus_state,
					msg.proof_upgrade_client,
					msg.proof_upgrade_consensus_state,
				)
				.map_err(|e| ContractError::Client(e.to_string()))
				.and_then(|(cs, cu)| {
					store_client_and_consensus_states(ctx, client_id.clone(), cs, cu)
				})
		},
	};
	Ok(result??)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
	let client_id = ClientId::from_str("08-wasm-0").expect("client id is valid");
	match msg {
		QueryMsg::ClientTypeMsg(_) => unimplemented!("ClientTypeMsg"),
		QueryMsg::GetLatestHeightsMsg(_) => unimplemented!("GetLatestHeightsMsg"),
		QueryMsg::ExportMetadata(ExportMetadataMsg {}) =>
			to_binary(&QueryResponse::genesis_metadata(None)),
		QueryMsg::Status(StatusMsg {}) => {
			let client_state = match get_client_state::<HostFunctions>(deps) {
				Ok(client_state) => client_state,
				Err(_) => return to_binary(&QueryResponse::status("Unknown".to_string())),
			};

			if client_state.frozen_height().is_some() {
				to_binary(&QueryResponse::status("Frozen".to_string()))
			} else {
				let height = client_state.latest_height();
				deps.api.debug(&format!("Querying consensus state at: {:?}", height));
				// match get_consensus_state(deps, &client_id, height) {
				// 	Ok(_) => to_binary(&QueryResponse::status("Active".to_string())),
				// 	Err(_) => to_binary(&QueryResponse::status("Expired".to_string())),
				// }
				to_binary(&QueryResponse::status("Active".to_string()))
			}
		},
	}
}

fn store_client_and_consensus_states<H>(
	ctx: &mut Context<H>,
	client_id: ClientId,
	client_state: ClientState<H>,
	consensus_update: ConsensusUpdateResult<Context<H>>,
) -> Result<StdResult<Binary>, ContractError>
where
	H: Clone + Eq + Send + Sync + Debug + Default + 'static,
{
	let height = client_state.latest_height();
	match consensus_update {
		ConsensusUpdateResult::Single(cs) => {
			log!(ctx, "Storing consensus state: {:?}", height);
			ctx.store_consensus_state(client_id.clone(), height, cs)
				.map_err(|e| ContractError::Client(e.to_string()))?;
		},
		ConsensusUpdateResult::Batch(css) =>
			for (height, cs) in css {
				log!(ctx, "Storing consensus state: {:?}", height);
				ctx.store_consensus_state(client_id.clone(), height, cs)
					.map_err(|e| ContractError::Client(e.to_string()))?;
			},
	}
	log!(ctx, "Storing client state with height: {:?}", height);
	ctx.store_client_state(client_id, client_state)
		.map_err(|e| ContractError::Client(e.to_string()))?;
	Ok(to_binary(&ContractResult::success()))
}
