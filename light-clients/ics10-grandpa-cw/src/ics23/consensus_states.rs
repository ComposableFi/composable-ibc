use crate::STORAGE_PREFIX;
use cosmwasm_std::Storage;
use cosmwasm_storage::{prefixed, PrefixedStorage};
use ibc::{core::ics24_host::identifier::ClientId, Height};

/// client_id, height => consensus_state
/// trie key path: "clients/{client_id}/consensusStates/{height}"
pub struct ConsensusStates<'a>(PrefixedStorage<'a>);

impl<'a> ConsensusStates<'a> {
	pub fn new(storage: &'a mut dyn Storage) -> Self {
		ConsensusStates(prefixed(storage, STORAGE_PREFIX))
	}

	pub fn consensus_state_client_key(client_id: ClientId) -> Vec<u8> {
		format!("clients/{}/consensusStates/", client_id).into_bytes()
	}

	pub fn consensus_state_height_key(height: Height) -> Vec<u8> {
		format!("{}", height).into_bytes()
	}

	pub fn consensus_state_key(client_id: ClientId, height: Height) -> (Vec<u8>, Vec<u8>) {
		let client_id_key = Self::consensus_state_client_key(client_id);
		let height_key = Self::consensus_state_height_key(height);
		(client_id_key, height_key)
	}

	pub fn get(&self, client_id: &ClientId, height: Height) -> Option<Vec<u8>> {
		let (consensus_state_key_1, consensus_state_key_2) =
			Self::consensus_state_key(client_id.clone(), height);
		let full_key =
			[consensus_state_key_1.as_slice(), consensus_state_key_2.as_slice()].concat();
		self.0.get(&full_key)
	}

	pub fn insert(&mut self, _client_id: ClientId, _height: Height, _consensus_state: Vec<u8>) {
		todo!("insert")
	}
}
