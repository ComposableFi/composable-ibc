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

use core::str::FromStr;

use cosmwasm_std::{Api, Deps, DepsMut, Env, Storage};

use crate::{ibc, state};

type Result<T, E = crate::Error> = core::result::Result<T, E>;

/// Base context for handling CosmWasm operations.
///
/// It wraps together access to CosmWasm API and information about the request
/// such as block height, current time and IBC client id corresponding to this
/// contract.
///
/// The object dereferences into [`state::Metadata`] such that metadata fields
/// are directly accessible through this object.
#[derive(derive_more::Deref)]
pub(crate) struct ContextBase<'a> {
	#[deref]
	pub metadata: state::Metadata,
	pub client_id: ibc::ClientId,
	pub api: &'a dyn Api,
}

/// Mutable execution context for handling CosmWasm operations.
///
/// It wraps together access to CosmWasm APIs, storage and information about the
/// request.  To construct a new context object use [`new`] function.
#[derive(derive_more::Deref)]
pub(crate) struct ContextMut<'a> {
	#[deref]
	base: ContextBase<'a>,
	storage: &'a mut dyn Storage,
}

/// Constructs a new mutable execution context.
pub(crate) fn new<'a>(deps: DepsMut<'a>, env: Env) -> ContextMut<'a> {
	ContextMut { base: ContextBase::new(env, deps.api), storage: deps.storage }
}

/// Read-only execution context for handling CosmWasm operations.
///
/// It wraps together access to CosmWasm APIs, storage and information about the
/// request.  To construct a new context object use [`new_ro`] function.
///
/// The object dereferences into [`ContextBase`] which holds data common between
/// read-only and mutable execution contexts.
#[derive(derive_more::Deref)]
pub(crate) struct Context<'a> {
	#[deref]
	base: ContextBase<'a>,
	storage: &'a dyn Storage,
}

/// Constructs a new read-only execution context.
pub(crate) fn new_ro<'a>(deps: Deps<'a>, env: Env) -> Context<'a> {
	Context { base: ContextBase::new(env, deps.api), storage: deps.storage }
}

impl<'a> ContextBase<'a> {
	fn new(env: Env, api: &'a dyn Api) -> Self {
		let metadata = state::Metadata {
			host_timestamp_ns: env.block.time.nanos(),
			host_height: env.block.height,
		};
		let address = env.contract.address.as_str();
		let client_id = ibc::ClientId::from_str(address).unwrap();
		Self { client_id, metadata, api }
	}

	pub fn log(&self, msg: impl alloc::string::ToString) {
		self.api.debug(&msg.to_string())
	}
}

/// Logs formatted text using CosmWasm API.
///
/// To log string literals prefer [`ContextBase::log`] method.
macro_rules! log {
	($self:expr, $($tt:tt)*) => {
		$self.log(format_args!($($tt)*))
	};
}

pub(crate) use log;

impl<'a> Context<'a> {
	/// Reads this light client’s client state from storage.
	pub fn client_state(&self) -> Result<state::ClientState> {
		req_client_state(&self.client_id, self.client_states().get())
	}

	/// Returns object providing access to read client state from the
	/// storage.
	pub fn client_states(&self) -> &'a state::ClientStates {
		state::ClientStates::new_ro(self.storage)
	}

	/// Reads this light client’s consensus state at given height from
	/// storage.
	pub fn consensus_state(&self, height: ibc::Height) -> Result<state::ConsensusState> {
		req_consensus_state(&self.client_id, height, self.consensus_states().get(height))
	}

	/// Returns object providing access to read consensus states from the
	/// storage.
	pub fn consensus_states(&self) -> &'a state::ConsensusStates {
		state::ConsensusStates::new_ro(self.storage)
	}
}

impl<'a> ContextMut<'a> {
	/// Reads this light client’s client state from storage.
	pub fn client_state(&self) -> Result<state::ClientState> {
		req_client_state(&self.client_id, self.client_states().get())
	}

	/// Returns object providing access to read client state from the
	/// storage.
	pub fn client_states(&self) -> &state::ClientStates {
		state::ClientStates::new_ro(self.storage)
	}

	/// Returns object providing access to read or write client state
	/// from/to the storage.
	pub fn client_states_mut(&mut self) -> &mut state::ClientStates {
		state::ClientStates::new(self.storage)
	}

	/// Reads this light client’s consensus state at given height from
	/// storage.
	pub fn consensus_state(&self, height: ibc::Height) -> Result<state::ConsensusState> {
		req_consensus_state(&self.client_id, height, self.consensus_states().get(height))
	}

	/// Returns object providing access to read consensus states from the
	/// storage.
	pub fn consensus_states(&self) -> &state::ConsensusStates {
		state::ConsensusStates::new_ro(self.storage)
	}

	/// Returns object providing access to read or write consensus states
	/// from/to the storage.
	pub fn consensus_states_mut(&mut self) -> &mut state::ConsensusStates {
		state::ConsensusStates::new(self.storage)
	}
}

/// Returns an error if client state is not present.
fn req_client_state(
	client_id: &ibc::ClientId,
	state: Result<Option<state::ClientState>>,
) -> Result<state::ClientState> {
	let make_err = || ibc::ClientError::ClientStateNotFound { client_id: client_id.clone() }.into();
	state.and_then(|state| state.ok_or_else(make_err))
}

/// Returns an error if consensus state is not present.
fn req_consensus_state(
	client_id: &ibc::ClientId,
	height: ibc::Height,
	state: Result<Option<(state::ConsensusState, state::Metadata)>>,
) -> Result<state::ConsensusState> {
	let make_err =
		|| ibc::ClientError::ConsensusStateNotFound { client_id: client_id.clone(), height }.into();
	state.and_then(|state| state.map(|(state, _metadata)| state).ok_or_else(make_err))
}
