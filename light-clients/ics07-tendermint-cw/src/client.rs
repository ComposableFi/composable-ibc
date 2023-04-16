use crate::{
	context::Context,
	ics23::{
		ConsensusStates, FakeInner, ReadonlyClientStates, ReadonlyClients, ReadonlyConsensusStates, 
		ClientStates, ProcessedStates, ReadonlyProcessedStates,
	},
	//log,
};
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState as _,
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
use ibc_proto::google::protobuf::Any;
use ics07_tendermint::{
	client_def::TendermintClient,
	client_message::ClientMessage,
	client_state::ClientState,
	consensus_state::ConsensusState, HostFunctionsProvider,
};
use prost::Message;
use std::str::FromStr;

impl<'a, H: HostFunctionsProvider + 'static> ClientTypes for Context<'a, H> {
	type AnyClientMessage = ClientMessage;
	type AnyClientState = ClientState<H>;
	type AnyConsensusState = ConsensusState;
	type ClientDef = TendermintClient<H>;
}

impl<'a, H: HostFunctionsProvider + 'static> ClientReader for Context<'a, H> {
	fn client_type(&self, client_id: &ClientId) -> Result<ClientType, Error> {
		//log!(self, "in client : [client_type] >> client_id = {:?}", client_id);

		let clients = ReadonlyClients::new(self.storage());
		if !clients.contains_key(client_id) {
			//log!(self, "in client : [client_type] >> read client_type is None");
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
				//log!(self, "in client : [client_type] >> client_type : {:?}", val);
				Ok(val)
			},
		}
	}
	
	fn client_state(&self, client_id: &ClientId) -> Result<ClientState<H>, Error> {
		//log!(self, "in client : [client_state]");
		let client_states = ReadonlyClientStates::new(self.storage());
		let data = client_states
			.get()
			.ok_or_else(|| Error::client_not_found(client_id.clone()))?;
		/*let any = Any::decode(&*data).map_err(Error::decode)?;
		let wasm_state =
			ics08_wasm::client_state::ClientState::<FakeInner, FakeInner, FakeInner>::decode_vec(
				&any.value,
			)
			.map_err(|e| {
				Error::implementation_specific(format!(
					"[client_state]: error decoding client state bytes to WasmConsensusState {}",
					e
				))
			})?;
		let any = Any::decode(&*wasm_state.data).map_err(|e| Error::decode(e))?;
		let state =
			ClientState::<H>::decode_vec(&*any.value).map_err(Error::invalid_any_client_state)?;*/
		let state = Self::decode_client_state(&data)?;
		//log!(self, "in client : [client_state] >> any client_state: {:?}", state);
		Ok(state)
	}

	fn consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<ConsensusState, Error> {
		/*log!(
			self,
			"in client : [consensus_state] >> client_id = {:?}, height = {:?}",
			client_id,
			height
		);*/

		let consensus_states = ReadonlyConsensusStates::new(self.storage());
		let value = consensus_states
			.get(height)
			.ok_or_else(|| Error::consensus_state_not_found(client_id.clone(), height))?;
		/*log!(
			self,
			"in client : [consensus_state] >> consensus_state (raw): {}",
			hex::encode(&value)
		);*/
		/*let any = Any::decode(&mut &value[..]).map_err(Error::decode)?;
		let wasm_consensus_state =
			ics08_wasm::consensus_state::ConsensusState::<FakeInner>::decode_vec(&*any.value)
				.map_err(Error::invalid_any_consensus_state)?;
		let any = Any::decode(&mut &wasm_consensus_state.data[..]).map_err(Error::decode)?;
		let any_consensus_state =
			ConsensusState::decode_vec(&*any.value).map_err(Error::invalid_any_consensus_state)?;*/
		let any_consensus_state = Self::decode_consensus_state(&value)?;
		/*log!(
			self,
			"in client : [consensus_state] >> any consensus state = {:?}",
			any_consensus_state
		);*/
		Ok(any_consensus_state)
	}

	fn host_client_type(&self) -> String {
		"ics07-tendermint".to_string()
	}

	/// Similar to `consensus_state`, attempt to retrieve the consensus state,
	/// but return `None` if no state exists at the given height.
	/*fn maybe_consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<Option<ConsensusState>, Error> {
		unimplemented!()
		//match self.consensus_state(client_id, height, &mut Vec::new()) {
		//	Ok(cs) => Ok(Some(cs)),
		//	Err(e) => match e.detail() {
		//		ErrorDetail::ConsensusStateNotFound(_) => Ok(None),
		//		_ => Err(e),
		//	},
		//}
	}*/

	fn next_consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<Option<ConsensusState>, Error> {

		let processed_state = ReadonlyProcessedStates::new(self.storage());
		match processed_state.get_next_height(height) {
			Some(next_height) => {
				self.consensus_state(&client_id.clone(), next_height)
				.and_then(|cs| {
					Ok(Some(cs))
				})
			},
			None => Ok(None)
		}
	}

	fn prev_consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<Option<ConsensusState>, Error> {
		let processed_state = ReadonlyProcessedStates::new(self.storage());
		match processed_state.get_prev_height(height) {
			Some(prev_height) => {
				self.consensus_state(&client_id.clone(), prev_height)
				.and_then(|cs| {
					Ok(Some(cs))
				})

			},
			None => Ok(None)
		}
	}

	fn host_height(&self) -> Height {
		//log!(self, "in client: [host_height]");
		Height::new(0, self.env.block.height)
	}

	fn host_timestamp(&self) -> Timestamp {
		let time = self.env.block.time;
		Timestamp::from_nanoseconds(time.nanos()).expect("invalid timestamp")
	}

	fn host_consensus_state(
		&self,
		_height: Height,
		_proof: Option<Vec<u8>>,
		_client_state: &ClientState<H>,
	) -> Result<ConsensusState, Error> {
		unimplemented!()
		/*let consensus_state =
			HOST_CONSENSUS_STATE.load(self.storage(), height.revision_height).map_err(|_| {
				Error::implementation_specific(format!(
					"[host_consensus_state]: consensus state not found for host at height {}",
					height
				))
			})?;
		Ok(consensus_state)*/
	}

	fn processed_timestamp(
		&self,
		height: Height,
	) -> Result<u64, Error> {

		let processed_state = ReadonlyProcessedStates::new(self.storage());
		match processed_state.get_processed_time(height, &mut Vec::new()) {
			Some(time) => Ok(time),
			None => ibc::prelude::Err(Error::implementation_specific(format!(
				"problem getting processed timestamp"
			))),
		}
	}

	fn processed_height(
		&self,
		height: Height,
	) -> Result<u64, Error> {
		
		let processed_state = ReadonlyProcessedStates::new(self.storage());
		match processed_state.get_processed_height(height, &mut Vec::new()) {
			Some(p_height) => Ok(p_height),
			None => ibc::prelude::Err(Error::implementation_specific(format!(
				"problem getting processed height"
			))),
		}
	}

	fn client_counter(&self) -> Result<u64, Error> {
		unimplemented!()
		/*let count = CLIENT_COUNTER.load(self.storage()).unwrap_or_default();
		log!(self, "in client : [client_counter] >> client_counter: {:?}", count);
		Ok(count as u64)*/
	}
}

