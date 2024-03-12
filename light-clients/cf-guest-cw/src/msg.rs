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

use crate::{
	ibc,
	serialisation::{AsStr, Base64, MaybeBase64},
	state,
};

#[cw_serde]
pub struct InstantiateMsg {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub client_state: state::ClientState,
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
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
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub client_message: Vec<u8>,
}

#[cw_serde]
pub struct CheckForMisbehaviourMsg {
	#[serde(with = "Base64")]
	#[schemars(with = "String")]
	pub client_message: Vec<u8>,
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
	pub height: Height,
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
	// #[serde(flatten)]
	pub height: Height,
}

#[cw_serde]
pub struct ExportMetadataMsg {}

#[cw_serde]
pub struct ConsensusStateMetadata {
	// #[serde(flatten)]
	pub height: Height,
	pub host_timestamp_ns: Uint64,
	pub host_height: Uint64,
}

fn is_zero(num: &Uint64) -> bool {
	u64::from(*num) == 0
}

/// IBC height.
///
/// This is essentially a copy of [`ibc::Height`] which we have so that we can
/// implement `JsonSchema` on it without having to enable `schema` feature on
/// `ibc` which pulls in `std` which we don’t want.
#[derive(
	Copy,
	Clone,
	PartialEq,
	Eq,
	derive_more::Display,
	serde::Serialize,
	serde::Deserialize,
	schemars::JsonSchema,
)]
#[display(fmt = "{}-{}", revision_number, revision_height)]
pub struct Height {
	/// Previously known as "epoch"
	#[serde(default, skip_serializing_if = "is_zero")]
	pub revision_number: Uint64,

	/// The height of a block
	pub revision_height: Uint64,
}

impl TryFrom<Height> for ibc::Height {
	type Error = cosmwasm_std::StdError;
	fn try_from(height: Height) -> Result<Self, Self::Error> {
		Ok(ibc::Height::new(height.revision_number.into(), height.revision_height.into()))
	}
}

impl From<ibc::Height> for Height {
	fn from(height: ibc::Height) -> Self {
		Self {
			revision_number: height.revision_number.into(),
			revision_height: height.revision_height.into(),
		}
	}
}

impl core::fmt::Debug for Height {
	fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> core::fmt::Result {
		core::fmt::Display::fmt(self, fmtr)
	}
}
