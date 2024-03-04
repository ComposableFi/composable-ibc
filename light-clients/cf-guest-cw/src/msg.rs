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

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;

use crate::{
	ibc,
	serialisation::{AsStr, Base64, MaybeBase64},
	state,
};

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
pub enum SudoMsg {
	//MigrateClientStore(MigrateClientStoreMsg),
	UpdateState(UpdateStateMsg),
	UpdateStateOnMisbehaviour(UpdateStateOnMisbehaviourMsg),
	// VerifyUpgradeAndUpdateState(VerifyUpgradeAndUpdateStateMsg),
}

// #[cw_serde]
// pub struct MigrateClientStoreMsg {}

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

// #[cw_serde]
// pub struct VerifyUpgradeAndUpdateStateMsg {
// 	#[serde(with = "Base64")]
// 	#[schemars(with = "String")]
// 	pub upgrade_client_state: state::ClientState,
// 	#[serde(with = "Base64")]
// 	#[schemars(with = "String")]
// 	pub upgrade_consensus_state: state::ConsensusState,
// 	#[serde(with = "Base64")]
// 	#[schemars(with = "String")]
// 	pub proof_upgrade_client: ibc::CommitmentProofBytes,
// 	#[serde(with = "Base64")]
// 	#[schemars(with = "String")]
// 	pub proof_upgrade_consensus_state: ibc::CommitmentProofBytes,
// }

#[cw_serde]
pub enum QueryMsg {
	/// Verifies client message.
	///
	/// Response is empty or error if proof is invalid.
	VerifyClientMessage(VerifyClientMessageMsg),

	/// Checks client message for misbehaviour.
	///
	/// Response is JSON-serialised boolean (i.e. `true` or `false`) or
	/// error.
	CheckForMisbehaviour(CheckForMisbehaviourMsg),

	/// Checks whether provided membership or non-membership proof is valid.
	///
	/// The proof is a membership proof is `self.0.value` field is `Some`.
	/// Otherwise, if `self.0.value` is `None`, the proof is non-membership
	/// proof.
	///
	/// Response is empty or error if proof is invalid.
	VerifyStateProof(VerifyStateProofMsg),

	/// Checks status of the client.
	///
	/// Response is JSON-serialised [`StatusResponse`] object.
	Status(StatusMsg),

	GetLatestHeights(GetLatestHeightsMsg),

	/// Returns metadata for consensus at given height.
	///
	/// Response is JSON-serialised `Uint64` object representing Unix
	/// timestamp in nanoseconds.
	TimestampAtHeight(TimestampAtHeightMsg),

	/// Gets metadata of all consensus states.
	///
	/// Response is JSON-serialised array of [`ConsensusStateMetadata`]
	/// objects.
	ExportMetadata(ExportMetadataMsg),
}

#[cw_serde]
pub struct VerifyClientMessageMsg {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub client_message: state::ClientMessage,
}

#[cw_serde]
pub struct CheckForMisbehaviourMsg {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub client_message: state::ClientMessage,
}

#[cw_serde]
pub struct VerifyStateProofMsg {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub proof: ibc::CommitmentProofBytes,
	#[serde(with = "AsStr")]
	#[schemars(with = "String")]
	pub path: ibc::path::Path,
	#[serde(with = "MaybeBase64", default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String")]
	pub value: Option<Vec<u8>>,
	#[serde(flatten)]
	pub height: ibc::Height,
}

#[cw_serde]
pub struct StatusMsg {}

#[cw_serde]
pub enum StatusResponse {
	Active,
	Expired,
	Frozen,
}

#[cw_serde]
pub struct GetLatestHeightsMsg {}

#[cw_serde]
pub struct TimestampAtHeightMsg {
	#[serde(flatten)]
	pub height: ibc::Height,
}

#[cw_serde]
pub struct ExportMetadataMsg {}

#[cw_serde]
pub struct ConsensusStateMetadata {
	pub revision_number: Uint64,
	pub revision_height: Uint64,
	pub host_timestamp_ns: Uint64,
	pub host_height: Uint64,
}

fn is_false(val: &bool) -> bool {
	!*val
}
