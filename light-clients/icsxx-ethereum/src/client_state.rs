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
	abi::EthereumClientAbi::EthereumClientPrimitivesClientState, client_def::EthereumClient,
	error::Error, proto::ClientState as RawClientState,
};
use alloc::{format, string::ToString, vec::Vec};
use alloy_sol_types::SolValue;
use compress::{compress, decompress};
use core::{fmt::Debug, marker::PhantomData, time::Duration};
use ethabi::Address;
use ibc::{
	core::{ics02_client::client_state::ClientType, ics24_host::identifier::ChainId},
	Height,
};
use ibc_proto::google::protobuf::Any;
use serde::{Deserialize, Serialize};
use sync_committee_primitives::types::VerifierState as LightClientState;
use tendermint_proto::Protobuf;

/// Protobuf type url for GRANDPA ClientState
pub const ETHEREUM_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.ethereum.v1.ClientState";

#[derive(PartialEq, Debug, Default, Eq)]
pub struct ClientState<H> {
	pub inner: LightClientState,
	pub frozen_height: Option<Height>,
	pub latest_height: u32,
	pub ibc_core_address: Address,
	pub _phantom: PhantomData<H>,
}

impl<H> Clone for ClientState<H> {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
			frozen_height: self.frozen_height,
			latest_height: self.latest_height,
			ibc_core_address: self.ibc_core_address,
			_phantom: Default::default(),
		}
	}
}

impl<H: Clone> Protobuf<RawClientState> for ClientState<H> {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeOptions {
	latest_relay_hash: (),
}

impl<H: Clone> ClientState<H> {
	/// Verify that the client is at a sufficient height and unfrozen at the given height
	pub fn verify_height(&self, height: Height) -> Result<(), Error> {
		// let latest_para_height = Height::new(self.para_id.into(),
		// self.latest_para_height.into()); if latest_para_height < height {
		// 	return Err(Error::Custom(format!(
		// 		"Insufficient height, known height: {latest_para_height}, given height: {height}"
		// 	)))
		// }

		match self.frozen_height {
			Some(frozen_height) if frozen_height <= height =>
				Err(Error::Custom(format!("Client has been frozen at height {frozen_height}"))),
			_ => Ok(()),
		}
	}

	pub fn to_any(&self) -> Any {
		Any {
			type_url: ETHEREUM_CLIENT_STATE_TYPE_URL.to_string(),
			value: self.encode_vec().unwrap(),
		}
	}
}

impl<H> ClientState<H> {
	pub fn latest_height(&self) -> Height {
		Height::new(0, self.latest_height.into())
	}

	pub fn chain_id(&self) -> ChainId {
		ChainId::new("ethereum".to_string(), self.inner.latest_finalized_epoch)
	}

	pub fn client_type() -> ClientType {
		"xx-ethereum".to_string()
	}

	pub fn frozen_height(&self) -> Option<Height> {
		self.frozen_height
	}

	pub fn upgrade(
		mut self,
		_upgrade_height: Height,
		_upgrade_options: UpgradeOptions,
		_chain_id: ChainId,
	) -> Self {
		self.frozen_height = None;
		// Upgrade the client state
		// self.latest_relay_hash = upgrade_options.latest_relay_hash;

		self
	}

	/// Check if the state is expired when `elapsed` time has passed since the latest consensus
	/// state timestamp
	pub fn expired(&self, _elapsed: Duration) -> bool {
		// elapsed > self.relay_chain.trusting_period()
		// TODO
		false
	}

	pub fn with_frozen_height(self, h: Height) -> Result<Self, Error> {
		if h == Height::zero() {
			return Err(Error::Custom(
				"ClientState frozen height must be greater than zero".to_string(),
			))
		}
		Ok(Self { frozen_height: Some(h), ..self })
	}

	pub fn abi_encode(self) -> Vec<u8> {
		let data = EthereumClientPrimitivesClientState::from(self).abi_encode();
		let x = compress(data, 16);
		// info!("COMP={}", hex::encode(&x));
		x
	}

