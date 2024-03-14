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

use cosmwasm_std::Storage;
use prost::Message;

use crate::{fake_inner::FakeInner, ibc, ibc::proto::google::protobuf::Any};
use ibc::protobuf::Protobuf;

type Result<T, E = crate::Error> = core::result::Result<T, E>;

pub type ClientMessage = cf_guest::ClientMessage<crate::PubKey>;
pub type ClientState = cf_guest::ClientState<crate::PubKey>;
pub type ConsensusState = cf_guest::ConsensusState;
pub type Header = cf_guest::Header<crate::PubKey>;
pub type Misbehaviour = cf_guest::Misbehaviour<crate::PubKey>;

type WasmClientState = ics08_wasm::client_state::ClientState<FakeInner, FakeInner, FakeInner>;
type WasmConsensusState = ics08_wasm::consensus_state::ConsensusState<FakeInner>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Metadata {
	pub host_timestamp_ns: u64,
	pub host_height: u64,
}

/// Wrapper for accessing client state saved in the storage.
#[repr(transparent)]
pub(crate) struct ClientStates(dyn Storage);

impl ClientStates {
	pub fn new<'a>(storage: &'a mut dyn Storage) -> &'a mut Self {
		// SAFETY: Self is repr(transparent) over dyn Storage so the
		// transmutation is sound.
		unsafe { wrap_mut(storage) }
	}

	pub fn new_ro<'a>(storage: &'a dyn Storage) -> &'a Self {
		// SAFETY: Self is repr(transparent) over dyn Storage so the
		// transmutation is sound.
		unsafe { wrap_ref(storage) }
	}

	pub fn get(&self) -> Result<Option<ClientState>> {
		self.get_impl(Self::KEY)
	}

	pub fn set(&mut self, state: &ClientState) {
		self.set_impl(Self::KEY, state)
	}

	const KEY: &'static [u8] = b"clientState";

	fn get_impl(&self, key: &[u8]) -> Result<Option<ClientState>> {
		let value = match self.0.get(&key) {
			None => return Ok(None),
			Some(value) => value,
		};
		let any = Any::decode(value.as_slice())?;
		let wasm_state = WasmClientState::decode_vec(&any.value)?;
		let any = Any::decode(wasm_state.data.as_slice())?;
		Ok(Some(ClientState::try_from(any)?))
	}

	fn set_impl(&mut self, _key: &[u8], _state: &ClientState) {
		todo!()
	}
}

/// Wrapper for accessing consensus state saved in the storage.
#[repr(transparent)]
pub(crate) struct ConsensusStates(dyn Storage);

impl ConsensusStates {
	pub fn new<'a>(storage: &'a mut dyn Storage) -> &'a mut Self {
		// SAFETY: Self is repr(transparent) over dyn Storage so the
		// transmutation is sound.
		unsafe { wrap_mut(storage) }
	}

	pub fn new_ro<'a>(storage: &'a dyn Storage) -> &'a Self {
		// SAFETY: Self is repr(transparent) over dyn Storage so the
		// transmutation is sound.
		unsafe { wrap_ref(storage) }
	}

	pub fn get(&self, height: ibc::Height) -> Result<Option<ConsensusState>> {
		self.get_impl(&Self::key(height))
	}

	pub fn set(&mut self, height: ibc::Height, state: &ConsensusState, metadata: Metadata) {
		self.set_impl(Self::key(height), state, metadata)
	}

	pub fn prune_oldest_consensus_state(
		&mut self,
		_client_state: &ClientState,
		_now_ns: u64,
	) -> Result<()> {
		todo!()
	}

	pub fn del(&mut self, height: ibc::Height) {
		self.0.remove(&Self::key(height))
	}

	fn key(height: ibc::Height) -> Vec<u8> {
		format!("consensusStates/{height}").into_bytes()
	}

	fn get_impl(&self, key: &[u8]) -> Result<Option<ConsensusState>> {
		let value = match self.0.get(key) {
			None => return Ok(None),
			Some(value) => value,
		};
		let any = Any::decode(value.as_slice())?;
		let wasm_state = WasmConsensusState::decode_vec(&any.value)?;
		let any = Any::decode(wasm_state.data.as_slice())?;
		Ok(Some(ConsensusState::try_from(any)?))
	}

	fn set_impl(&mut self, _key: Vec<u8>, _state: &ConsensusState, _metadata: Metadata) {
		todo!()
	}
}

/// Extension of protobuf’s Any type to include host height and host timestamp.
#[derive(Clone, PartialEq, prost::Message)]
struct ConsensusWithMetadata {
	#[prost(string, tag = "1")]
	pub type_url: ::prost::alloc::string::String,
	#[prost(bytes = "vec", tag = "2")]
	pub value: ::prost::alloc::vec::Vec<u8>,
	#[prost(uint64, tag = "3")]
	pub host_timestamp_ns: u64,
	#[prost(uint64, tag = "4")]
	pub host_height: u64,
}

impl ConsensusWithMetadata {
	fn new(state: impl Into<Any>, metadata: Metadata) -> Self {
		let Any { type_url, value } = state.into();
		let Metadata { host_timestamp_ns, host_height } = metadata;
		Self { type_url, value, host_timestamp_ns, host_height }
	}

	fn into_parts(self) -> (Any, Metadata) {
		(
			Any { type_url: self.type_url, value: self.value },
			Metadata { host_timestamp_ns: self.host_timestamp_ns, host_height: self.host_height },
		)
	}
}

unsafe fn wrap_ref<F: ?Sized, T: ?Sized>(from: &F) -> &T {
	assert!(core::mem::size_of::<*const F>() == core::mem::size_of::<*const T>());
	let inner_ptr = core::mem::ManuallyDrop::new(from as *const F);
	// SAFETY: Caller guarantees soundness.
	unsafe {
		// A pointer cast doesn't work here because rustc can't tell
		// that the vtables match (because of the `?Sized` restriction
		// relaxation).  A `transmute` doesn't work because the sizes
		// are unspecified.
		let outer_ptr: *const T = core::mem::transmute_copy(&inner_ptr);
		&*outer_ptr
	}
}

unsafe fn wrap_mut<F: ?Sized, T: ?Sized>(from: &mut F) -> &mut T {
	assert!(core::mem::size_of::<*mut F>() == core::mem::size_of::<*mut T>());
	let inner_ptr = core::mem::ManuallyDrop::new(from as *mut F);
	// SAFETY: Caller guarantees soundness.
	unsafe {
		// A pointer cast doesn't work here because rustc can't tell
		// that the vtables match (because of the `?Sized` restriction
		// relaxation).  A `transmute` doesn't work because the sizes
		// are unspecified.
		let outer_ptr: *mut T = core::mem::transmute_copy(&inner_ptr);
		&mut *outer_ptr
	}
}
