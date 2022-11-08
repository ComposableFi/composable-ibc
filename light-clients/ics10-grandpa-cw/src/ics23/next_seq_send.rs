use crate::STORAGE_PREFIX;
use cosmwasm_std::Storage;
use cosmwasm_storage::{prefixed, prefixed_read, PrefixedStorage, ReadonlyPrefixedStorage};
use ibc::core::ics24_host::{
	identifier::{ChannelId, PortId},
	path::SeqSendsPath,
};
use sp_std::prelude::*;

/// (port_id, channel_id) => Sequence
/// trie key path: "nextSequenceSend/ports/{port_id}/channels/{channel_id}"
pub struct NextSequenceSend<'a>(PrefixedStorage<'a>);

impl<'a> NextSequenceSend<'a> {
	pub fn new(storage: &'a mut dyn Storage) -> Self {
		NextSequenceSend(prefixed(storage, STORAGE_PREFIX))
	}

	pub fn key(port_id: PortId, channel_id: ChannelId) -> Vec<u8> {
		let next_seq_send_path = format!("{}", SeqSendsPath(port_id, channel_id));
		next_seq_send_path.into_bytes()
	}

	pub fn get(&self, key: (PortId, ChannelId)) -> Option<u64> {
		let next_seq_send_path = Self::key(key.0, key.1);
		self.0.get(&next_seq_send_path).map(|_bytes| todo!())
	}

	pub fn insert(&mut self, key: (PortId, ChannelId), seq: u64) {
		let next_seq_send_path = Self::key(key.0, key.1);
		self.0.set(&next_seq_send_path, &seq.to_le_bytes()); // TODO
	}
}

/// (port_id, channel_id) => Sequence
/// trie key path: "nextSequenceSend/ports/{port_id}/channels/{channel_id}"
pub struct ReadonlyNextSequenceSend<'a>(ReadonlyPrefixedStorage<'a>);

impl<'a> ReadonlyNextSequenceSend<'a> {
	pub fn new(storage: &'a dyn Storage) -> Self {
		ReadonlyNextSequenceSend(prefixed_read(storage, STORAGE_PREFIX))
	}

	pub fn get(&self, key: (PortId, ChannelId)) -> Option<u64> {
		let next_seq_send_path = NextSequenceSend::key(key.0, key.1);
		self.0.get(&next_seq_send_path).map(|_bytes| todo!())
	}
}
