// super::wrap!(cf_guest_upstream::Misbehaviour<PK> as Misbehaviour);
// super::wrap!(impl<PK> proto for Misbehaviour);

use crate::proto;
use guestchain::PubKey;
use proto_utils::BadMessage;
use std::marker::PhantomData;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Misbehaviour<T> {
	_never: std::convert::Infallible,
	_phantom: PhantomData<T>,
}

impl<PK: PubKey> From<Misbehaviour<PK>> for proto::Misbehaviour {
	fn from(msg: Misbehaviour<PK>) -> Self {
		Self::from(&msg)
	}
}

impl<PK: PubKey> From<&Misbehaviour<PK>> for proto::Misbehaviour {
	fn from(_msg: &Misbehaviour<PK>) -> Self {
		todo!("Misbehaviour::from")
	}
}

impl<PK: PubKey> TryFrom<proto::Misbehaviour> for Misbehaviour<PK> {
	type Error = BadMessage;
	fn try_from(msg: proto::Misbehaviour) -> Result<Self, Self::Error> {
		Self::try_from(&msg)
	}
}

impl<PK: PubKey> TryFrom<&proto::Misbehaviour> for Misbehaviour<PK> {
	type Error = BadMessage;
	fn try_from(_msg: &proto::Misbehaviour) -> Result<Self, Self::Error> {
		todo!("Misbehaviour::try_from")
	}
}

proto_utils::define_wrapper! {
	proto: proto::Misbehaviour,
	wrapper: Misbehaviour<PK> where
		PK: guestchain::PubKey = guestchain::validators::MockPubKey,
}
