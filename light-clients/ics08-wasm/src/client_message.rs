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
use crate::msg::Base64;
use crate::Bytes;
use alloc::{
	boxed::Box,
	string::{String, ToString},
	vec::Vec,
};
use core::{
	convert::Infallible,
	fmt::{Debug, Display},
};
#[cfg(feature = "cosmwasm")]
use cosmwasm_schema::cw_serde;
use ibc::{
	core::ics02_client::{client_message::ClientMessage as IbcClientMessage, error::Error},
	protobuf::Protobuf,
};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::wasm::v1::ClientMessage as RawClientMessage;
use prost::Message;

pub const WASM_CLIENT_MESSAGE_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.ClientMessage";

//#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Clone, Debug, PartialEq))]
#[derive(Eq)]
pub struct ClientMessage<AnyClientMessage> {
	#[cfg_attr(feature = "cosmwasm", serde(skip))]
	#[cfg_attr(feature = "cosmwasm", schemars(skip))]
	pub inner: Box<AnyClientMessage>,
	#[cfg_attr(feature = "cosmwasm", schemars(with = "String"))]
	#[cfg_attr(feature = "cosmwasm", serde(with = "Base64", default))]
	pub data: Bytes,
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

impl<AnyClientMessage> ClientMessage<AnyClientMessage>
where
	AnyClientMessage: Clone,
	AnyClientMessage: TryFrom<Any> + IbcClientMessage,
	<AnyClientMessage as TryFrom<Any>>::Error: Display,
{
	pub fn to_any(&self) -> Any {
		Any {
			value: self.encode_vec().expect("encode client message"),
			type_url: WASM_CLIENT_MESSAGE_TYPE_URL.to_string(),
		}
	}
}

impl<AnyClientMessage> TryFrom<RawClientMessage> for ClientMessage<AnyClientMessage>
where
	AnyClientMessage: Clone + TryFrom<Any>,
	<AnyClientMessage as TryFrom<Any>>::Error: Display,
{
	type Error = String;

	fn try_from(raw: RawClientMessage) -> Result<Self, Self::Error> {
		let any = Any::decode(&mut &raw.data[..])
			.map_err(|e| format!("failed to decode ClientMessage::data into Any: {}", e))?;
		let inner = AnyClientMessage::try_from(any).map_err(
			|e| {
				format!("failed to decode ClientMessage::data into ClientMessage: {}", e)
			})?;
		Ok(Self { data: raw.data, inner: Box::new(inner)})

	}
}

impl<AnyClientMessage: IbcClientMessage> From<ClientMessage<AnyClientMessage>>
	for RawClientMessage
{
	fn from(value: ClientMessage<AnyClientMessage>) -> Self {
		Self { data: value.data }
	}
}

impl<AnyClientMessage> Protobuf<RawClientMessage> for ClientMessage<AnyClientMessage>
where
	AnyClientMessage: Clone + IbcClientMessage + TryFrom<Any>,
	<AnyClientMessage as TryFrom<Any>>::Error: Display,
{
}


