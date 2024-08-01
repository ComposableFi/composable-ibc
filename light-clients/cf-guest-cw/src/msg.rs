// // Copyright (C) 2022 ComposableFi.
// // SPDX-License-Identifier: Apache-2.0

// // Licensed under the Apache License, Version 2.0 (the "License");
// // you may not use this file except in compliance with the License.
// // You may obtain a copy of the License at
// //
// // 	http://www.apache.org/licenses/LICENSE-2.0
// //
// // Unless required by applicable law or agreed to in writing, software
// // distributed under the License is distributed on an "AS IS" BASIS,
// // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// // See the License for the specific language governing permissions and
// // limitations under the License.

use crate::{ics23::FakeInner, Bytes, ContractError};
use core::str::FromStr;
use cosmwasm_schema::cw_serde;
use ibc::{
	core::{
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes},
		ics24_host::Path,
	},
	protobuf::Protobuf,
	Height,
};
use ibc_proto::{google::protobuf::Any, ibc::core::client::v1::Height as HeightRaw};

use cf_guest::{ClientMessage, ClientState};

use ics08_wasm::{
	client_message::ClientMessage as WasmHeader, client_state::ClientState as WasmClientState,
	consensus_state::ConsensusState as WasmConsensusState,
};
use prost::Message;
use serde::{Deserializer, Serializer};

struct Base64;

impl Base64 {
	pub fn serialize<S: Serializer>(v: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
		ibc_proto::base64::serialize(v, serializer)
	}

	pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
		ibc_proto::base64::deserialize(deserializer)
	}
}

#[cw_serde]
pub struct GenesisMetadata {
	pub key: Vec<u8>,
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
	pub heights: Option<Vec<Height>>,
}

impl ContractResult {
	pub fn success() -> Self {
		Self { heights: None }
	}

	pub fn heights(mut self, heights: Vec<Height>) -> Self {
		self.heights = Some(heights);
		self
	}
}

#[cw_serde]
pub enum SudoMsg {
	MigrateClientStore(MigrateClientStoreMsg),
	UpdateStateOnMisbehaviour(UpdateStateOnMisbehaviourMsgRaw),
	UpdateState(UpdateStateMsgRaw),
	VerifyMembership(VerifyMembershipMsgRaw),
	VerifyNonMembership(VerifyNonMembershipMsgRaw),
	VerifyUpgradeAndUpdateState(VerifyUpgradeAndUpdateStateMsgRaw),
}

#[cw_serde]
pub enum QueryMsg {
	CheckForMisbehaviour(CheckForMisbehaviourMsgRaw),
	ClientTypeMsg(ClientTypeMsg),
	GetLatestHeightsMsg(GetLatestHeightsMsg),
	ExportMetadata(ExportMetadataMsg),
	Status(StatusMsg),
	TimestampAtHeight(TimestampAtHeightMsg),
	VerifyClientMessage(VerifyClientMessageRaw),
}

#[cw_serde]
pub struct TimestampAtHeightMsg {
	pub height: Height,
}

#[cw_serde]
pub struct ClientTypeMsg {}

#[cw_serde]
pub struct GetLatestHeightsMsg {}

#[cw_serde]
pub struct StatusMsg {}

#[cw_serde]
pub struct ExportMetadataMsg {}

#[cw_serde]
pub struct MerklePath {
	pub key_path: Vec<String>,
}

#[cw_serde]
pub struct VerifyMembershipMsgRaw {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub proof: Bytes,
	pub path: MerklePath,
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub value: Bytes,
	pub height: HeightRaw,
	pub delay_block_period: u64,
	pub delay_time_period: u64,
}

pub struct VerifyMembershipMsg {
	pub prefix: CommitmentPrefix,
	pub proof: CommitmentProofBytes,
	pub path: Path,
	pub value: Vec<u8>,
	pub height: Height,
	pub delay_block_period: u64,
	pub delay_time_period: u64,
}

impl TryFrom<VerifyMembershipMsgRaw> for VerifyMembershipMsg {
	type Error = ContractError;

