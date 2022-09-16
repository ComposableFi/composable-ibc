pub mod context;
pub mod host;

use crate::any::client_def::AnyClient;
use crate::any::client_state::AnyClientState;
use crate::any::consensus_state::AnyConsensusState;
use crate::any::header::AnyHeader;
use crate::any::misbehaviour::AnyMisbehaviour;
use crate::any::mock::context::Crypto;
use crate::ics07_tendermint::mock::host::MockHostBlock;
use ibc::mock::client_state::{MockClientState, MockConsensusState};
use ibc::mock::context::ClientTypes;
use ibc::prelude::*;

impl From<MockConsensusState> for AnyConsensusState {
    fn from(mcs: MockConsensusState) -> Self {
        Self::Mock(mcs)
    }
}

impl From<MockClientState> for AnyClientState<Crypto> {
    fn from(mcs: MockClientState) -> Self {
        Self::Mock(mcs)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct MockClientTypes;
impl ClientTypes for MockClientTypes {
    type AnyHeader = AnyHeader;
    type AnyClientState = AnyClientState<Crypto>;
    type AnyConsensusState = AnyConsensusState;
    type AnyMisbehaviour = AnyMisbehaviour;
    type HostFunctions = Crypto;
    type ClientDef = AnyClient<Crypto>;
    type HostBlock = MockHostBlock;
}

impl From<MockHostBlock> for AnyConsensusState {
    fn from(any_block: MockHostBlock) -> Self {
        match any_block {
            MockHostBlock::Mock(mock_header) => mock_header.into(),
            MockHostBlock::SyntheticTendermint(light_block) => (*light_block).into(),
        }
    }
}

impl From<MockHostBlock> for AnyHeader {
    fn from(any_block: MockHostBlock) -> Self {
        match any_block {
            MockHostBlock::Mock(mock_header) => mock_header.into(),
            MockHostBlock::SyntheticTendermint(light_block_box) => {
                // Conversion from TMLightBlock to AnyHeader
                AnyHeader::Tendermint((*light_block_box).into())
            }
        }
    }
}
