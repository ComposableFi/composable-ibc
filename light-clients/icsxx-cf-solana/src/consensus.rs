use crate::{ClientState, Header};
use core::{convert::Infallible, num::NonZeroU64};
use ibc::core::ics02_client::error::Error as Ics02ClientError;
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
		header: &Header<()>,
		client_state: &ClientState<()>,
	) -> Result<Self, Ics02ClientError> {
		let hash = header.hash();
		let timestamp = client_state.timestamp_for_slot(header.slot());
		let nanos = NonZeroU64::try_from(timestamp.nanoseconds()).map_err(|e| {
			Ics02ClientError::implementation_specific(alloc::format!("invalid timestamp: {}", e))
		})?;
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

#[cfg(test)]
mod tests {
	use super::*;

	const ANY_MESSAGE: [u8; 85] = [
		10, 37, 47, 108, 105, 103, 104, 116, 99, 108, 105, 101, 110, 116, 115, 46, 103, 117, 101,
		115, 116, 46, 118, 49, 46, 67, 111, 110, 115, 101, 110, 115, 117, 115, 83, 116, 97, 116,
		101, 18, 44, 10, 32, 74, 147, 61, 207, 26, 96, 73, 253, 54, 118, 91, 237, 36, 210, 58, 218,
		179, 236, 158, 187, 5, 231, 241, 133, 178, 150, 85, 151, 36, 160, 36, 105, 16, 128, 220,
		164, 128, 131, 220, 190, 228, 23,
	];

	fn message() -> &'static [u8] {
		&ANY_MESSAGE[41..]
	}

	const BLOCK_HASH: CryptoHash = CryptoHash([
		74, 147, 61, 207, 26, 96, 73, 253, 54, 118, 91, 237, 36, 210, 58, 218, 179, 236, 158, 187,
		5, 231, 241, 133, 178, 150, 85, 151, 36, 160, 36, 105,
	]);

	fn check(state: ConsensusState) {
		let want = ConsensusState::new(&BLOCK_HASH, NonZeroU64::new(1713895499000000000).unwrap());
		assert_eq!(want, state);
	}

	#[test]
	fn test_decode_vec() {
		check(ibc::protobuf::Protobuf::decode_vec(message()).unwrap());
	}

	#[test]
	fn test_from_any() {
		use ibc_proto::google::protobuf::Any;

		let any: Any = prost::Message::decode(ANY_MESSAGE.as_ref()).unwrap();
		check(any.try_into().unwrap());
	}
}
