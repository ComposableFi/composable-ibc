use guestchain::PubKey;

use crate::{proto, Header};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Misbehaviour<PK: PubKey> {
    header1: Header<PK>,
    header2: Header<PK>,
}

impl<PK: PubKey> From<Misbehaviour<PK>> for proto::Misbehaviour {
    fn from(msg: Misbehaviour<PK>) -> Self { Self::from(&msg) }
}

impl<PK: PubKey> From<&Misbehaviour<PK>> for proto::Misbehaviour {
    fn from(msg: &Misbehaviour<PK>) -> Self {
        Self {
            header1: Some((&msg.header1).into()),
            header2: Some((&msg.header2).into()),
        }
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
        Ok(Self {
            header1: msg
                .header1
                .as_ref()
                .ok_or(proto::BadMessage)?
                .try_into()?,
            header2: msg
                .header2
                .as_ref()
                .ok_or(proto::BadMessage)?
                .try_into()?,
        })
    }
}

super::any_convert! {
    proto::Misbehaviour,
    Misbehaviour<PK: guestchain::PubKey = guestchain::validators::MockPubKey>,
    // TODO(mina86): Add `obj: ...`.
}
