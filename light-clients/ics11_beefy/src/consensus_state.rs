use ibc::prelude::*;

use core::convert::Infallible;
use core::fmt::Debug;
use serde::Serialize;
use tendermint::time::Time;
use tendermint_proto::google::protobuf as tpb;
use tendermint_proto::Protobuf;

use ibc_proto::ibc::lightclients::beefy::v1::ConsensusState as RawConsensusState;

use crate::ics11_beefy::error::Error;
use crate::ics11_beefy::header::ParachainHeader;
use ibc::core::ics23_commitment::commitment::CommitmentRoot;
use ibc::timestamp::Timestamp;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsensusState {
    pub timestamp: Time,
    pub root: CommitmentRoot,
}

impl ConsensusState {
    pub fn new(root: Vec<u8>, timestamp: Time) -> Self {
        Self {
            timestamp,
            root: root.into(),
        }
    }

    pub fn from_header(header: ParachainHeader) -> Result<Self, Error> {
        use crate::ics11_beefy::header::decode_timestamp_extrinsic;
        use sp_runtime::SaturatedConversion;
        let root = header.parachain_header.state_root.0.to_vec();

        let timestamp = decode_timestamp_extrinsic(&header)?;
        let duration = core::time::Duration::from_millis(timestamp);
        let timestamp = Timestamp::from_nanoseconds(duration.as_nanos().saturated_into::<u64>())
            .map_err(|e| {
                Error::invalid_header(format!(
                    "Failed to decode timestamp extrinsic, got {}",
                    e.to_string()
                ))
            })?
            .into_tm_time()
            .ok_or_else(|| {
                Error::invalid_header(
                    "Error decoding Timestamp, timestamp cannot be zero".to_string(),
                )
            })?;

        Ok(Self {
            root: root.into(),
            timestamp,
        })
    }
}

impl ibc::core::ics02_client::client_consensus::ConsensusState for ConsensusState {
    type Error = Infallible;

    fn root(&self) -> &CommitmentRoot {
        &self.root
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp.into()
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        self.encode_vec()
    }
}

impl Protobuf<RawConsensusState> for ConsensusState {}

impl TryFrom<RawConsensusState> for ConsensusState {
    type Error = Error;

    fn try_from(raw: RawConsensusState) -> Result<Self, Self::Error> {
        let ibc_proto::google::protobuf::Timestamp { seconds, nanos } = raw
            .timestamp
            .ok_or_else(|| Error::invalid_raw_consensus_state("missing timestamp".into()))?;
        let proto_timestamp = tpb::Timestamp { seconds, nanos };
        let timestamp = proto_timestamp
            .try_into()
            .map_err(|e| Error::invalid_raw_consensus_state(format!("invalid timestamp: {}", e)))?;

        Ok(Self {
            root: raw.root.into(),
            timestamp,
        })
    }
}

impl From<ConsensusState> for RawConsensusState {
    fn from(value: ConsensusState) -> Self {
        let tpb::Timestamp { seconds, nanos } = value.timestamp.into();
        let timestamp = ibc_proto::google::protobuf::Timestamp { seconds, nanos };

        RawConsensusState {
            timestamp: Some(timestamp),
            root: value.root.into_vec(),
        }
    }
}

#[cfg(any(test, feature = "mocks"))]
pub mod test_util {
    use super::*;
    use crate::ics11_beefy::mock::AnyConsensusState;

    pub fn get_dummy_beefy_consensus_state() -> AnyConsensusState {
        AnyConsensusState::Beefy(ConsensusState {
            timestamp: Time::now(),
            root: vec![0; 32].into(),
        })
    }
}
