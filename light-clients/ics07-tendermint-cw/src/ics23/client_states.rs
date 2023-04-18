use cosmwasm_std::Storage;

/// client_id => client_states
/// trie key path: "clients/{client_id}/clientState"
pub struct ClientStates<'a>(&'a mut dyn Storage);

impl<'a> ClientStates<'a> {
	pub fn new(storage: &'a mut dyn Storage) -> Self {
		ClientStates(storage)
	}

	pub fn key() -> Vec<u8> {
		let client_state_path = format!("clientState");
		client_state_path.into_bytes()
	}

	pub fn get(&self) -> Option<Vec<u8>> {
		ReadonlyClientStates::new(self.0).get()
	}

	pub fn get_prefixed(&self, prefix: &[u8]) -> Option<Vec<u8>> {
		ReadonlyClientStates::new(self.0).get_prefixed(prefix)
	}

	pub fn insert(&mut self, client_state: Vec<u8>) {
		self.0.set(&Self::key(), &client_state);
	}

	pub fn insert_prefixed(&mut self, client_state: Vec<u8>, prefix: &[u8]) {
		self.0.set(&[prefix, Self::key().as_slice()].concat(), &client_state);
	}

	pub fn contains_key(&self) -> bool {
		self.get().is_some()
	}
}

pub struct ReadonlyClientStates<'a>(&'a dyn Storage);

impl<'a> ReadonlyClientStates<'a> {
	pub fn new(storage: &'a dyn Storage) -> Self {
		ReadonlyClientStates(storage)
	}

	pub fn get(&self) -> Option<Vec<u8>> {
		self.0.get(&ClientStates::key())
	}

	pub fn get_prefixed(&self, prefix: &[u8]) -> Option<Vec<u8>> {
		self.0.get(&[prefix, ClientStates::key().as_slice()].concat())
	}

	pub fn contains_key(&self) -> bool {
		self.get().is_some()
	}
}
