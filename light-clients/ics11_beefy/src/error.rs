use ibc::prelude::*;

use ibc::{
	core::{ics02_client::error::Error as Ics02Error, ics24_host::error::ValidationError},
	timestamp::TimestampOverflowError,
};

use crate::client_state::ClientState;

#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum Error {
	Codec(codec::Error),
	TimeStamp(TimestampOverflowError),
	ValidationError(ValidationError),
}

impl From<Error> for Ics02Error {
	fn from(e: Error) -> Self {
		Ics02Error::client_error(ClientState::<()>::client_type().to_owned(), e.to_string())
	}
}
