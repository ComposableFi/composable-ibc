use crate::{
	context::Context,
	error::ContractError,
	ics23::FakeInner,
	log,
	msg::{
		CheckForMisbehaviourMsg, ClientStateCallResponse, ContractResult, ExecuteMsg,
		ExportMetadataMsg, InitializeState, InstantiateMsg, QueryMsg, QueryResponse, StatusMsg,
		UpdateStateMsg, UpdateStateOnMisbehaviourMsg, VerifyClientMessage, VerifyMembershipMsg,
		VerifyNonMembershipMsg, VerifyUpgradeAndUpdateStateMsg,
	},
	state::{get_client_state, get_consensus_state},
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
use grandpa_light_client_primitives::justification::AncestryChain;
use ibc::core::{
	ics02_client::{
		client_def::{ClientDef, ConsensusUpdateResult},
		context::{ClientKeeper, ClientReader},
		height::Height,
	},
	ics24_host::identifier::ClientId,
};
use ics08_wasm::client_state::ClientState as WasmClientState;
use ics10_grandpa::{
	client_def::GrandpaClient,
	client_message::{ClientMessage, RelayChainHeader},
	consensus_state::ConsensusState,
};
use light_client_common::{verify_membership, verify_non_membership};
use prost::Message;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Header};
use sp_runtime_interface::unpack_ptr_and_len;
use std::{collections::BTreeSet, str::FromStr};
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
pub const GRANDPA_HEADER_HASHES_STORAGE: Item<Vec<H256>> = Item::new("grandpa_header_hashes");
pub const GRANDPA_HEADER_HASHES_SET_STORAGE: Map<Vec<u8>, ()> =
	Map::new("grandpa_header_hashes_set");

pub const GRANDPA_BLOCK_HASHES_CACHE_SIZE: usize = 500;

#[derive(Clone, Copy, Debug, PartialEq, Default, Eq)]
pub struct HostFunctions;

impl light_client_common::HostFunctions for HostFunctions {
	type BlakeTwo256 = BlakeTwo256;
}

impl grandpa_light_client_primitives::HostFunctions for HostFunctions {
	type Header = RelayChainHeader;

	fn ed25519_verify(
		sig: &sp_core::ed25519::Signature,
		msg: &[u8],
		pub_key: &sp_core::ed25519::Public,
	) -> bool {
		use ed25519_zebra::{Signature, VerificationKey as PublicKey, VerificationKeyBytes};
		let bytes: [u8; 64] = (sig.clone()).try_into().expect("signature is 64 bytes; qed");
		let sig = Signature::from(bytes);
		let slice: &[u8] = pub_key.as_ref();
		let pub_key = PublicKey::try_from(
			VerificationKeyBytes::try_from(slice).expect("key is always valid; qed"),
		)
		.expect("key is always valid; qed");
		pub_key.verify(&sig, msg).is_ok()
	}

	fn insert_relay_header_hashes(_headers: &[<Self::Header as Header>::Hash]) {
		// implementation of this method is in `Context`
	}

