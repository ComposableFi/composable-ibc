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

use cosmwasm_std::StdError;
use std::error::Error;

#[derive(derive_more::From, derive_more::Display, Debug)]
pub enum ContractError {
	Std(StdError),

	ProofVerification(cf_guest::proof::VerifyError),

	Client(ibc::core::ics02_client::error::Error),
	Path(ibc::core::ics24_host::path::PathError),
	Proof(ibc::proofs::ProofError),
	Commitment(ibc::core::ics23_commitment::error::Error),
	Protobuf(ibc::protobuf::Error),

	Prost(prost::DecodeError),

	Other(String),
}

impl Error for ContractError {}
