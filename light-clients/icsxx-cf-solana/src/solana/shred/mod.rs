//! File source: solana/ledger/src/shred.rs
//!
//! The `shred` module defines data structures and methods to pull MTU sized data frames from the
//! network. There are two types of shreds: data and coding. Data shreds contain entry information
//! while coding shreds provide redundancy to protect against dropped network packets (erasures).
//!
//! +---------------------------------------------------------------------------------------------+
//! | Data Shred                                                                                  |
//! +---------------------------------------------------------------------------------------------+
//! | common       | data       | payload                                                         |
//! | header       | header     |                                                                 |
//! |+---+---+---  |+---+---+---|+----------------------------------------------------------+----+|
//! || s | s | .   || p | f | s || data (ie ledger entries)                                 | r  ||
//! || i | h | .   || a | l | i ||                                                          | e  ||
//! || g | r | .   || r | a | z || See notes immediately after shred diagrams for an        | s  ||
//! || n | e |     || e | g | e || explanation of the "restricted" section in this payload  | t  ||
//! || a | d |     || n | s |   ||                                                          | r  ||
//! || t |   |     || t |   |   ||                                                          | i  ||
//! || u | t |     ||   |   |   ||                                                          | c  ||
//! || r | y |     || o |   |   ||                                                          | t  ||
//! || e | p |     || f |   |   ||                                                          | e  ||
//! ||   | e |     || f |   |   ||                                                          | d  ||
//! |+---+---+---  |+---+---+---+|----------------------------------------------------------+----+|
//! +---------------------------------------------------------------------------------------------+
//!
//! +---------------------------------------------------------------------------------------------+
//! | Coding Shred                                                                                |
//! +---------------------------------------------------------------------------------------------+
//! | common       | coding     | payload                                                         |
//! | header       | header     |                                                                 |
//! |+---+---+---  |+---+---+---+----------------------------------------------------------------+|
//! || s | s | .   || n | n | p || data (encoded data shred data)                                ||
//! || i | h | .   || u | u | o ||                                                               ||
//! || g | r | .   || m | m | s ||                                                               ||
//! || n | e |     ||   |   | i ||                                                               ||
//! || a | d |     || d | c | t ||                                                               ||
//! || t |   |     ||   |   | i ||                                                               ||
//! || u | t |     || s | s | o ||                                                               ||
//! || r | y |     || h | h | n ||                                                               ||
//! || e | p |     || r | r |   ||                                                               ||
//! ||   | e |     || e | e |   ||                                                               ||
//! ||   |   |     || d | d |   ||                                                               ||
//! |+---+---+---  |+---+---+---+|+--------------------------------------------------------------+|
//! +---------------------------------------------------------------------------------------------+
//!
//! Notes:
//! a) Coding shreds encode entire data shreds: both of the headers AND the payload.
//! b) Coding shreds require their own headers for identification and etc.
//! c) The erasure algorithm requires data shred and coding shred bytestreams to be equal in length.
//!
//! So, given a) - c), we must restrict data shred's payload length such that the entire coding
//! payload can fit into one coding shred / packet.
//!
//! TODO: check if `Legacy` variants of shreds are still being used in Solana. If not, get rif of
//! them, not breaking the encoding

#[cfg(test)]
pub(crate) use self::shred_code::MAX_CODE_SHREDS_PER_SLOT;
pub use self::shred_data::ShredData;
use self::{shred_code::ShredCode, traits::Shred as _};
use crate::solana::Error;
use alloc::vec::Vec;
use bitflags::bitflags;
use core::fmt::Debug;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use solana_sdk::{
	clock::Slot,
	hash::Hash,
	pubkey::Pubkey,
	signature::{Keypair, Signature, Signer, SIGNATURE_BYTES},
};
use static_assertions::const_assert_eq;

mod common;
pub mod legacy;
pub mod merkle;
pub mod shred_code;
pub mod shred_data;
mod traits;

pub type Nonce = u32;
const_assert_eq!(SIZE_OF_NONCE, 4);
pub const SIZE_OF_NONCE: usize = std::mem::size_of::<Nonce>();

