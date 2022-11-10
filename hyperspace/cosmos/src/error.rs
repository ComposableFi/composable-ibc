use ibc::{core::ics02_client, timestamp::ParseTimestampError};
use std::num::ParseIntError;
use thiserror::Error;

/// Error definitions for the cosmos client in accordance with the parachain's Error type.
#[derive(Error, Debug)]
pub enum Error {
	/// An error from the rpc interface
	#[error("Rpc client error: {0}")]
	RpcError(String),
	/// Custom error
	#[error("{0}")]
	Custom(String),

}

impl From<String> for Error {
	fn from(error: String) -> Self {
		Self::Custom(error)
	}
}