	fn try_from(mut raw: VerifyMembershipMsgRaw) -> Result<Self, Self::Error> {
		let proof = CommitmentProofBytes::try_from(raw.proof)?;
		let prefix = raw.path.key_path.remove(0).into_bytes();
		let path_str = raw.path.key_path.join("");
		let path = Path::from_str(&path_str)?;
		let height = Height::from(raw.height);
		Ok(Self {
			proof,
			path,
			value: raw.value,
			height,
			prefix: CommitmentPrefix::try_from(prefix)?,
			delay_block_period: raw.delay_block_period,
			delay_time_period: raw.delay_time_period,
		})
	}
}

#[cw_serde]
pub struct MigrateClientStoreMsg {}

#[cw_serde]
pub struct VerifyNonMembershipMsgRaw {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub proof: Bytes,
	pub path: MerklePath,
	pub height: HeightRaw,
	pub delay_block_period: u64,
	pub delay_time_period: u64,
}

pub struct VerifyNonMembershipMsg {
	pub prefix: CommitmentPrefix,
	pub proof: CommitmentProofBytes,
	pub path: Path,
	pub height: Height,
	pub delay_block_period: u64,
	pub delay_time_period: u64,
}

#[cw_serde]
pub struct VerifyClientMessageRaw2 {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub client_message: Bytes,
}

impl TryFrom<VerifyNonMembershipMsgRaw> for VerifyNonMembershipMsg {
	type Error = ContractError;

	fn try_from(mut raw: VerifyNonMembershipMsgRaw) -> Result<Self, Self::Error> {
		let proof = CommitmentProofBytes::try_from(raw.proof)?;
		let prefix = raw.path.key_path.remove(0).into_bytes();
		let path_str = raw.path.key_path.join("");
		let path = Path::from_str(&path_str)?;
		let height = Height::from(raw.height);
		Ok(Self {
			proof,
			path,
			height,
			prefix: CommitmentPrefix::try_from(prefix)?,
			delay_block_period: raw.delay_block_period,
			delay_time_period: raw.delay_time_period,
		})
	}
}

impl TryFrom<VerifyClientMessageRaw2> for VerifyClientMessage {
	type Error = ContractError;

	fn try_from(raw: VerifyClientMessageRaw2) -> Result<Self, Self::Error> {
		let client_message = Self::decode_client_message_raw(raw.client_message)?;
		Ok(Self { client_message })
	}
}

#[cw_serde]
pub struct WasmMisbehaviour {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub data: Bytes,
}

#[cw_serde]
pub enum ClientMessageRaw {
	Header(WasmHeader<FakeInner>),
	Misbehaviour(WasmMisbehaviour),
}

#[cw_serde]
pub struct VerifyClientMessageRaw {
	pub client_message: ClientMessageRaw,
}

pub struct VerifyClientMessage {
	pub client_message: ClientMessage<crate::crypto::PubKey>,
}

impl TryFrom<VerifyClientMessageRaw> for VerifyClientMessage {
	type Error = ContractError;

	fn try_from(raw: VerifyClientMessageRaw) -> Result<Self, Self::Error> {
		let client_message = Self::decode_client_message(raw.client_message)?;
		Ok(Self { client_message })
	}
}

impl VerifyClientMessage {
	fn decode_client_message(
		raw: ClientMessageRaw,
	) -> Result<ClientMessage<crate::crypto::PubKey>, ContractError> {
		let client_message = match raw {
			ClientMessageRaw::Header(header) => {
				let any = Any::decode(&mut header.data.as_slice())?;
				ClientMessage::decode_vec(&any.value)?.into()
			},
			ClientMessageRaw::Misbehaviour(misbehaviour) => {
				let any = Any::decode(&mut misbehaviour.data.as_slice())?;
				ClientMessage::decode_vec(&any.value)?.into()
			},
		};
		Ok(client_message)
	}
	fn decode_client_message_raw(raw: Bytes) -> Result<ClientMessage<crate::crypto::PubKey>, ContractError> {
		let any = Any::decode(&mut raw.as_slice())?;
		let client_message = cf_guest::ClientMessage::decode_vec(&any.value)?;
		// let client_message = match &*any.type_url {
		// 	cf_guest::proto::Header::IBC_TYPE_URL => ClientMessage::Header(Header::decode_vec(&any.value)?),
		// 	cf_guest::proto::Misbehaviour::IBC_TYPE_URL => {
		// 		ClientMessage::Misbehaviour(Misbehaviour::decode_vec(&any.value)?)
		// 	},
		// 	_ => return Err(ContractError::Client("unknown client message type".to_string())),
		// };
		Ok(client_message)
	}
}

