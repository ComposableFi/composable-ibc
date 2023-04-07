use std::time::Duration;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};
use ibc_proto::google::protobuf::Any;
use crate::{
	context::Context,
	msg::ExecuteMsg,
	ics23::{FakeInner, ProcessedStates, ConsensusStates},
	contract::{SUBJECT_PREFIX, SUBSTITUTE_PREFIX},
};
use tendermint_proto::Protobuf;
use ibc_proto::ibc::core::commitment::v1::{MerkleProof as RawMerkleProof, MerklePath};
use prost::Message;
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState as _,
			context::ClientReader,
			error::Error as Ics02Error,
		},
		ics23_commitment::{
			commitment::CommitmentProofBytes,
			merkle::MerkleProof,
		},
		ics24_host::identifier::ClientId,
	},
};

use ics07_tendermint::{
	HostFunctionsProvider,
	consensus_state::ConsensusState,
	client_state::{ClientState, UpgradeOptions},
};

use ics08_wasm::{
	client_state::ClientState as WasmClientState,
	consensus_state::ConsensusState as WasmConsensusState,
};

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
	pub fn addr(&self) -> Addr {
		self.0.clone()
	}

	pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
		let msg = to_binary(&msg.into())?;
		Ok(WasmMsg::Execute { contract_addr: self.addr().into(), msg, funds: vec![] }.into())
	}
}

pub fn verify_upgrade_and_update_state<H: HostFunctionsProvider + 'static>(
	ctx: &mut Context<H>,
	client_id: ClientId,
	old_client_state: ClientState<H>,
	upgrade_client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,//,ClientState<H>,
	upgrade_consensus_state: WasmConsensusState<FakeInner>,//ConsensusState,
	proof_upgrade_client: CommitmentProofBytes,
	proof_upgrade_consensus_state: CommitmentProofBytes,//Vec<u8>,
) -> Result<(ClientState<H>, ConsensusState), Ics02Error> {
	if old_client_state.upgrade_path.is_empty() {
		return Err(Ics02Error::implementation_specific("No upgrade path set".to_string()))
	}

	let latest_height = old_client_state.latest_height;
	if upgrade_client_state.latest_height.lt(&latest_height) {
		return Err(Ics02Error::implementation_specific("upgrade cs is less than current height".to_string()))
	}

	let consensus_state = 
		ctx.consensus_state(&client_id.clone(), latest_height, &mut Vec::new())?;
	
	let mut upgrade_path = old_client_state.upgrade_path.clone();
	let last_key = upgrade_path.remove(upgrade_path.len()-1);
	
	let mut client_path_vec = upgrade_path.clone();
	client_path_vec.append(&mut vec![format!("{}/{}/{}", last_key.as_str(), latest_height.revision_height, "upgradedClient")]);
	let client_path = MerklePath { key_path: client_path_vec };

	let mut cons_path_vec = upgrade_path.clone();
	cons_path_vec.append(&mut vec![format!("{}/{}/{}", last_key, latest_height.revision_height, "upgradedConsState")]);
	let cons_path = MerklePath {  key_path: cons_path_vec };

	let client_merkle_proof: MerkleProof<H> =
		RawMerkleProof::try_from(proof_upgrade_client)
			.map_err(Ics02Error::invalid_commitment_proof)?
			.into();
	client_merkle_proof
		.verify_membership(&old_client_state.proof_specs, consensus_state.root().clone().into(), client_path, upgrade_client_state.to_any().encode_to_vec(), 0)
		.unwrap();

	let cons_merkle_proof: MerkleProof<H> = 
		RawMerkleProof::try_from(proof_upgrade_consensus_state)
			.map_err(Ics02Error::invalid_commitment_proof)?
			.into();
	
	cons_merkle_proof
		.verify_membership(&old_client_state.proof_specs, consensus_state.root().clone().into(), cons_path, upgrade_consensus_state.to_any().encode_to_vec(), 0)
		.unwrap();

	let any = Any::decode(&mut upgrade_client_state.data.as_slice()).unwrap();
	let upgrade_client_state_inner = ClientState::<H>::decode_vec(&any.value).unwrap();
	let new_client_state = old_client_state.clone().upgrade(
		upgrade_client_state_inner.latest_height.clone(),
		UpgradeOptions {
			unbonding_period: upgrade_client_state_inner.unbonding_period.clone(),
			proof_specs: upgrade_client_state_inner.proof_specs.clone(),
			upgrade_path: upgrade_client_state_inner.upgrade_path.clone(),
		},
		upgrade_client_state_inner.chain_id.clone(),
	);

	let any = Any::decode(&mut upgrade_consensus_state.data.as_slice()).unwrap();
	let upgrade_consensus_state_inner = ConsensusState::decode_vec(&any.value).unwrap();
	Ok((new_client_state, upgrade_consensus_state_inner))

}


