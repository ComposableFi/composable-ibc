use guestchain::PubKey;
use prost::Message as _;

use crate::proto;

super::wrap!(cf_guest_upstream::ClientMessage<PK> as ClientMessage);
super::wrap!(impl<PK> proto for ClientMessage);

impl<PK: PubKey> ClientMessage<PK> {
	pub fn maybe_header_height(&self) -> Option<ibc::Height> {
		if let cf_guest_upstream::ClientMessage::Header(hdr) = &self.0 {
			let height = hdr.block_header.block_height;
			Some(ibc::Height::new(1, height.into()))
		} else {
			None
		}
	}
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

impl<PK: guestchain::PubKey> From<crate::Header<PK>> for ClientMessage<PK> {
	fn from(msg: crate::Header<PK>) -> Self {
		Self(cf_guest_upstream::ClientMessage::Header(msg.0))
	}
}

impl<PK: guestchain::PubKey> From<crate::Misbehaviour<PK>> for ClientMessage<PK> {
	fn from(msg: crate::Misbehaviour<PK>) -> Self {
		Self(cf_guest_upstream::ClientMessage::Misbehaviour(msg.0.into()))
	}
}