/// The following constants are computed by hand, and hardcoded.
/// `test_shred_constants` ensures that the values are correct.
/// Constants are used over lazy_static for performance reasons.
const SIZE_OF_COMMON_SHRED_HEADER: usize = 83;
const SIZE_OF_DATA_SHRED_HEADERS: usize = 88;
const SIZE_OF_CODING_SHRED_HEADERS: usize = 89;
const SIZE_OF_SIGNATURE: usize = SIGNATURE_BYTES;
const SIZE_OF_SHRED_VARIANT: usize = 1;
const SIZE_OF_SHRED_SLOT: usize = 8;

const OFFSET_OF_SHRED_VARIANT: usize = SIZE_OF_SIGNATURE;
const OFFSET_OF_SHRED_SLOT: usize = SIZE_OF_SIGNATURE + SIZE_OF_SHRED_VARIANT;
const OFFSET_OF_SHRED_INDEX: usize = OFFSET_OF_SHRED_SLOT + SIZE_OF_SHRED_SLOT;

// Shreds are uniformly split into erasure batches with a "target" number of
// data shreds per each batch as below. The actual number of data shreds in
// each erasure batch depends on the number of shreds obtained from serializing
// a &[Entry].
pub const DATA_SHREDS_PER_FEC_BLOCK: usize = 32;

// LAST_SHRED_IN_SLOT also implies DATA_COMPLETE_SHRED.
// So it cannot be LAST_SHRED_IN_SLOT if not also DATA_COMPLETE_SHRED.
bitflags! {
	#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
	pub struct ShredFlags:u8 {
		const SHRED_TICK_REFERENCE_MASK = 0b0011_1111;
		const DATA_COMPLETE_SHRED       = 0b0100_0000;
		const LAST_SHRED_IN_SLOT        = 0b1100_0000;
	}
}

#[repr(u8)]
#[derive(
	Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, IntoPrimitive, Serialize, TryFromPrimitive,
)]
#[serde(into = "u8", try_from = "u8")]
pub enum ShredType {
	Data = 0b1010_0101,
	Code = 0b0101_1010,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(into = "u8", try_from = "u8")]
pub enum ShredVariant {
	LegacyCode, // 0b0101_1010
	LegacyData, // 0b1010_0101
	// proof_size is the number of Merkle proof entries, and is encoded in the
	// lowest 4 bits of the binary representation. The first 4 bits identify
	// the shred variant:
	//   0b0100_????  MerkleCode
	//   0b0110_????  MerkleCode chained
	//   0b0111_????  MerkleCode chained resigned
	//   0b1000_????  MerkleData
	//   0b1001_????  MerkleData chained
	//   0b1011_????  MerkleData chained resigned
	MerkleCode { proof_size: u8, chained: bool, resigned: bool }, // 0b01??_????
	MerkleData { proof_size: u8, chained: bool, resigned: bool }, // 0b10??_????
}

/// A common header that is present in data and code shred headers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ShredCommonHeader {
	pub(crate) signature: Signature,
	pub(crate) shred_variant: ShredVariant,
	pub(crate) slot: Slot,
	pub(crate) index: u32,
	pub(crate) version: u16,
	pub(crate) fec_set_index: u32,
}

/// The data shred header has parent offset and flags
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DataShredHeader {
	pub(crate) parent_offset: u16,
	pub(crate) flags: ShredFlags,
	pub(crate) size: u16, // common shred header + data shred header + data
}

