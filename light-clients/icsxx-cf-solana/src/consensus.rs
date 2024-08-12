use crate::{error::Error, ClientState, Header};
use core::{convert::Infallible, num::NonZeroU64};
use lib::hash::CryptoHash;
use solana_sdk::hash::Hash;

super::wrap!(cf_guest_upstream::ConsensusState as ConsensusState);
super::wrap!(impl Eq for ConsensusState);
// super::wrap!(impl proto for ConsensusState);

impl ConsensusState {
	pub fn new(block_hash: &Hash, timestamp_ns: NonZeroU64) -> Self {
		Self(cf_guest_upstream::ConsensusState::new(
			&CryptoHash(block_hash.to_bytes()),
			timestamp_ns,
		))
	}

	pub fn from_header_and_client_state(
		header: &Header,
		client_state: &ClientState,
	) -> Result<Self, Error> {
		let hash = header.calculate_hash()?;
		let timestamp_ns = client_state.timestamp_for_slot_ns(header.slot());
		let nanos = NonZeroU64::try_from(timestamp_ns).map_err(|_| Error::InvalidTimestamp)?;
		Ok(Self::new(&hash, nanos))
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

	fn encode_to_vec(&self) -> Result<ibc::prelude::Vec<u8>, ibc::protobuf::Error> {
		Ok(self.0.encode_to_vec().expect("encoding failed"))
	}
}
