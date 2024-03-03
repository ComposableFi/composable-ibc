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

use crate::{ibc, ibc::proto::Any};

type Result<T, E = crate::Error> = core::result::Result<T, E>;

pub type ClientMessage = cf_guest::ClientMessage<crate::PubKey>;
pub type ClientState = cf_guest::ClientState<crate::PubKey>;
pub type ConsensusState = cf_guest::ConsensusState;
pub type Header = cf_guest::Header<crate::PubKey>;
pub type Misbehaviour = cf_guest::Misbehaviour<crate::PubKey>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Metadata {
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

	pub fn get<T, E>(&self) -> Result<Option<T>, E>
	where
		T: TryFrom<Any>,
		E: From<T::Error> + From<prost::DecodeError>,
	{
		self.get_impl(Self::KEY)
	}

	pub fn get_prefixed<T, E>(&self, prefix: &[u8]) -> Result<Option<T>, E>
	where
		T: TryFrom<Any>,
		E: From<T::Error> + From<prost::DecodeError>,
	{
		self.get_impl(&[prefix, Self::KEY].concat())
	}

	pub fn set(&mut self, state: impl Into<Any>) {
		self.set_impl(Self::KEY, state)
	}

	pub fn set_prefixed(&mut self, prefix: &[u8], state: impl Into<Any>) {
		self.set_impl(&[prefix, Self::KEY].concat(), state)
	}

	pub fn rem(&mut self) {
		self.0.remove(Self::KEY)
	}

	pub fn rem_prefixed(&mut self, prefix: &[u8]) {
		self.0.remove(&[prefix, Self::KEY].concat())
	}

	const KEY: &'static [u8] = b"clientState/";

	fn get_impl<T, E>(&self, key: &[u8]) -> Result<Option<T>, E>
	where
		T: TryFrom<Any>,
		E: From<T::Error> + From<prost::DecodeError>,
	{
		self.0
			.get(&key)
			.map(|value| {
				let any = Any::decode(value.as_slice())?;
				T::try_from(any).map_err(|err| err.into())
			})
			.transpose()
	}

	fn set_impl(&mut self, key: &[u8], state: impl Into<Any>) {
		self.0.set(&key, state.into().encode_to_vec().as_slice())
	}
}

/// Wrapper for accessing consensus state saved in the storage.
#[repr(transparent)]
pub struct ConsensusStates(dyn Storage);

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

	pub fn get<T, E>(&self, height: ibc::Height) -> Result<Option<(T, Metadata)>, E>
	where
		T: TryFrom<Any>,
		E: From<T::Error> + From<prost::DecodeError>,
	{
		self.get_impl(&Self::key(b"", height))
	}

	pub fn get_prefixed<T, E>(
		&self,
		prefix: &[u8],
		height: ibc::Height,
	) -> Result<Option<(T, Metadata)>, E>
	where
		T: TryFrom<Any>,
		E: From<T::Error> + From<prost::DecodeError>,
	{
		self.get_impl(&Self::key(prefix, height))
	}

	pub fn set(&mut self, height: ibc::Height, state: impl Into<Any>, metadata: Metadata) {
		self.set_impl(Self::key(b"", height), state, metadata)
	}

	pub fn set_prefixed(
		&mut self,
		prefix: &[u8],
		height: ibc::Height,
		state: impl Into<Any>,
		metadata: Metadata,
	) {
		self.set_impl(Self::key(prefix, height), state, metadata)
	}

	pub(crate) fn prune_oldest_consensus_state(
		&mut self,
		client_state: &ClientState,
		now_ns: u64,
	) -> Result<()> {
		let start = Self::key_impl(b"", 0, 0);
		let end = Self::key_impl(b"", u64::MAX, u64::MAX);
		let record = self
			.0
			.range(Some(start.as_slice()), Some(end.as_slice()), cosmwasm_std::Order::Ascending)
			.next();
		let (key, value) = match record {
			None => return Ok(()),
			Some(pair) => pair,
		};
		let state: ConsensusState = match Self::decode(&value) {
			Ok((state, _metadata)) => state,
			Err(err) => return Err(err),
		};
		let elapsed = now_ns.saturating_sub(state.timestamp_ns.get());
		if elapsed >= client_state.trusting_period_ns {
			self.0.remove(key.as_slice());
		}
		Ok(())
	}

	pub fn del(&mut self, height: ibc::Height) {
		self.0.remove(&Self::key(b"", height))
	}

	pub fn del_prefixed(&mut self, prefix: &[u8], height: ibc::Height) {
		self.0.remove(&Self::key(prefix, height))
	}

	fn key(prefix: &[u8], height: ibc::Height) -> Vec<u8> {
		Self::key_impl(prefix, height.revision_number(), height.revision_height())
	}

	pub fn key_impl(prefix: &[u8], rev_number: u64, rev_height: u64) -> Vec<u8> {
		let rev_number = rev_number.to_be_bytes();
		let rev_height = rev_height.to_be_bytes();
		[prefix, b"consensusState/", &rev_number[..], &rev_height[..]].concat()
	}

	fn get_impl<T, E>(&self, key: &[u8]) -> Result<Option<(T, Metadata)>, E>
	where
		T: TryFrom<Any>,
		E: From<T::Error> + From<prost::DecodeError>,
	{
		self.0.get(&key).map(|value| Self::decode(value.as_slice())).transpose()
	}

	fn decode<T, E>(value: &[u8]) -> Result<(T, Metadata), E>
	where
		T: TryFrom<Any>,
		E: From<T::Error> + From<prost::DecodeError>,
	{
		let (any, metadata) = ConsensusWithMetadata::decode(value)?.into_parts();
		Ok((T::try_from(any)?, metadata))
	}

	fn set_impl(&mut self, key: Vec<u8>, state: impl Into<Any>, metadata: Metadata) {
		let state = ConsensusWithMetadata::new(state, metadata);
		self.0.set(&key, state.encode_to_vec().as_slice())
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
