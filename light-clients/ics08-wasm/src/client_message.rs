use crate::{client_state::Any as MyAny, Bytes};
use alloc::{boxed::Box, string::ToString, vec::Vec};
use ibc::{
	core::{
		ics02_client::{
			client_message::ClientMessage as IbcClientMessage, client_state::ClientType,
			error::Error,
		},
		ics24_host::identifier::ClientId,
	},
	protobuf::Protobuf,
	Height,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::lightclients::wasm::v1::{Header as RawHeader, Misbehaviour as RawMisbehaviour},
};

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
	AnyClientMessage: for<'a> TryFrom<(ClientType, &'a Bytes)>,
{
	fn encode_to_vec(&self) -> Vec<u8> {
		self.encode_vec().expect("encode to vec cannot fail")
	}
}

impl<AnyClientMessage> Protobuf<Any> for ClientMessage<AnyClientMessage>
where
	AnyClientMessage: Clone,
	AnyClientMessage: for<'a> TryFrom<(ClientType, &'a Bytes)>,
{
}

impl<AnyClientMessage> TryFrom<Any> for ClientMessage<AnyClientMessage>
where
	AnyClientMessage: Clone,
	AnyClientMessage: for<'a> TryFrom<(ClientType, &'a Bytes)>,
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
	AnyClientMessage: for<'a> TryFrom<(ClientType, &'a Bytes)>,
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
	AnyClientMessage: for<'a> TryFrom<(ClientType, &'a Bytes)>,
{
}

impl<AnyClientMessage> TryFrom<RawMisbehaviour> for Misbehaviour<AnyClientMessage>
where
	AnyClientMessage: for<'a> TryFrom<(ClientType, &'a Bytes)>,
{
	type Error = Error;

	fn try_from(raw: RawMisbehaviour) -> Result<Self, Self::Error> {
		let code_id =
			hex::decode("e0e0fda1d756f6ed060b32f392291b119194b38dcbefcf0e6cc762dc1dc0507b")
				.unwrap();
		let client_type = crate::get_wasm_client_type(&code_id)
			.expect("WASM client type is not set for the code_id");
		let any = MyAny::decode_vec(&raw.data).unwrap();
		let inner = AnyClientMessage::try_from((client_type, &any.value))
			.map_err(|_| ())
			// .map_err(|e| println!("Error: {:?}", e))?;
			.expect("Any* cannot be decoded");
		Ok(Self {
			inner: Box::new(inner),
			data: raw.data,
			client_id: raw.client_id.parse().map_err(|e| Error::invalid_raw_misbehaviour(e))?,
		})
	}
}

impl<AnyClientMessage> Protobuf<RawHeader> for Header<AnyClientMessage>
where
	AnyClientMessage: Clone,
	AnyClientMessage: for<'a> TryFrom<(ClientType, &'a Bytes)>,
{
}

impl<AnyClientMessage> TryFrom<RawHeader> for Header<AnyClientMessage>
where
	AnyClientMessage: for<'a> TryFrom<(ClientType, &'a Bytes)>,
{
	type Error = Error;

	fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
		let code_id =
			hex::decode("e0e0fda1d756f6ed060b32f392291b119194b38dcbefcf0e6cc762dc1dc0507b")
				.unwrap();
		let client_type = crate::get_wasm_client_type(&code_id)
			.expect("WASM client type is not set for the code_id");
		let any = MyAny::decode_vec(&raw.data).unwrap();
		let inner = AnyClientMessage::try_from((client_type, &any.value))
			.map_err(|_| ())
			// .map_err(|e| println!("Error: {:?}", e))?;
			.expect("Any* cannot be decoded");

		let header =
			Self { inner: Box::new(inner), data: raw.data, height: raw.height.map(|h| h.into()) };
		Ok(header)
	}
}

// #[cw_serde]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header<AnyClientMessage> {
	// #[schemars(with = "String")]
	// #[serde(with = "Base64", default)]
	pub inner: Box<AnyClientMessage>,
	pub data: Bytes,
	pub height: Option<Height>,
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
		RawHeader { data: value.data, height: None }
	}
}
