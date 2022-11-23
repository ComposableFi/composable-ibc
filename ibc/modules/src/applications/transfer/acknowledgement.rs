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
use crate::{core::ics26_routing::context::Acknowledgement as AckTrait, prelude::*};
use core::fmt::{Display, Formatter};

use serde::{Deserialize, Deserializer};

/// A string constant included in error acknowledgements.
/// NOTE: Changing this const is state machine breaking as acknowledgements are written into state
pub const ACK_ERR_STR: &str = "error handling packet on destination chain: see events for details";
pub const ACK_SUCCESS_B64: &[u8] = b"AQ==";

#[derive(Clone, Debug)]
pub enum Acknowledgement {
	/// Equivalent to b"AQ==" (i.e. `base64::encode(0x01)`)
	Success(Vec<u8>),
	/// Error Acknowledgement
	Error(String),
}

impl Acknowledgement {
	pub fn success() -> Self {
		Self::Success(ACK_SUCCESS_B64.to_vec())
	}

	pub fn from_error(err: Error) -> Self {
		Self::Error(format!("{}: {}", ACK_ERR_STR, err))
	}

	pub fn is_successful(&self) -> bool {
		matches!(self, Self::Success(_))
	}
}

impl AsRef<[u8]> for Acknowledgement {
	fn as_ref(&self) -> &[u8] {
		match self {
			Acknowledgement::Success(b) => b.as_slice(),
			Acknowledgement::Error(s) => s.as_bytes(),
		}
	}
}

impl<'de> Deserialize<'de> for Acknowledgement {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let s = String::deserialize(deserializer)?;
		let ack = if s.as_bytes() == ACK_SUCCESS_B64 {
			Self::Success(ACK_SUCCESS_B64.to_vec())
		} else {
			Self::Error(s)
		};
		Ok(ack)
	}
}

impl Display for Acknowledgement {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		match self {
			Acknowledgement::Success(_) => write!(f, "AQ=="),
			Acknowledgement::Error(err_str) => write!(f, "{}", err_str),
		}
	}
}

impl AckTrait for Acknowledgement {}