#[cw_serde]
pub struct CheckForMisbehaviourMsgRaw {
	pub client_message: ClientMessageRaw,
}

pub struct CheckForMisbehaviourMsg {
	pub client_message: ClientMessage<crate::crypto::PubKey>,
}

#[cw_serde]
pub struct CheckForMisbehaviourMsgRaw2 {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub client_message: Bytes,
}

impl TryFrom<CheckForMisbehaviourMsgRaw> for CheckForMisbehaviourMsg {
	type Error = ContractError;

	fn try_from(raw: CheckForMisbehaviourMsgRaw) -> Result<Self, Self::Error> {
		let client_message = VerifyClientMessage::decode_client_message(raw.client_message)?;
		Ok(Self { client_message })
	}
}

impl TryFrom<CheckForMisbehaviourMsgRaw2> for CheckForMisbehaviourMsg {
	type Error = ContractError;

	fn try_from(raw: CheckForMisbehaviourMsgRaw2) -> Result<Self, Self::Error> {
		let client_message = VerifyClientMessage::decode_client_message_raw(raw.client_message)?;
		Ok(Self { client_message })
	}
}

#[cw_serde]
pub struct UpdateStateOnMisbehaviourMsgRaw {
	pub client_message: ClientMessageRaw,
}

pub struct UpdateStateOnMisbehaviourMsg {
	pub client_message: ClientMessage<crate::crypto::PubKey>,
}

#[cw_serde]
pub struct UpdateStateMsgRawSdk50 {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub client_message: Bytes,
}

impl TryFrom<UpdateStateOnMisbehaviourMsgRaw> for UpdateStateOnMisbehaviourMsg {
	type Error = ContractError;

	fn try_from(raw: UpdateStateOnMisbehaviourMsgRaw) -> Result<Self, Self::Error> {
		let client_message = VerifyClientMessage::decode_client_message(raw.client_message)?;
		Ok(Self { client_message })
	}
}

#[cw_serde]
pub struct UpdateStateMsgRaw {
	pub client_message: ClientMessageRaw,
}

pub struct UpdateStateMsg {
	pub client_message: ClientMessage<crate::crypto::PubKey>,
}

impl TryFrom<UpdateStateMsgRaw> for UpdateStateMsg {
	type Error = ContractError;

	fn try_from(raw: UpdateStateMsgRaw) -> Result<Self, Self::Error> {
		let client_message = VerifyClientMessage::decode_client_message(raw.client_message)?;
		Ok(Self { client_message })
	}
}

impl TryFrom<UpdateStateMsgRawSdk50> for UpdateStateMsg {
	type Error = ContractError;

	fn try_from(raw: UpdateStateMsgRawSdk50) -> Result<Self, Self::Error> {
		let client_message = VerifyClientMessage::decode_client_message_raw(raw.client_message)?;
		Ok(Self { client_message })
	}
}

#[cw_serde]
pub struct CheckSubstituteAndUpdateStateMsg {}

#[cw_serde]
pub struct VerifyUpgradeAndUpdateStateMsgRaw {
	pub upgrade_client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
	pub upgrade_consensus_state: WasmConsensusState<FakeInner>,
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub proof_upgrade_client: Bytes,
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub proof_upgrade_consensus_state: Bytes,
}

