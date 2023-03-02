#[cfg(feature = "cosmwasm")]
use crate::msg::Base64;
use crate::Bytes;
use alloc::{
	boxed::Box,
	string::{String, ToString},
	vec::Vec,
};
use core::fmt::Display;
#[cfg(feature = "cosmwasm")]
use cosmwasm_schema::cw_serde;
use ibc::{
	core::{
		ics02_client::{client_message::ClientMessage as IbcClientMessage, error::Error},
		ics24_host::identifier::ClientId,
	},
	protobuf::Protobuf,
	Height,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::lightclients::wasm::v1::{Header as RawHeader, Misbehaviour as RawMisbehaviour},
};
use prost::Message;

pub const WASM_HEADER_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.Header";
pub const WASM_MISBEHAVIOUR_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.Misbehaviour";
pub const WASM_CLIENT_MESSAGE_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.ClientMessage";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientMessage<AnyClientMessage> {
	Header(Header<AnyClientMessage>),
	Misbehaviour(Misbehaviour<AnyClientMessage>),
}

impl<AnyClientMessage> ClientMessage<AnyClientMessage> {
	pub fn inner(&self) -> &AnyClientMessage {
		match self {
			ClientMessage::Header(h) => &h.inner,
			ClientMessage::Misbehaviour(m) => &m.inner,
		}
	}

	pub fn into_inner(self) -> AnyClientMessage {
		match self {
			ClientMessage::Header(h) => *h.inner,
			ClientMessage::Misbehaviour(m) => *m.inner,
		}
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
			WASM_HEADER_TYPE_URL =>
				Self::Header(Header::decode(&*any.value).map_err(|e| Error::decode_raw_header(e))?),
			WASM_MISBEHAVIOUR_TYPE_URL => Self::Misbehaviour(
				Misbehaviour::decode(&*any.value).map_err(Error::decode_raw_misbehaviour)?,
			),
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
		match msg {
			ClientMessage::Header(header) => Any {
				value: header.encode_vec().expect("encode header"),
				type_url: WASM_HEADER_TYPE_URL.to_string(),
			},
			ClientMessage::Misbehaviour(misbheaviour) => Any {
				value: misbheaviour.encode_vec().expect("encode misbehaviour"),
				type_url: WASM_MISBEHAVIOUR_TYPE_URL.to_string(),
			},
		}
	}
}

impl<AnyClientMessage> Protobuf<RawMisbehaviour> for Misbehaviour<AnyClientMessage>
where
	AnyClientMessage: Clone,
	AnyClientMessage: TryFrom<Any>,
	<AnyClientMessage as TryFrom<Any>>::Error: Display,
{
}

impl<AnyClientMessage> TryFrom<RawMisbehaviour> for Misbehaviour<AnyClientMessage>
where
	AnyClientMessage: TryFrom<Any>,
	<AnyClientMessage as TryFrom<Any>>::Error: Display,
{
	type Error = String;

	fn try_from(raw: RawMisbehaviour) -> Result<Self, Self::Error> {
		let any = Any::decode(&mut &raw.data[..]).map_err(|e| e.to_string())?;
		let inner = AnyClientMessage::try_from(any).map_err(|e| e.to_string())?;
		Ok(Self {
			inner: Box::new(inner),
			data: raw.data,
			client_id: raw
				.client_id
				.parse()
				.map_err(|e| format!("failed to parse client id from raw misbehaviour: {}", e))?,
		})
	}
}

impl<AnyClientMessage> Protobuf<RawHeader> for Header<AnyClientMessage>
where
	AnyClientMessage: Clone,
	AnyClientMessage: TryFrom<Any>,
	<AnyClientMessage as TryFrom<Any>>::Error: Display,
{
}

impl<AnyClientMessage> TryFrom<RawHeader> for Header<AnyClientMessage>
where
	AnyClientMessage: TryFrom<Any>,
	<AnyClientMessage as TryFrom<Any>>::Error: Display,
{
	type Error = String;

	fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
		let any = Any::decode(&mut &raw.data[..])
			.map_err(|e| format!("failed to decode raw header into Any: {}", e.to_string()))?;
		let inner = AnyClientMessage::try_from(any).map_err(|e| {
			format!("failed to decode raw header into AnyClientMessage: {}", e.to_string())
		})?;

		let header = Self {
			inner: Box::new(inner),
			data: raw.data,
			height: raw
				.height
				.ok_or_else(|| format!("failed to decode raw header into Header: missing height",))?
				.into(),
		};
		Ok(header)
	}
}

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Clone, Debug, PartialEq))]
#[derive(Eq)]
pub struct Header<AnyClientMessage> {
	#[cfg_attr(feature = "cosmwasm", serde(skip))]
	#[cfg_attr(feature = "cosmwasm", schemars(skip))]
	pub inner: Box<AnyClientMessage>,
	#[cfg_attr(feature = "cosmwasm", schemars(with = "String"))]
	#[cfg_attr(feature = "cosmwasm", serde(with = "Base64", default))]
	pub data: Bytes,
	pub height: Height,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Misbehaviour<AnyClientMessage> {
	// #[schemars(with = "String")]
	// #[serde(with = "Base64", default)]
	pub inner: Box<AnyClientMessage>,
	pub data: Bytes,
	pub client_id: ClientId,
}

impl<AnyClientMessage> From<Misbehaviour<AnyClientMessage>> for RawMisbehaviour {
	fn from(value: Misbehaviour<AnyClientMessage>) -> Self {
		RawMisbehaviour { client_id: value.client_id.to_string(), data: value.data }
	}
}

impl<AnyClientMessage> From<Header<AnyClientMessage>> for RawHeader {
	fn from(value: Header<AnyClientMessage>) -> Self {
		RawHeader { data: value.data, height: Some(value.height.into()) }
	}
}
