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

use ::ibc::Height;
use cosmwasm_std::Storage;
use prost::Message;

use crate::{
	fake_inner::FakeInner,
	ibc,
	ibc::{proto::google::protobuf::Any, protobuf::Protobuf},
};
use ::ibc::core::ics02_client::client_consensus::ConsensusState as ConsensusStateTrait;
use ibc_proto::ibc::lightclients::wasm::v1::ClientState as RawClientState;

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

	pub fn set(&mut self, state: ClientState) {
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
		Ok(Some(ClientState::decode(any.value.as_slice())?))
	}

	fn set_impl(&mut self, key: &[u8], state: ClientState) {
		let value = match self.0.get(&key) {
			None => panic!("No value found"),
			Some(value) => value,
		};
		let any = Any::decode(value.as_slice()).unwrap();
		let mut wasm_client_state =
			ics08_wasm::client_state::ClientState::<FakeInner, FakeInner, FakeInner>::decode_vec(
				&any.value,
			)
			.unwrap();
		wasm_client_state.data = state.encode_to_vec().unwrap();
		wasm_client_state.latest_height = Height::new(1, u64::from(state.latest_height));
		let vec1 = wasm_client_state.to_any().encode_to_vec();
		// let raw_client_state = RawClientState {
		// 	data: state.encode_to_vec().unwrap(),
		// 	code_id: wasm_state.code_id,
		// 	latest_height: Some(wasm_state.latest_height.into()),
		// };
		// let wasm_client_state: ics08_wasm::client_state::ClientState<FakeInner, FakeInner,
		// FakeInner> = ics08_wasm::client_state::ClientState::try_from(raw_client_state).unwrap();
		self.0.set(&key, vec1.as_slice())
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

	pub fn set(&mut self, height: ibc::Height, state: ConsensusState, metadata: Metadata) {
		self.set_impl(Self::key(height), state, metadata)
	}

	fn all<'a>(
		&'a self,
	) -> impl Iterator<Item = Result<(Vec<u8>, Any, Metadata), prost::DecodeError>> + 'a {
		self.0
			.range(
				Some(Self::key(Height::new(0, 0)).as_slice()),
				Some(Self::key(Height::new(u64::MAX, u64::MAX)).as_slice()),
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
		let wasm_state =
			ics08_wasm::consensus_state::ConsensusState::<FakeInner>::decode_vec(&any.value)
				.unwrap();
		let any = Any::decode(wasm_state.data.as_slice())?;
		let state = ConsensusState::decode(any.value.as_slice())?;
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
		format!("consensusStates/{height}").into_bytes()
	}

	fn get_impl(&self, key: &[u8]) -> Result<Option<ConsensusState>> {
		let value = match self.0.get(&key) {
			None => return Ok(None),
			Some(value) => value,
		};
		let any = Any::decode(value.as_slice())?;
		let wasm_state =
			ics08_wasm::consensus_state::ConsensusState::<FakeInner>::decode_vec(&any.value)
				.unwrap();
		let any = Any::decode(wasm_state.data.as_slice())?;
		Ok(Some(ConsensusState::decode(any.value.as_slice())?))
	}

	fn set_impl(&mut self, key: Vec<u8>, consensus_state: ConsensusState, metadata: Metadata) {
		let wasm_consensus_state = ics08_wasm::consensus_state::ConsensusState {
			data: consensus_state.encode_to_vec().unwrap(),
			timestamp: consensus_state.timestamp().nanoseconds(),
			inner: Box::new(FakeInner),
		};
		let wasm_consensus_state = wasm_consensus_state.to_any().encode_to_vec();
		// let state = ConsensusWithMetadata::new(state, metadata);
		self.0.set(&key, wasm_consensus_state.as_slice())
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
			10, 54, 99, 111, 109, 112, 111, 115, 97, 98, 108, 101, 46, 102, 105, 110, 97, 110, 99,
			101, 47, 108, 105, 103, 104, 116, 99, 108, 105, 101, 110, 116, 115, 46, 103, 117, 101,
			115, 116, 46, 118, 49, 46, 67, 108, 105, 101, 110, 116, 77, 101, 115, 115, 97, 103,
			101, 18, 222, 2, 10, 47, 99, 111, 109, 112, 111, 115, 97, 98, 108, 101, 46, 102, 105,
			110, 97, 110, 99, 101, 47, 108, 105, 103, 104, 116, 99, 108, 105, 101, 110, 116, 115,
			46, 103, 117, 101, 115, 116, 46, 118, 49, 46, 72, 101, 97, 100, 101, 114, 18, 170, 2,
			10, 32, 101, 237, 176, 45, 187, 52, 214, 1, 58, 10, 117, 77, 241, 134, 115, 208, 230,
			118, 88, 164, 160, 16, 82, 154, 235, 236, 229, 166, 167, 103, 43, 143, 18, 122, 0, 94,
			197, 73, 107, 151, 104, 159, 43, 144, 178, 143, 109, 122, 147, 221, 198, 90, 74, 94,
			132, 195, 93, 142, 39, 183, 173, 18, 146, 28, 173, 194, 147, 199, 0, 0, 0, 0, 0, 0, 0,
			62, 10, 0, 0, 0, 0, 0, 0, 0, 164, 28, 75, 93, 199, 188, 23, 20, 2, 191, 82, 235, 2,
			150, 30, 106, 159, 131, 167, 151, 71, 206, 243, 65, 57, 216, 20, 32, 46, 50, 71, 129,
			218, 248, 97, 79, 68, 12, 171, 101, 237, 176, 45, 187, 52, 214, 1, 58, 10, 117, 77,
			241, 134, 115, 208, 230, 118, 88, 164, 160, 16, 82, 154, 235, 236, 229, 166, 167, 103,
			43, 143, 0, 26, 70, 0, 1, 0, 0, 0, 0, 12, 8, 4, 81, 129, 165, 153, 230, 192, 225, 51,
			119, 216, 14, 69, 225, 73, 7, 204, 144, 39, 213, 91, 255, 136, 38, 95, 131, 197, 4,
			101, 186, 208, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 233, 3, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 34, 66, 18, 64, 45, 11, 100, 232, 23, 25, 151, 70, 245, 58, 39,
			54, 227, 197, 46, 148, 178, 61, 250, 97, 208, 158, 242, 48, 110, 23, 31, 112, 77, 205,
			81, 236, 82, 186, 67, 198, 132, 122, 129, 246, 136, 74, 236, 220, 218, 254, 208, 152,
			229, 3, 76, 0, 224, 46, 100, 131, 89, 248, 101, 71, 221, 16, 173, 2,
		];
		let any = Any::decode(value.as_slice()).expect("jaskldjald");
		println!("This is any {:?}", any);
		ClientState::<PubKey>::try_from(any).unwrap();
	}
}
