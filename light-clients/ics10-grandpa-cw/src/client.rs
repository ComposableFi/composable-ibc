use crate::context::Context;
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
	timestamp::Timestamp,
	Height,
};
use ics10_grandpa::{
	client_def::GrandpaClient, client_message::ClientMessage, client_state::ClientState,
	consensus_state::ConsensusState,
};

impl<'a, H: HostFunctions> ClientTypes for Context<'a, H> {
	type AnyClientMessage = ClientMessage;
	type AnyClientState = ClientState<H>;
	type AnyConsensusState = ConsensusState;
	type ClientDef = GrandpaClient<H>;
}

impl<'a, H: HostFunctions> ClientReader for Context<'a, H> {
	fn client_type(&self, _client_id: &ClientId) -> Result<ClientType, Error> {
		todo!()
	}

	fn client_state(&self, _client_id: &ClientId) -> Result<Self::AnyClientState, Error> {
		todo!()
	}

	fn consensus_state(
		&self,
		_client_id: &ClientId,
		_height: Height,
	) -> Result<Self::AnyConsensusState, Error> {
		todo!()
	}

	fn host_client_type(&self) -> String {
		todo!()
	}

	fn next_consensus_state(
		&self,
		_client_id: &ClientId,
		_height: Height,
	) -> Result<Option<Self::AnyConsensusState>, Error> {
		todo!()
	}

	fn prev_consensus_state(
		&self,
		_client_id: &ClientId,
		_height: Height,
	) -> Result<Option<Self::AnyConsensusState>, Error> {
		todo!()
	}

	fn host_height(&self) -> Height {
		todo!()
	}

	fn host_timestamp(&self) -> Timestamp {
		todo!()
	}

	fn host_consensus_state(
		&self,
		_height: Height,
		_proof: Option<Vec<u8>>,
	) -> Result<Self::AnyConsensusState, Error> {
		todo!()
	}

	fn client_counter(&self) -> Result<u64, Error> {
		todo!()
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
