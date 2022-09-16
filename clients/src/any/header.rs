use ibc_proto::google::protobuf::Any;
use serde_derive::{Deserialize, Serialize};
use subtle_encoding::hex;
use tendermint_proto::Protobuf;

use crate::ics07_tendermint::header::Header as TendermintHeader;
#[cfg(any(test, feature = "ics11_beefy"))]
use crate::ics11_beefy::header::BeefyHeader;
#[cfg(any(test, feature = "ics11_beefy"))]
use crate::ics13_near::header::NearHeader;
use ibc::core::ics02_client::error::Error;
use ibc::core::ics02_client::header::Header;
#[cfg(any(test, feature = "mocks"))]
use ibc::mock::header::MockHeader;
use ibc::prelude::*;
use ibc::timestamp::Timestamp;
use ibc::Height;

pub const TENDERMINT_HEADER_TYPE_URL: &str = "/ibc.lightclients.tendermint.v1.Header";
pub const BEEFY_HEADER_TYPE_URL: &str = "/ibc.lightclients.beefy.v1.Header";
pub const NEAR_HEADER_TYPE_URL: &str = "/ibc.lightclients.near.v1.Header";
pub const MOCK_HEADER_TYPE_URL: &str = "/ibc.mock.Header";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Header, Protobuf)]
#[allow(clippy::large_enum_variant)]
pub enum AnyHeader {
    #[ibc(proto_url = "TENDERMINT_HEADER_TYPE_URL")]
    Tendermint(TendermintHeader),
    #[serde(skip)]
    #[cfg(any(test, feature = "ics11_beefy"))]
    #[ibc(proto_url = "BEEFY_HEADER_TYPE_URL")]
    Beefy(BeefyHeader),
    #[serde(skip)]
    #[cfg(any(test, feature = "ics11_beefy"))]
    #[ibc(proto_url = "NEAR_HEADER_TYPE_URL")]
    Near(NearHeader),
    #[cfg(any(test, feature = "mocks"))]
    #[ibc(proto_url = "MOCK_HEADER_TYPE_URL")]
    Mock(MockHeader),
}

impl AnyHeader {
    pub fn timestamp(&self) -> Timestamp {
        match self {
            Self::Tendermint(header) => header.timestamp(),
            #[cfg(any(test, feature = "ics11_beefy"))]
            Self::Beefy(_header) => Default::default(),
            #[cfg(any(test, feature = "ics11_beefy"))]
            Self::Near(_header) => Default::default(),
            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(header) => header.timestamp(),
        }
    }
}

impl AnyHeader {
    pub fn encode_to_string(&self) -> String {
        let buf = Protobuf::encode_vec(self);
        let encoded = hex::encode(buf);
        String::from_utf8(encoded).expect("hex-encoded string should always be valid UTF-8")
    }

    pub fn decode_from_string(s: &str) -> Result<Self, Error> {
        let header_bytes = hex::decode(s).unwrap();
        Protobuf::decode(header_bytes.as_ref()).map_err(Error::invalid_raw_header)
    }
}
