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

#[derive(derive_more::Deref)]
pub(crate) struct ContextBase<'a> {
	#[deref]
	pub metadata: state::Metadata,
	pub client_id: ibc::ClientId,
	pub api: &'a dyn Api,
}

#[derive(derive_more::Deref)]
pub(crate) struct Context<'a> {
	#[deref]
	base: ContextBase<'a>,
	storage: &'a dyn Storage,
}

#[derive(derive_more::Deref)]
pub(crate) struct ContextMut<'a> {
	#[deref]
	base: ContextBase<'a>,
	storage: &'a mut dyn Storage,
}

pub(crate) fn new<'a>(deps: DepsMut<'a>, env: Env) -> ContextMut<'a> {
	ContextMut { base: ContextBase::new(env, deps.api), storage: deps.storage }
}

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

macro_rules! log {
	($self:expr, $($tt:tt)*) => {
		$self.log(format_args!($($tt)*))
	};
}

pub(crate) use log;

impl<'a> Context<'a> {
	pub fn client_state(&self) -> Result<state::ClientState> {
		req_client_state(&self.client_id, self.client_states().get())
	}

	pub fn client_states(&self) -> &'a state::ClientStates {
		state::ClientStates::new_ro(self.storage)
	}

	pub fn consensus_state(&self, height: ibc::Height) -> Result<state::ConsensusState> {
		req_consensus_state(&self.client_id, height, self.consensus_states().get(height))
	}

	pub fn consensus_states(&self) -> &'a state::ConsensusStates {
		state::ConsensusStates::new_ro(self.storage)
	}
}

impl<'a> ContextMut<'a> {
	pub fn client_state(&self) -> Result<state::ClientState> {
		req_client_state(&self.client_id, self.client_states().get())
	}

	pub fn client_states(&self) -> &state::ClientStates {
		state::ClientStates::new_ro(self.storage)
	}

	// pub fn consensus_state(&self, height: ibc::Height) -> Result<state::ConsensusState> {
	// 	req_consensus_state(&self.client_id, height, self.consensus_states().get(height))
	// }

	// pub fn consensus_states(&self) -> &state::ConsensusStates {
	// 	state::ConsensusStates::new_ro(self.storage)
	// }

	pub fn client_states_mut(&mut self) -> &mut state::ClientStates {
		state::ClientStates::new(self.storage)
	}

	pub fn consensus_states_mut(&mut self) -> &mut state::ConsensusStates {
		state::ConsensusStates::new(self.storage)
	}
}

fn req_client_state(
	client_id: &ibc::ClientId,
	state: Result<Option<state::ClientState>>,
) -> Result<state::ClientState> {
	match state {
		Ok(Some(state)) => Ok(state),
		Ok(None) =>
			Err(ibc::ClientError::ClientStateNotFound { client_id: client_id.clone() }.into()),
		Err(err) => Err(err),
	}
}

pub fn req_consensus_state(
	client_id: &ibc::ClientId,
	height: ibc::Height,
	state: Result<Option<(state::ConsensusState, state::Metadata)>>,
) -> Result<state::ConsensusState> {
	match state {
		Ok(Some((state, _metadata))) => Ok(state),
		Ok(None) =>
			Err(ibc::ClientError::ConsensusStateNotFound { client_id: client_id.clone(), height }
				.into()),
		Err(err) => Err(err),
	}
}