	fn contains_relay_header_hash(_hash: <Self::Header as Header>::Hash) -> bool {
		// implementation of this method is in `Context`
		true
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
	env: Env,
	_info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	let client = GrandpaClient::<HostFunctions>::default();
	let mut ctx = Context::<HostFunctions>::new(deps, env);
	let client_id = ClientId::from_str("08-wasm-0").expect("client id is valid");
	let result = process_message(msg, client, &mut ctx, client_id.clone());
	let data = match result {
		Ok(res) => res,
		Err(ContractError::Grandpa(e)) => to_binary(&ContractResult::error(e))?,
		Err(e) => return Err(e),
	};
	let mut response = Response::default();
	response.data = Some(data);
	Ok(response)
}

fn process_message(
	msg: ExecuteMsg,
	client: GrandpaClient<HostFunctions>,
	ctx: &mut Context<HostFunctions>,
	client_id: ClientId,
) -> Result<Binary, ContractError> {
	// log!(ctx, "process_message: {:?}", msg);
	let result = match msg {
		ExecuteMsg::VerifyMembership(msg) => {
			let msg = VerifyMembershipMsg::try_from(msg)?;
			let consensus_state = ctx
				.consensus_state(&client_id, msg.height)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
			verify_membership::<BlakeTwo256, _>(
				&msg.prefix,
				&msg.proof,
				&consensus_state.root,
				msg.path,
				msg.value,
			)
			.map_err(|e| ContractError::Grandpa(e.to_string()))?;
			Ok(()).map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::VerifyNonMembership(msg) => {
			let msg = VerifyNonMembershipMsg::try_from(msg)?;
			let consensus_state = ctx
				.consensus_state(&client_id, msg.height)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;

			verify_non_membership::<BlakeTwo256, _>(
				&msg.prefix,
				&msg.proof,
				&consensus_state.root,
				msg.path,
			)
			.map_err(|e| ContractError::Grandpa(e.to_string()))
			.map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::VerifyClientMessage(msg) => {
			CODE_ID.save(ctx.deps.storage, &msg.client_state.code_id)?;
			let msg = VerifyClientMessage::try_from(msg)?;

			match &msg.client_message {
				ClientMessage::Misbehaviour(misbehavior) => {
					let first_proof = &misbehavior.first_finality_proof;
					let first_base = first_proof
						.unknown_headers
						.iter()
						.min_by_key(|h| *h.number())
						.ok_or_else(|| {
							ContractError::Grandpa("Unknown headers can't be empty!".to_string())
						})?;
					let first_parent = first_base.parent_hash;
					if !ctx.contains_relay_header_hash(first_parent) {
						Err(ContractError::Grandpa(
							"Could not find the known header for first finality proof".to_string(),
						))?
					}
				},
				_ => {},
			}

			let f = client
				.verify_client_message(ctx, client_id, msg.client_state, msg.client_message)
				.map_err(|e| ContractError::Grandpa(format!("{e:?}")))
				.map(|_| to_binary(&ContractResult::success()));
			f
		},
		ExecuteMsg::CheckForMisbehaviour(msg) => {
			let msg = CheckForMisbehaviourMsg::try_from(msg)?;
			client
				.check_for_misbehaviour(ctx, client_id, msg.client_state, msg.client_message)
				.map_err(|e| ContractError::Grandpa(e.to_string()))
				.map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::UpdateStateOnMisbehaviour(msg_raw) => {
			let mut client_state: WasmClientState<FakeInner, FakeInner, FakeInner> =
				msg_raw.client_state.clone();
			let msg = UpdateStateOnMisbehaviourMsg::try_from(msg_raw)?;
			client
				.update_state_on_misbehaviour(msg.client_state, msg.client_message)
				.map_err(|e| ContractError::Grandpa(e.to_string()))
				.and_then(|cs| {
					client_state.data = cs.to_any().encode_to_vec();
					Ok(to_binary(&client_state)
						.and_then(|data| to_binary(&ContractResult::success().data(data.0))))
				})
		},
		ExecuteMsg::UpdateState(msg_raw) => {
			let mut client_state: WasmClientState<FakeInner, FakeInner, FakeInner> =
				msg_raw.client_state.clone();
			let msg = UpdateStateMsg::try_from(msg_raw)?;

			let finalized_headers = match &msg.client_message {
				ClientMessage::Header(header) => {
					use finality_grandpa::Chain;
					let ancestry = AncestryChain::<RelayChainHeader>::new(
						&header.finality_proof.unknown_headers,
					);
					let from = msg.client_state.latest_relay_hash;
					let mut finalized =
						ancestry.ancestry(from, header.finality_proof.block).map_err(|_| {
							ContractError::Grandpa(format!("[update_state] Invalid ancestry!"))
						})?;
					finalized.reverse();
					finalized
				},
				_ => Vec::new(),
			};

			client
				.update_state(ctx, client_id.clone(), msg.client_state, msg.client_message)
				.map_err(|e| ContractError::Grandpa(e.to_string()))
				.and_then(|(cs, cu)| {
					let height = cs.latest_height();
					client_state.latest_height = height.into();
					client_state.data = cs.to_any().encode_to_vec();
					ctx.insert_relay_header_hashes(&finalized_headers);

					match cu {
						ConsensusUpdateResult::Single(cs) => {
							log!(ctx, "Storing consensus state: {:?}", height);
							ctx.store_consensus_state(client_id.clone(), height, cs)
								.map_err(|e| ContractError::Grandpa(e.to_string()))?;
						},
						ConsensusUpdateResult::Batch(css) =>
							for (height, cs) in css {
								log!(ctx, "Storing consensus state: {:?}", height);
								ctx.store_consensus_state(client_id.clone(), height, cs)
									.map_err(|e| ContractError::Grandpa(e.to_string()))?;
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
				.map_err(|e| ContractError::Grandpa(e.to_string()))
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
				match get_consensus_state(deps, &client_id, height) {
					Ok(_) => to_binary(&QueryResponse::status("Active".to_string())),
					Err(_) => to_binary(&QueryResponse::status("Expired".to_string())),
				}
			}
		},
	}
}

// The FFIs below are required because of sp-io dependency that expects the functions to be
// available on the host.
#[no_mangle]
// &[u8] -> [u8; 32]
pub extern "C" fn ext_hashing_blake2_256_version_1(data: i64) -> i32 {
	let (ptr, len) = unpack_ptr_and_len(data as _);
	let data = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as _) };
	let hash = blake2_rfc::blake2b::blake2b(32, &[], data);
	let out_ptr = hash.as_ref().to_vec().leak().as_ptr();
	out_ptr as i32
}

pub fn twox_64_into(data: &[u8], dest: &mut [u8; 8]) {
	let r0 = twox_hash::XxHash::with_seed(0).chain_update(data).finish();
	LittleEndian::write_u64(&mut dest[0..8], r0);
}

pub fn twox_128_into(data: &[u8], dest: &mut [u8; 16]) {
	let r0 = twox_hash::XxHash::with_seed(0).chain_update(data).finish();
	let r1 = twox_hash::XxHash::with_seed(1).chain_update(data).finish();
	LittleEndian::write_u64(&mut dest[0..8], r0);
	LittleEndian::write_u64(&mut dest[8..16], r1);
}

#[no_mangle]
// &[u8] -> [u8; 16]
pub extern "C" fn ext_hashing_twox_128_version_1(data: i64) -> i32 {
	let (ptr, len) = unpack_ptr_and_len(data as _);
	let data = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as _) };
	let mut hash = Box::new([0u8; 16]);
	twox_128_into(data, hash.as_mut());
	let out_ptr = Box::leak(hash).as_ptr();
	out_ptr as i32
}

#[no_mangle]
// &[u8] -> [u8; 8]
pub extern "C" fn ext_hashing_twox_64_version_1(data: i64) -> i32 {
	let (ptr, len) = unpack_ptr_and_len(data as _);
	let data = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as _) };
	let mut hash = Box::new([0u8; 8]);
	twox_64_into(data, hash.as_mut());
	let out_ptr = Box::leak(hash).as_ptr();
	out_ptr as i32
}

pub fn code_id(store: &dyn Storage) -> Vec<u8> {
	CODE_ID.load(store).expect("code id must be set")
}
