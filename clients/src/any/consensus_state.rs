use crate::ics07_tendermint::consensus_state;
use core::convert::Infallible;
use core::fmt::Debug;
use ibc::core::ics02_client::client_consensus::ConsensusState;
use ibc::core::ics02_client::error::Error;
use ibc::core::ics23_commitment::commitment::CommitmentRoot;
use ibc::prelude::*;
use ibc::timestamp::Timestamp;
use ibc_proto::google::protobuf::Any;
use serde::Serialize;
use tendermint_proto::Protobuf;

#[cfg(any(test, feature = "ics11_beefy"))]
use crate::ics11_beefy::consensus_state as beefy_consensus_state;

#[cfg(any(test, feature = "ics11_beefy"))]
use crate::ics13_near::consensus_state as near_consensus_state;

#[cfg(any(test, feature = "mocks"))]
use ibc::mock::client_state::MockConsensusState;

pub const TENDERMINT_CONSENSUS_STATE_TYPE_URL: &str =
    "/ibc.lightclients.tendermint.v1.ConsensusState";
pub const BEEFY_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.lightclients.beefy.v1.ConsensusState";
pub const NEAR_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.lightclients.near.v1.ConsensusState";
pub const MOCK_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.mock.ConsensusState";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, ConsensusState, Protobuf)]
#[serde(tag = "type")]
pub enum AnyConsensusState {
    #[ibc(proto_url = "TENDERMINT_CONSENSUS_STATE_TYPE_URL")]
    Tendermint(consensus_state::ConsensusState),
    #[cfg(any(test, feature = "ics11_beefy"))]
    #[ibc(proto_url = "BEEFY_CONSENSUS_STATE_TYPE_URL")]
    Beefy(beefy_consensus_state::ConsensusState),
    #[cfg(any(test, feature = "ics11_beefy"))]
    #[ibc(proto_url = "NEAR_CONSENSUS_STATE_TYPE_URL")]
    Near(near_consensus_state::ConsensusState),
    #[cfg(any(test, feature = "mocks"))]
    #[ibc(proto_url = "MOCK_CONSENSUS_STATE_TYPE_URL")]
    Mock(MockConsensusState),
}