pub struct VerifyUpgradeAndUpdateStateMsg {
	pub upgrade_client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
	pub upgrade_consensus_state: WasmConsensusState<FakeInner>,
	pub proof_upgrade_client: CommitmentProofBytes,
	pub proof_upgrade_consensus_state: CommitmentProofBytes,
}

impl TryFrom<VerifyUpgradeAndUpdateStateMsgRaw> for VerifyUpgradeAndUpdateStateMsg {
	type Error = ContractError;

	fn try_from(raw: VerifyUpgradeAndUpdateStateMsgRaw) -> Result<Self, Self::Error> {
		let any = Any::decode(&mut raw.upgrade_client_state.data.as_slice())?;
		let upgrade_client_state: ClientState<crate::crypto::PubKey> =
			ClientState::decode_vec(&any.value)?;
		if upgrade_client_state.0.is_frozen {
			return Err(ContractError::Other("Upgrade client state not zeroed".into()));
		}

		Ok(VerifyUpgradeAndUpdateStateMsg {
			upgrade_client_state: raw.upgrade_client_state,
			upgrade_consensus_state: raw.upgrade_consensus_state,
			proof_upgrade_client: CommitmentProofBytes::try_from(raw.proof_upgrade_client)?,
			proof_upgrade_consensus_state: CommitmentProofBytes::try_from(
				raw.proof_upgrade_consensus_state,
			)?,
		})
	}
}

#[test]
fn testing() {
	let mut x: Vec<u8> = vec![
		10, 36, 47, 108, 105, 103, 104, 116, 99, 108, 105, 101, 110, 116, 115, 46, 103, 117, 101,
		115, 116, 46, 118, 49, 46, 67, 108, 105, 101, 110, 116, 77, 101, 115, 115, 97, 103, 101,
		18, 173, 2, 10, 170, 2, 10, 32, 245, 46, 29, 47, 245, 89, 230, 120, 70, 62, 107, 19, 173,
		50, 168, 129, 194, 176, 205, 246, 105, 25, 104, 215, 11, 180, 151, 90, 231, 176, 168, 17,
		18, 122, 0, 120, 76, 22, 77, 119, 10, 26, 8, 44, 224, 43, 117, 132, 119, 163, 69, 136, 251,
		30, 49, 228, 55, 5, 23, 15, 218, 35, 78, 24, 246, 66, 89, 154, 5, 0, 0, 0, 0, 0, 0, 36,
		105, 0, 0, 0, 0, 0, 0, 0, 230, 100, 238, 244, 29, 201, 23, 181, 56, 107, 15, 133, 89, 139,
		8, 120, 213, 21, 182, 7, 43, 185, 89, 104, 167, 149, 127, 236, 243, 165, 6, 89, 227, 23,
		221, 176, 228, 218, 142, 245, 46, 29, 47, 245, 89, 230, 120, 70, 62, 107, 19, 173, 50, 168,
		129, 194, 176, 205, 246, 105, 25, 104, 215, 11, 180, 151, 90, 231, 176, 168, 17, 0, 26, 70,
		0, 1, 0, 0, 0, 0, 12, 8, 4, 81, 129, 165, 153, 230, 192, 225, 51, 119, 216, 14, 69, 225,
		73, 7, 204, 144, 39, 213, 91, 255, 136, 38, 95, 131, 197, 4, 101, 186, 208, 7, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 233, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 66, 18,
		64, 243, 241, 195, 11, 26, 46, 157, 148, 16, 26, 191, 59, 144, 193, 172, 154, 201, 88, 35,
		219, 229, 111, 161, 138, 21, 85, 17, 152, 66, 128, 155, 181, 214, 186, 126, 224, 5, 147,
		19, 158, 82, 136, 28, 115, 226, 14, 135, 3, 104, 51, 240, 198, 75, 3, 198, 72, 118, 214,
		187, 160, 75, 118, 80, 5,
	];
	let any = Any::decode(&mut x.as_slice()).unwrap();
	println!("{:?}", any);
	let header: cf_guest::ClientMessage<crate::crypto::PubKey> =
		ClientMessage::decode_vec(&any.value).unwrap();
	println!("{:?}", header);
}
