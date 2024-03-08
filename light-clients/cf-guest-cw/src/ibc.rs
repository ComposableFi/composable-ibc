//! A helper module which collects IBC types we’re using in a flatter namespace.

pub mod wasm {
	pub use ibc::clients::wasm_types::{
		client_state::ClientState, consensus_state::ConsensusState, error::Error,
	};
}

pub use ibc::{
	core::{
		client::{context::types::error::ClientError, types::Height},
		commitment_types::commitment::{CommitmentPrefix, CommitmentProofBytes},
		host::types::{identifiers::ClientId, path},
	},
	primitives::{proto, Timestamp},
};