impl<'a, H: HostFunctionsProvider + 'static> ClientKeeper for Context<'a, H> {
	fn store_client_type(
		&mut self,
		_client_id: ClientId,
		_client_type: ClientType,
	) -> Result<(), Error> {
		unimplemented!()
	}

	fn store_client_state(
		&mut self,
		client_id: ClientId,
		client_state: Self::AnyClientState,
	) -> Result<(), Error> {
		//log!(self, "in client : [store_client_state]");
		let client_states = ReadonlyClientStates::new(self.storage());
		let data = client_states
			.get()
			.ok_or_else(|| Error::client_not_found(client_id.clone()))?;
		/*let any = Any::decode(&*data).map_err(Error::decode)?;
		let mut wasm_client_state =
			ics08_wasm::client_state::ClientState::<FakeInner, FakeInner, FakeInner>::decode_vec(
				&any.value,
			)
			.map_err(|e| {
				Error::implementation_specific(format!(
					"[client_state]: error decoding client state bytes to WasmConsensusState {}",
					e
				))
			})?;
		wasm_client_state.data = client_state.to_any().encode_to_vec();
		wasm_client_state.latest_height = client_state.latest_height().into();
		let vec1 = wasm_client_state.to_any().encode_to_vec();*/
		let encoded = Self::encode_client_state(client_state, data)?;
		//log!(self, "in client : [store_client_state] >> wasm client state (raw)");
		let mut client_state_storage = ClientStates::new(self.storage_mut());
		client_state_storage.insert(encoded);
		//client_state_storage.insert(vec1);
		Ok(())
	}

	fn store_consensus_state(
		&mut self,
		client_id: ClientId,
		height: Height,
		consensus_state: Self::AnyConsensusState,
	) -> Result<(), Error> {
		/*log!(
			self,
			"in client : [store_consensus_state] >> client_id = {:?}, height = {:?}",
			client_id,
			height,
		);*/

		/*let wasm_consensus_state = ics08_wasm::consensus_state::ConsensusState {
			data: consensus_state.to_any().encode_to_vec(),
			timestamp: consensus_state.timestamp().nanoseconds(),
			inner: Box::new(FakeInner),
		};
		let vec1 = wasm_consensus_state.to_any().encode_to_vec();*/
		let encoded = Self::encode_consensus_state(consensus_state);
		/*log!(
			self,
			"in client : [store_consensus_state] >> wasm consensus state (raw) = {}",
			hex::encode(&vec1)
		);*/
		let mut consensus_states = ConsensusStates::new(self.storage_mut());
		consensus_states.insert(height, encoded);
		//consensus_states.insert(height, vec1);

		self.store_update_time(client_id.clone(), height, self.host_timestamp())?;
		self.store_update_height(client_id, height, self.host_height())?;

		Ok(())
	}

	fn increase_client_counter(&mut self) {
		unimplemented!()
	}

	fn store_update_time(
		&mut self,
		_client_id: ClientId,
		height: Height,
		timestamp: Timestamp,
	) -> Result<(), Error> {
		let mut processed_state = ProcessedStates::new(self.storage_mut());
		processed_state.set_processed_time(height, timestamp.nanoseconds(), &mut Vec::new());
		
		Ok(())
	}

	fn store_update_height(
		&mut self,
		_client_id: ClientId,
		height: Height,
		host_height: Height,
	) -> Result<(), Error> {
		
		let mut processed_state = ProcessedStates::new(self.storage_mut());
		processed_state.set_processed_height(height, host_height.revision_height, &mut Vec::new());
		processed_state.set_iteration_key(height, &mut Vec::new());
		Ok(())
	}

	fn validate_self_client(&self, _client_state: &Self::AnyClientState) -> Result<(), Error> {
		unimplemented!()
	}
}

