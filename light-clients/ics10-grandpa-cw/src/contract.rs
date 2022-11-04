#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use ibc::core::ics02_client::client_def::ClientDef;
use ics10_grandpa::{client_def::GrandpaClient, client_message::RelayChainHeader};
use light_client_common::{verify_membership, verify_non_membership};
use sp_runtime::traits::{BlakeTwo256, Header};
use std::{cell::RefCell, rc::Rc};

use crate::{
	context::Context,
	error::ContractError,
	msg::{
		CheckForMisbehaviourMsg, ExecuteMsg, InstantiateMsg, QueryMsg, UpdateStateMsg,
		UpdateStateOnMisbehaviourMsg, VerifyClientMessage, VerifyMembershipMsg,
		VerifyNonMembershipMsg, VerifyUpgradeAndUpdateStateMsg,
	},
};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ics10-grandpa-cw";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[derive(Clone, Copy, Debug, PartialEq, Default, Eq)]
pub struct HostFunctions;

impl light_client_common::HostFunctions for HostFunctions {
	type BlakeTwo256 = BlakeTwo256;
}

impl grandpa_light_client_primitives::HostFunctions for HostFunctions {
	type Header = RelayChainHeader;

	fn ed25519_verify(
		_sig: &sp_core::ed25519::Signature,
		_msg: &[u8],
		_pub_key: &sp_core::ed25519::Public,
	) -> bool {
		todo!()
	}

	fn insert_relay_header_hashes(_headers: &[<Self::Header as Header>::Hash]) {
		todo!()
	}

	fn contains_relay_header_hash(_hash: <Self::Header as Header>::Hash) -> bool {
		todo!()
	}
}

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
	_env: Env,
	_info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	let deps = Rc::new(RefCell::new(deps));
	let client = GrandpaClient::<HostFunctions>::default();
	let ctx = Context::<HostFunctions>::new(deps);
	match msg {
		ExecuteMsg::ValidateMsg(_) => todo!(),
		ExecuteMsg::StatusMsg(_) => todo!(),
		ExecuteMsg::ExportedMetadataMsg(_) => todo!(),
		ExecuteMsg::ZeroCustomFieldsMsg(_) => todo!(),
		ExecuteMsg::GetTimestampAtHeightMsg(_) => todo!(),
		ExecuteMsg::InitializeMsg(_) => todo!(),
		ExecuteMsg::VerifyMembershipMsg(msg) => {
			let msg = VerifyMembershipMsg::try_from(msg)?;
			verify_membership::<BlakeTwo256, _>(
				&msg.prefix,
				&msg.proof,
				&msg.root,
				msg.path,
				msg.value,
			)
			.map_err(|e| ContractError::Grandpa(e.to_string()))?;
		},
		ExecuteMsg::VerifyNonMembershipMsg(msg) => {
			let msg = VerifyNonMembershipMsg::try_from(msg)?;
			verify_non_membership::<BlakeTwo256, _>(&msg.prefix, &msg.proof, &msg.root, msg.path)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
		},
		ExecuteMsg::VerifyClientMessage(msg) => {
			let msg = VerifyClientMessage::try_from(msg)?;
			client
				.verify_client_message(&ctx, msg.client_id, msg.client_state, msg.client_message)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
		},
		ExecuteMsg::CheckForMisbehaviourMsg(msg) => {
			let msg = CheckForMisbehaviourMsg::try_from(msg)?;
			client
				.check_for_misbehaviour(&ctx, msg.client_id, msg.client_state, msg.client_message)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
		},
		ExecuteMsg::UpdateStateOnMisbehaviourMsg(msg) => {
			let msg = UpdateStateOnMisbehaviourMsg::try_from(msg)?;
			client
				.update_state_on_misbehaviour(msg.client_state, msg.client_message)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
		},
		ExecuteMsg::UpdateStateMsg(msg) => {
			let msg = UpdateStateMsg::try_from(msg)?;
			client
				.update_state(&ctx, msg.client_id, msg.client_state, msg.client_message)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
		},
		ExecuteMsg::CheckSubstituteAndUpdateStateMsg(_msg) => {
			todo!("check substitute and update state")
		},
		ExecuteMsg::VerifyUpgradeAndUpdateStateMsg(msg) => {
			let msg = VerifyUpgradeAndUpdateStateMsg::try_from(msg)?;
			client
				.verify_upgrade_and_update_state(
					&ctx,
					msg.client_id,
					&msg.old_client_state,
					&msg.upgrade_client_state,
					&msg.upgrade_consensus_state,
					msg.proof_upgrade_client,
					msg.proof_upgrade_consensus_state,
				)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
		},
	}
	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
	match msg {
		QueryMsg::ClientTypeMsg(_) => todo!(),
		QueryMsg::GetLatestHeightsMsg(_) => todo!(),
	}
}

#[cfg(test)]
mod tests {}
