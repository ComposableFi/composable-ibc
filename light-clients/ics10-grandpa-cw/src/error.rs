use cosmwasm_std::StdError;
use ics10_grandpa::error::Error as GrandpaError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),
	#[error("Unauthorized")]
	Unauthorized {},
	// Add any other custom errors you like here.
	// Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
	#[error("Storage error")]
	StorageError,
	#[error("Grandpa error: {0}")]
	Grandpa(String),
	#[error("Protobuf error: {0}")]
	Protobuf(#[from] ibc::protobuf::Error),
	#[error("IBC validation error: {0}")]
	Validation(#[from] ibc::core::ics24_host::error::ValidationError),
	#[error("IBC path error: {0}")]
	Path(#[from] ibc::core::ics24_host::path::PathError),
	#[error("IBC proof error: {0}")]
	Proof(#[from] ibc::proofs::ProofError),
}

impl From<GrandpaError> for ContractError {
	fn from(e: GrandpaError) -> Self {
		ContractError::Grandpa(e.to_string())
	}
}
