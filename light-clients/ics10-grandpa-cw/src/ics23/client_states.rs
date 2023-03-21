use cosmwasm_std::Storage;
use ibc::core::ics24_host::identifier::ClientId;

/// client_id => client_states
/// trie key path: "clients/{client_id}/clientState"
pub struct ClientStates<'a>(&'a mut dyn Storage);

impl<'a> ClientStates<'a> {
	pub fn new(storage: &'a mut dyn Storage) -> Self {
		ClientStates(storage)
	}

	pub fn key(_client_id: ClientId) -> Vec<u8> {
		let client_state_path = format!("clientState");
		// let client_state_path = format!("{}", ClientStatePath(client_id));
		client_state_path.into_bytes()
	}

	pub fn get(&self, client_id: &ClientId) -> Option<Vec<u8>> {
		self.0.get(&Self::key(client_id.clone()))
	}

	pub fn insert(&mut self, client_id: ClientId, client_state: Vec<u8>) {
		self.0.set(&Self::key(client_id), &client_state);
	}

	pub fn contains_key(&self, client_id: &ClientId) -> bool {
		self.get(client_id).is_some()
	}
}

pub struct ReadonlyClientStates<'a>(&'a dyn Storage);

impl<'a> ReadonlyClientStates<'a> {
	pub fn new(storage: &'a dyn Storage) -> Self {
		ReadonlyClientStates(storage)
	}

	pub fn get(&self, client_id: &ClientId) -> Option<Vec<u8>> {
		self.0.get(&ClientStates::key(client_id.clone()))
	}

	pub fn contains_key(&self, client_id: &ClientId) -> bool {
		self.get(client_id).is_some()
	}
}
