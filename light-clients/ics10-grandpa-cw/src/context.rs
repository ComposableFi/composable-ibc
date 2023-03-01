use crate::contract::{
	GRANDPA_BLOCK_HASHES_CACHE_SIZE, GRANDPA_HEADER_HASHES_SET_STORAGE,
	GRANDPA_HEADER_HASHES_STORAGE,
};
use cosmwasm_std::{DepsMut, Env, Storage};
use grandpa_light_client_primitives::HostFunctions;
use ibc::core::ics26_routing::context::ReaderContext;
use ics10_grandpa::client_message::RelayChainHeader;
use sp_core::H256;
use std::{fmt, fmt::Debug, marker::PhantomData};

pub struct Context<'a, H> {
	pub deps: DepsMut<'a>,
	pub env: Env,
	_phantom: PhantomData<H>,
}

impl<'a, H> PartialEq for Context<'a, H> {
	fn eq(&self, _other: &Self) -> bool {
		true
	}
}

impl<'a, H> Eq for Context<'a, H> {}

impl<'a, H> Debug for Context<'a, H> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Context {{ deps: DepsMut }}")
	}
}

impl<'a, H> Clone for Context<'a, H> {
	fn clone(&self) -> Self {
		panic!("Context is not cloneable")
	}
}

impl<'a, H> Context<'a, H> {
	pub fn new(deps: DepsMut<'a>, env: Env) -> Self {
		Self { deps, _phantom: Default::default(), env }
	}

	pub fn log(&self, msg: &str) {
		self.deps.api.debug(msg)
	}

	pub fn storage(&self) -> &dyn Storage {
		self.deps.storage
	}

	pub fn storage_mut(&mut self) -> &mut dyn Storage {
		self.deps.storage
	}

	pub fn insert_relay_header_hashes(&mut self, headers: &[H256]) {
		if headers.is_empty() {
			return
		}

		let mut xs = GRANDPA_HEADER_HASHES_STORAGE.load(self.storage()).unwrap_or_default();
		xs.reserve(headers.len());
		for header in headers {
			xs.push(*header);
			let _ = GRANDPA_HEADER_HASHES_SET_STORAGE
				.save(self.storage_mut(), header.0.to_vec(), &())
				.map_err(|e| {
					self.log(&format!("error saving hash to set: {:?}", e));
				});
		}
		if xs.len() > GRANDPA_BLOCK_HASHES_CACHE_SIZE {
			let drained = xs.drain(0..xs.len() - GRANDPA_BLOCK_HASHES_CACHE_SIZE);
			for hash in drained {
				GRANDPA_HEADER_HASHES_SET_STORAGE.remove(self.storage_mut(), hash.0.to_vec());
			}
		}
		GRANDPA_HEADER_HASHES_STORAGE
			.save(self.storage_mut(), &xs)
			.expect("error saving header hashes");
	}

	pub fn contains_relay_header_hash(&self, hash: H256) -> bool {
		GRANDPA_HEADER_HASHES_STORAGE
			.load(self.storage())
			.unwrap_or_default()
			.contains(&hash)
	}
}

impl<'a, H: HostFunctions<Header = RelayChainHeader>> ReaderContext for Context<'a, H> {}
