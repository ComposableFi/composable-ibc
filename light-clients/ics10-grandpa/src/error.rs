use crate::client_state::ClientState;
use alloc::{
	borrow::ToOwned,
	string::{String, ToString},
};
use ibc::{
	core::{ics02_client, ics04_channel, ics24_host::error::ValidationError},
	timestamp::{ParseTimestampError, TimestampOverflowError},
};
use prost::DecodeError;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(derive_more::From, derive_more::Display)]
pub enum Error {
	Codec(codec::Error),
	TimeStamp(TimestampOverflowError),
	ParseTimeStamp(ParseTimestampError),
	ValidationError(ValidationError),
	Ics02(ics02_client::error::Error),
	Ics04(ics04_channel::error::Error),
	ProtoBuf(DecodeError),
	GrandpaPrimitives(grandpa_client_primitives::error::Error),
	Anyhow(anyhow::Error),
	Custom(String),
}

impl From<Error> for ics02_client::error::Error {
	fn from(e: Error) -> Self {
		ics02_client::error::Error::client_error(
			ClientState::<()>::client_type().to_owned(),
			e.to_string(),
		)
	}
}
