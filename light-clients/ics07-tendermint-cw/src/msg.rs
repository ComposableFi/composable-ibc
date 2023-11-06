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

use crate::{contract::HostFunctions, ics23::FakeInner, Bytes, ContractError};
use core::{str::FromStr, time::Duration};
use cosmwasm_schema::cw_serde;
use ibc::{
	core::{
		ics02_client::trust_threshold::TrustThreshold,
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes},
		ics24_host::Path,
	},
	protobuf::Protobuf,
	Height,
};
use ibc_proto::{google::protobuf::Any, ibc::core::client::v1::Height as HeightRaw};
use ics07_tendermint::{
	client_message::{ClientMessage, Header, Misbehaviour, TENDERMINT_HEADER_TYPE_URL, TENDERMINT_MISBEHAVIOUR_TYPE_URL},
	client_state::ClientState,
};
use ics08_wasm::{
	client_state::ClientState as WasmClientState,
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
		Self { is_valid: true, status: None, genesis_metadata: None, found_misbehaviour: None, timestamp: None }
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

	pub fn error(msg: String) -> Self {
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
pub struct ClientMessageRaw {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub data: Bytes,
}

#[cw_serde]
pub struct VerifyClientMessageRaw {
	pub client_message: ClientMessageRaw,
}

pub struct VerifyClientMessage {
	pub client_message: ClientMessage,
}

impl TryFrom<VerifyClientMessageRaw> for VerifyClientMessage {
	type Error = ContractError;

	fn try_from(raw: VerifyClientMessageRaw) -> Result<Self, Self::Error> {
		let client_message = Self::decode_client_message(raw.client_message)?;
		Ok(Self { client_message })
	}
}

impl VerifyClientMessage {
	fn decode_client_message(raw: ClientMessageRaw) -> Result<ClientMessage, ContractError> {
		let any = Any::decode(raw.data.as_slice())?;
		let client_message = match &*any.type_url {
			TENDERMINT_HEADER_TYPE_URL => ClientMessage::Header(Header::decode_vec(&any.value)?),
			TENDERMINT_MISBEHAVIOUR_TYPE_URL => ClientMessage::Misbehaviour(Misbehaviour::decode_vec(&any.value)?),
			_ => return Err(ContractError::Tendermint("unknown client message type".to_string())),
		};
		Ok(client_message)
	}
}

#[cw_serde]
pub struct CheckForMisbehaviourMsgRaw {
	pub client_message: ClientMessageRaw,
}

pub struct CheckForMisbehaviourMsg {
	pub client_message: ClientMessage,
}

impl TryFrom<CheckForMisbehaviourMsgRaw> for CheckForMisbehaviourMsg {
	type Error = ContractError;

	fn try_from(raw: CheckForMisbehaviourMsgRaw) -> Result<Self, Self::Error> {
		let client_message = VerifyClientMessage::decode_client_message(raw.client_message)?;
		Ok(Self { client_message })
	}
}

#[cw_serde]
pub struct UpdateStateOnMisbehaviourMsgRaw {
	pub client_message: ClientMessageRaw,
}

pub struct UpdateStateOnMisbehaviourMsg {
	pub client_message: ClientMessage,
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
	pub client_message: ClientMessage,
}

impl TryFrom<UpdateStateMsgRaw> for UpdateStateMsg {
	type Error = ContractError;

	fn try_from(raw: UpdateStateMsgRaw) -> Result<Self, Self::Error> {
		let client_message = VerifyClientMessage::decode_client_message(raw.client_message)?;
		Ok(Self { client_message })
	}
}

#[cw_serde]
pub struct MigrateClientStoreMsg {}

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
		let upgrade_client_state: ics07_tendermint::client_state::ClientState<HostFunctions> =
			ClientState::decode_vec(&any.value)?;
		if upgrade_client_state.trust_level != TrustThreshold::ZERO ||
			upgrade_client_state.trusting_period != Duration::ZERO ||
			upgrade_client_state.max_clock_drift != Duration::ZERO ||
			upgrade_client_state.frozen_height != None
		{
			return ibc::prelude::Err(ContractError::Tendermint(
				"Upgrade client state not zeroed".to_string(),
			))
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
