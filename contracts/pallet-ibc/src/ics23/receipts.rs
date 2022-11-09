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
use frame_support::storage::{child, child::ChildInfo};
use ibc::core::{
	ics04_channel::packet::Sequence,
	ics24_host::{
		identifier::{ChannelId, PortId},
		path::ReceiptsPath,
	},
};
use ibc_primitives::apply_prefix;
use sp_std::{marker::PhantomData, prelude::*};

// todo: pruning
/// (port_id, channel_id, sequence) => hash
/// trie key path: "receipts/ports/{port_id}/channels/{channel_id}/sequences/{sequence}"
pub struct PacketReceipt<T>(PhantomData<T>);

impl<T: Config> PacketReceipt<T> {
	pub fn insert(
		(port_id, channel_id, sequence): (PortId, ChannelId, Sequence),
		receipt: Vec<u8>,
	) {
		let receipt_path = ReceiptsPath { port_id, channel_id, sequence };
		let receipt_path = format!("{}", receipt_path);
		let receipt_key = apply_prefix(T::PALLET_PREFIX, vec![receipt_path]);
		child::put(&ChildInfo::new_default(T::PALLET_PREFIX), &receipt_key, &receipt)
	}

	pub fn get((port_id, channel_id, sequence): (PortId, ChannelId, Sequence)) -> Option<Vec<u8>> {
		let receipt_path = ReceiptsPath { port_id, channel_id, sequence };
		let receipt_path = format!("{}", receipt_path);
		let receipt_key = apply_prefix(T::PALLET_PREFIX, vec![receipt_path]);
		child::get(&ChildInfo::new_default(T::PALLET_PREFIX), &receipt_key)
	}

	// pub fn remove((port_id, channel_id, sequence): (PortId, ChannelId, Sequence)) {
	// 	let receipt_path = ReceiptsPath { port_id, channel_id, sequence };
	// 	let receipt_path = format!("{}", receipt_path);
	// 	let receipt_key = apply_prefix_and_encode(T::PALLET_PREFIX, vec![receipt_path]);
	// 	child::kill(&ChildInfo::new_default(T::PALLET_PREFIX), &receipt_key)
	// }

	pub fn contains_key((port_id, channel_id, sequence): (PortId, ChannelId, Sequence)) -> bool {
		let receipt_path = ReceiptsPath { port_id, channel_id, sequence };
		let receipt_path = format!("{}", receipt_path);
		let receipt_key = apply_prefix(T::PALLET_PREFIX, vec![receipt_path]);
		child::exists(&ChildInfo::new_default(T::PALLET_PREFIX), &receipt_key)
	}
}
