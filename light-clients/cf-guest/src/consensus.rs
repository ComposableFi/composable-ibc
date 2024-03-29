use core::{convert::Infallible, num::NonZeroU64};

use lib::hash::CryptoHash;
use prost::Message as _;

use crate::proto;

super::wrap!(cf_guest_upstream::ConsensusState as ConsensusState);
super::wrap!(impl Eq for ConsensusState);
super::wrap!(impl display Debug for ConsensusState);
super::wrap!(impl any for ConsensusState);

impl ConsensusState {
	pub fn new(block_hash: &CryptoHash, timestamp_ns: NonZeroU64) -> Self {
		Self(cf_guest_upstream::ConsensusState::new(block_hash, timestamp_ns))
	}
}

impl ibc::core::ics02_client::client_consensus::ConsensusState for ConsensusState {
	type Error = Infallible;

	fn root(&self) -> &ibc::core::ics23_commitment::commitment::CommitmentRoot {
		// SAFETY: Both types are wrappers around Vec<u8>.
		unsafe { core::mem::transmute(&self.0.block_hash) }
	}

	fn timestamp(&self) -> ibc::timestamp::Timestamp {
		ibc::timestamp::Timestamp::from_nanoseconds(self.0.timestamp_ns.get()).unwrap()
	}

	fn encode_to_vec(&self) -> Result<ibc::prelude::Vec<u8>, tendermint_proto::Error> {
		Ok(proto::ConsensusState::from(self).encode_to_vec())
	}
}

impl<PK: guestchain::PubKey> From<crate::Header<PK>> for ConsensusState {
	fn from(header: crate::Header<PK>) -> Self {
		Self::from(&header.0)
	}
}

impl<PK: guestchain::PubKey> From<&crate::Header<PK>> for ConsensusState {
	fn from(header: &crate::Header<PK>) -> Self {
		Self::from(&header.0)
	}
}

impl<PK: guestchain::PubKey> From<cf_guest_upstream::Header<PK>> for ConsensusState {
	fn from(header: cf_guest_upstream::Header<PK>) -> Self {
		Self::from(&header)
	}
}

impl<PK: guestchain::PubKey> From<&cf_guest_upstream::Header<PK>> for ConsensusState {
	fn from(header: &cf_guest_upstream::Header<PK>) -> Self {
		Self(header.into())
	}
}

impl From<ConsensusState> for proto::ConsensusState {
	fn from(state: ConsensusState) -> Self {
		Self(cf_guest_upstream::proto::ConsensusState::from(&state.0))
	}
}

impl From<&ConsensusState> for proto::ConsensusState {
	fn from(state: &ConsensusState) -> Self {
		Self(cf_guest_upstream::proto::ConsensusState::from(&state.0))
	}
}

impl TryFrom<proto::ConsensusState> for ConsensusState {
	type Error = proto::BadMessage;
	fn try_from(msg: proto::ConsensusState) -> Result<Self, Self::Error> {
		Self::try_from(&msg)
	}
}

impl TryFrom<&proto::ConsensusState> for ConsensusState {
	type Error = proto::BadMessage;
	fn try_from(msg: &proto::ConsensusState) -> Result<Self, Self::Error> {
		Ok(Self(cf_guest_upstream::ConsensusState::try_from(msg.0)?))
	}
}
