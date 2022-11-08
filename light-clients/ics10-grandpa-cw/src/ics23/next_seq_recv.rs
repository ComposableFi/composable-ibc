use crate::STORAGE_PREFIX;
use cosmwasm_std::Storage;
use cosmwasm_storage::{prefixed, prefixed_read, PrefixedStorage, ReadonlyPrefixedStorage};
use ibc::core::ics24_host::{
	identifier::{ChannelId, PortId},
	path::SeqRecvsPath,
};

/// (port_id, channel_id) => Sequence
/// trie key path: "nextSequenceRecv/ports/{port_id}/channels/{channel_id}"
pub struct NextSequenceRecv<'a>(PrefixedStorage<'a>);

impl<'a> NextSequenceRecv<'a> {
	pub fn new(storage: &'a mut dyn Storage) -> Self {
		NextSequenceRecv(prefixed(storage, STORAGE_PREFIX))
	}

	pub fn key(port_id: PortId, channel_id: ChannelId) -> Vec<u8> {
		let next_seq_recv_path = SeqRecvsPath(port_id, channel_id);
		let next_seq_recv_path = format!("{}", next_seq_recv_path);
		next_seq_recv_path.into_bytes()
	}

	pub fn get(&self, key: (PortId, ChannelId)) -> Option<u64> {
		let next_seq_recv_path = Self::key(key.0, key.1);
		self.0.get(&next_seq_recv_path).map(|_bytes| todo!())
	}

	pub fn insert(&mut self, key: (PortId, ChannelId), seq: u64) {
		let next_seq_recv_path = Self::key(key.0, key.1);
		self.0.set(&next_seq_recv_path, &seq.to_le_bytes()); // TODO
	}
}

/// (port_id, channel_id) => Sequence
/// trie key path: "nextSequenceRecv/ports/{port_id}/channels/{channel_id}"
pub struct ReadonlyNextSequenceRecv<'a>(ReadonlyPrefixedStorage<'a>);

impl<'a> ReadonlyNextSequenceRecv<'a> {
	pub fn new(storage: &'a dyn Storage) -> Self {
		ReadonlyNextSequenceRecv(prefixed_read(storage, STORAGE_PREFIX))
	}

	pub fn get(&self, key: (PortId, ChannelId)) -> Option<u64> {
		let next_seq_recv_path = NextSequenceRecv::key(key.0, key.1);
		self.0.get(&next_seq_recv_path).map(|_bytes| todo!())
	}
}
