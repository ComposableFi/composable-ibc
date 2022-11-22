use crate::context::Context;
use grandpa_light_client_primitives::HostFunctions;
use ibc::{
	core::{
		ics03_connection::{
			connection::ConnectionEnd,
			context::{ConnectionKeeper, ConnectionReader},
			error::Error,
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ClientId, ConnectionId},
	},
	Height,
};

impl<'a, H: HostFunctions> ConnectionReader for Context<'a, H> {
	fn connection_end(&self, _conn_id: &ConnectionId) -> Result<ConnectionEnd, Error> {
		todo!()
	}

	fn host_oldest_height(&self) -> Height {
		todo!()
	}

	fn commitment_prefix(&self) -> CommitmentPrefix {
		todo!()
	}

	fn connection_counter(&self) -> Result<u64, Error> {
		todo!()
	}
}

impl<'a, H: HostFunctions> ConnectionKeeper for Context<'a, H> {
	fn store_connection(
		&mut self,
		_connection_id: ConnectionId,
		_connection_end: &ConnectionEnd,
	) -> Result<(), Error> {
		todo!()
	}

	fn store_connection_to_client(
		&mut self,
		_connection_id: ConnectionId,
		_client_id: &ClientId,
	) -> Result<(), Error> {
		todo!()
	}

	fn increase_connection_counter(&mut self) {
		todo!()
	}
}
