use cosmwasm_std::Storage;

/// client_id => client_states
/// trie key path: "clients/{client_id}/clientState"
pub struct ClientStates<'a>(&'a mut dyn Storage);

impl<'a> ClientStates<'a> {
	pub fn new(storage: &'a mut dyn Storage) -> Self {
		ClientStates(storage)
	}

	pub fn key(prefix: &mut Vec<u8>) -> Vec<u8> {
		let client_state_path = format!("clientState");
		prefix.append(&mut client_state_path.into_bytes());
		prefix.clone()
	}

	pub fn get(&self, prefix: &mut Vec<u8>) -> Option<Vec<u8>> {
		self.0.get(&Self::key(prefix))
	}

	pub fn insert(&mut self, prefix: &mut Vec<u8>, client_state: Vec<u8>) {
		self.0.set(&Self::key(prefix), &client_state);
	}

	pub fn contains_key(&self, prefix: &mut Vec<u8>) -> bool {
		self.get(prefix).is_some()
	}
}

pub struct ReadonlyClientStates<'a>(&'a dyn Storage);

impl<'a> ReadonlyClientStates<'a> {
	pub fn new(storage: &'a dyn Storage) -> Self {
		ReadonlyClientStates(storage)
	}

	pub fn get(&self, prefix: &mut Vec<u8>) -> Option<Vec<u8>> {
		self.0.get(&ClientStates::key(prefix))
	}

	pub fn contains_key(&self, prefix: &mut Vec<u8>) -> bool {
		self.get(prefix).is_some()
	}
}