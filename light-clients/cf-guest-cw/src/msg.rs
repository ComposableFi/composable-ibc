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
use cosmwasm_schema::cw_serde;

use ibc::{
	core::{ics23_commitment::commitment::CommitmentProofBytes, ics24_host::Path},
	protobuf::Protobuf,
	Height,
};
use ibc_proto::{google::protobuf::Any, ibc::core::client::v1::Height as HeightRaw};
use ics08_wasm::{
	client_message::Header as WasmHeader, client_state::ClientState as WasmClientState,
	consensus_state::ConsensusState as WasmConsensusState,
};
use prost::Message;
use serde::{Deserializer, Serializer};

use crate::{fake_inner::FakeInner, state, Error};

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
pub struct QueryResponse {
	pub status: String,
}

impl QueryResponse {
	pub fn new(status: &str) -> Self {
		Self { status: status.into() }
	}
}

#[cw_serde]
pub struct ContractResult {
	pub is_valid: bool,
	pub found_misbehaviour: bool,
}

impl ContractResult {
	pub fn success() -> Self {
		Self { is_valid: true, found_misbehaviour: false }
	}

	pub fn misbehaviour(mut self, found: bool) -> Self {
		self.found_misbehaviour = found;
		self
	}
}

#[cw_serde]
pub struct ClientStateCallResponse {
	pub client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
	pub new_consensus_state: WasmConsensusState<FakeInner>,
	pub new_client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
	pub result: ContractResult,
}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
	VerifyMembership(VerifyMembershipMsgRaw),
	VerifyNonMembership(VerifyNonMembershipMsgRaw),
	VerifyClientMessage(VerifyClientMessageMsgRaw),
	CheckForMisbehaviour(CheckForMisbehaviourMsgRaw),
	UpdateStateOnMisbehaviour(UpdateStateOnMisbehaviourMsgRaw),
	UpdateState(UpdateStateMsgRaw),
	CheckSubstituteAndUpdateState(CheckSubstituteAndUpdateStateMsgRaw),
	VerifyUpgradeAndUpdateState(VerifyUpgradeAndUpdateStateMsgRaw),
}

#[cw_serde]
#[derive(cosmwasm_schema::QueryResponses)]
pub enum QueryMsg {
	#[returns(QueryResponse)]
	ClientTypeMsg(ClientTypeMsg),
	#[returns(QueryResponse)]
	GetLatestHeightsMsg(GetLatestHeightsMsg),
	#[returns(QueryResponse)]
	ExportMetadata(ExportMetadataMsg),
	#[returns(QueryResponse)]
	Status(StatusMsg),
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
	pub proof: Vec<u8>,
	pub path: MerklePath,
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub value: Vec<u8>,
	pub height: HeightRaw,
	pub delay_block_period: u64,
	pub delay_time_period: u64,
}

#[cw_serde]
pub struct VerifyNonMembershipMsgRaw {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub proof: Vec<u8>,
	pub path: MerklePath,
	pub height: HeightRaw,
}

pub(crate) struct VerifyStateProof {
	pub proof: CommitmentProofBytes,
	pub path: Path,
	pub value: Option<Vec<u8>>,
	pub height: Height,
}

impl TryFrom<VerifyMembershipMsgRaw> for VerifyStateProof {
	type Error = crate::Error;

	fn try_from(raw: VerifyMembershipMsgRaw) -> Result<Self, Self::Error> {
		Self::new(raw.proof, raw.path, Some(raw.value), raw.height)
	}
}

impl TryFrom<VerifyNonMembershipMsgRaw> for VerifyStateProof {
	type Error = crate::Error;

	fn try_from(raw: VerifyNonMembershipMsgRaw) -> Result<Self, Self::Error> {
		Self::new(raw.proof, raw.path, None, raw.height)
	}
}

impl VerifyStateProof {
	fn new(
		proof: Vec<u8>,
		path: MerklePath,
		value: Option<Vec<u8>>,
		height: HeightRaw,
	) -> Result<Self, Error> {
		let proof = CommitmentProofBytes::try_from(proof).map_err(|_| Error::BadMessage)?;
		let path_str = path.key_path.join("");
		let path = Path::from_str(&path_str).map_err(|_| Error::BadMessage)?;
		let height = Height::from(height);
		Ok(Self { proof, path, value, height })
	}
}

#[cw_serde]
pub struct WasmMisbehaviour {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub data: Vec<u8>,
}

#[cw_serde]
pub enum ClientMessageRaw {
	Header(WasmHeader<FakeInner>),
	Misbehaviour(WasmMisbehaviour),
}

#[cw_serde]
pub struct VerifyClientMessageMsgRaw {
	pub client_message: ClientMessageRaw,
}

pub struct VerifyClientMessageMsg {
	pub client_message: state::ClientMessage,
}

impl TryFrom<VerifyClientMessageMsgRaw> for VerifyClientMessageMsg {
	type Error = Error;

