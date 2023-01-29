use crate::{format, Config};
use alloc::string::ToString;
use core::str::FromStr;
use frame_support::storage::{child, child::ChildInfo, ChildTriePrefixIterator};
use ibc::core::{
	ics03_connection::connection::ConnectionEnd,
	ics24_host::{identifier::ConnectionId, path::ConnectionsPath, Path},
};
use ibc_primitives::apply_prefix;
use scale_info::prelude::string::String;
use sp_std::{marker::PhantomData, prelude::*};

// todo: pruning
/// connection_id => ConnectionEnd
/// trie key path: "connections/{}"
pub struct Connections<T>(PhantomData<T>);

impl<T: Config> Connections<T> {
	pub fn get(connection_id: &ConnectionId) -> Option<ConnectionEnd> {
		let connection_path = format!("{}", ConnectionsPath(connection_id.clone()));
		let connection_key = apply_prefix(T::PALLET_PREFIX, vec![connection_path]);
		child::get(&ChildInfo::new_default(T::PALLET_PREFIX), &connection_key)
	}

	pub fn insert(connection_id: &ConnectionId, connection_end: &ConnectionEnd) {
		let connection_path = format!("{}", ConnectionsPath(connection_id.clone()));
		let connection_key = apply_prefix(T::PALLET_PREFIX, vec![connection_path]);
		child::put(&ChildInfo::new_default(T::PALLET_PREFIX), &connection_key, &connection_end);
	}

	// WARNING: too expensive to be called from an on-chain context, only here for rpc layer.
	pub fn iter() -> impl Iterator<Item = (ConnectionId, ConnectionEnd)> {
		let prefix_path = "connections/".to_string();
		let key = apply_prefix(T::PALLET_PREFIX, vec![prefix_path.clone()]);
		ChildTriePrefixIterator::with_prefix(&ChildInfo::new_default(T::PALLET_PREFIX), &key)
			.filter_map(move |(key, value)| {
				let path = format!("{prefix_path}{}", String::from_utf8(key).ok()?);
				if let Path::Connections(ConnectionsPath(connection_id)) =
					Path::from_str(&path).ok()?
				{
					return Some((connection_id, value))
				}
				None
			})
	}
}