/// The coding shred header has FEC information
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CodingShredHeader {
	pub(crate) num_data_shreds: u16,
	pub(crate) num_coding_shreds: u16,
	pub(crate) position: u16, // [0..num_coding_shreds)
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Shred {
	ShredCode(ShredCode),
	ShredData(ShredData),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SignedData<'a> {
	Chunk(&'a [u8]), // Chunk of payload past signature.
	MerkleRoot(Hash),
}

impl<'a> AsRef<[u8]> for SignedData<'a> {
	fn as_ref(&self) -> &[u8] {
		match self {
			Self::Chunk(chunk) => chunk,
			Self::MerkleRoot(root) => root.as_ref(),
		}
	}
}

#[macro_export]
macro_rules! dispatch {
    ($vis:vis fn $name:ident(&self $(, $arg:ident : $ty:ty)?) $(-> $out:ty)?) => {
        #[inline]
        $vis fn $name(&self $(, $arg:$ty)?) $(-> $out)? {
            match self {
                Self::ShredCode(shred) => shred.$name($($arg, )?),
                Self::ShredData(shred) => shred.$name($($arg, )?),
            }
        }
    };
    ($vis:vis fn $name:ident(self $(, $arg:ident : $ty:ty)?) $(-> $out:ty)?) => {
        #[inline]
        $vis fn $name(self $(, $arg:$ty)?) $(-> $out)? {
            match self {
                Self::ShredCode(shred) => shred.$name($($arg, )?),
                Self::ShredData(shred) => shred.$name($($arg, )?),
            }
        }
    };
    ($vis:vis fn $name:ident(&mut self $(, $arg:ident : $ty:ty)?) $(-> $out:ty)?) => {
        #[inline]
        $vis fn $name(&mut self $(, $arg:$ty)?) $(-> $out)? {
            match self {
                Self::ShredCode(shred) => shred.$name($($arg, )?),
                Self::ShredData(shred) => shred.$name($($arg, )?),
            }
        }
    }
}

pub use dispatch;

impl Shred {
	dispatch!(fn common_header(&self) -> &ShredCommonHeader);
	dispatch!(fn set_signature(&mut self, signature: Signature));
	dispatch!(fn signed_data(&self) -> Result<SignedData, Error>);

	// Returns the portion of the shred's payload which is erasure coded.
	dispatch!(pub(crate) fn erasure_shard(self) -> Result<Vec<u8>, Error>);
	// Like Shred::erasure_shard but returning a slice.
	dispatch!(pub(crate) fn erasure_shard_as_slice(&self) -> Result<&[u8], Error>);
	// Returns the shard index within the erasure coding set.
	dispatch!(pub(crate) fn erasure_shard_index(&self) -> Result<usize, Error>);

	dispatch!(pub fn into_payload(self) -> Vec<u8>);
	dispatch!(pub fn merkle_root(&self) -> Result<Hash, Error>);
	dispatch!(pub fn payload(&self) -> &Vec<u8>);
	dispatch!(pub fn sanitize(&self) -> Result<(), Error>);

	// Only for tests.
	dispatch!(pub fn set_index(&mut self, index: u32));
	dispatch!(pub fn set_slot(&mut self, slot: Slot));

	// TODO: Should this sanitize output?
	pub fn new_from_data(
		slot: Slot,
		index: u32,
		parent_offset: u16,
		data: &[u8],
		flags: ShredFlags,
		reference_tick: u8,
		version: u16,
		fec_set_index: u32,
	) -> Self {
		Self::from(ShredData::new_from_data(
			slot,
			index,
			parent_offset,
			data,
			flags,
			reference_tick,
			version,
			fec_set_index,
		))
	}

	pub fn new_from_serialized_shred(shred: Vec<u8>) -> Result<Self, Error> {
		Ok(match layout::get_shred_variant(&shred)? {
			ShredVariant::LegacyCode => {
				let shred = legacy::ShredCode::from_payload(shred)?;
				Self::from(ShredCode::from(shred))
			},
			ShredVariant::LegacyData => {
				let shred = legacy::ShredData::from_payload(shred)?;
				Self::from(ShredData::from(shred))
			},
			ShredVariant::MerkleCode { .. } => {
				let shred = merkle::ShredCode::from_payload(shred)?;
				Self::from(ShredCode::from(shred))
			},
			ShredVariant::MerkleData { .. } => {
				let shred = merkle::ShredData::from_payload(shred)?;
				Self::from(ShredData::from(shred))
			},
		})
	}

	pub fn slot(&self) -> Slot {
		self.common_header().slot
	}

	pub fn parent(&self) -> Result<Slot, Error> {
		match self {
			Self::ShredCode(_) => Err(Error::InvalidShredType),
			Self::ShredData(shred) => shred.parent(),
		}
	}

	pub fn index(&self) -> u32 {
		self.common_header().index
	}

	pub(crate) fn data(&self) -> Result<&[u8], Error> {
		match self {
			Self::ShredCode(_) => Err(Error::InvalidShredType),
			Self::ShredData(shred) => shred.data(),
		}
	}

	// Possibly trimmed payload;
	// Should only be used when storing shreds to blockstore.
	pub(crate) fn bytes_to_store(&self) -> &[u8] {
		match self {
			Self::ShredCode(shred) => shred.payload(),
			Self::ShredData(shred) => shred.bytes_to_store(),
		}
	}

	pub fn fec_set_index(&self) -> u32 {
		self.common_header().fec_set_index
	}

	pub(crate) fn first_coding_index(&self) -> Option<u32> {
		match self {
			Self::ShredCode(shred) => shred.first_coding_index(),
			Self::ShredData(_) => None,
		}
	}

	pub fn version(&self) -> u16 {
		self.common_header().version
	}

	pub fn signature(&self) -> &Signature {
		&self.common_header().signature
	}

	pub fn sign(&mut self, keypair: &Keypair) {
		let data = self.signed_data().unwrap();
		let signature = keypair.sign_message(data.as_ref());
		self.set_signature(signature);
	}

	#[inline]
	pub fn shred_type(&self) -> ShredType {
		ShredType::from(self.common_header().shred_variant)
	}

	pub fn is_data(&self) -> bool {
		self.shred_type() == ShredType::Data
	}
	pub fn is_code(&self) -> bool {
		self.shred_type() == ShredType::Code
	}

	pub fn last_in_slot(&self) -> bool {
		match self {
			Self::ShredCode(_) => false,
			Self::ShredData(shred) => shred.last_in_slot(),
		}
	}

	/// This is not a safe function. It only changes the meta information.
	/// Use this only for test code which doesn't care about actual shred
	pub fn set_last_in_slot(&mut self) {
		match self {
			Self::ShredCode(_) => (),
			Self::ShredData(shred) => shred.set_last_in_slot(),
		}
	}

	pub fn data_complete(&self) -> bool {
		match self {
			Self::ShredCode(_) => false,
			Self::ShredData(shred) => shred.data_complete(),
		}
	}

	pub(crate) fn reference_tick(&self) -> u8 {
		match self {
			Self::ShredCode(_) => ShredFlags::SHRED_TICK_REFERENCE_MASK.bits(),
			Self::ShredData(shred) => shred.reference_tick(),
		}
	}

	#[must_use]
	pub fn verify(&self, pubkey: &Pubkey) -> bool {
		match self.signed_data() {
			Ok(data) => self.signature().verify(pubkey.as_ref(), data.as_ref()),
			Err(_) => false,
		}
	}

	#[must_use]
	pub fn verify_with_root(&self, leader: &Pubkey) -> Result<Hash, Error> {
		// Verifies the proof and returns the root
		if !self.verify(leader) {
			return Err(Error::ShredSignatureVerificationFailed);
		}
		self.merkle_root()
	}

	// Returns true if the erasure coding of the two shreds mismatch.
	pub(crate) fn erasure_mismatch(&self, other: &Self) -> Result<bool, Error> {
		match (self, other) {
			(Self::ShredCode(shred), Self::ShredCode(other)) => Ok(shred.erasure_mismatch(other)),
			_ => Err(Error::InvalidShredType),
		}
	}

	pub fn num_data_shreds(&self) -> Result<u16, Error> {
		match self {
			Self::ShredCode(shred) => Ok(shred.num_data_shreds()),
			Self::ShredData(_) => Err(Error::InvalidShredType),
		}
	}

	pub fn num_coding_shreds(&self) -> Result<u16, Error> {
		match self {
			Self::ShredCode(shred) => Ok(shred.num_coding_shreds()),
			Self::ShredData(_) => Err(Error::InvalidShredType),
		}
	}
}

// Helper methods to extract pieces of the shred from the payload
// without deserializing the entire payload.
pub mod layout {
	use super::*;
	use std::ops::Range;

	pub(crate) fn get_signature(shred: &[u8]) -> Option<Signature> {
		shred.get(..SIZE_OF_SIGNATURE).map(Signature::try_from)?.ok()
	}

	pub(crate) const fn get_signature_range() -> Range<usize> {
		0..SIZE_OF_SIGNATURE
	}

	pub(super) fn get_shred_variant(shred: &[u8]) -> Result<ShredVariant, Error> {
		let Some(&shred_variant) = shred.get(OFFSET_OF_SHRED_VARIANT) else {
			return Err(Error::InvalidPayloadSize(shred.len()));
		};
		ShredVariant::try_from(shred_variant).map_err(|_| Error::InvalidShredVariant)
	}

	#[inline]
	pub(super) fn get_shred_type(shred: &[u8]) -> Result<ShredType, Error> {
		let shred_variant = get_shred_variant(shred)?;
		Ok(ShredType::from(shred_variant))
	}

	#[inline]
	pub fn get_slot(shred: &[u8]) -> Option<Slot> {
		<[u8; 8]>::try_from(shred.get(OFFSET_OF_SHRED_SLOT..)?.get(..8)?)
			.map(Slot::from_le_bytes)
			.ok()
	}

	#[inline]
	pub(super) fn get_index(shred: &[u8]) -> Option<u32> {
		<[u8; 4]>::try_from(shred.get(OFFSET_OF_SHRED_INDEX..)?.get(..4)?)
			.map(u32::from_le_bytes)
			.ok()
	}

	pub fn get_version(shred: &[u8]) -> Option<u16> {
		<[u8; 2]>::try_from(shred.get(77..79)?).map(u16::from_le_bytes).ok()
	}

	// The caller should verify first that the shred is data and not code!
	pub(super) fn get_parent_offset(shred: &[u8]) -> Option<u16> {
		debug_assert_eq!(get_shred_type(shred).unwrap(), ShredType::Data);
		<[u8; 2]>::try_from(shred.get(83..85)?).map(u16::from_le_bytes).ok()
	}

	pub(crate) fn get_signed_data(shred: &[u8]) -> Option<SignedData> {
		let data = match get_shred_variant(shred).ok()? {
			ShredVariant::LegacyCode | ShredVariant::LegacyData => {
				let chunk = shred.get(self::legacy::SIGNED_MESSAGE_OFFSETS)?;
				SignedData::Chunk(chunk)
			},
			ShredVariant::MerkleCode { proof_size, chained, resigned } => {
				let merkle_root =
					self::merkle::ShredCode::get_merkle_root(shred, proof_size, chained, resigned)?;
				SignedData::MerkleRoot(merkle_root)
			},
			ShredVariant::MerkleData { proof_size, chained, resigned } => {
				let merkle_root =
					self::merkle::ShredData::get_merkle_root(shred, proof_size, chained, resigned)?;
				SignedData::MerkleRoot(merkle_root)
			},
		};
		Some(data)
	}

	// Returns offsets within the shred payload which is signed.
	pub(crate) fn get_signed_data_offsets(shred: &[u8]) -> Option<Range<usize>> {
		match get_shred_variant(shred).ok()? {
			ShredVariant::LegacyCode | ShredVariant::LegacyData => {
				let offsets = self::legacy::SIGNED_MESSAGE_OFFSETS;
				(offsets.end <= shred.len()).then_some(offsets)
			},
			// Merkle shreds sign merkle tree root which can be recovered from
			// the merkle proof embedded in the payload but itself is not
			// stored the payload.
			ShredVariant::MerkleCode { .. } => None,
			ShredVariant::MerkleData { .. } => None,
		}
	}

	pub fn get_reference_tick(shred: &[u8]) -> Result<u8, Error> {
		if get_shred_type(shred)? != ShredType::Data {
			return Err(Error::InvalidShredType);
		}
		let Some(flags) = shred.get(85) else {
			return Err(Error::InvalidPayloadSize(shred.len()));
		};
		Ok(flags & ShredFlags::SHRED_TICK_REFERENCE_MASK.bits())
	}

	pub fn get_merkle_root(shred: &[u8]) -> Option<Hash> {
		match get_shred_variant(shred).ok()? {
			ShredVariant::LegacyCode | ShredVariant::LegacyData => None,
			ShredVariant::MerkleCode { proof_size, chained, resigned } =>
				merkle::ShredCode::get_merkle_root(shred, proof_size, chained, resigned),
			ShredVariant::MerkleData { proof_size, chained, resigned } =>
				merkle::ShredData::get_merkle_root(shred, proof_size, chained, resigned),
		}
	}
}

impl From<ShredCode> for Shred {
	fn from(shred: ShredCode) -> Self {
		Self::ShredCode(shred)
	}
}

impl From<ShredData> for Shred {
	fn from(shred: ShredData) -> Self {
		Self::ShredData(shred)
	}
}

impl From<merkle::Shred> for Shred {
	fn from(shred: merkle::Shred) -> Self {
		match shred {
			merkle::Shred::ShredCode(shred) => Self::ShredCode(ShredCode::Merkle(shred)),
			merkle::Shred::ShredData(shred) => Self::ShredData(ShredData::Merkle(shred)),
		}
	}
}

impl TryFrom<Shred> for merkle::Shred {
	type Error = Error;

	fn try_from(shred: Shred) -> Result<Self, Self::Error> {
		match shred {
			Shred::ShredCode(ShredCode::Legacy(_)) => Err(Error::InvalidShredVariant),
			Shred::ShredCode(ShredCode::Merkle(shred)) => Ok(Self::ShredCode(shred)),
			Shred::ShredData(ShredData::Legacy(_)) => Err(Error::InvalidShredVariant),
			Shred::ShredData(ShredData::Merkle(shred)) => Ok(Self::ShredData(shred)),
		}
	}
}

impl From<ShredVariant> for ShredType {
	#[inline]
	fn from(shred_variant: ShredVariant) -> Self {
		match shred_variant {
			ShredVariant::LegacyCode => ShredType::Code,
			ShredVariant::LegacyData => ShredType::Data,
			ShredVariant::MerkleCode { .. } => ShredType::Code,
			ShredVariant::MerkleData { .. } => ShredType::Data,
		}
	}
}

impl From<ShredVariant> for u8 {
	fn from(shred_variant: ShredVariant) -> u8 {
		match shred_variant {
			ShredVariant::LegacyCode => u8::from(ShredType::Code),
			ShredVariant::LegacyData => u8::from(ShredType::Data),
			ShredVariant::MerkleCode { proof_size, chained: false, resigned: false } =>
				proof_size | 0x40,
			ShredVariant::MerkleCode { proof_size, chained: true, resigned: false } =>
				proof_size | 0x60,
			ShredVariant::MerkleCode { proof_size, chained: true, resigned: true } =>
				proof_size | 0x70,
			ShredVariant::MerkleData { proof_size, chained: false, resigned: false } =>
				proof_size | 0x80,
			ShredVariant::MerkleData { proof_size, chained: true, resigned: false } =>
				proof_size | 0x90,
			ShredVariant::MerkleData { proof_size, chained: true, resigned: true } =>
				proof_size | 0xb0,
			ShredVariant::MerkleCode { proof_size: _, chained: false, resigned: true } |
			ShredVariant::MerkleData { proof_size: _, chained: false, resigned: true } =>
				panic!("Invalid shred variant: {shred_variant:?}"),
		}
	}
}

impl TryFrom<u8> for ShredVariant {
	type Error = Error;

	fn try_from(shred_variant: u8) -> Result<Self, Self::Error> {
		if shred_variant == u8::from(ShredType::Code) {
			Ok(ShredVariant::LegacyCode)
		} else if shred_variant == u8::from(ShredType::Data) {
			Ok(ShredVariant::LegacyData)
		} else {
			let proof_size = shred_variant & 0x0F;
			match shred_variant & 0xF0 {
				0x40 =>
					Ok(ShredVariant::MerkleCode { proof_size, chained: false, resigned: false }),
				0x60 => Ok(ShredVariant::MerkleCode { proof_size, chained: true, resigned: false }),
				0x70 => Ok(ShredVariant::MerkleCode { proof_size, chained: true, resigned: true }),
				0x80 =>
					Ok(ShredVariant::MerkleData { proof_size, chained: false, resigned: false }),
				0x90 => Ok(ShredVariant::MerkleData { proof_size, chained: true, resigned: false }),
				0xb0 => Ok(ShredVariant::MerkleData { proof_size, chained: true, resigned: true }),
				_ => Err(Error::InvalidShredVariant),
			}
		}
	}
}

// TODO: use `should_discard_shred` in header verification?
/*
// Accepts shreds in the slot range [root + 1, max_slot].
#[must_use]
pub fn should_discard_shred(
	packet: &Packet,
	root: Slot,
	max_slot: Slot,
	shred_version: u16,
	should_drop_legacy_shreds: impl Fn(Slot) -> bool,
	enable_chained_merkle_shreds: impl Fn(Slot) -> bool,
	stats: &mut ShredFetchStats,
) -> bool {
	debug_assert!(root < max_slot);
	let shred = match layout::get_shred(packet) {
		None => {
			stats.index_overrun += 1;
			return true;
		},
		Some(shred) => shred,
	};
	match layout::get_version(shred) {
		None => {
			stats.index_overrun += 1;
			return true;
		},
		Some(version) =>
			if version != shred_version {
				stats.shred_version_mismatch += 1;
				return true;
			},
	}
	let Ok(shred_variant) = layout::get_shred_variant(shred) else {
		stats.bad_shred_type += 1;
		return true;
	};
	let slot = match layout::get_slot(shred) {
		Some(slot) => {
			if slot > max_slot {
				stats.slot_out_of_range += 1;
				return true;
			}
			slot
		},
		None => {
			stats.slot_bad_deserialize += 1;
			return true;
		},
	};
	let Some(index) = layout::get_index(shred) else {
		stats.index_bad_deserialize += 1;
		return true;
	};
	match ShredType::from(shred_variant) {
		ShredType::Code => {
			if index >= shred_code::MAX_CODE_SHREDS_PER_SLOT as u32 {
				stats.index_out_of_bounds += 1;
				return true;
			}
			if slot <= root {
				stats.slot_out_of_range += 1;
				return true;
			}
		},
		ShredType::Data => {
			if index >= MAX_DATA_SHREDS_PER_SLOT as u32 {
				stats.index_out_of_bounds += 1;
				return true;
			}
			let Some(parent_offset) = layout::get_parent_offset(shred) else {
				stats.bad_parent_offset += 1;
				return true;
			};
			let Some(parent) = slot.checked_sub(Slot::from(parent_offset)) else {
				stats.bad_parent_offset += 1;
				return true;
			};
			if !blockstore::verify_shred_slots(slot, parent, root) {
				stats.slot_out_of_range += 1;
				return true;
			}
		},
	}
	match shred_variant {
		ShredVariant::LegacyCode | ShredVariant::LegacyData =>
			if should_drop_legacy_shreds(slot) {
				return true;
			},
		ShredVariant::MerkleCode { chained: false, .. } => {
			stats.num_shreds_merkle_code = stats.num_shreds_merkle_code.saturating_add(1);
		},
		ShredVariant::MerkleCode { chained: true, .. } => {
			if !enable_chained_merkle_shreds(slot) {
				return true;
			}
			stats.num_shreds_merkle_code_chained =
				stats.num_shreds_merkle_code_chained.saturating_add(1);
		},
		ShredVariant::MerkleData { chained: false, .. } => {
			stats.num_shreds_merkle_data = stats.num_shreds_merkle_data.saturating_add(1);
		},
		ShredVariant::MerkleData { chained: true, .. } => {
			if !enable_chained_merkle_shreds(slot) {
				return true;
			}
			stats.num_shreds_merkle_data_chained =
				stats.num_shreds_merkle_data_chained.saturating_add(1);
		},
	}
	false
}
 */
