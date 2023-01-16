#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

use alloc::{collections::BTreeMap, vec::Vec};
use ibc::core::ics02_client::{client_state::ClientType, events::CodeId};
use spin::RwLock;

pub mod client_def;
pub mod client_message;
pub mod client_state;
pub mod consensus_state;
pub mod msg;

pub type Bytes = Vec<u8>;

/// Hack to always be able to proto-deserialize `Any*::Wasm` variants from `protobuf::Any` type.
static WASM_CODE_ID_TO_CLIENT_TYPE: RwLock<BTreeMap<CodeId, ClientType>> =
	RwLock::new(BTreeMap::new());

// #[cfg(feature = "std")]
pub fn add_wasm_client_type(code_id: CodeId, client_type: ClientType) {
	let mut map = WASM_CODE_ID_TO_CLIENT_TYPE.write();
	map.insert(code_id, client_type);
}

// #[cfg(feature = "std")]
pub fn get_wasm_client_type(code_id: &CodeId) -> Option<ClientType> {
	let map = WASM_CODE_ID_TO_CLIENT_TYPE.read();
	map.get(code_id).cloned()
}
