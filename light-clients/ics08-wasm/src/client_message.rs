use crate::Bytes;
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

pub const WASM_HEADER_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.Header";
pub const WASM_MISBEHAVIOUR_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.Misbehaviour";
pub const WASM_CLIENT_MESSAGE_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.ClientMessage";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientMessage {
	Header(Header),
	Misbehaviour(Misbehaviour),
}

impl IbcClientMessage for ClientMessage {
	fn encode_to_vec(&self) -> Vec<u8> {
		self.encode_vec().expect("encode to vec cannot fail")
	}
}

impl Protobuf<Any> for ClientMessage {}

impl TryFrom<Any> for ClientMessage {
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

impl From<ClientMessage> for Any {
	fn from(msg: ClientMessage) -> Self {
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

impl Protobuf<RawMisbehaviour> for Misbehaviour {}

impl TryFrom<RawMisbehaviour> for Misbehaviour {
	type Error = Error;

	fn try_from(raw: RawMisbehaviour) -> Result<Self, Self::Error> {
		Ok(Self {
			data: raw.data,
			client_id: raw.client_id.parse().map_err(|e| Error::invalid_raw_misbehaviour(e))?,
		})
	}
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
	type Error = Error;

	fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
		let header = Self { data: raw.data, height: raw.height.map(|h| h.into()) };

		Ok(header)
	}
}

// #[cw_serde]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header {
	// #[schemars(with = "String")]
	// #[serde(with = "Base64", default)]
	pub data: Bytes,
	pub height: Option<Height>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Misbehaviour {
	// #[schemars(with = "String")]
	// #[serde(with = "Base64", default)]
	pub data: Bytes,
	pub client_id: ClientId,
}

impl From<Misbehaviour> for RawMisbehaviour {
	fn from(value: Misbehaviour) -> Self {
		RawMisbehaviour { client_id: value.client_id.to_string(), data: value.data }
	}
}

impl From<Header> for RawHeader {
	fn from(value: Header) -> Self {
		RawHeader { data: value.data, height: None }
	}
}
