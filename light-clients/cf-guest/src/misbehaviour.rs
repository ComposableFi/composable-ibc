use guestchain::PubKey;
use ibc_proto::google::protobuf::Any;
use tendermint_proto::Protobuf;

use crate::proto;

super::wrap!(cf_guest_upstream::Misbehaviour<PK> as Misbehaviour);
super::wrap!(impl<PK> display Debug for Misbehaviour);
super::wrap!(impl<PK> any for Misbehaviour);

impl<PK: PubKey> Protobuf<Any> for Misbehaviour<PK> {}

impl<PK: PubKey> From<Misbehaviour<PK>> for proto::Misbehaviour {
	fn from(msg: Misbehaviour<PK>) -> Self {
		Self(cf_guest_upstream::proto::Misbehaviour::from(&msg.0))
	}
}

impl<PK: PubKey> From<&Misbehaviour<PK>> for proto::Misbehaviour {
	fn from(msg: &Misbehaviour<PK>) -> Self {
		Self(cf_guest_upstream::proto::Misbehaviour::from(&msg.0))
	}
}

impl<PK: PubKey> TryFrom<proto::Misbehaviour> for Misbehaviour<PK> {
	type Error = proto::BadMessage;
	fn try_from(msg: proto::Misbehaviour) -> Result<Self, Self::Error> {
		Self::try_from(&msg)
	}
}

impl<PK: PubKey> TryFrom<&proto::Misbehaviour> for Misbehaviour<PK> {
	type Error = proto::BadMessage;
	fn try_from(msg: &proto::Misbehaviour) -> Result<Self, Self::Error> {
		Ok(Self(cf_guest_upstream::Misbehaviour::try_from(msg.0)?))
	}
}