pub fn check_substitute_and_update_state<H: HostFunctionsProvider + 'static>(
	ctx: &mut Context<H>,
	client_id: ClientId,
) -> Result<(ClientState<H>, ConsensusState), Ics02Error> {
	let mut subject_client_state = ctx.client_state(&client_id, &mut SUBJECT_PREFIX.clone().to_vec())?;
		//.map_err(|e| ContractError::Tendermint(e.to_string()))?;
	let substitute_client_state = ctx.client_state(&client_id, &mut SUBSTITUTE_PREFIX.clone().to_vec())?;
		//.map_err(|e| ContractError::Tendermint(e.to_string()))?;
	
	if subject_client_state.trust_level != substitute_client_state.trust_level ||
		subject_client_state.unbonding_period != substitute_client_state.unbonding_period ||
		subject_client_state.max_clock_drift != substitute_client_state.max_clock_drift ||
		subject_client_state.proof_specs != substitute_client_state.proof_specs ||
		subject_client_state.upgrade_path != substitute_client_state.upgrade_path {
			return Err(Ics02Error::implementation_specific("Clients do not match".to_string()))
		}

	let height = substitute_client_state.latest_height();
	let substitute_consensus_state = ctx.consensus_state(&client_id, height, &mut SUBSTITUTE_PREFIX.clone().to_vec())?;
	//			.map_err(|e| ContractError::Tendermint(e.to_string()))?;

	let mut process_states = ProcessedStates::new(ctx.storage_mut());
	let substitute_processed_time = process_states.get_processed_time(height, &mut SUBSTITUTE_PREFIX.clone().to_vec()).unwrap();
	let substitute_processed_height = process_states.get_processed_height(height, &mut SUBSTITUTE_PREFIX.clone().to_vec()).unwrap();
	let substitute_iteration_key = process_states.get_iteration_key(height, &mut SUBSTITUTE_PREFIX.clone().to_vec()).unwrap();
	process_states.set_processed_time(height, substitute_processed_time, &mut SUBJECT_PREFIX.clone().to_vec());
	process_states.set_processed_height(height, substitute_processed_height, &mut SUBJECT_PREFIX.clone().to_vec());
	process_states.set_iteration_key(substitute_iteration_key, &mut SUBJECT_PREFIX.clone().to_vec());
	
	subject_client_state.latest_height = substitute_client_state.latest_height;
	subject_client_state.chain_id = substitute_client_state.chain_id;
	subject_client_state.trusting_period = substitute_client_state.trusting_period;
	subject_client_state.frozen_height = substitute_client_state.frozen_height;
	
	Ok((subject_client_state, substitute_consensus_state))

}

pub fn prune_oldest_consensus_state<H: HostFunctionsProvider + 'static>(
	ctx: &mut Context<H>,
	client_state: &ClientState<H>,
	current_time: u64,
) {
	let mut processed_states = ProcessedStates::new(ctx.storage_mut());
	match processed_states.get_earliest_height(client_state.latest_height.clone()) {
		Some(earliest_height) => {
			let processed_time = processed_states.get_processed_time(earliest_height, &mut Vec::new()).unwrap();
			let elapsed = Duration::from_nanos(current_time-processed_time);
			if client_state.expired(elapsed) {
				processed_states.remove_states_at_height(earliest_height);
				let mut consensus_states = ConsensusStates::new(ctx.storage_mut());
				consensus_states.remove(earliest_height);
			}
		},
		None => {},
	}


}