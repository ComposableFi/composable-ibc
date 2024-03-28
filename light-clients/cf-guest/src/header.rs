use guestchain::PubKey;

use crate::proto;

super::wrap!(cf_guest_upstream::Header<PK> as Header);
super::wrap!(impl<PK> display Debug for Header);
super::wrap!(impl<PK> any for Header);

impl<PK: PubKey> From<Header<PK>> for proto::Header {
	fn from(header: Header<PK>) -> Self {
		Self(cf_guest_upstream::proto::Header::from(&header.0))
	}
}

impl<PK: PubKey> From<&Header<PK>> for proto::Header {
	fn from(header: &Header<PK>) -> Self {
		Self(cf_guest_upstream::proto::Header::from(&header.0))
	}
}

impl<PK: PubKey> TryFrom<proto::Header> for Header<PK> {
	type Error = proto::BadMessage;
	fn try_from(msg: proto::Header) -> Result<Self, Self::Error> {
		Self::try_from(&msg)
	}
}

impl<PK: PubKey> TryFrom<&proto::Header> for Header<PK> {
	type Error = proto::BadMessage;
	fn try_from(msg: &proto::Header) -> Result<Self, Self::Error> {
		Ok(Self(cf_guest_upstream::Header::try_from(msg.0)?))
	}
}
