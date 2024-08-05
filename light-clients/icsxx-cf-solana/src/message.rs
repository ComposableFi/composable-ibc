use crate::{
	proto,
	proto::{client_message, client_message::Message},
	Header, Misbehaviour,
};
use guestchain::PubKey;
use prost::Message as _;
use proto_utils::{BadMessage, DecodeError};

#[derive(Clone, PartialEq, Eq, Debug, derive_more::From, derive_more::TryInto)]
// For the time being allow large enum variants.  Header is short of 400 bytes
// and Misbehaviour is short of 700.  We may want to box the values if we run
// into stack size issues.
#[allow(clippy::large_enum_variant)]
pub enum ClientMessage<PK: PubKey> {
	Header(Header<PK>),
	Misbehaviour(Misbehaviour<PK>),
}

impl<PK> ibc::core::ics02_client::client_message::ClientMessage for ClientMessage<PK>
where
	PK: PubKey + Send + Sync,
	PK::Signature: Send + Sync,
{
	fn encode_to_vec(&self) -> Result<ibc::prelude::Vec<u8>, ibc::protobuf::Error> {
		Ok(proto::ClientMessage::from(self).encode_to_vec())
	}
}

impl<PK: guestchain::PubKey> From<ClientMessage<PK>> for Message {
	fn from(msg: ClientMessage<PK>) -> Self {
		Self::from(&msg)
	}
}

impl<PK: guestchain::PubKey> From<&ClientMessage<PK>> for Message {
	fn from(msg: &ClientMessage<PK>) -> Self {
		match msg {
			ClientMessage::Header(msg) => Self::Header(msg.into()),
			ClientMessage::Misbehaviour(msg) => Self::Misbehaviour(msg.into()),
		}
	}
}

impl<PK: guestchain::PubKey> TryFrom<Message> for ClientMessage<PK> {
	type Error = BadMessage;
	fn try_from(msg: Message) -> Result<Self, Self::Error> {
		Self::try_from(&msg)
	}
}

impl<PK: guestchain::PubKey> TryFrom<&Message> for ClientMessage<PK> {
	type Error = BadMessage;
	fn try_from(msg: &Message) -> Result<Self, Self::Error> {
		match msg {
			Message::Header(msg) => msg.try_into().map(Self::Header),
			Message::Misbehaviour(mb) => mb.try_into().map(Self::Misbehaviour),
		}
	}
}

impl<PK: guestchain::PubKey> From<ClientMessage<PK>> for proto::ClientMessage {
	fn from(msg: ClientMessage<PK>) -> Self {
		Self::from(&msg)
	}
}

impl<PK: guestchain::PubKey> From<&ClientMessage<PK>> for proto::ClientMessage {
	fn from(msg: &ClientMessage<PK>) -> Self {
		let message = Some(match msg {
			ClientMessage::Header(msg) => client_message::Message::Header(msg.into()),
			ClientMessage::Misbehaviour(msg) => client_message::Message::Misbehaviour(msg.into()),
		});
		Self { message }
	}
}

impl<PK: guestchain::PubKey> TryFrom<proto::ClientMessage> for ClientMessage<PK> {
	type Error = BadMessage;
	fn try_from(msg: proto::ClientMessage) -> Result<Self, Self::Error> {
		Self::try_from(&msg)
	}
}

impl<PK: guestchain::PubKey> TryFrom<&proto::ClientMessage> for ClientMessage<PK> {
	type Error = BadMessage;
	fn try_from(msg: &proto::ClientMessage) -> Result<Self, Self::Error> {
		msg.message.as_ref().ok_or(BadMessage).and_then(Self::try_from)
	}
}

proto_utils::define_wrapper! {
	proto: proto::ClientMessage,
	wrapper: ClientMessage<PK> where
		PK: guestchain::PubKey = guestchain::validators::MockPubKey,
	custom_any
}

impl<PK: guestchain::PubKey> proto_utils::AnyConvert for ClientMessage<PK> {
	fn to_any(&self) -> (&'static str, alloc::vec::Vec<u8>) {
		match self {
			Self::Header(msg) => msg.to_any(),
			Self::Misbehaviour(msg) => msg.to_any(),
		}
	}

	fn try_from_any(type_url: &str, value: &[u8]) -> Result<Self, proto_utils::DecodeError> {
		if type_url.ends_with(proto::ClientMessage::IBC_TYPE_URL) {
			Self::decode(value)
		} else if type_url.ends_with(proto::Header::IBC_TYPE_URL) {
			Header::decode(value).map(Self::Header)
		} else if type_url.ends_with(proto::Misbehaviour::IBC_TYPE_URL) {
			Misbehaviour::decode(value).map(Self::Misbehaviour)
		} else {
			Err(DecodeError::BadType)
		}
	}
}