	pub fn abi_decode(bytes: &[u8]) -> Result<Self, Error> {
		// panic!("compressed = {}", hex::encode(&bytes));
		let decompressed = decompress(&mut &bytes[..], 16);
		// panic!("decompressed = {}", hex::encode(&decompressed));
		let value = EthereumClientPrimitivesClientState::abi_decode(&decompressed, true)?;
		value.try_into()
	}
}

impl<H> ibc::core::ics02_client::client_state::ClientState for ClientState<H>
where
	H: Send + Sync + Clone + Debug + Default + Eq,
{
	type UpgradeOptions = UpgradeOptions;
	type ClientDef = EthereumClient<H>;

	fn chain_id(&self) -> ChainId {
		self.chain_id()
	}

	fn client_def(&self) -> Self::ClientDef {
		EthereumClient::default()
	}

	fn client_type(&self) -> ClientType {
		Self::client_type()
	}

	fn latest_height(&self) -> Height {
		self.latest_height()
	}

	fn frozen_height(&self) -> Option<Height> {
		self.frozen_height()
	}

	fn upgrade(
		self,
		upgrade_height: Height,
		upgrade_options: UpgradeOptions,
		chain_id: ChainId,
	) -> Self {
		self.upgrade(upgrade_height, upgrade_options, chain_id)
	}

	fn expired(&self, elapsed: Duration) -> bool {
		self.expired(elapsed)
	}

	fn encode_to_vec(&self) -> Result<Vec<u8>, tendermint_proto::Error> {
		self.encode_vec()
	}
}

impl<H> TryFrom<RawClientState> for ClientState<H> {
	type Error = Error;

	fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
		ClientState::abi_decode(&raw.abi_data)
	}
}

impl<H> From<ClientState<H>> for RawClientState {
	fn from(client_state: ClientState<H>) -> Self {
		let abi_data = client_state.clone().abi_encode();
		RawClientState { abi_data }
	}
}

#[cfg(test)]
mod test_proto {
	fn skip_field(ptr: &[u8], wire_type: u8) -> usize {
		match wire_type {
			2 => {
				let (len, offset) = decode_varint(ptr);
				return offset + len as usize
			},
			0 => {
				let (_value, offset) = decode_varint(ptr);
				return offset
			},
			n => panic!("unknown wire type {}", n),
		}
	}

	fn decode_len_field<'a>(mut ptr: &'a [u8], index: u8) -> &'a [u8] {
		loop {
			let (field_number, wire_type, offset) = decode_tag(ptr);
			ptr = &ptr[offset..];

			if wire_type != 2 || field_number != index {
				let offset = skip_field(ptr, wire_type);
				ptr = &ptr[offset..];
				continue
			}
			let (len, offset) = decode_varint(ptr);
			ptr = &ptr[offset..];
			let value = &ptr[..len as usize];
			return value
		}
	}

	fn decode_any<'a>(ptr: &'a [u8]) -> &'a [u8] {
		let value = decode_len_field(&ptr, 2);
		value
	}

	fn decode_tag(ptr: &[u8]) -> (u8, u8, usize) {
		let tag = ptr[0];
		let wire_type = tag & 0b111;
		let field_number = tag >> 3;
		(field_number, wire_type, 1)
	}

	fn decode_varint(ptr: &[u8]) -> (u64, usize) {
		let mut value = 0u64;
		let mut shift = 0;
		let mut bytes_read = 0;
		for i in 0.. {
			let byte = ptr[i];
			value |= ((byte & 0b0111_1111) as u64) << shift;
			shift += 7;
			bytes_read += 1;
			if byte & 0b1000_0000 == 0 {
				break
			}
		}
		(value, bytes_read)
	}

	#[test]
	fn test_proto() {
		let data =
			hex::decode(include_bytes!("/Users/vmark/work/centauri-private/proto.txt")).unwrap();
		let mut ptr = &mut &data[..];

		let wasm_any = decode_any(&mut ptr);
		let wasm_data = decode_len_field(wasm_any, 1);
		let client_state_any = decode_any(wasm_data);
		let _abi_data = decode_len_field(client_state_any, 1);
		// println!("abi_data = {}", hex::encode(abi_data));
	}
}
