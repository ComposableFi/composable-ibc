use ibc::prelude::*;

use crate::ics07_tendermint::misbehaviour::Misbehaviour as TmMisbehaviour;
use ibc::core::ics02_client::error::Error;
use ibc::core::ics02_client::misbehaviour::Misbehaviour;
use ibc_proto::google::protobuf::Any;
use tendermint_proto::Protobuf;

#[cfg(any(test, feature = "mocks"))]
use ibc::mock::misbehaviour::MockMisbehaviour;

use ibc::core::ics24_host::identifier::ClientId;
use ibc::Height;

use super::header::AnyHeader;

pub const TENDERMINT_MISBEHAVIOR_TYPE_URL: &str = "/ibc.lightclients.tendermint.v1.Misbehaviour";

#[cfg(any(test, feature = "mocks"))]
pub const MOCK_MISBEHAVIOUR_TYPE_URL: &str = "/ibc.mock.Misbehavior";

#[derive(Clone, Debug, PartialEq, Misbehaviour, Protobuf)] // TODO: Add Eq bound once possible
#[allow(clippy::large_enum_variant)]
pub enum AnyMisbehaviour {
    #[ibc(proto_url = "TENDERMINT_MISBEHAVIOR_TYPE_URL")]
    Tendermint(TmMisbehaviour),

    #[cfg(any(test, feature = "mocks"))]
    #[ibc(proto_url = "MOCK_MISBEHAVIOUR_TYPE_URL")]
    Mock(MockMisbehaviour),
}

impl core::fmt::Display for AnyMisbehaviour {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            AnyMisbehaviour::Tendermint(tm) => write!(f, "{}", tm),

            #[cfg(any(test, feature = "mocks"))]
            AnyMisbehaviour::Mock(mock) => write!(f, "{:?}", mock),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MisbehaviourEvidence {
    pub misbehaviour: AnyMisbehaviour,
    pub supporting_headers: Vec<AnyHeader>,
}
