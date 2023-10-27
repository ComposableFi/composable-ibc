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
	helpers::{
		check_substitute_and_update_state, prune_oldest_consensus_state, verify_delay_passed,
		verify_upgrade_and_update_state,
	},
	ics23::{FakeInner, ReadonlyProcessedStates},
	msg::{
		CheckForMisbehaviourMsg, ContractResult, ExportMetadataMsg, QueryMsg,
		QueryResponse, StatusMsg, SudoMsg, UpdateStateMsg, UpdateStateOnMisbehaviourMsg,
		VerifyClientMessage, VerifyMembershipMsg, VerifyNonMembershipMsg,
		VerifyUpgradeAndUpdateStateMsg,
	},
	state::{get_client_state, get_consensus_state},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use ed25519_consensus::VerificationKey;
use ibc::{
	core::{
		ics02_client::{
			client_def::{ClientDef, ConsensusUpdateResult},
			context::{ClientKeeper, ClientReader},
		},
		ics24_host::identifier::ClientId,
	},
	Height,
};
use ibc_proto::google::protobuf::Any;
use ics07_tendermint::{
	client_def::{verify_membership, verify_non_membership, TendermintClient},
	client_state::ClientState,
	consensus_state::ConsensusState,
	HostFunctionsProvider,
};
use ics08_wasm::{instantiate::InstantiateMessage, SUBJECT_PREFIX};
use prost::Message;
use sha2::{Digest, Sha256};
use std::str::FromStr;
use tendermint::{
	crypto::{
		signature::{Error as TendermintCryptoError, Verifier},
		Sha256 as TendermintSha256,
	},
	merkle::{Hash, MerkleHash, NonIncremental, HASH_SIZE},
	PublicKey, Signature,
};
use tendermint_light_client_verifier::operations::CommitValidator;
use tendermint_proto::Protobuf;

#[derive(Clone, Copy, Debug, PartialEq, Default, Eq)]
pub struct HostFunctions;

impl ics23::HostFunctionsProvider for HostFunctions {
	fn sha2_256(message: &[u8]) -> [u8; 32] {
		let mut hasher = Sha256::default();
		hasher.update(message);
		hasher.finalize().as_slice().try_into().expect("slice with incorrect length")
	}

	fn sha2_512(_message: &[u8]) -> [u8; 64] {
		unimplemented!()
	}

	fn sha2_512_truncated(_message: &[u8]) -> [u8; 32] {
		unimplemented!()
	}

	fn sha3_512(_message: &[u8]) -> [u8; 64] {
		unimplemented!()
	}

	fn ripemd160(_message: &[u8]) -> [u8; 20] {
		unimplemented!()
	}
}

impl TendermintSha256 for HostFunctions {
	fn digest(data: impl AsRef<[u8]>) -> [u8; HASH_SIZE] {
		<Self as ics23::HostFunctionsProvider>::sha2_256(data.as_ref())
	}
}

impl MerkleHash for HostFunctions {
	fn empty_hash(&mut self) -> Hash {
		NonIncremental::<Self>::default().empty_hash()
	}

	fn leaf_hash(&mut self, bytes: &[u8]) -> Hash {
		NonIncremental::<Self>::default().leaf_hash(bytes)
	}

	fn inner_hash(&mut self, left: Hash, right: Hash) -> Hash {
		NonIncremental::<Self>::default().inner_hash(left, right)
	}
}

impl Verifier for HostFunctions {
	fn verify(
		pubkey: PublicKey,
		msg: &[u8],
		signature: &Signature,
	) -> Result<(), TendermintCryptoError> {
		let vk = pubkey.ed25519().expect("");
		let pubkey2 = VerificationKey::try_from(vk.as_bytes())
			.map_err(|_| TendermintCryptoError::MalformedPublicKey)?;
		let sig = ed25519_consensus::Signature::try_from(signature.as_bytes())
			.map_err(|_| TendermintCryptoError::MalformedSignature)?;
		pubkey2.verify(&sig, msg).map_err(|_| TendermintCryptoError::VerificationFailed)
	}
}
impl CommitValidator for HostFunctions {}
impl HostFunctionsProvider for HostFunctions {}

