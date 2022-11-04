use ibc::{core::ics02_client, timestamp::ParseTimestampError};
use std::num::ParseIntError;
use thiserror::Error;

/// Error definitions for the cosmos client in accordance with the parachain's Error type.
#[derive(Error, Debug)]
pub enum Error {
	/// An error from the rpc interface
	#[error("Rpc client error: {0}")]
	RpcError(String),
	/// Update module name in call definition
	#[error("Module '{0}' not found in metadata, update static definition of call")]
	ModuleNotFound(&'static str),
	/// Call not found, update function name in call definition
	#[error("Call '{0}' not found in metadata, update static definition of call")]
	CallNotFound(&'static str),
	/// Custom error
	#[error("{0}")]
	Custom(String),
	#[error("Ibc channel error")]
	IbcChannel(#[from] ibc::core::ics04_channel::error::Error),
	/// Error querying packets
	#[error("Could not retrieve packets from {channel_id}/{port_id} for sequences {:?}", .sequences)]
	QueryPackets { channel_id: String, port_id: String, sequences: Vec<u64>, err: String },
	/// Failed to rehydrate client state
	#[error("Error decoding some value: {0}")]
	ClientStateRehydration(String),
	/// Failed to get client update header from finality notification
	#[error("Error constructing a client update header: {0}")]
	HeaderConstruction(String),
	/// Errors associated with ics-02 client
	#[error("Ibc client error: {0}")]
	IbcClient(#[from] ics02_client::error::Error),
	/// Ics-20 errors
	#[error("Ics-20 error: {0}")]
	Ics20Error(#[from] ibc::applications::transfer::error::Error),
	/// parse error
	#[error("Failed to parse block numbers: {0}")]
	ParseIntError(#[from] ParseIntError),
	/// Error occured parsing timestamp
	#[error("Timestamp error: {0}")]
	ParseTimestamp(#[from] ParseTimestampError),
	#[error("Jsonrpsee error: {0}")]
	JosnrpseeError(#[from] jsonrpsee::core::Error),
}

impl From<String> for Error {
	fn from(error: String) -> Self {
		Self::Custom(error)
	}
}
