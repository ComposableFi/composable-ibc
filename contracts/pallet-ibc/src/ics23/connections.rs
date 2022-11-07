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
use alloc::string::ToString;
use frame_support::storage::{child, child::ChildInfo, ChildTriePrefixIterator};
use ibc::core::{
	ics03_connection::connection::ConnectionEnd,
	ics24_host::{identifier::ConnectionId, path::ConnectionsPath},
};
use ibc_primitives::apply_prefix;
use sp_std::{marker::PhantomData, prelude::*};
use tendermint_proto::Protobuf;

// todo: pruning
/// connection_id => ConnectionEnd
/// trie key path: "connections/{}"
pub struct Connections<T>(PhantomData<T>);

impl<T: Config> Connections<T> {
	pub fn get(connection_id: &ConnectionId) -> Option<Vec<u8>> {
		let connection_path = format!("{}", ConnectionsPath(connection_id.clone()));
		let connection_key = apply_prefix(T::PALLET_PREFIX, vec![connection_path]);
		child::get(&ChildInfo::new_default(T::PALLET_PREFIX), &connection_key)
	}

	pub fn insert(connection_id: &ConnectionId, connection_end: &ConnectionEnd) {
		let connection_path = format!("{}", ConnectionsPath(connection_id.clone()));
		let connection_key = apply_prefix(T::PALLET_PREFIX, vec![connection_path]);
		child::put(
			&ChildInfo::new_default(T::PALLET_PREFIX),
			&connection_key,
			&connection_end.encode_vec(),
		);
	}

	// WARNING: too expensive to be called from an on-chain context, only here for rpc layer.
	pub fn iter() -> ChildTriePrefixIterator<(Vec<u8>, Vec<u8>)> {
		let prefix_path = "connections/".to_string();
		let key = apply_prefix(T::PALLET_PREFIX, vec![prefix_path]);
		ChildTriePrefixIterator::with_prefix(&ChildInfo::new_default(T::PALLET_PREFIX), &key)
	}
}
