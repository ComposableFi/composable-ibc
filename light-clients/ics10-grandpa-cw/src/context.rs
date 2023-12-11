// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
	contract::{
		GRANDPA_BLOCK_HASHES_CACHE_SIZE, GRANDPA_HEADER_HASHES_SET_STORAGE,
		GRANDPA_HEADER_HASHES_STORAGE,
	},
	ics23::{
		ClientStates, ConsensusStates, FakeInner, ReadonlyClientStates, ReadonlyConsensusStates,
	},
	ContractError,
};
use cosmwasm_std::{Deps, DepsMut, Env, Storage};
use grandpa_light_client_primitives::HostFunctions;
use ibc::{
	core::{
		ics02_client::{error::Error, events::Checksum},
		ics24_host::identifier::ClientId,
		ics26_routing::context::ReaderContext,
	},
	Height,
};
use ibc_proto::google::protobuf::Any;
use ics10_grandpa::{
	client_message::RelayChainHeader, client_state::ClientState, consensus_state::ConsensusState,
};
use sp_core::H256;
use std::{fmt, fmt::Debug, marker::PhantomData};

pub struct Context<'a, H> {
	pub deps_mut: Option<DepsMut<'a>>,
	pub deps: Option<Deps<'a>>,
	pub env: Env,
	pub checksum: Option<Checksum>,
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
		Self { deps_mut: Some(deps), deps: None, _phantom: Default::default(), env, checksum: None }
	}

	pub fn new_ro(deps: Deps<'a>, env: Env) -> Self {
		Self { deps_mut: None, deps: Some(deps), _phantom: Default::default(), env, checksum: None }
	}

	pub fn log(&self, msg: &str) {
		match &self.deps_mut {
			Some(deps_mut) => deps_mut.api.debug(msg),
			None => unimplemented!(),
		}
	}

	pub fn storage(&self) -> &dyn Storage {
		match &self.deps_mut {
			Some(deps_mut) => deps_mut.storage,
			None => match &self.deps {
				Some(deps) => deps.storage,
				None => unimplemented!(),
			},
		}
	}

	pub fn storage_mut(&mut self) -> &mut dyn Storage {
		match &mut self.deps_mut {
			Some(deps_mut) => deps_mut.storage,
			None => unimplemented!(),
		}
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
					self.log(&format!("error saving hash to set: {e:?}"));
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

impl<'a, H> Context<'a, H>
where
	H: Clone,
{
	pub fn consensus_state_prefixed(
		&self,
		height: Height,
		prefix: &[u8],
	) -> Result<ConsensusState, ContractError> {
		let bytes = ReadonlyConsensusStates::new(self.storage())
			.get_prefixed(height, prefix)
			.ok_or_else(|| {
				ContractError::Grandpa(format!(
					"no consensus state found for height {height} and prefix {prefix:?}",
				))
			})?;
		Context::<H>::decode_consensus_state(&bytes)
			.map_err(|e| ContractError::Grandpa(format!("error decoding consensus state: {e:?}")))
	}

	pub fn store_consensus_state_prefixed(
		&mut self,
		height: Height,
		consensus_state: ConsensusState,
		prefix: &[u8],
	) {
		let encoded = Context::<H>::encode_consensus_state(consensus_state);
		let mut consensus_states = ConsensusStates::new(self.storage_mut());
		consensus_states.insert_prefixed(height, encoded, prefix);
	}

	pub fn client_state_prefixed(&self, prefix: &[u8]) -> Result<ClientState<H>, ContractError> {
		let bytes =
			ReadonlyClientStates::new(self.storage()).get_prefixed(prefix).ok_or_else(|| {
				ContractError::Grandpa(format!("no client state found for prefix {prefix:?}",))
			})?;
		Context::decode_client_state(&bytes)
			.map_err(|e| ContractError::Grandpa(format!("error decoding client state: {e:?}")))
	}

	pub fn store_client_state_prefixed(
		&mut self,
		client_state: ClientState<H>,
		prefix: &[u8],
		client_id: ClientId,
	) -> Result<(), ContractError> {
		use prost::Message;
		use tendermint_proto::Protobuf;
		let client_states = ReadonlyClientStates::new(self.storage());
		let checksum = match self.checksum.clone() {
			None => {
				let encoded_wasm_client_state =
					client_states.get_prefixed(prefix).ok_or_else(|| {
						ContractError::Grandpa(Error::client_not_found(client_id).to_string())
					})?;
				let any = Any::decode(&*encoded_wasm_client_state)
					.map_err(Error::decode)
					.map_err(|e| ContractError::Grandpa(e.to_string()))?;
				let wasm_client_state = ics08_wasm::client_state::ClientState::<
					FakeInner,
					FakeInner,
					FakeInner,
				>::decode_vec(&any.value)
				.map_err(|e| {
					ContractError::Grandpa(
						Error::implementation_specific(format!(
								"[client_state]: error decoding client state bytes to WasmConsensusState {}",
								e
							))
						.to_string(),
					)
				})?;
				wasm_client_state.checksum
			},
			Some(x) => x,
		};
		let encoded = Context::<H>::encode_client_state(client_state, checksum)
			.map_err(|e| ContractError::Grandpa(format!("error encoding client state: {:?}", e)))?;
		let mut client_states = ClientStates::new(self.storage_mut());
		client_states.insert_prefixed(encoded, prefix);
		Ok(())
	}
}

impl<'a, H: HostFunctions<Header = RelayChainHeader>> ReaderContext for Context<'a, H> {}
