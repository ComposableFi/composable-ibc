use guestchain::PubKey;
use ibc_proto::google::protobuf::Any;
use prost::Message as _;
use tendermint_proto::Protobuf;

use crate::proto;

super::wrap!(cf_guest_upstream::ClientMessage<PK> as ClientMessage);
super::wrap!(impl<PK> display Debug for ClientMessage);
super::wrap!(impl<PK> any for ClientMessage);
super::wrap!(impl<PK> proto for ClientMessage);

impl<PK> ibc::core::ics02_client::client_message::ClientMessage for ClientMessage<PK>
where
	PK: PubKey + Send + Sync,
	PK::Signature: Send + Sync,
{
	fn encode_to_vec(&self) -> Result<ibc::prelude::Vec<u8>, tendermint_proto::Error> {
		Ok(proto::ClientMessage::from(self).encode_to_vec())
	}
}

impl<PK: PubKey> Protobuf<Any> for ClientMessage<PK> {}
