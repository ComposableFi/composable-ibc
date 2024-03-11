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
use alloc::{fmt, string::ToString};
use ibc::{core::{ics02_client::error::Error as Ics02Error, ics24_host::identifier::ClientId}, timestamp::Timestamp, Height};

#[derive(Clone, Debug)]
pub enum Error {
	ProcessedHeightNotFound { height: Height },
	ProcessedTimeNotFound { height: Height },
	NotEnoughTimeElapsed { current_time: Timestamp, earliest_time: u64 },
	NotEnoughBlocksElapsed { current_height: Height, earliest_height: u64 },
	InsufficientHeight { latest_height: Height, target_height: Height },
	ClientFrozen { client_id: ClientId },
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl From<Error> for Ics02Error {
	fn from(e: Error) -> Self {
		Ics02Error::client_error(CLIENT_TYPE.to_string(), e.to_string())
	}
}