	fn try_from(raw: VerifyClientMessageMsgRaw) -> Result<Self, Self::Error> {
		let client_message = Self::decode_client_message(raw.client_message)?;
		Ok(Self { client_message })
	}
}

impl VerifyClientMessageMsg {
	fn decode_client_message(raw: ClientMessageRaw) -> Result<state::ClientMessage, Error> {
		let client_message = match raw {
			ClientMessageRaw::Header(header) => {
				let any = Any::decode(&mut header.data.as_slice())?;
				state::ClientMessage::Header(state::Header::decode_vec(&any.value)?)
			},
			ClientMessageRaw::Misbehaviour(misbehaviour) => {
				let any = Any::decode(&mut misbehaviour.data.as_slice())?;
				state::ClientMessage::Misbehaviour(state::Misbehaviour::decode_vec(&any.value)?)
			},
		};
		Ok(client_message)
	}
}

#[cw_serde]
pub struct CheckForMisbehaviourMsgRaw {
	pub client_message: ClientMessageRaw,
}

pub struct CheckForMisbehaviourMsg {
	pub client_message: state::ClientMessage,
}

impl TryFrom<CheckForMisbehaviourMsgRaw> for CheckForMisbehaviourMsg {
	type Error = Error;

	fn try_from(raw: CheckForMisbehaviourMsgRaw) -> Result<Self, Self::Error> {
		let client_message = VerifyClientMessageMsg::decode_client_message(raw.client_message)?;
		Ok(Self { client_message })
	}
}

#[cw_serde]
pub struct UpdateStateOnMisbehaviourMsgRaw {
	pub client_message: ClientMessageRaw,
}

pub struct UpdateStateOnMisbehaviourMsg {
	pub client_message: state::ClientMessage,
}

impl TryFrom<UpdateStateOnMisbehaviourMsgRaw> for UpdateStateOnMisbehaviourMsg {
	type Error = Error;

	fn try_from(raw: UpdateStateOnMisbehaviourMsgRaw) -> Result<Self, Self::Error> {
		let client_message = VerifyClientMessageMsg::decode_client_message(raw.client_message)?;
		Ok(Self { client_message })
	}
}

#[cw_serde]
pub struct UpdateStateMsgRaw {
	pub client_message: ClientMessageRaw,
}

pub struct UpdateStateMsg {
	pub client_message: state::ClientMessage,
}

impl TryFrom<UpdateStateMsgRaw> for UpdateStateMsg {
	type Error = Error;

	fn try_from(raw: UpdateStateMsgRaw) -> Result<Self, Self::Error> {
		let client_message = VerifyClientMessageMsg::decode_client_message(raw.client_message)?;
		Ok(Self { client_message })
	}
}

#[cw_serde]
pub struct CheckSubstituteAndUpdateStateMsgRaw {}

pub struct CheckSubstituteAndUpdateStateMsg {}

impl TryFrom<CheckSubstituteAndUpdateStateMsgRaw> for CheckSubstituteAndUpdateStateMsg {
	type Error = Error;

	fn try_from(
		CheckSubstituteAndUpdateStateMsgRaw {}: CheckSubstituteAndUpdateStateMsgRaw,
	) -> Result<Self, Self::Error> {
		Ok(Self {})
	}
}

#[cw_serde]
pub struct VerifyUpgradeAndUpdateStateMsgRaw {
	pub upgrade_client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
	pub upgrade_consensus_state: WasmConsensusState<FakeInner>,
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub proof_upgrade_client: Vec<u8>,
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub proof_upgrade_consensus_state: Vec<u8>,
}

pub struct VerifyUpgradeAndUpdateStateMsg {
	pub upgrade_client_state: state::ClientState,
	pub upgrade_consensus_state: state::ConsensusState,
	pub proof_upgrade_client: Vec<u8>,
	pub proof_upgrade_consensus_state: Vec<u8>,
}

impl TryFrom<VerifyUpgradeAndUpdateStateMsgRaw> for VerifyUpgradeAndUpdateStateMsg {
	type Error = Error;

	fn try_from(raw: VerifyUpgradeAndUpdateStateMsgRaw) -> Result<Self, Self::Error> {
		let any = Any::decode(&mut raw.upgrade_client_state.data.as_slice())?;
		let upgrade_client_state = state::ClientState::decode_vec(&any.value)?;
		let any = Any::decode(&mut raw.upgrade_consensus_state.data.as_slice())?;
		let upgrade_consensus_state = state::ConsensusState::decode_vec(&any.value)?;
		Ok(VerifyUpgradeAndUpdateStateMsg {
			upgrade_client_state,
			upgrade_consensus_state,
			proof_upgrade_client: raw.proof_upgrade_client,
			proof_upgrade_consensus_state: raw.proof_upgrade_consensus_state,
		})
	}
}
