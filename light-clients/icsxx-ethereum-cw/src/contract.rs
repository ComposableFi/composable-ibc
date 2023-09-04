use crate::{
	context::Context,
	error::ContractError,
	ics23::FakeInner,
	log,
	msg::{
		CheckForMisbehaviourMsg, ClientStateCallResponse, ContractResult, ExecuteMsg,
		InitializeState, InstantiateMsg, QueryMsg, StatusMsg, UpdateStateMsg,
		UpdateStateOnMisbehaviourMsg, VerifyClientMessage, VerifyMembershipMsg,
		VerifyNonMembershipMsg, VerifyUpgradeAndUpdateStateMsg,
	},
	Bytes,
};
use byteorder::{ByteOrder, LittleEndian};
use core::hash::Hasher;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage,
};
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
use ics08_wasm::client_state::ClientState as WasmClientState;
use icsxx_ethereum::{
	client_def::EthereumClient, client_message::ClientMessage, consensus_state::ConsensusState,
};
use prost::Message;
use std::{collections::BTreeSet, str::FromStr};
use sync_committee_verifier::BlsVerify;
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
	let result = process_message(msg, client, &mut ctx, client_id.clone());
	let data = match result {
		Ok(res) => res,
		Err(ContractError::Client(e)) => to_binary(&ContractResult::error(e))?,
		Err(e) => return Err(e),
	};
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
		ExecuteMsg::Status(StatusMsg {}) => {
			let client_state =
				ctx.client_state(&client_id).map_err(|e| ContractError::Client(e.to_string()))?;
			if client_state.frozen_height().is_some() {
				Ok(to_binary("Frozen"))
			} else {
				Ok(to_binary("Active"))
			}
		},
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
			// .map_err(|e| ContractError::Client(e.to_string()))?;
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
			// .map_err(|e| ContractError::Client(e.to_string()))
			// .map(|_| to_binary(&ContractResult::success()))
			Ok(()).map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::VerifyClientMessage(msg) => {
			CODE_ID.save(ctx.deps.storage, &msg.client_state.code_id)?;
			let msg = VerifyClientMessage::try_from(msg)?;

			let f = client
				.verify_client_message(ctx, client_id, msg.client_state, msg.client_message)
				.map_err(|e| ContractError::Client(format!("{e:?}")))
				.map(|_| to_binary(&ContractResult::success()));
			f
		},
		ExecuteMsg::CheckForMisbehaviour(msg) => {
			let msg = CheckForMisbehaviourMsg::try_from(msg)?;
			client
				.check_for_misbehaviour(ctx, client_id, msg.client_state, msg.client_message)
				.map_err(|e| ContractError::Client(e.to_string()))
				.map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::UpdateStateOnMisbehaviour(msg) => {
			let msg = UpdateStateOnMisbehaviourMsg::try_from(msg)?;
			client
				.update_state_on_misbehaviour(msg.client_state, msg.client_message)
				.map_err(|e| ContractError::Client(e.to_string()))
				.map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::UpdateState(msg_raw) => {
			let mut client_state: WasmClientState<FakeInner, FakeInner, FakeInner> =
				msg_raw.client_state.clone();
			let msg = UpdateStateMsg::try_from(msg_raw)?;

			client
				.update_state(ctx, client_id.clone(), msg.client_state, msg.client_message)
				.map_err(|e| ContractError::Client(e.to_string()))
				.and_then(|(cs, cu)| {
					let height = cs.latest_height();
					client_state.latest_height = height.into();
					client_state.data = cs.to_any().encode_to_vec();

					match cu {
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
					Ok(to_binary(&client_state)
						.and_then(|data| to_binary(&ContractResult::success().data(data.0))))
				})
		},
		ExecuteMsg::CheckSubstituteAndUpdateState(_msg) => {
			todo!("check substitute and update state")
		},
		ExecuteMsg::VerifyUpgradeAndUpdateState(msg) => {
			let msg = VerifyUpgradeAndUpdateStateMsg::try_from(msg)?;
			client
				.verify_upgrade_and_update_state(
					ctx,
					client_id,
					&msg.old_client_state,
					&msg.upgrade_client_state,
					&msg.upgrade_consensus_state,
					msg.proof_upgrade_client,
					msg.proof_upgrade_consensus_state,
				)
				.map_err(|e| ContractError::Client(e.to_string()))
				.map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::InitializeState(InitializeState { client_state, consensus_state }) => {
			let state_call_response = ClientStateCallResponse {
				new_consensus_state: consensus_state.clone(),
				new_client_state: client_state.clone(),
				client_state,
				result: ContractResult::success(),
			};
			let response = to_binary(&state_call_response);
			Ok(response)
		},
		ExecuteMsg::ClientCreateRequest(_) => Ok(to_binary(&ContractResult::success())),
	};
	Ok(result??)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
	match msg {
		QueryMsg::ClientTypeMsg(_) => unimplemented!("ClientTypeMsg"),
		QueryMsg::GetLatestHeightsMsg(_) => unimplemented!("GetLatestHeightsMsg"),
	}
}

pub fn code_id(store: &dyn Storage) -> Vec<u8> {
	CODE_ID.load(store).expect("code id must be set")
}

impl BlsVerify for HostFunctions {
	fn verify(
		public_keys: &[&ethereum_consensus::crypto::PublicKey],
		msg: &[u8],
		signature: &ethereum_consensus::crypto::Signature,
	) -> Result<(), sync_committee_verifier::error::Error> {
		todo!()
	}
}
