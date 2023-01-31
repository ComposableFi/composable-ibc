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

use super::error::Error;
use crate::prelude::*;
use core::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// A string constant included in error acknowledgements.
/// NOTE: Changing this const is state machine breaking as acknowledgements are written into state
pub const ACK_ERR_STR: &str = "error handling packet on destination chain: see events for details";
pub const ACK_SUCCESS_B64: &str = "AQ==";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Acknowledgement {
	#[serde(skip_serializing_if = "Option::is_none")]
	result: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	error: Option<String>,
}

impl Acknowledgement {
	pub fn success() -> Self {
		Self {
			result: Some(ACK_SUCCESS_B64.to_string()),
			error: None,
		}
	}

	pub fn from_error(err: Error) -> Self {
		Self {
			result: None,
			error: Some(err.to_string()),
		}
	}

	pub fn is_successful(&self) -> bool {
		self.result.as_deref() == Some(ACK_SUCCESS_B64)
	}

	pub fn into_result(self) -> Result<String, String> {
		match (self.result, self.error) {
			(Some(r), None) if r == ACK_SUCCESS_B64 => Ok(r),
			(Some(r), None) => Err(r),
			(None, Some(e)) => Err(e),
			_ => unreachable!("invalid response"),
		}
	}
}

impl Display for Acknowledgement {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		serde_json::to_string(self)
			.map_err(|_| core::fmt::Error)
			.and_then(|s| write!(f, "{}", s))
	}
}
