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
use ibc::core::ics24_host::{identifier::ClientId, path::ClientTypePath};
use ibc_primitives::apply_prefix;
use sp_std::{marker::PhantomData, prelude::*};

/// client_id => client_type
/// trie key path: "clients/{}/clientType"
pub struct Clients<T>(PhantomData<T>);

impl<T: Config> Clients<T> {
	pub fn get(client_id: &ClientId) -> Option<Vec<u8>> {
		let client_type_path = format!("{}", ClientTypePath(client_id.clone()));
		let client_type_key = apply_prefix(T::PALLET_PREFIX, vec![client_type_path]);
		child::get(&ChildInfo::new_default(T::PALLET_PREFIX), &client_type_key)
	}

	pub fn insert(client_id: &ClientId, client_type: Vec<u8>) {
		let client_type_path = format!("{}", ClientTypePath(client_id.clone()));
		let client_type_key = apply_prefix(T::PALLET_PREFIX, vec![client_type_path]);
		child::put(&ChildInfo::new_default(T::PALLET_PREFIX), &client_type_key, &client_type);
	}

	pub fn contains_key(client_id: &ClientId) -> bool {
		let client_type_path = format!("{}", ClientTypePath(client_id.clone()));
		let client_type_key = apply_prefix(T::PALLET_PREFIX, vec![client_type_path]);
		child::exists(&ChildInfo::new_default(T::PALLET_PREFIX), &client_type_key)
	}
}
