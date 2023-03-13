use crate::context::Context;
use grandpa_light_client_primitives::HostFunctions;
use ibc::{
	core::{
		ics03_connection::{connection::ConnectionEnd, context::ConnectionReader, error::Error},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::ConnectionId,
	},
	Height,
};
use std::time::Duration;

impl<'a, H: HostFunctions> ConnectionReader for Context<'a, H> {
	fn minimum_delay_period(&self) -> Duration {
		unimplemented!("minimum_delay_period")
	}

	fn connection_end(&self, _conn_id: &ConnectionId) -> Result<ConnectionEnd, Error> {
		Err(Error::implementation_specific(
			"'connection_end' is unavailable from the client".to_string(),
		))
	}

	fn host_oldest_height(&self) -> Height {
		unimplemented!("the method should be removed in the future");
	}

	#[allow(clippy::disallowed_methods)]
	fn commitment_prefix(&self) -> CommitmentPrefix {
		unimplemented!("'commitment_prefix' is unavailable from the client");
	}

	fn connection_counter(&self) -> Result<u64, Error> {
		Err(Error::implementation_specific(
			"'connection_counter' is unavailable from the client".to_string(),
		))
	}
}
