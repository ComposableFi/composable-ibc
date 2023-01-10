use crate::Bytes;
use ibc::{
	core::{
		ics02_client::client_consensus::ConsensusState as IbcConsensusState,
		ics23_commitment::commitment::CommitmentRoot,
	},
	protobuf::Protobuf,
	timestamp::Timestamp,
};
use ibc_proto::ibc::lightclients::wasm::v1::{
	ClientState as RawClientState, ConsensusState as RawConsensusState,
};
use std::convert::Infallible;

pub const WASM_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.ConsensusState";

impl IbcConsensusState for ConsensusState {
	type Error = Infallible;

	fn root(&self) -> &CommitmentRoot {
		todo!()
	}

	fn timestamp(&self) -> Timestamp {
		todo!()
	}

	fn encode_to_vec(&self) -> Vec<u8> {
		todo!()
	}
}

impl From<ConsensusState> for RawClientState {
	fn from(value: ConsensusState) -> Self {
		Self {
			data: value.data,
			code_id: value.code_id,
			latest_height: None,
			proof_specs: vec![],
			repository: "".to_string(),
		}
	}
}

// #[cw_serde]
#[derive(Clone, PartialEq, Debug, Eq)]
pub struct ConsensusState {
	// #[schemars(with = "String")]
	// #[serde(with = "Base64", default)]
	pub data: Bytes,
	// #[schemars(with = "String")]
	// #[serde(with = "Base64", default)]
	pub code_id: Bytes,
	pub timestamp: u64,
	// pub root: MerkleRoot,
}

impl From<RawConsensusState> for ConsensusState {
	fn from(value: RawConsensusState) -> Self {
		Self { data: value.data, code_id: value.code_id, timestamp: 0 }
	}
}

impl From<ConsensusState> for RawConsensusState {
	fn from(value: ConsensusState) -> Self {
		Self { data: value.data, code_id: value.code_id, timestamp: 0, root: None }
	}
}

impl Protobuf<RawConsensusState> for ConsensusState {}
