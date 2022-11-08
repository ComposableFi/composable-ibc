use crate::{
	context::Context,
	contract::{CONNECTION_COUNTER, CONNECTION_PREFIX},
	ics23::ReadonlyConnections,
	log,
};
use grandpa_light_client_primitives::HostFunctions;
use ibc::{
	core::{
		ics03_connection::{connection::ConnectionEnd, context::ConnectionReader, error::Error},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::ConnectionId,
	},
	protobuf::Protobuf,
	Height,
};

impl<'a, H: HostFunctions> ConnectionReader for Context<'a, H> {
	fn connection_end(&self, conn_id: &ConnectionId) -> Result<ConnectionEnd, Error> {
		log!(self, "in connection : [connection_end] >> connection_id = {:?}", conn_id);

		let data = ReadonlyConnections::new(self.storage())
			.get(conn_id)
			.ok_or_else(|| Error::connection_not_found(conn_id.clone()))?;
		let conn = ConnectionEnd::decode_vec(&*data).map_err(|e| {
			Error::implementation_specific(format!(
				"[connection_end]: error decoding connection end bytes{}",
				e
			))
		})?;
		log!(self, "in connection : [connection_end] >>  connection_end = {:?}", conn);
		Ok(conn)
	}

	fn host_oldest_height(&self) -> Height {
		#[cfg(not(any(test, feature = "wasm_test")))]
		unreachable!("the method should be removed in the future");
		#[cfg(any(test, feature = "wasm_test"))]
		Height::new(0, 0)
	}

	#[allow(clippy::disallowed_methods)]
	fn commitment_prefix(&self) -> CommitmentPrefix {
		log!(self, "in connection : [commitment_prefix] >> CommitmentPrefix = {:?}", "ibc");
		CONNECTION_PREFIX
			.load(self.storage())
			.unwrap_or_default()
			.try_into()
			.map_err(|_| panic!("Connection prefix supplied in the config is invalid"))
			.unwrap()
	}

	fn connection_counter(&self) -> Result<u64, Error> {
		let count = CONNECTION_COUNTER.load(self.storage()).unwrap_or_default();
		log!(self, "in connection : [connection_counter] >> Connection_counter = {:?}", count);
		Ok(count as u64)
	}
}
