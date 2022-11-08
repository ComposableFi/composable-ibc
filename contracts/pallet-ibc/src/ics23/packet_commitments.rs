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
use alloc::string::{String, ToString};
use frame_support::storage::{child, child::ChildInfo, ChildTriePrefixIterator};
use ibc::core::{
	ics04_channel::{commitment::PacketCommitment as PacketCommitmentType, packet::Sequence},
	ics24_host::{
		identifier::{ChannelId, PortId},
		path::CommitmentsPath,
		Path,
	},
};
use ibc_primitives::apply_prefix;
use sp_std::{marker::PhantomData, prelude::*, str::FromStr};

/// (port_id, channel_id, sequence) => hash
/// trie key path: "commitments/ports/{port_id}/channels/{channel_id}/sequences/{sequence}"
pub struct PacketCommitment<T>(PhantomData<T>);

impl<T: Config> PacketCommitment<T> {
	pub fn insert(
		(port_id, channel_id, sequence): (PortId, ChannelId, Sequence),
		commitment: PacketCommitmentType,
	) {
		let commitment_path = CommitmentsPath { port_id, channel_id, sequence };
		let commitment_path = format!("{}", commitment_path);
		let commitment_key = apply_prefix(T::PALLET_PREFIX, vec![commitment_path]);
		child::put(
			&ChildInfo::new_default(T::PALLET_PREFIX),
			&commitment_key,
			&commitment.into_vec(),
		)
	}

	pub fn get((port_id, channel_id, sequence): (PortId, ChannelId, Sequence)) -> Option<Vec<u8>> {
		let commitment_path = CommitmentsPath { port_id, channel_id, sequence };
		let commitment_path = format!("{}", commitment_path);
		let commitment_key = apply_prefix(T::PALLET_PREFIX, vec![commitment_path]);
		child::get(&ChildInfo::new_default(T::PALLET_PREFIX), &commitment_key)
	}

	pub fn remove((port_id, channel_id, sequence): (PortId, ChannelId, Sequence)) {
		let commitment_path = CommitmentsPath { port_id, channel_id, sequence };
		let commitment_path = format!("{}", commitment_path);
		let commitment_key = apply_prefix(T::PALLET_PREFIX, vec![commitment_path]);
		child::kill(&ChildInfo::new_default(T::PALLET_PREFIX), &commitment_key)
	}

	pub fn contains_key((port_id, channel_id, sequence): (PortId, ChannelId, Sequence)) -> bool {
		let commitment_path = CommitmentsPath { port_id, channel_id, sequence };
		let commitment_path = format!("{}", commitment_path);
		let commitment_key = apply_prefix(T::PALLET_PREFIX, vec![commitment_path]);
		child::exists(&ChildInfo::new_default(T::PALLET_PREFIX), &commitment_key)
	}

	// WARNING: too expensive to be called from an on-chain context, only here for rpc layer.
	pub fn iter() -> impl Iterator<Item = ((PortId, ChannelId, Sequence), Vec<u8>)> {
		let prefix = "commitments/ports/".to_string();
		let prefix_key = apply_prefix(T::PALLET_PREFIX, vec![prefix.clone()]);
		ChildTriePrefixIterator::with_prefix(&ChildInfo::new_default(T::PALLET_PREFIX), &prefix_key)
			.filter_map(move |(remaining_key, value)| {
				let path = format!("{prefix}{}", String::from_utf8(remaining_key).ok()?);
				if let Path::Commitments(CommitmentsPath { port_id, channel_id, sequence }) =
					Path::from_str(&path).ok()?
				{
					return Some(((port_id, channel_id, sequence), value))
				}
				None
			})
	}
}