impl<'a, H: Clone> Context<'a, H> {
	pub fn decode_client_state(data: &[u8]) -> Result<ClientState<H>, Error> {
		let any = Any::decode(data).map_err(Error::decode)?;
		let wasm_state =
			ics08_wasm::client_state::ClientState::<FakeInner, FakeInner, FakeInner>::decode_vec(
				&any.value,
			)
			.map_err(|e| {
				Error::implementation_specific(format!(
					"[client_state]: error decoding client state bytes to WasmConsensusState {}",
					e
				))
			})?;
		let any = Any::decode(&*wasm_state.data).map_err(|e| Error::decode(e))?;
		let state =
			ClientState::<H>::decode_vec(&*any.value).map_err(Error::invalid_any_client_state)?;
		Ok(state)
	}

	pub fn decode_consensus_state(value: &[u8]) -> Result<ConsensusState, Error> {
		let any = Any::decode(&mut &*value).map_err(Error::decode)?;
		let wasm_consensus_state =
			ics08_wasm::consensus_state::ConsensusState::<FakeInner>::decode_vec(&*any.value)
				.map_err(Error::invalid_any_consensus_state)?;
		let any = Any::decode(&mut &wasm_consensus_state.data[..]).map_err(Error::decode)?;
		let any_consensus_state =
			ConsensusState::decode_vec(&*any.value).map_err(Error::invalid_any_consensus_state)?;
		Ok(any_consensus_state)
	}

	pub fn encode_client_state(
		client_state: ClientState<H>,
		encoded_wasm_client_state: Vec<u8>,
	) -> Result<Vec<u8>, Error> {
		let any = Any::decode(&*encoded_wasm_client_state).map_err(Error::decode)?;
		let mut wasm_client_state =
			ics08_wasm::client_state::ClientState::<FakeInner, FakeInner, FakeInner>::decode_vec(
				&any.value,
			)
			.map_err(|e| {
				Error::implementation_specific(format!(
					"[client_state]: error decoding client state bytes to WasmConsensusState {}",
					e
				))
			})?;
		wasm_client_state.data = client_state.to_any().encode_to_vec();
		wasm_client_state.latest_height = client_state.latest_height().into();
		let vec1 = wasm_client_state.to_any().encode_to_vec();
		Ok(vec1)
	}

	pub fn encode_consensus_state(consensus_state: ConsensusState) -> Vec<u8> {
		let wasm_consensus_state = ics08_wasm::consensus_state::ConsensusState {
			data: consensus_state.to_any().encode_to_vec(),
			timestamp: consensus_state.timestamp().nanoseconds(),
			inner: Box::new(FakeInner),
		};
		wasm_consensus_state.to_any().encode_to_vec()
	}
}
