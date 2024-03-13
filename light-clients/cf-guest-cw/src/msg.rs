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

use crate::{ibc, serialisation, state};

#[cw_serde]
pub struct InstantiateMsg { }

#[cw_serde]
pub enum ExecuteMsg {
	VerifyMembership(VerifyMembershipMsg),
	VerifyNonMembership(VerifyNonMembershipMsg),
	VerifyClientMessage(VerifyClientMessage),
	CheckForMisbehaviour(CheckForMisbehaviourMsg),
	UpdateStateOnMisbehaviour(UpdateStateOnMisbehaviourMsg),
	UpdateState(UpdateStateMsg),
//	CheckSubstituteAndUpdateState(CheckSubstituteAndUpdateStateMsg),
//	VerifyUpgradeAndUpdateState(VerifyUpgradeAndUpdateStateMsg),
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

	pub fn found_misbehaviour(mut self, found: bool) -> Self {
		self.found_misbehaviour = found;
		self
	}
}

#[cw_serde]
pub struct VerifyMembershipMsg {
	#[serde(with = "serialisation::Base64")]
	pub proof: ibc::CommitmentProofBytes,
	#[serde(with = "serialisation::AsStr")]
	pub path: ibc::path::Path,
	#[serde(with = "serialisation::Base64")]
	pub value: Vec<u8>,
	#[serde(with = "serialisation::Height")]
	pub height: ibc::Height,
}

#[cw_serde]
pub struct VerifyNonMembershipMsg {
	#[serde(with = "serialisation::Base64")]
	pub proof: ibc::CommitmentProofBytes,
	#[serde(with = "serialisation::AsStr")]
	pub path: ibc::path::Path,
	#[serde(with = "serialisation::Height")]
	pub height: ibc::Height,
}

#[cw_serde]
pub struct VerifyClientMessageMsg {
	#[serde(with = "serialisation::Base64")]
	pub client_message: state::ClientMessage,
}

#[cw_serde]
pub struct CheckForMisbehaviourMsg {
	#[serde(with = "serialisation::Base64")]
	pub client_message: state::ClientMessage,
}

#[cw_serde]
pub struct UpdateStateOnMisbehaviourMsg {
	#[serde(with = "serialisation::Base64")]
	pub client_message: state::ClientMessage,
}

#[cw_serde]
pub struct UpdateStateMsg {
	#[serde(with = "serialisation::Base64")]
	pub client_message: Vec<u8>,
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
	// #[returns(QueryResponses>)]
	// ClientTypeMsg(ClientTypeMsg),
	// #[returns(QueryResponses>)]
	// GetLatestHeightsMsg(GetLatestHeightsMsg),

	#[returns(QueryResponses>)]
	ExportMetadata(ExportMetadataMsg),

	#[returns(QueryResponses>)]
	Status(StatusMsg),
}

#[cw_serde]
pub struct QueryResponse {
	#[serde(default, skip_serializing_if = "String::is_empty")]
	pub status: String,
}

impl QueryResponse {
	pub fn new(status: &str) -> Self {
		Self { status: status.into() }
	}

	pub fn active() -> Self { Self::new("Active") }
	pub fn frozen() -> Self { Self::new("Frozen") }
	pub fn expired() -> Self { Self::new("Expired") }
}

#[cw_serde]
pub struct StatusMsg {}

#[cw_serde]
pub struct ExportMetadataMsg {}
