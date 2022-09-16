use crate::any::client_state::AnyClientState;
use crate::any::consensus_state::AnyConsensusState;
use crate::any::header::AnyHeader;
use crate::ics07_tendermint::client_def::TendermintClient;
#[cfg(any(test, feature = "ics11_beefy"))]
use crate::ics11_beefy::client_def::BeefyClient;
#[cfg(any(test, feature = "ics11_beefy"))]
use crate::ics13_near::client_def::NearClient;
use crate::AnyHostFunctionsTrait;
use core::fmt::Debug;
use ibc::core::ics02_client::client_def::ClientDef;
use ibc::core::ics02_client::client_def::ConsensusUpdateResult;
use ibc::core::ics02_client::client_state::ClientState;
use ibc::core::ics02_client::error::Error;
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics04_channel::channel::ChannelEnd;
use ibc::core::ics04_channel::commitment::{AcknowledgementCommitment, PacketCommitment};
use ibc::core::ics04_channel::packet::Sequence;
use ibc::core::ics23_commitment::commitment::{
    CommitmentPrefix, CommitmentProofBytes, CommitmentRoot,
};
use ibc::core::ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId};
use ibc::core::ics26_routing::context::ReaderContext;
use ibc::downcast;
#[cfg(any(test, feature = "mocks"))]
use ibc::mock::client_def::MockClient;

use ibc::prelude::*;
use ibc::Height;

#[derive(Clone, Debug, PartialEq, Eq, ClientDef)]
pub enum AnyClient<H>
where
    H: AnyHostFunctionsTrait,
{
    Tendermint(TendermintClient<H>),
    #[cfg(any(test, feature = "ics11_beefy"))]
    Beefy(BeefyClient<H>),
    #[cfg(any(test, feature = "ics11_beefy"))]
    Near(NearClient<H>),
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockClient),
}
