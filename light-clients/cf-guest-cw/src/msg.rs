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

use alloc::borrow::Cow;

use cosmwasm_schema::cw_serde;
use prost::Message;

use crate::{ibc, state};

#[cw_serde]
pub struct InstantiateMessage {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub client_state: state::ClientState,
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub consensus_state: state::ConsensusState,
}

#[cw_serde]
pub struct GenesisMetadata {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub hash: Vec<u8>,
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub value: Vec<u8>,
}

#[cw_serde]
pub struct QueryResponse {
	pub is_valid: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub genesis_metadata: Option<Vec<GenesisMetadata>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub found_misbehaviour: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timestamp: Option<u64>,
}

impl QueryResponse {
	pub fn success() -> Self {
		Self {
			is_valid: true,
			status: None,
			genesis_metadata: None,
			found_misbehaviour: None,
			timestamp: None,
		}
	}

	pub fn status(mut self, status: String) -> Self {
		self.status = Some(status);
		self
	}

	pub fn genesis_metadata(mut self, genesis_metadata: Option<Vec<GenesisMetadata>>) -> Self {
		self.genesis_metadata = genesis_metadata;
		self
	}

	pub fn misbehaviour(mut self, found_misbehavior: bool) -> Self {
		self.found_misbehaviour = Some(found_misbehavior);
		self
	}

	pub fn timestamp(mut self, timestamp: u64) -> Self {
		self.timestamp = Some(timestamp);
		self
	}
}

#[cw_serde]
pub struct ContractResult {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub heights: Option<Vec<ibc::Height>>,
}

impl ContractResult {
	pub fn success() -> Self {
		Self { heights: None }
	}

	pub fn heights(mut self, heights: Vec<ibc::Height>) -> Self {
		self.heights = Some(heights);
		self
	}
}

#[cw_serde]
pub enum SudoMsg {
	MigrateClientStore(MigrateClientStoreMsg),
	UpdateStateOnMisbehaviour(UpdateStateOnMisbehaviourMsg),
	UpdateState(UpdateStateMsg),
	VerifyMembership(VerifyMembershipMsg),
	VerifyNonMembership(VerifyNonMembershipMsg),
	VerifyUpgradeAndUpdateState(VerifyUpgradeAndUpdateStateMsg),
}

#[cw_serde]
pub enum QueryMsg {
	CheckForMisbehaviour(CheckForMisbehaviourMsg),
	GetLatestHeightsMsg(GetLatestHeightsMsg),
	ExportMetadata(ExportMetadataMsg),
	Status(StatusMsg),
	TimestampAtHeight(TimestampAtHeightMsg),
	VerifyClientMessage(VerifyClientMessage),
}

#[cw_serde]
pub struct TimestampAtHeightMsg {
	#[serde(flatten)]
	pub height: ibc::Height,
}

#[cw_serde]
pub struct GetLatestHeightsMsg {}

#[cw_serde]
pub struct StatusMsg {}

#[cw_serde]
pub struct ExportMetadataMsg {}

#[cw_serde]
pub struct VerifyClientMessage {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub client_message: state::ClientMessage,
}

#[cw_serde]
pub struct CheckForMisbehaviourMsg {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub client_message: state::Misbehaviour,
}

#[cw_serde]
pub struct UpdateStateMsg {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub header: state::Header,
}

#[cw_serde]
pub struct UpdateStateOnMisbehaviourMsg {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub misbehaviour_message: state::Misbehaviour,
}

#[cw_serde]
pub struct MigrateClientStoreMsg {}

#[cw_serde]
pub struct VerifyMembershipMsg {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub proof: Vec<u8>,
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub path: Vec<u8>,
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub value: Vec<u8>,
	#[serde(flatten)]
	pub height: ibc::Height,
}

#[cw_serde]
pub struct VerifyNonMembershipMsg {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub proof: ibc::CommitmentProofBytes,
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub path: Vec<u8>,
	#[serde(flatten)]
	pub height: ibc::Height,
}

#[cw_serde]
pub struct VerifyUpgradeAndUpdateStateMsg {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub upgrade_client_state: state::ClientState,
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub upgrade_consensus_state: state::ConsensusState,
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub proof_upgrade_client: ibc::CommitmentProofBytes,
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub proof_upgrade_consensus_state: ibc::CommitmentProofBytes,
}

struct Base64;

impl Base64 {
	pub fn serialize<T: BytesConv, S: serde::ser::Serializer>(
		obj: &T,
		ser: S,
	) -> Result<S::Ok, S::Error> {
		use base64::engine::Engine;

		let bytes = obj.to_bytes()?;
		let bytes = bytes.as_ref();
		// Unfortunately because there’s no `<&Binary>::From<&Vec>` we
		// need to open-code base64 encoding.  TODO(mina86): Change it
		// once https://github.com/CosmWasm/cosmwasm/pull/2036 lands.
		ser.serialize_str(&base64::engine::general_purpose::STANDARD.encode(bytes))
	}

	pub fn deserialize<'de, T: BytesConv, D: serde::Deserializer<'de>>(
		de: D,
	) -> Result<T, D::Error> {
		serde::Deserialize::deserialize(de).and_then(T::from_bytes)
	}
}

trait BytesConv: Sized {
	fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E>;
	fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E>;
}

impl BytesConv for Vec<u8> {
	fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E> {
		Ok(Cow::Borrowed(self.as_slice()))
	}

	fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E> {
		Ok(bytes)
	}
}

impl BytesConv for ibc::CommitmentProofBytes {
	fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E> {
		Ok(Cow::Borrowed(self.as_ref()))
	}

	fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E> {
		Self::try_from(bytes).map_err(E::custom)
	}
}

macro_rules! conv_via_any {
	($msg:ty) => {
		impl BytesConv for $msg {
			fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E> {
				Ok(Cow::Owned(ibc::proto::Any::from(self).encode_to_vec()))
			}

			fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E> {
				let any = ibc::proto::Any::decode(bytes.as_slice()).map_err(E::custom)?;
				<$msg>::try_from(any).map_err(E::custom)
			}
		}
	};
}

conv_via_any!(state::ClientMessage);
conv_via_any!(state::ClientState);
conv_via_any!(state::ConsensusState);
conv_via_any!(state::Header);
conv_via_any!(state::Misbehaviour);
