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

#[cfg(feature = "cosmwasm")]
use crate::Bytes;
use alloc::{
	boxed::Box,
	string::ToString,
	vec::Vec,
};
use core::fmt::Display;
#[cfg(feature = "cosmwasm")]
use ibc::{
	core::ics02_client::{client_message::ClientMessage as IbcClientMessage, error::Error},
	protobuf::Protobuf,
};
use ibc_proto::google::protobuf::Any;

pub const WASM_CLIENT_MESSAGE_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.ClientMessage";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientMessage<AnyClientMessage> {
	pub inner: Box<AnyClientMessage>,
	pub data: Bytes,
}

impl<AnyClientMessage> ClientMessage<AnyClientMessage> {
	pub fn inner(&self) -> &AnyClientMessage {
		&self.inner
	}

	pub fn into_inner(self) -> AnyClientMessage {
		*self.inner
	}
}

impl<AnyClientMessage> IbcClientMessage for ClientMessage<AnyClientMessage>
where
	AnyClientMessage: IbcClientMessage,
	AnyClientMessage: TryFrom<Any>,
	<AnyClientMessage as TryFrom<Any>>::Error: Display,
{
	fn encode_to_vec(&self) -> Result<Vec<u8>, tendermint_proto::Error> {
		self.encode_vec()
	}
}

impl<AnyClientMessage> Protobuf<Any> for ClientMessage<AnyClientMessage>
where
	AnyClientMessage: Clone,
	AnyClientMessage: TryFrom<Any>,
	<AnyClientMessage as TryFrom<Any>>::Error: Display,
{
}

impl<AnyClientMessage> TryFrom<Any> for ClientMessage<AnyClientMessage>
where
	AnyClientMessage: Clone,
	AnyClientMessage: TryFrom<Any>,
	<AnyClientMessage as TryFrom<Any>>::Error: Display,
{
	type Error = Error;

	fn try_from(any: Any) -> Result<Self, Self::Error> {
		let msg = match &*any.type_url {
			WASM_CLIENT_MESSAGE_TYPE_URL =>
				ClientMessage::decode(&*any.value).map_err(|e| Error::decode_raw_header(e))?,
			_ => return Err(Error::malformed_header()), // TODO: choose a better error
		};

		Ok(msg)
	}
}

impl<AnyClientMessage> From<ClientMessage<AnyClientMessage>> for Any
where
	AnyClientMessage: Clone,
	AnyClientMessage: TryFrom<Any>,
	<AnyClientMessage as TryFrom<Any>>::Error: Display,
{
	fn from(msg: ClientMessage<AnyClientMessage>) -> Self {
		Any {
			value: msg.encode_vec().expect("encode client message"),
			type_url: WASM_CLIENT_MESSAGE_TYPE_URL.to_string(),
		}
	}
}
