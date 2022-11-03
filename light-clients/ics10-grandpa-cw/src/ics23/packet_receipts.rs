use crate::{Bytes, STORAGE_PREFIX};
use cosmwasm_std::Storage;
use cosmwasm_storage::{prefixed, PrefixedStorage};
use ibc::core::{
	ics04_channel::packet::Sequence,
	ics24_host::{
		identifier::{ChannelId, PortId},
		path::CommitmentsPath,
	},
};
use sp_std::prelude::*;

/// (port_id, channel_id, sequence) => hash
/// trie key path: "receipts/ports/{port_id}/channels/{channel_id}/sequences/{sequence}"
pub struct PacketReceipts<'a>(PrefixedStorage<'a>);

impl<'a> PacketReceipts<'a> {
	pub fn new(storage: &'a mut dyn Storage) -> Self {
		Self(prefixed(storage, STORAGE_PREFIX))
	}

	pub fn key((port_id, channel_id, sequence): (PortId, ChannelId, Sequence)) -> Vec<u8> {
		let commitment_path = CommitmentsPath { port_id, channel_id, sequence };
		let commitment_path = format!("{}", commitment_path);
		commitment_path.into_bytes()
	}

	pub fn get(&self, key: (PortId, ChannelId, Sequence)) -> Option<Bytes> {
		let commitment_path = Self::key(key);
		self.0.get(&commitment_path)
	}

	pub fn insert(&mut self, key: (PortId, ChannelId, Sequence), _commitment: &Bytes) {
		let commitment_path = Self::key(key);
		self.0.set(&commitment_path, todo!());
	}

	pub fn contains_key(&self, key: (PortId, ChannelId, Sequence)) -> bool {
		self.0.get(&Self::key(key)).is_some()
	}
}
