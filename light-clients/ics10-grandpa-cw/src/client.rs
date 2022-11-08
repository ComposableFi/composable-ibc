use crate::{
	context::Context,
	contract::{CLIENT_COUNTER, CONSENSUS_STATES_HEIGHTS, HOST_CONSENSUS_STATE},
	ics23::{ReadonlyClientStates, ReadonlyClients, ReadonlyConsensusStates},
	log,
};
use grandpa_light_client_primitives::HostFunctions;
use ibc::{
	core::{
		ics02_client::{
			client_state::ClientType,
			context::{ClientKeeper, ClientReader, ClientTypes},
			error::Error,
		},
		ics24_host::identifier::ClientId,
	},
	protobuf::Protobuf,
	timestamp::Timestamp,
	Height,
};
use ics10_grandpa::{
	client_def::GrandpaClient, client_message::ClientMessage, client_state::ClientState,
	consensus_state::ConsensusState,
};
use light_client_common::LocalHeight;
use std::str::FromStr;

impl<'a, H: HostFunctions> ClientTypes for Context<'a, H> {
	type AnyClientMessage = ClientMessage;
	type AnyClientState = ClientState<H>;
	type AnyConsensusState = ConsensusState;
	type ClientDef = GrandpaClient<H>;
}

impl<'a, H: HostFunctions> ClientReader for Context<'a, H> {
	fn client_type(&self, client_id: &ClientId) -> Result<ClientType, Error> {
		log!(self, "in client : [client_type] >> client_id = {:?}", client_id);

		let clients = ReadonlyClients::new(self.storage());
		if !clients.contains_key(client_id) {
			log!(self, "in client : [client_type] >> read client_type is None");
			return Err(Error::client_not_found(client_id.clone()))
		}

		let data = clients
			.get(client_id)
			.ok_or_else(|| Error::client_not_found(client_id.clone()))?;
		let data = String::from_utf8(data).map_err(|e| {
			Error::implementation_specific(format!(
				"[client_type]: error decoding client type bytes to string {}",
				e
			))
		})?;
		match ClientType::from_str(&data) {
			Err(_err) => Err(Error::unknown_client_type(data.to_string())),
			Ok(val) => {
				log!(self, "in client : [client_type] >> client_type : {:?}", val);
				Ok(val)
			},
		}
	}

	fn client_state(&self, client_id: &ClientId) -> Result<ClientState<H>, Error> {
		log!(self, "in client : [client_state] >> client_id = {:?}", client_id);
		let client_states = ReadonlyClientStates::new(self.storage());
		let data = client_states
			.get(client_id)
			.ok_or_else(|| Error::client_not_found(client_id.clone()))?;
		let state =
			ClientState::<H>::decode_vec(&*data).map_err(Error::invalid_any_client_state)?;
		log!(self, "in client : [client_state] >> any client_state: {:?}", state);
		Ok(state)
	}

	fn consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<ConsensusState, Error> {
		log!(
			self,
			"in client : [consensus_state] >> client_id = {:?}, height = {:?}",
			client_id,
			height
		);

		let consensus_states = ReadonlyConsensusStates::new(self.storage());
		let value = consensus_states
			.get(client_id, height)
			.ok_or_else(|| Error::consensus_state_not_found(client_id.clone(), height))?;

		let any_consensus_state =
			ConsensusState::decode_vec(&*value).map_err(Error::invalid_any_client_state)?;
		log!(
			self,
			"in client : [consensus_state] >> any consensus state = {:?}",
			any_consensus_state
		);
		Ok(any_consensus_state)
	}

	fn host_client_type(&self) -> String {
		unimplemented!()
	}

	fn next_consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<Option<ConsensusState>, Error> {
		let from_height = LocalHeight::from(height).succ();
		CONSENSUS_STATES_HEIGHTS
			.load(self.storage(), client_id.as_bytes().to_owned())
			.unwrap_or_default()
			.range(from_height..)
			.next()
			.map(|height| self.consensus_state(client_id, Height::from(*height)))
			.transpose()
	}

	fn prev_consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<Option<ConsensusState>, Error> {
		CONSENSUS_STATES_HEIGHTS
			.load(self.storage(), client_id.as_bytes().to_owned())
			.unwrap_or_default()
			.range(..LocalHeight::from(height))
			.rev()
			.next()
			.map(|height| self.consensus_state(client_id, Height::from(*height)))
			.transpose()
	}

	fn host_height(&self) -> Height {
		log!(self, "in client: [host_height]");
		Height::new(self.env.block.height, 0)
	}

	fn host_timestamp(&self) -> Timestamp {
		let time = self.env.block.time;
		Timestamp::from_nanoseconds(time.nanos())
			.map_err(|e| panic!("{:?}, caused by {:?} from contract-ibc", e, time))
			.unwrap()
	}

	fn host_consensus_state(
		&self,
		height: Height,
		_proof: Option<Vec<u8>>,
	) -> Result<ConsensusState, Error> {
		let consensus_state =
			HOST_CONSENSUS_STATE.load(self.storage(), height.revision_height).map_err(|_| {
				Error::implementation_specific(format!(
					"[host_consensus_state]: consensus state not found for host at height {}",
					height
				))
			})?;
		Ok(consensus_state)
	}

	fn client_counter(&self) -> Result<u64, Error> {
		let count = CLIENT_COUNTER.load(self.storage()).unwrap_or_default();
		log!(self, "in client : [client_counter] >> client_counter: {:?}", count);
		Ok(count as u64)
	}
}

impl<'a, H: HostFunctions> ClientKeeper for Context<'a, H> {
	fn store_client_type(
		&mut self,
		_client_id: ClientId,
		_client_type: ClientType,
	) -> Result<(), Error> {
		todo!()
	}

	fn store_client_state(
		&mut self,
		_client_id: ClientId,
		_client_state: Self::AnyClientState,
	) -> Result<(), Error> {
		todo!()
	}

	fn store_consensus_state(
		&mut self,
		_client_id: ClientId,
		_height: Height,
		_consensus_state: Self::AnyConsensusState,
	) -> Result<(), Error> {
		todo!()
	}

	fn increase_client_counter(&mut self) {
		todo!()
	}

	fn store_update_time(
		&mut self,
		_client_id: ClientId,
		_height: Height,
		_timestamp: Timestamp,
	) -> Result<(), Error> {
		todo!()
	}

	fn store_update_height(
		&mut self,
		_client_id: ClientId,
		_height: Height,
		_host_height: Height,
	) -> Result<(), Error> {
		todo!()
	}

	fn validate_self_client(&self, _client_state: &Self::AnyClientState) -> Result<(), Error> {
		todo!()
	}
}
