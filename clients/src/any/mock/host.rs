//! Host chain types and methods, used by context mock.

use tendermint_testgen::light_block::TmLightBlock;
use tendermint_testgen::{Generator, LightBlock as TestgenLightBlock};

use crate::any::consensus_state::AnyConsensusState;
use crate::any::header::AnyHeader;
use crate::ics07_tendermint::consensus_state::ConsensusState as TMConsensusState;
use ibc::core::ics24_host::identifier::ChainId;
use ibc::mock::client_state::MockConsensusState;
use ibc::mock::header::MockHeader;
use ibc::mock::host::HostBlock;
use ibc::prelude::*;
use ibc::timestamp::Timestamp;
use ibc::Height;

/// Defines the different types of host chains that a mock context can emulate.
/// The variants are as follows:
/// - `Mock` defines that the context history consists of `MockHeader` blocks.
/// - `SyntheticTendermint`: the context has synthetically-generated Tendermint (light) blocks.
/// See also the `HostBlock` enum to get more insights into the underlying block type.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum MockHostType {
    Mock,
    SyntheticTendermint,
    Beefy,
}

impl Default for MockHostType {
    fn default() -> Self {
        MockHostType::Mock
    }
}

/// Depending on `HostType` (the type of host chain underlying a context mock), this enum defines
/// the type of blocks composing the history of the host chain.
#[derive(Clone, Debug)]
pub enum MockHostBlock {
    Mock(MockHeader),
    SyntheticTendermint(Box<TmLightBlock>),
}

impl HostBlock for MockHostBlock {
    type HostType = MockHostType;

    /// Returns the height of a block.
    fn height(&self) -> Height {
        match self {
            MockHostBlock::Mock(header) => header.height(),
            MockHostBlock::SyntheticTendermint(light_block) => Height::new(
                ChainId::chain_version(light_block.signed_header.header.chain_id.as_str()),
                light_block.signed_header.header.height.value(),
            ),
        }
    }

    /// Returns the timestamp of a block.
    fn timestamp(&self) -> Timestamp {
        match self {
            MockHostBlock::Mock(header) => header.timestamp,
            MockHostBlock::SyntheticTendermint(light_block) => {
                light_block.signed_header.header.time.into()
            }
        }
    }

    /// Generates a new block at `height` for the given chain identifier and chain type.
    fn generate_block(
        chain_id: ChainId,
        chain_type: MockHostType,
        height: u64,
        timestamp: Timestamp,
    ) -> MockHostBlock {
        match chain_type {
            MockHostType::Mock | MockHostType::Beefy => MockHostBlock::Mock(MockHeader {
                height: Height::new(chain_id.version(), height),
                timestamp,
            }),
            MockHostType::SyntheticTendermint => MockHostBlock::SyntheticTendermint(Box::new(
                Self::generate_tm_block(chain_id, height, timestamp),
            )),
        }
    }
}

impl MockHostBlock {
    pub fn generate_tm_block(chain_id: ChainId, height: u64, timestamp: Timestamp) -> TmLightBlock {
        TestgenLightBlock::new_default_with_time_and_chain_id(
            chain_id.to_string(),
            timestamp.into_tm_time().unwrap(),
            height,
        )
        .generate()
        .unwrap()
    }
}

impl From<TmLightBlock> for AnyConsensusState {
    fn from(light_block: TmLightBlock) -> Self {
        let cs = TMConsensusState::from(light_block.signed_header.header);
        AnyConsensusState::Tendermint(cs)
    }
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

impl From<MockHeader> for AnyHeader {
    fn from(mh: MockHeader) -> Self {
        Self::Mock(mh)
    }
}

impl From<MockHeader> for AnyConsensusState {
    fn from(h: MockHeader) -> Self {
        AnyConsensusState::Mock(MockConsensusState::new(h))
    }
}
