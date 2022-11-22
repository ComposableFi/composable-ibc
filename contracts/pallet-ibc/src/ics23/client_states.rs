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
use ibc::core::ics24_host::{identifier::ClientId, path::ClientStatePath, Path};
use ibc_primitives::apply_prefix;
use sp_std::{marker::PhantomData, prelude::*, str::FromStr};

/// client_id => client_states
/// trie key path: "clients/{client_id}/clientState"
pub struct ClientStates<T>(PhantomData<T>);

impl<T: Config> ClientStates<T> {
	pub fn get(client_id: &ClientId) -> Option<Vec<u8>> {
		let client_state_path = format!("{}", ClientStatePath(client_id.clone()));
		let client_state_key = apply_prefix(T::PALLET_PREFIX, vec![client_state_path]);
		child::get(&ChildInfo::new_default(T::PALLET_PREFIX), &client_state_key)
	}

	pub fn insert(client_id: &ClientId, client_state: Vec<u8>) {
		let client_state_path = format!("{}", ClientStatePath(client_id.clone()));
		let client_state_key = apply_prefix(T::PALLET_PREFIX, vec![client_state_path]);
		child::put(&ChildInfo::new_default(T::PALLET_PREFIX), &client_state_key, &client_state);
	}

	pub fn _contains_key(client_id: &ClientId) -> bool {
		let client_state_path = format!("{}", ClientStatePath(client_id.clone()));
		let client_state_key = apply_prefix(T::PALLET_PREFIX, vec![client_state_path]);
		child::exists(&ChildInfo::new_default(T::PALLET_PREFIX), &client_state_key)
	}

	// WARNING: too expensive to be called from an on-chain context, only here for rpc layer.
	// client_id => client_state
	pub fn iter() -> impl Iterator<Item = (ClientId, Vec<u8>)> {
		let prefix_path = "clients/".to_string();
		let key = apply_prefix(T::PALLET_PREFIX, vec![prefix_path.clone()]);
		ChildTriePrefixIterator::with_prefix(&ChildInfo::new_default(T::PALLET_PREFIX), &key)
			.filter_map(move |(remaining_key, value)| {
				let path = format!("{prefix_path}{}", String::from_utf8(remaining_key).ok()?);
				if let Path::ClientState(ClientStatePath(client_id)) = Path::from_str(&path).ok()? {
					return Some((client_id, value))
				}
				None
			})
	}
}
