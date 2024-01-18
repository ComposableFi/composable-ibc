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
	ics23::{ClientStates, ConsensusStates, ReadonlyClientStates, ReadonlyConsensusStates},
	ContractError,
};
use cosmwasm_std::{DepsMut, Env, Storage};
use ibc::{core::ics26_routing::context::ReaderContext, Height};
use icsxx_ethereum::{client_state::ClientState, consensus_state::ConsensusState};
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
				ContractError::Client(format!(
					"no consensus state found for height {height} and prefix {prefix:?}",
				))
			})?;
		Context::<H>::decode_consensus_state(&bytes)
			.map_err(|e| ContractError::Client(format!("error decoding consensus state: {e:?}")))
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
				ContractError::Client(format!("no client state found for prefix {prefix:?}",))
			})?;
		Context::decode_client_state(&bytes)
			.map_err(|e| ContractError::Client(format!("error decoding client state: {e:?}")))
	}

	pub fn store_client_state_prefixed(
		&mut self,
		client_state: ClientState<H>,
		prefix: &[u8],
	) -> Result<(), ContractError> {
		let client_states = ReadonlyClientStates::new(self.storage());
		let data = client_states
			.get_prefixed(prefix)
			.ok_or_else(|| ContractError::Client("no client state found for prefix".to_string()))?;
		let encoded = Context::<H>::encode_client_state(client_state, data)
			.map_err(|e| ContractError::Client(format!("error encoding client state: {e:?}")))?;
		let mut client_states = ClientStates::new(self.storage_mut());
		client_states.insert_prefixed(encoded, prefix);
		Ok(())
	}
}

impl<'a, H: Clone + Eq + Send + Sync + Debug + Default + 'static> ReaderContext for Context<'a, H> {}