use crate::{format, Config};
use frame_support::storage::{child, child::ChildInfo};
use ibc::core::{
	ics02_client::client_state::ClientType,
	ics24_host::{identifier::ClientId, path::ClientTypePath},
};
use ibc_primitives::apply_prefix;
use sp_std::{marker::PhantomData, prelude::*};

/// client_id => client_type
/// trie key path: "clients/{}/clientType"
pub struct Clients<T>(PhantomData<T>);

impl<T: Config> Clients<T> {
	pub fn get(client_id: &ClientId) -> Option<ClientType> {
		let client_type_path = format!("{}", ClientTypePath(client_id.clone()));
		let client_type_key = apply_prefix(T::PALLET_PREFIX, vec![client_type_path]);
		child::get(&ChildInfo::new_default(T::PALLET_PREFIX), &client_type_key)
	}

	pub fn insert(client_id: &ClientId, client_type: ClientType) {
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
