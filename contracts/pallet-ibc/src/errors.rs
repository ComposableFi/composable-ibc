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

use super::*;
use ibc::core::ics26_routing::error::{Error as RoutingError, ErrorDetail};

#[derive(
	PartialEq, Eq, Clone, frame_support::RuntimeDebug, scale_info::TypeInfo, Encode, Decode,
)]
pub enum IbcError {
	/// ICS02 client error
	Ics02Client { message: Vec<u8> },
	/// ICS03 connection error
	Ics03Connection { message: Vec<u8> },
	/// ICS04 channel error
	Ics04Channel { message: Vec<u8> },
	/// ICS20 fungible token transfer error
	Ics20FungibleTokenTransfer { message: Vec<u8> },
	/// Unknown message type URL
	UnknownMessageTypeUrl { message: Vec<u8> },
	/// The message is malformed and cannot be decoded
	MalformedMessageBytes { message: Vec<u8> },
}

impl From<RoutingError> for IbcError {
	fn from(err: RoutingError) -> Self {
		match err.0 {
			ErrorDetail::Ics03Connection(e) =>
				IbcError::Ics03Connection { message: format!("{:?}", e).as_bytes().to_vec() },
			ErrorDetail::Ics02Client(e) =>
				IbcError::Ics02Client { message: format!("{:?}", e).as_bytes().to_vec() },
			ErrorDetail::Ics04Channel(e) =>
				IbcError::Ics04Channel { message: format!("{:?}", e).as_bytes().to_vec() },
			ErrorDetail::Ics20FungibleTokenTransfer(e) => IbcError::Ics20FungibleTokenTransfer {
				message: format!("{:?}", e).as_bytes().to_vec(),
			},
			ErrorDetail::UnknownMessageTypeUrl(e) =>
				IbcError::UnknownMessageTypeUrl { message: format!("{:?}", e).as_bytes().to_vec() },
			ErrorDetail::MalformedMessageBytes(e) =>
				IbcError::MalformedMessageBytes { message: format!("{:?}", e).as_bytes().to_vec() },
		}
	}
}

impl<T: Config> From<Vec<RoutingError>> for Event<T> {
	fn from(errors: Vec<RoutingError>) -> Self {
		let errors: Vec<IbcError> = errors.into_iter().map(|err| err.into()).collect();
		Event::<T>::Errors { errors }
	}
}
