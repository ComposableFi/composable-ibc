use crate::solana::shred::ShredType;
use core::fmt::{Display, Formatter};
use solana_sdk::clock::Slot;
use std::prelude::rust_2015::{String, ToString};

#[derive(Clone, Debug)]
pub enum Error {
	InvalidShredVariant,
	ShredSignatureVerificationFailed,
	ShredMerkleRootCalculationFailed,
	InvalidShredType,
	InvalidMerkleProof,
	InvalidPayloadSize(usize),
	InvalidErasureShardIndex,
	InvalidParentOffset { slot: Slot, parent_offset: u16 },
	InvalidProofSize(u8),
	InvalidShardSize(usize),
	InvalidDataSize(u16),
	InvalidShredIndex(ShredType, u32),
	InvalidNumCodingShreds(u16),
	InvalidShredFlags(u8),
	TooFewDataShards,
	Bincode(String), // `bincode::Error` doesn't implement `Clone`
}

// TODO: implement proper Display formatting
impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(f, "{self:?}")
	}
}

impl From<bincode::Error> for Error {
	fn from(err: bincode::Error) -> Self {
		Self::Bincode(err.to_string())
	}
}
