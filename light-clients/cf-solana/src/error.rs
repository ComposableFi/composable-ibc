// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::CLIENT_TYPE;
use alloc::{
	fmt,
	string::{String, ToString},
};
use ibc::{core::ics24_host::identifier::ClientId, timestamp::Timestamp, Height};

#[derive(Clone, Debug)]
pub enum Error {
	ProcessedHeightNotFound { height: Height },
	ProcessedTimeNotFound { height: Height },
	NotEnoughTimeElapsed { current_time: Timestamp, earliest_time: u64 },
	NotEnoughBlocksElapsed { current_height: Height, earliest_height: u64 },
	InsufficientHeight { latest_height: Height, target_height: Height },
	ClientFrozen { client_id: ClientId },
	UnknownConsensusStateType { description: String },
}

impl fmt::Display for Error {
	fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, fmtr)
	}
}

impl From<Error> for ibc::core::ics02_client::error::Error {
	fn from(err: Error) -> Self {
		Self::client_error(CLIENT_TYPE.into(), err.to_string())
	}
}
