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

use crate::{
	fake_inner::FakeInner,
	ibc,
	ibc::{proto::google::protobuf::Any, protobuf::Protobuf},
};

type Result<T, E = crate::Error> = core::result::Result<T, E>;

pub type ClientMessage = cf_guest::ClientMessage<crate::PubKey>;
pub type ClientState = cf_guest::ClientState<crate::PubKey>;
pub type ConsensusState = cf_guest::ConsensusState;
pub type Header = cf_guest::Header<crate::PubKey>;
pub type Misbehaviour = cf_guest::Misbehaviour<crate::PubKey>;

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

	pub fn set(&mut self, state: impl Into<Any>) {
		self.set_impl(Self::KEY, state)
	}

	const KEY: &'static [u8] = b"clientState";

	fn get_impl(&self, key: &[u8]) -> Result<Option<ClientState>> {
		let value = match self.0.get(&key) {
			None => return Ok(None),
			Some(value) => value,
		};
		let any = Any::decode(value.as_slice())?;
		let wasm_state =
			ics08_wasm::client_state::ClientState::<FakeInner, FakeInner, FakeInner>::decode_vec(
				&any.value,
			)?;
		let any = Any::decode(wasm_state.data.as_slice())?;
		Ok(Some(ClientState::try_from(any)?))
	}

	fn set_impl(&mut self, key: &[u8], state: impl Into<Any>) {
		self.0.set(&key, state.into().encode_to_vec().as_slice())
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

	pub fn get<T, E>(&self, height: ibc::Height) -> Result<Option<(T, Metadata)>, E>
	where
		T: TryFrom<Any>,
		E: From<T::Error> + From<prost::DecodeError>,
	{
		self.get_impl(&Self::key(height))
	}

	pub fn set(&mut self, height: ibc::Height, state: impl Into<Any>, metadata: Metadata) {
		self.set_impl(Self::key(height), state, metadata)
	}

	fn all<'a>(
		&'a self,
	) -> impl Iterator<Item = Result<(Vec<u8>, Any, Metadata), prost::DecodeError>> + 'a {
		self.0
			.range(
				Some(Self::key_impl(0, 0).as_slice()),
				Some(Self::key_impl(u64::MAX, u64::MAX).as_slice()),
				cosmwasm_std::Order::Ascending,
			)
			.map(|(key, value)| {
				let (any, metadata) = ConsensusWithMetadata::decode(value.as_slice())?.into_parts();
				Ok((key, any, metadata))
			})
	}

	pub fn prune_oldest_consensus_state(
		&mut self,
		client_state: &ClientState,
		now_ns: u64,
	) -> Result<()> {
		let (key, any) = match self.all().next() {
			None => return Ok(()),
			Some(Err(err)) => return Err(err.into()),
			Some(Ok((key, any, _metadata))) => (key, any),
		};
		let state = ConsensusState::try_from(any)?;
		let elapsed = now_ns.saturating_sub(state.timestamp_ns.get());
		if elapsed >= client_state.trusting_period_ns {
			self.0.remove(key.as_slice());
		}
		Ok(())
	}

	pub fn del(&mut self, height: ibc::Height) {
		self.0.remove(&Self::key(height))
	}

	fn key(height: ibc::Height) -> Vec<u8> {
		Self::key_impl(height.revision_number, height.revision_height)
	}

	fn key_impl(rev_number: u64, rev_height: u64) -> Vec<u8> {
		let rev_number = rev_number.to_be_bytes();
		let rev_height = rev_height.to_be_bytes();
		[b"consensusStates/", &rev_number[..], b"-", &rev_height[..]].concat()
	}

	fn get_impl<T, E>(&self, key: &[u8]) -> Result<Option<(T, Metadata)>, E>
	where
		T: TryFrom<Any>,
		E: From<T::Error> + From<prost::DecodeError>,
	{
		// panic!("key {:?}",  String::from_utf8(key);
		let value = match self.0.get(&key) {
			None => return Ok(None),
			Some(value) => value,
		};
		let (any, metadata) = ConsensusWithMetadata::decode(value.as_slice())?.into_parts();
		Ok(Some((T::try_from(any)?, metadata)))
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

mod tests {
	use cf_guest::ClientState;
	use ibc_proto::google::protobuf::Any;

	use crate::crypto::PubKey;

	#[test]
	fn test_something() {
		use prost::Message;
		let value: Vec<u8> = vec![
			10, 37, 47, 105, 98, 99, 46, 108, 105, 103, 104, 116, 99, 108, 105, 101, 110, 116, 115,
			46, 119, 97, 115, 109, 46, 118, 49, 46, 67, 108, 105, 101, 110, 116, 83, 116, 97, 116,
			101, 18, 179, 1, 10, 135, 1, 10, 52, 99, 111, 109, 112, 111, 115, 97, 98, 108, 101, 46,
			102, 105, 110, 97, 110, 99, 101, 47, 108, 105, 103, 104, 116, 99, 108, 105, 101, 110,
			116, 115, 46, 103, 117, 101, 115, 116, 46, 118, 49, 46, 67, 108, 105, 101, 110, 116,
			83, 116, 97, 116, 101, 18, 79, 10, 32, 23, 73, 231, 109, 175, 164, 147, 98, 253, 128,
			9, 235, 178, 102, 105, 78, 146, 41, 118, 36, 214, 13, 209, 83, 105, 66, 53, 156, 72,
			20, 209, 47, 16, 241, 4, 24, 128, 128, 144, 202, 210, 198, 14, 34, 32, 23, 73, 231,
			109, 175, 164, 147, 98, 253, 128, 9, 235, 178, 102, 105, 78, 146, 41, 118, 36, 214, 13,
			209, 83, 105, 66, 53, 156, 72, 20, 209, 47, 18, 32, 209, 131, 161, 8, 250, 155, 213,
			117, 252, 41, 115, 130, 66, 71, 146, 79, 9, 83, 151, 243, 192, 113, 135, 219, 133, 172,
			74, 201, 183, 16, 77, 4, 26, 5, 8, 1, 16, 241, 4,
		];
		let any = Any::decode(value.as_slice()).expect("jaskldjald");
		ClientState::<PubKey>::try_from(any).unwrap();
	}
}
