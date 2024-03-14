//! A helper module which collects IBC types we’re using in a flatter namespace.

// pub mod wasm {
// 	pub use ibc_new::clients::wasm_types::{
// 		client_state::ClientState, consensus_state::ConsensusState, error::Error,
// 	};
// }

pub use ibc::{
	core::{
		ics02_client::error::Error as ClientError,
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes},
		ics24_host::{identifier::ClientId, path},
	},
	protobuf,
	timestamp::Timestamp,
	Height,
};

pub use ibc_proto as proto;