fn process_instantiate_msg(
	msg: InstantiateMessage<FakeInner, FakeInner, FakeInner>,
	ctx: &mut Context<HostFunctions>,
	client_id: ClientId,
) -> Result<Binary, ContractError> {
	let any = Any::decode(&mut msg.client_state.data.as_slice())?;
	let client_state = ClientState::decode_vec(&any.value)?;
	let any = Any::decode(&mut msg.consensus_state.data.as_slice())?;
	let consensus_state = ConsensusState::decode_vec(&any.value)?;

	ctx.code_hash = Some(msg.client_state.code_hash);
	let height = client_state.latest_height();
	ctx.store_client_state(client_id.clone(), client_state)
		.map_err(|e| ContractError::Tendermint(e.to_string()))?;
	ctx.store_consensus_state(client_id.clone(), height, consensus_state)
		.map_err(|e| ContractError::Tendermint(e.to_string()))?;

	ctx.store_update_height(client_id.clone(), height, ctx.host_height())
		.map_err(|e| ContractError::Tendermint(e.to_string()))?;
	ctx.store_update_time(client_id, height, ctx.host_timestamp())
		.map_err(|e| ContractError::Tendermint(e.to_string()))?;

	Ok(to_binary(&ContractResult::success())?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	msg: InstantiateMessage<FakeInner, FakeInner, FakeInner>,
) -> Result<Response, ContractError> {
	let mut ctx = Context::<HostFunctions>::new(deps, env);
	let client_id = ClientId::from_str("08-wasm-0").expect("client id is valid");
	let data = process_instantiate_msg(msg, &mut ctx, client_id.clone())?;
	let mut response = Response::default();
	response.data = Some(data);
	Ok(response)
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(
	deps: DepsMut,
	env: Env,
	msg: SudoMsg,
) -> Result<Response, ContractError> {
	let client = TendermintClient::<HostFunctions>::default();
	let mut ctx = Context::<HostFunctions>::new(deps, env);
	let client_id = ClientId::from_str("08-wasm-0").expect("client id is valid");
	let data = process_message(msg, client, &mut ctx, client_id.clone())?;
	let mut response = Response::default();
	response.data = Some(data);
	Ok(response)
}

fn process_message(
	msg: SudoMsg,
	client: TendermintClient<HostFunctions>,
	ctx: &mut Context<HostFunctions>,
	client_id: ClientId,
) -> Result<Binary, ContractError> {
	//log!(ctx, "process_message: {:?}", msg);
	let result = match msg {
		SudoMsg::UpdateStateOnMisbehaviour(msg_raw) => {
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
		SudoMsg::UpdateState(msg_raw) => {
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let msg = UpdateStateMsg::try_from(msg_raw)?;
			let latest_revision_height = client_state.latest_height().revision_height;
			prune_oldest_consensus_state(ctx, &client_state, ctx.host_timestamp().nanoseconds());
			client
				.update_state(ctx, client_id.clone(), client_state, msg.client_message)
				.map_err(|e| ContractError::Tendermint(e.to_string()))
				.and_then(|(cs, cu)| {
					let height = cs.latest_height();
					let mut heights: Vec<Height> = vec![];
					match cu {
						ConsensusUpdateResult::Single(cs) => {
							heights.push(height);
							ctx.store_consensus_state(client_id.clone(), height, cs)
								.map_err(|e| ContractError::Tendermint(e.to_string()))?;
						},
						ConsensusUpdateResult::Batch(css) =>
							for (height, cs) in css {
								heights.push(height);
								ctx.store_consensus_state(client_id.clone(), height, cs)
									.map_err(|e| ContractError::Tendermint(e.to_string()))?;
							},
					}
					if cs.latest_height().revision_height > latest_revision_height {
						ctx.store_client_state(client_id, cs)
							.map_err(|e| ContractError::Tendermint(e.to_string()))?;
					}
					Ok(to_binary(&ContractResult::success().heights(heights)))
				})
		},
		SudoMsg::MigrateClientStore(_msg) =>
			check_substitute_and_update_state::<HostFunctions>(ctx)
				.map_err(|e| ContractError::Tendermint(e.to_string()))
				.and_then(|(cs, cu)| {
					let height = cs.latest_height();
					ctx.store_consensus_state_prefixed(height, cu, SUBJECT_PREFIX);
					ctx.store_client_state_prefixed(cs, SUBJECT_PREFIX)
						.map_err(|e| ContractError::Tendermint(e.to_string()))?;
					Ok(to_binary(&ContractResult::success()))
				}),
		SudoMsg::VerifyMembership(msg) => {
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let msg = VerifyMembershipMsg::try_from(msg)?;
			verify_delay_passed(&ctx, msg.height, msg.delay_time_period, msg.delay_block_period)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let consensus_state = ctx
				.consensus_state(&client_id, msg.height)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			verify_membership::<HostFunctions, _>(
				&client_state,
				&msg.prefix,
				&msg.proof,
				&consensus_state.root,
				msg.path,
				msg.value,
			)
			.map_err(|e| ContractError::Tendermint(e.to_string()))
		  .map(|_| to_binary(&ContractResult::success()))
		},
		SudoMsg::VerifyNonMembership(msg) => {
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let msg = VerifyNonMembershipMsg::try_from(msg)?;
			verify_delay_passed(&ctx, msg.height, msg.delay_time_period, msg.delay_block_period)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let consensus_state = ctx
				.consensus_state(&client_id, msg.height)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;

			verify_non_membership::<HostFunctions, _>(
				&client_state,
				&msg.prefix,
				&msg.proof,
				&consensus_state.root,
				msg.path,
			)
			.map_err(|e| ContractError::Tendermint(e.to_string()))
			.map(|_| to_binary(&ContractResult::success()))
		},
		SudoMsg::VerifyUpgradeAndUpdateState(msg) => {
			let old_client_state = ctx
				.client_state(&client_id.clone())
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let msg: VerifyUpgradeAndUpdateStateMsg =
				VerifyUpgradeAndUpdateStateMsg::try_from(msg)?;
			verify_upgrade_and_update_state::<HostFunctions>(
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
	};
	Ok(result??)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
	let client_id = ClientId::from_str("08-wasm-0").expect("client id is valid");
	match msg {
		QueryMsg::CheckForMisbehaviour(msg) => {
			let ctx = Context::<HostFunctions>::new_ro(deps, env);
			let client = TendermintClient::<HostFunctions>::default();
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let msg = CheckForMisbehaviourMsg::try_from(msg)?;
			client
				.check_for_misbehaviour(&ctx, client_id, client_state, msg.client_message)
				.map_err(|e| ContractError::Tendermint(e.to_string()))
				.map(|result| to_binary(&QueryResponse::success().misbehaviour(result)))?
		},
		QueryMsg::ClientTypeMsg(_) => unimplemented!("ClientTypeMsg"),
		QueryMsg::GetLatestHeightsMsg(_) => unimplemented!("GetLatestHeightsMsg"),
		QueryMsg::ExportMetadata(ExportMetadataMsg {}) => {
			let ro_proceeded_state = ReadonlyProcessedStates::new(deps.storage);
			to_binary(&QueryResponse::success().genesis_metadata(ro_proceeded_state.get_metadata()))
		},
		QueryMsg::Status(StatusMsg {}) => {
			let client_state = match get_client_state::<HostFunctions>(deps) {
				Ok(client_state) => client_state,
				Err(_) => return to_binary(&QueryResponse::success().status("Unknown".to_string())),
			};

			if client_state.frozen_height().is_some() {
				to_binary(&QueryResponse::success().status("Frozen".to_string()))
			} else {
				let height = client_state.latest_height();
				match get_consensus_state(deps, &client_id, height) {
					Ok(consensus_state) => {
						let last_update = consensus_state.timestamp.unix_timestamp().unsigned_abs();
						let tp = client_state.trusting_period.as_secs();
						let now = env.block.time.seconds();
						if (last_update + tp) < now {
							return to_binary(&QueryResponse::success().status("Expired".to_string()))
						}
						to_binary(&QueryResponse::success().status("Active".to_string()))
					},
					Err(_) => to_binary(&QueryResponse::success().status("Expired".to_string())),
				}
			}
		},
		QueryMsg::TimestampAtHeight(msg) => {
			let ctx = Context::<HostFunctions>::new_ro(deps, env);
			let consensus_state = ctx
				.consensus_state(&client_id, msg.height)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			to_binary(&QueryResponse::success().timestamp(consensus_state.timestamp.unix_timestamp().unsigned_abs()))
		},
		QueryMsg::VerifyClientMessage(msg) => {
			let ctx = Context::<HostFunctions>::new_ro(deps, env);
			let client = TendermintClient::<HostFunctions>::default();
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Tendermint(e.to_string()))?;
			let msg = VerifyClientMessage::try_from(msg)?;
			client
				.verify_client_message(&ctx, client_id, client_state, msg.client_message)
				.map_err(|e| ContractError::Tendermint(format!("{e:?}")))
				.map(|_| to_binary(&QueryResponse::success()))?
		},
	}
}
