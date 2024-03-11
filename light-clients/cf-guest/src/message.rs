use guestchain::PubKey;
use ibc_proto::google::protobuf::Any;
use tendermint_proto::Protobuf;

use crate::{Header, Misbehaviour};

#[derive(Clone, PartialEq, Eq, Debug, derive_more::From, derive_more::TryInto)]
// For the time being allow large enum variants.  Header is short of 400 bytes
// and Misbehaviour is short of 700.  We may want to box the values if we run
// into stack size issues.
#[allow(clippy::large_enum_variant)]
pub enum ClientMessage<PK: PubKey> {
	Header(Header<PK>),
	Misbehaviour(Misbehaviour<PK>),
}

impl<PK: PubKey> ibc::core::ics02_client::client_message::ClientMessage for ClientMessage<PK> {
	fn encode_to_vec(&self) -> Result<ibc::prelude::Vec<u8>, tendermint_proto::Error> {
		self.encode_vec()
	}
}

impl<PK: PubKey> TryFrom<Any> for ClientMessage<PK> {
	type Error = crate::proto::DecodeError;
	fn try_from(any: Any) -> Result<Self, Self::Error> {
		Self::try_from(&any)
	}
}

impl<PK: PubKey> Protobuf<Any> for ClientMessage<PK> {}

impl<PK: PubKey> TryFrom<&Any> for ClientMessage<PK> {
	type Error = crate::proto::DecodeError;

	fn try_from(any: &Any) -> Result<Self, Self::Error> {
		match any.type_url.as_str() {
			crate::proto::Header::TYPE_URL => Header::decode(&any.value).map(Self::Header),
			crate::proto::Misbehaviour::TYPE_URL =>
				Misbehaviour::decode(&any.value).map(Self::Misbehaviour),
			_ => Err(crate::proto::DecodeError::BadType),
		}
	}
}

impl<PK: PubKey> From<ClientMessage<PK>> for Any {
	fn from(msg: ClientMessage<PK>) -> Any {
		Self::from(&msg)
	}
}

impl<PK: PubKey> From<&ClientMessage<PK>> for Any {
	fn from(msg: &ClientMessage<PK>) -> Any {
		match msg {
			ClientMessage::Header(msg) => msg.into(),
			ClientMessage::Misbehaviour(msg) => msg.into(),
		}
	}
}
