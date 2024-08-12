// super::wrap!(cf_guest_upstream::Misbehaviour as Misbehaviour);
// super::wrap!(impl proto for Misbehaviour);

use crate::proto;
use proto_utils::BadMessage;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Misbehaviour {
	_never: std::convert::Infallible,
}

impl From<Misbehaviour> for proto::Misbehaviour {
	fn from(msg: Misbehaviour) -> Self {
		Self::from(&msg)
	}
}

impl From<&Misbehaviour> for proto::Misbehaviour {
	fn from(_msg: &Misbehaviour) -> Self {
		todo!("Misbehaviour::from")
	}
}

impl TryFrom<proto::Misbehaviour> for Misbehaviour {
	type Error = BadMessage;
	fn try_from(msg: proto::Misbehaviour) -> Result<Self, Self::Error> {
		Self::try_from(&msg)
	}
}

impl TryFrom<&proto::Misbehaviour> for Misbehaviour {
	type Error = BadMessage;
	fn try_from(_msg: &proto::Misbehaviour) -> Result<Self, Self::Error> {
		todo!("Misbehaviour::try_from")
	}
}

proto_utils::define_wrapper! {
	proto: proto::Misbehaviour,
	wrapper: Misbehaviour,
}
