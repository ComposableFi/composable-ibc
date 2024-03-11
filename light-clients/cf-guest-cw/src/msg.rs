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

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint64;

use crate::{ibc, serialisation, state};

#[cw_serde]
pub struct InstantiateMsg {
	#[serde(with = "serialisation::Base64")]
	pub client_state: state::ClientState,
	#[serde(with = "serialisation::Base64")]
	pub consensus_state: state::ConsensusState,
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
	#[serde(with = "serialisation::Base64")]
	pub header: state::Header,
}

#[cw_serde]
pub struct UpdateStateOnMisbehaviourMsg {
	#[serde(with = "serialisation::Base64")]
	pub misbehaviour_message: state::Misbehaviour,
}

// #[cw_serde]
// pub struct VerifyUpgradeAndUpdateStateMsg {
// 	#[serde(with = "serialisation::Base64")]
// 	pub upgrade_client_state: state::ClientState,
// 	#[serde(with = "serialisation::Base64")]
// 	pub upgrade_consensus_state: state::ConsensusState,
// 	#[serde(with = "serialisation::Base64")]
// 	pub proof_upgrade_client: ibc::CommitmentProofBytes,
// 	#[serde(with = "serialisation::Base64")]
// 	pub proof_upgrade_consensus_state: ibc::CommitmentProofBytes,
// }

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
	/// Verifies client message.
	#[returns(())]
	VerifyClientMessage(VerifyClientMessageMsg),

	/// Checks client message for misbehaviour.
	#[returns(bool)]
	CheckForMisbehaviour(CheckForMisbehaviourMsg),

	/// Checks whether provided membership or non-membership proof is valid.
	///
	/// The proof is a membership proof is `self.0.value` field is `Some`.
	/// Otherwise, if `self.0.value` is `None`, the proof is non-membership
	/// proof.
	#[returns(())]
	VerifyStateProof(VerifyStateProofMsg),

	/// Checks status of the client.
	#[returns(StatusResponse)]
	Status(StatusMsg),

	/// Returns timestamp for consensus at given height.
	///
	/// The timestamp is represented as nanoseconds since Unix epoch.
	#[returns(Uint64)]
	TimestampAtHeight(TimestampAtHeightMsg),

	/// Gets metadata of all consensus states.
	#[returns(Vec<ConsensusStateMetadata>)]
	ExportMetadata(ExportMetadataMsg),
}

#[cw_serde]
pub struct VerifyClientMessageMsg {
	#[serde(with = "serialisation::Base64")]
	pub client_message: Vec<u8>,
}

#[cw_serde]
pub struct CheckForMisbehaviourMsg {
	#[serde(with = "serialisation::Base64")]
	pub client_message: Vec<u8>,
}

#[cw_serde]
pub struct VerifyStateProofMsg {
	#[serde(with = "serialisation::Base64")]
	pub proof: ibc::CommitmentProofBytes,
	#[serde(with = "serialisation::AsStr")]
	pub path: ibc::path::Path,
	#[serde(with = "serialisation::OptBase64", default, skip_serializing_if = "Option::is_none")]
	pub value: Option<Vec<u8>>,
	#[serde(with = "serialisation::Height")]
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
	#[serde(with = "serialisation::Height")]
	pub height: ibc::Height,
}

#[cw_serde]
pub struct ExportMetadataMsg {}

#[cw_serde]
pub struct ConsensusStateMetadata {
	#[serde(with = "serialisation::Height")]
	pub height: ibc::Height,
	pub host_timestamp_ns: Uint64,
	pub host_height: Uint64,
}
