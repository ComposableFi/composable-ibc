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

#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum Error {
	Std(StdError),
	Client(crate::ibc::ClientError),
	Wasm(crate::ibc::wasm::Error),
	BadProto(prost::DecodeError),
	#[from(ignore)]
	BadMessage,
	#[from(ignore)]
	BadType,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<alloc::string::FromUtf8Error> for Error {
	fn from(err: alloc::string::FromUtf8Error) -> Self {
		Self::Std(StdError::InvalidUtf8 { msg: err.to_string() })
	}
}

impl From<cf_guest::DecodeError> for Error {
	fn from(err: cf_guest::DecodeError) -> Self {
		match err {
			cf_guest::DecodeError::BadMessage => Self::BadMessage,
			cf_guest::DecodeError::BadType => Self::BadType,
			cf_guest::DecodeError::BadProto(err) => err.into(),
		}
	}
}

impl From<Error> for StdError {
	fn from(err: Error) -> Self {
		match err {
			Error::Std(err) => err,
			_ => StdError::GenericErr { msg: err.to_string() },
		}
	}
}
