use crate::STORAGE_PREFIX;
use cosmwasm_std::Storage;
use cosmwasm_storage::{prefixed, PrefixedStorage};
use ibc::core::{
	ics04_channel::packet::{Packet, Sequence},
	ics24_host::identifier::{ChannelId, PortId},
};
use sp_std::prelude::*;

/// (port_id, channel_id, sequence) => Packet
pub struct Packets<'a>(PrefixedStorage<'a>);

impl<'a> Packets<'a> {
	pub fn new(storage: &'a mut dyn Storage) -> Self {
		Self(prefixed(storage, STORAGE_PREFIX))
	}

	pub fn key((port_id, channel_id, sequence): (PortId, ChannelId, Sequence)) -> Vec<u8> {
		let mut key_buf = Vec::new();
		let channel_id = channel_id.to_string().as_bytes().to_vec();
		let port_id = port_id.as_bytes().to_vec();
		let seq = u64::from(sequence);
		key_buf.extend(channel_id);
		key_buf.extend(port_id);
		key_buf.extend(seq.to_le_bytes());
		key_buf
	}

	pub fn get(&self, key: (PortId, ChannelId, Sequence)) -> Option<()> {
		let _packet_path = Self::key(key);
		// self.0.get(&packet_path)
		todo!("get packet")
	}

	pub fn insert(&mut self, key: (PortId, ChannelId, Sequence), _packet: Packet) {
		let packet_path = Self::key(key);
		self.0.set(&packet_path, todo!("serialize packet"));
	}

	pub fn remove(&mut self, key: (PortId, ChannelId, Sequence)) {
		let packet_path = Self::key(key);
		self.0.remove(&packet_path);
	}

	pub fn contains_key(&self, key: (PortId, ChannelId, Sequence)) -> bool {
		self.get(key).is_some()
	}
}
