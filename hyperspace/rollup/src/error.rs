use ibc::timestamp::ParseTimestampError;
use prost::DecodeError;

/// Error definitions for the cosmos client in accordance with the parachain's Error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
	/// An error from the rpc interface
	#[error("Rpc client error: {0}")]
	RpcError(String),
	/// Custom error
	#[error("{0}")]
	Custom(String),
	/// Decode error
	#[error("Decode error: {0}")]
	DecodeError(#[from] DecodeError),
	/// Encode error
	#[error("Encode error: {0}")]
	EncodeError(#[from] prost::EncodeError),
	/// Parse timestamp error
	#[error("Parse timestamp error: {0}")]
	ParseTimestampError(#[from] ParseTimestampError),
	/// Transfer error
	#[error("IBC transfer error: {0}")]
	TransferError(#[from] ibc::applications::transfer::error::Error),
	/// Tendermint error
	#[error("Tendermint error: {0}")]
	TendermintError(#[from] tendermint::Error),
}

impl From<String> for Error {
	fn from(error: String) -> Self {
		Self::Custom(error)
	}
}
