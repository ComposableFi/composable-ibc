use super::error::Error;
use ibc::core::ics02_client::client_consensus::{self};
use ibc::core::ics23_commitment::commitment::CommitmentRoot;
use ibc::timestamp::Timestamp;
use serde::Serialize;
use tendermint_proto::Protobuf;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsensusState {
    commitment_root: CommitmentRoot,
}

impl client_consensus::ConsensusState for ConsensusState {
    type Error = Error;

    fn root(&self) -> &CommitmentRoot {
        &self.commitment_root
    }

    fn timestamp(&self) -> Timestamp {
        todo!()
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        todo!("Encode to vec")
    }
}

impl Protobuf<()> for ConsensusState {}

impl From<ConsensusState> for () {
    fn from(_: ConsensusState) -> Self {
        todo!()
    }
}

impl From<()> for ConsensusState {
    fn from(_: ()) -> Self {
        todo!()
    }
}
