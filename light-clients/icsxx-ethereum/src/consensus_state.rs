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
	abi::EthereumClientAbi::EthereumClientPrimitivesConsensusState, alloc::string::ToString,
	error::Error, proto::ConsensusState as RawConsensusState,
};
use alloc::{format, vec::Vec};
use alloy_sol_types::SolValue;
use core::{convert::Infallible, fmt::Debug};
use ibc::{core::ics23_commitment::commitment::CommitmentRoot, timestamp::Timestamp};
use ibc_proto::google::protobuf::Any;
use primitive_types::H256;
use serde::{Deserialize, Serialize};
use tendermint::time::Time;
use tendermint_proto::{google::protobuf as tpb, Protobuf};

/// Protobuf type url for GRANDPA Consensus State
pub const ETHEREUM_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.lightclients.ethereum.v1.ConsensusState";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusState {
	pub timestamp: Time,
	pub root: CommitmentRoot,
}

impl ConsensusState {
	pub fn abi_encode(self) -> Vec<u8> {
		EthereumClientPrimitivesConsensusState::from(self).abi_encode()
	}

	pub fn abi_decode(bytes: &[u8]) -> Result<Self, Error> {
		let abi = EthereumClientPrimitivesConsensusState::abi_decode(bytes, true)?;
		Ok(abi.into())
	}
}

impl ConsensusState {
	pub fn new(root: H256, timestamp: u64) -> Self {
		let root_bytes: Vec<u8> = root.0.to_vec();
		Self {
			timestamp: Time::from_unix_timestamp(timestamp as i64, 0)
				.expect("timestamp should be valid"),
			root: root_bytes.into(),
		}
	}

	pub fn to_any(&self) -> Any {
		Any {
			type_url: ETHEREUM_CONSENSUS_STATE_TYPE_URL.to_string(),
			value: self.encode_vec().expect("encode ConsensusState"),
		}
	}
}

impl ibc::core::ics02_client::client_consensus::ConsensusState for ConsensusState {
	type Error = Infallible;

	fn root(&self) -> &CommitmentRoot {
		&self.root
	}

	fn timestamp(&self) -> Timestamp {
		self.timestamp.into()
	}

	fn encode_to_vec(&self) -> Result<Vec<u8>, tendermint_proto::Error> {
		self.encode_vec()
	}
}

impl Protobuf<RawConsensusState> for ConsensusState {}

impl TryFrom<RawConsensusState> for ConsensusState {
	type Error = Error;

	fn try_from(raw: RawConsensusState) -> Result<Self, Self::Error> {
		ConsensusState::abi_decode(&raw.abi_data)
	}
}

impl From<ConsensusState> for RawConsensusState {
	fn from(value: ConsensusState) -> Self {
		RawConsensusState { abi_data: value.abi_encode() }
	}
}
