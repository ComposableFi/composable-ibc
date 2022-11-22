// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{format, Config};
use alloc::vec;
use frame_support::storage::{child, child::ChildInfo};
use ibc::core::ics24_host::{
	identifier::{ChannelId, PortId},
	path::SeqRecvsPath,
};
use ibc_primitives::apply_prefix;
use sp_std::marker::PhantomData;

// todo: pruning
/// (port_id, channel_id) => Sequence
/// trie key path: "nextSequenceRecv/ports/{port_id}/channels/{channel_id}"
pub struct NextSequenceRecv<T>(PhantomData<T>);

impl<T: Config> NextSequenceRecv<T> {
	pub fn get(port_id: PortId, channel_id: ChannelId) -> Option<u64> {
		let next_seq_recv_path = format!("{}", SeqRecvsPath(port_id, channel_id));
		let next_seq_recv_key = apply_prefix(T::PALLET_PREFIX, vec![next_seq_recv_path]);
		child::get(&ChildInfo::new_default(T::PALLET_PREFIX), &next_seq_recv_key)
	}

	pub fn insert(port_id: PortId, channel_id: ChannelId, seq: u64) {
		let next_seq_recv_path = format!("{}", SeqRecvsPath(port_id, channel_id));
		let next_seq_recv_key = apply_prefix(T::PALLET_PREFIX, vec![next_seq_recv_path]);
		child::put(&ChildInfo::new_default(T::PALLET_PREFIX), &next_seq_recv_key, &seq)
	}
}
