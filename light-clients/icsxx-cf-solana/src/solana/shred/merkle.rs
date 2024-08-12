//! File source: solana/ledger/src/shred/merkle.rs

use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::solana::shred::ShredType;
use crate::solana::{
	packet::deserialize_from_with_limit,
	shred::{
		self,
		common::impl_shred_common,
		dispatch, shred_code, shred_data,
		traits::{Shred as ShredTrait, ShredCode as ShredCodeTrait, ShredData as ShredDataTrait},
		CodingShredHeader, DataShredHeader, ShredCommonHeader, ShredVariant,
		SIZE_OF_CODING_SHRED_HEADERS, SIZE_OF_DATA_SHRED_HEADERS, SIZE_OF_SIGNATURE,
	},
	Error,
};
use alloc::vec::Vec;
use itertools::Either;
use solana_sdk::{
	clock::Slot,
	hash::{hashv, Hash},
	pubkey::Pubkey,
	signature::Signature,
};
use static_assertions::const_assert_eq;
use std::{io::Cursor, ops::Range};

const_assert_eq!(SIZE_OF_MERKLE_ROOT, 32);
pub(crate) const SIZE_OF_MERKLE_ROOT: usize = std::mem::size_of::<Hash>();
const_assert_eq!(SIZE_OF_MERKLE_PROOF_ENTRY, 20);
pub const SIZE_OF_MERKLE_PROOF_ENTRY: usize = std::mem::size_of::<MerkleProofEntry>();
const_assert_eq!(ShredData::SIZE_OF_PAYLOAD, 1203);

// Defense against second preimage attack:
// https://en.wikipedia.org/wiki/Merkle_tree#Second_preimage_attack
// Following Certificate Transparency, 0x00 and 0x01 bytes are prepended to
// hash data when computing leaf and internal node hashes respectively.
const MERKLE_HASH_PREFIX_LEAF: &[u8] = b"\x00SOLANA_MERKLE_SHREDS_LEAF";
const MERKLE_HASH_PREFIX_NODE: &[u8] = b"\x01SOLANA_MERKLE_SHREDS_NODE";

type MerkleProofEntry = [u8; 20];

// Layout: {common, data} headers | data buffer
//     | [Merkle root of the previous erasure batch if chained]
//     | Merkle proof
//     | [Retransmitter's signature if resigned]
// The slice past signature till the end of the data buffer is erasure coded.
// The slice past signature and before the merkle proof is hashed to generate
// the Merkle tree. The root of the Merkle tree is signed.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ShredData {
	pub(crate) common_header: ShredCommonHeader,
	pub(crate) data_header: DataShredHeader,
	pub(crate) payload: Vec<u8>,
}

// Layout: {common, coding} headers | erasure coded shard
//     | [Merkle root of the previous erasure batch if chained]
//     | Merkle proof
//     | [Retransmitter's signature if resigned]
// The slice past signature and before the merkle proof is hashed to generate
// the Merkle tree. The root of the Merkle tree is signed.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ShredCode {
	pub(crate) common_header: ShredCommonHeader,
	pub(crate) coding_header: CodingShredHeader,
	pub(crate) payload: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Shred {
	ShredCode(ShredCode),
	ShredData(ShredData),
}

impl Shred {
	dispatch!(fn common_header(&self) -> &ShredCommonHeader);
	dispatch!(fn erasure_shard_as_slice(&self) -> Result<&[u8], Error>);
	dispatch!(fn erasure_shard_index(&self) -> Result<usize, Error>);
	dispatch!(fn merkle_node(&self) -> Result<Hash, Error>);
	dispatch!(fn payload(&self) -> &Vec<u8>);
	dispatch!(fn sanitize(&self) -> Result<(), Error>);
	dispatch!(fn set_merkle_proof(&mut self, proof: &[&MerkleProofEntry]) -> Result<(), Error>);
	dispatch!(fn set_signature(&mut self, signature: Signature));
	dispatch!(fn signed_data(&self) -> Result<Hash, Error>);

	fn merkle_proof(&self) -> Result<impl Iterator<Item = &MerkleProofEntry>, Error> {
		match self {
			Self::ShredCode(shred) => shred.merkle_proof().map(Either::Left),
			Self::ShredData(shred) => shred.merkle_proof().map(Either::Right),
		}
	}

	#[must_use]
	fn verify(&self, pubkey: &Pubkey) -> bool {
		match self.signed_data() {
			Ok(data) => self.signature().verify(pubkey.as_ref(), data.as_ref()),
			Err(_) => false,
		}
	}

	fn signature(&self) -> &Signature {
		&self.common_header().signature
	}

	fn from_payload(shred: Vec<u8>) -> Result<Self, Error> {
		match shred::layout::get_shred_variant(&shred)? {
			ShredVariant::LegacyCode | ShredVariant::LegacyData => Err(Error::InvalidShredVariant),
			ShredVariant::MerkleCode { .. } => Ok(Self::ShredCode(ShredCode::from_payload(shred)?)),
			ShredVariant::MerkleData { .. } => Ok(Self::ShredData(ShredData::from_payload(shred)?)),
		}
	}
}

#[cfg(test)]
impl Shred {
	dispatch!(fn merkle_root(&self) -> Result<Hash, Error>);
	dispatch!(fn proof_size(&self) -> Result<u8, Error>);

	fn index(&self) -> u32 {
		self.common_header().index
	}

	fn shred_type(&self) -> ShredType {
		ShredType::from(self.common_header().shred_variant)
	}
}

impl ShredData {
	// proof_size is the number of merkle proof entries.
	pub(crate) fn proof_size(&self) -> Result<u8, Error> {
		match self.common_header.shred_variant {
			ShredVariant::MerkleData { proof_size, .. } => Ok(proof_size),
			_ => Err(Error::InvalidShredVariant),
		}
	}

	// Maximum size of ledger data that can be embedded in a data-shred.
	// Also equal to:
	//   ShredCode::capacity(proof_size, chained, resigned).unwrap()
	//       - ShredData::SIZE_OF_HEADERS
	//       + SIZE_OF_SIGNATURE
	pub(super) fn capacity(proof_size: u8, chained: bool, resigned: bool) -> Result<usize, Error> {
		debug_assert!(chained || !resigned);
		Self::SIZE_OF_PAYLOAD
			.checked_sub(
				Self::SIZE_OF_HEADERS +
					if chained { SIZE_OF_MERKLE_ROOT } else { 0 } +
					usize::from(proof_size) * SIZE_OF_MERKLE_PROOF_ENTRY +
					if resigned { SIZE_OF_SIGNATURE } else { 0 },
			)
			.ok_or(Error::InvalidProofSize(proof_size))
	}

	// Where the merkle proof starts in the shred binary.
	pub(crate) fn proof_offset(&self) -> Result<usize, Error> {
		let ShredVariant::MerkleData { proof_size, chained, resigned } =
			self.common_header.shred_variant
		else {
			return Err(Error::InvalidShredVariant);
		};
		Self::get_proof_offset(proof_size, chained, resigned)
	}

	fn get_proof_offset(proof_size: u8, chained: bool, resigned: bool) -> Result<usize, Error> {
		Ok(Self::SIZE_OF_HEADERS +
			Self::capacity(proof_size, chained, resigned)? +
			if chained { SIZE_OF_MERKLE_ROOT } else { 0 })
	}

	fn chained_merkle_root_offset(&self) -> Result<usize, Error> {
		let ShredVariant::MerkleData { proof_size, chained: true, resigned } =
			self.common_header.shred_variant
		else {
			return Err(Error::InvalidShredVariant);
		};
		Ok(Self::SIZE_OF_HEADERS + Self::capacity(proof_size, /* chained: */ true, resigned)?)
	}

	fn set_chained_merkle_root(&mut self, chained_merkle_root: &Hash) -> Result<(), Error> {
		let offset = self.chained_merkle_root_offset()?;
		let Some(buffer) = self.payload.get_mut(offset..offset + SIZE_OF_MERKLE_ROOT) else {
			return Err(Error::InvalidPayloadSize(self.payload.len()));
		};
		buffer.copy_from_slice(chained_merkle_root.as_ref());
		Ok(())
	}

	pub(super) fn merkle_root(&self) -> Result<Hash, Error> {
		let proof_size = self.proof_size()?;
		let index = self.erasure_shard_index()?;
		let proof_offset = self.proof_offset()?;
		let proof = get_merkle_proof(&self.payload, proof_offset, proof_size)?;
		let node = get_merkle_node(&self.payload, SIZE_OF_SIGNATURE..proof_offset)?;
		get_merkle_root(index, node, proof)
	}

	pub(crate) fn merkle_proof(&self) -> Result<impl Iterator<Item = &MerkleProofEntry>, Error> {
		let proof_size = self.proof_size()?;
		let proof_offset = self.proof_offset()?;
		get_merkle_proof(&self.payload, proof_offset, proof_size)
	}

	pub(crate) fn merkle_node(&self) -> Result<Hash, Error> {
		let proof_offset = self.proof_offset()?;
		get_merkle_node(&self.payload, SIZE_OF_SIGNATURE..proof_offset)
	}

	fn from_recovered_shard(
		signature: &Signature,
		chained_merkle_root: &Option<Hash>,
		mut shard: Vec<u8>,
	) -> Result<Self, Error> {
		let shard_size = shard.len();
		if shard_size + SIZE_OF_SIGNATURE > Self::SIZE_OF_PAYLOAD {
			return Err(Error::InvalidShardSize(shard_size));
		}
		shard.resize(Self::SIZE_OF_PAYLOAD, 0u8);
		shard.copy_within(0..shard_size, SIZE_OF_SIGNATURE);
		shard[0..SIZE_OF_SIGNATURE].copy_from_slice(signature.as_ref());
		// Deserialize headers.
		let mut cursor = Cursor::new(&shard[..]);
		let common_header: ShredCommonHeader = deserialize_from_with_limit(&mut cursor)?;
		let ShredVariant::MerkleData { proof_size, chained, resigned } =
			common_header.shred_variant
		else {
			return Err(Error::InvalidShredVariant);
		};
		if ShredCode::capacity(proof_size, chained, resigned)? != shard_size {
			return Err(Error::InvalidShardSize(shard_size));
		}
		let data_header = deserialize_from_with_limit(&mut cursor)?;
		let mut shred = Self { common_header, data_header, payload: shard };
		if let Some(chained_merkle_root) = chained_merkle_root {
			shred.set_chained_merkle_root(chained_merkle_root)?;
		}
		shred.sanitize()?;
		Ok(shred)
	}

	fn set_merkle_proof(&mut self, proof: &[&MerkleProofEntry]) -> Result<(), Error> {
		let proof_size = self.proof_size()?;
		if proof.len() != usize::from(proof_size) {
			return Err(Error::InvalidMerkleProof);
		}
		let proof_offset = self.proof_offset()?;
		let mut cursor = Cursor::new(
			self.payload
				.get_mut(proof_offset..)
				.ok_or(Error::InvalidProofSize(proof_size))?,
		);
		for entry in proof {
			bincode::serialize_into(&mut cursor, entry)?;
		}
		Ok(())
	}

	pub(super) fn get_merkle_root(
		shred: &[u8],
		proof_size: u8,
		chained: bool,
		resigned: bool,
	) -> Option<Hash> {
		debug_assert_eq!(
			shred::layout::get_shred_variant(shred).unwrap(),
			ShredVariant::MerkleData { proof_size, chained, resigned },
		);
		// Shred index in the erasure batch.
		let index = {
			let fec_set_index =
				<[u8; 4]>::try_from(shred.get(79..83)?).map(u32::from_le_bytes).ok()?;
			shred::layout::get_index(shred)?
				.checked_sub(fec_set_index)
				.map(usize::try_from)?
				.ok()?
		};
		let proof_offset = Self::get_proof_offset(proof_size, chained, resigned).ok()?;
		let proof = get_merkle_proof(shred, proof_offset, proof_size).ok()?;
		let node = get_merkle_node(shred, SIZE_OF_SIGNATURE..proof_offset).ok()?;
		get_merkle_root(index, node, proof).ok()
	}
}

impl ShredCode {
	// proof_size is the number of merkle proof entries.
	pub(crate) fn proof_size(&self) -> Result<u8, Error> {
		match self.common_header.shred_variant {
			ShredVariant::MerkleCode { proof_size, .. } => Ok(proof_size),
			_ => Err(Error::InvalidShredVariant),
		}
	}

	// Size of buffer embedding erasure codes.
	fn capacity(proof_size: u8, chained: bool, resigned: bool) -> Result<usize, Error> {
		debug_assert!(chained || !resigned);
		// Merkle proof is generated and signed after coding shreds are
		// generated. Coding shred headers cannot be erasure coded either.
		Self::SIZE_OF_PAYLOAD
			.checked_sub(
				Self::SIZE_OF_HEADERS +
					if chained { SIZE_OF_MERKLE_ROOT } else { 0 } +
					usize::from(proof_size) * SIZE_OF_MERKLE_PROOF_ENTRY +
					if resigned { SIZE_OF_SIGNATURE } else { 0 },
			)
			.ok_or(Error::InvalidProofSize(proof_size))
	}

	// Where the merkle proof starts in the shred binary.
	pub(crate) fn proof_offset(&self) -> Result<usize, Error> {
		let ShredVariant::MerkleCode { proof_size, chained, resigned } =
			self.common_header.shred_variant
		else {
			return Err(Error::InvalidShredVariant);
		};
		Self::get_proof_offset(proof_size, chained, resigned)
	}

	fn get_proof_offset(proof_size: u8, chained: bool, resigned: bool) -> Result<usize, Error> {
		Ok(Self::SIZE_OF_HEADERS +
			Self::capacity(proof_size, chained, resigned)? +
			if chained { SIZE_OF_MERKLE_ROOT } else { 0 })
	}

	fn chained_merkle_root_offset(&self) -> Result<usize, Error> {
		let ShredVariant::MerkleCode { proof_size, chained: true, resigned } =
			self.common_header.shred_variant
		else {
			return Err(Error::InvalidShredVariant);
		};
		Ok(Self::SIZE_OF_HEADERS + Self::capacity(proof_size, /* chained: */ true, resigned)?)
	}

	fn chained_merkle_root(&self) -> Result<Hash, Error> {
		let offset = self.chained_merkle_root_offset()?;
		self.payload
			.get(offset..offset + SIZE_OF_MERKLE_ROOT)
			.map(Hash::new)
			.ok_or(Error::InvalidPayloadSize(self.payload.len()))
	}

	fn set_chained_merkle_root(&mut self, chained_merkle_root: &Hash) -> Result<(), Error> {
		let offset = self.chained_merkle_root_offset()?;
		let Some(buffer) = self.payload.get_mut(offset..offset + SIZE_OF_MERKLE_ROOT) else {
			return Err(Error::InvalidPayloadSize(self.payload.len()));
		};
		buffer.copy_from_slice(chained_merkle_root.as_ref());
		Ok(())
	}

	pub(super) fn merkle_root(&self) -> Result<Hash, Error> {
		let proof_size = self.proof_size()?;
		let index = self.erasure_shard_index()?;
		let proof_offset = self.proof_offset()?;
		let proof = get_merkle_proof(&self.payload, proof_offset, proof_size)?;
		let node = get_merkle_node(&self.payload, SIZE_OF_SIGNATURE..proof_offset)?;
		get_merkle_root(index, node, proof)
	}

	pub(crate) fn merkle_proof(&self) -> Result<impl Iterator<Item = &MerkleProofEntry>, Error> {
		let proof_size = self.proof_size()?;
		let proof_offset = self.proof_offset()?;
		get_merkle_proof(&self.payload, proof_offset, proof_size)
	}

	pub(crate) fn merkle_node(&self) -> Result<Hash, Error> {
		let proof_offset = self.proof_offset()?;
		get_merkle_node(&self.payload, SIZE_OF_SIGNATURE..proof_offset)
	}

	fn from_recovered_shard(
		common_header: ShredCommonHeader,
		coding_header: CodingShredHeader,
		chained_merkle_root: &Option<Hash>,
		mut shard: Vec<u8>,
	) -> Result<Self, Error> {
		let ShredVariant::MerkleCode { proof_size, chained, resigned } =
			common_header.shred_variant
		else {
			return Err(Error::InvalidShredVariant);
		};
		let shard_size = shard.len();
		if Self::capacity(proof_size, chained, resigned)? != shard_size {
			return Err(Error::InvalidShardSize(shard_size));
		}
		if shard_size + Self::SIZE_OF_HEADERS > Self::SIZE_OF_PAYLOAD {
			return Err(Error::InvalidShardSize(shard_size));
		}
		shard.resize(Self::SIZE_OF_PAYLOAD, 0u8);
		shard.copy_within(0..shard_size, Self::SIZE_OF_HEADERS);
		let mut cursor = Cursor::new(&mut shard[..]);
		bincode::serialize_into(&mut cursor, &common_header)?;
		bincode::serialize_into(&mut cursor, &coding_header)?;
		let mut shred = Self { common_header, coding_header, payload: shard };
		if let Some(chained_merkle_root) = chained_merkle_root {
			shred.set_chained_merkle_root(chained_merkle_root)?;
		}
		shred.sanitize()?;
		Ok(shred)
	}

	fn set_merkle_proof(&mut self, proof: &[&MerkleProofEntry]) -> Result<(), Error> {
		let proof_size = self.proof_size()?;
		if proof.len() != usize::from(proof_size) {
			return Err(Error::InvalidMerkleProof);
		}
		let proof_offset = self.proof_offset()?;
		let mut cursor = Cursor::new(
			self.payload
				.get_mut(proof_offset..)
				.ok_or(Error::InvalidProofSize(proof_size))?,
		);
		for entry in proof {
			bincode::serialize_into(&mut cursor, entry)?;
		}
		Ok(())
	}

	pub(super) fn get_merkle_root(
		shred: &[u8],
		proof_size: u8,
		chained: bool,
		resigned: bool,
	) -> Option<Hash> {
		debug_assert_eq!(
			shred::layout::get_shred_variant(shred).unwrap(),
			ShredVariant::MerkleCode { proof_size, chained, resigned },
		);
		// Shred index in the erasure batch.
		let index = {
			let num_data_shreds = <[u8; 2]>::try_from(shred.get(83..85)?)
				.map(u16::from_le_bytes)
				.map(usize::from)
				.ok()?;
			let position = <[u8; 2]>::try_from(shred.get(87..89)?)
				.map(u16::from_le_bytes)
				.map(usize::from)
				.ok()?;
			num_data_shreds.checked_add(position)?
		};
		let proof_offset = Self::get_proof_offset(proof_size, chained, resigned).ok()?;
		let proof = get_merkle_proof(shred, proof_offset, proof_size).ok()?;
		let node = get_merkle_node(shred, SIZE_OF_SIGNATURE..proof_offset).ok()?;
		get_merkle_root(index, node, proof).ok()
	}
}

impl<'a> ShredTrait<'a> for ShredData {
	type SignedData = Hash;

	impl_shred_common!();

	// Also equal to:
	// ShredData::SIZE_OF_HEADERS
	//       + ShredData::capacity(proof_size, chained, resigned).unwrap()
	//       + if chained { SIZE_OF_MERKLE_ROOT } else { 0 }
	//       + usize::from(proof_size) * SIZE_OF_MERKLE_PROOF_ENTRY
	//       + if resigned { SIZE_OF_SIGNATURE } else { 0 }
	const SIZE_OF_PAYLOAD: usize =
		ShredCode::SIZE_OF_PAYLOAD - ShredCode::SIZE_OF_HEADERS + SIZE_OF_SIGNATURE;
	const SIZE_OF_HEADERS: usize = SIZE_OF_DATA_SHRED_HEADERS;

	fn from_payload(mut payload: Vec<u8>) -> Result<Self, Error> {
		// see: https://github.com/solana-labs/solana/pull/10109
		if payload.len() < Self::SIZE_OF_PAYLOAD {
			return Err(Error::InvalidPayloadSize(payload.len()));
		}
		payload.truncate(Self::SIZE_OF_PAYLOAD);
		let mut cursor = Cursor::new(&payload[..]);
		let common_header: ShredCommonHeader = deserialize_from_with_limit(&mut cursor)?;
		if !matches!(common_header.shred_variant, ShredVariant::MerkleData { .. }) {
			return Err(Error::InvalidShredVariant);
		}
		let data_header = deserialize_from_with_limit(&mut cursor)?;
		let shred = Self { common_header, data_header, payload };
		shred.sanitize()?;
		Ok(shred)
	}

	fn erasure_shard_index(&self) -> Result<usize, Error> {
		shred_data::erasure_shard_index(self).ok_or_else(|| Error::InvalidErasureShardIndex)
	}

	fn erasure_shard(self) -> Result<Vec<u8>, Error> {
		if self.payload.len() != Self::SIZE_OF_PAYLOAD {
			return Err(Error::InvalidPayloadSize(self.payload.len()));
		}
		let ShredVariant::MerkleData { proof_size, chained, resigned } =
			self.common_header.shred_variant
		else {
			return Err(Error::InvalidShredVariant);
		};
		let offset = Self::SIZE_OF_HEADERS + Self::capacity(proof_size, chained, resigned)?;
		let mut shard = self.payload;
		shard.truncate(offset);
		shard.drain(..SIZE_OF_SIGNATURE);
		Ok(shard)
	}

	fn erasure_shard_as_slice(&self) -> Result<&[u8], Error> {
		if self.payload.len() != Self::SIZE_OF_PAYLOAD {
			return Err(Error::InvalidPayloadSize(self.payload.len()));
		}
		let ShredVariant::MerkleData { proof_size, chained, resigned } =
			self.common_header.shred_variant
		else {
			return Err(Error::InvalidShredVariant);
		};
		let offset = Self::SIZE_OF_HEADERS + Self::capacity(proof_size, chained, resigned)?;
		self.payload
			.get(SIZE_OF_SIGNATURE..offset)
			.ok_or(Error::InvalidPayloadSize(self.payload.len()))
	}

	fn sanitize(&self) -> Result<(), Error> {
		let shred_variant = self.common_header.shred_variant;
		if !matches!(shred_variant, ShredVariant::MerkleData { .. }) {
			return Err(Error::InvalidShredVariant);
		}
		let _ = self.merkle_proof()?;
		shred_data::sanitize(self)
	}

	fn signed_data(&'a self) -> Result<Self::SignedData, Error> {
		self.merkle_root()
	}
}

impl<'a> ShredTrait<'a> for ShredCode {
	type SignedData = Hash;

	impl_shred_common!();
	const SIZE_OF_PAYLOAD: usize = shred_code::ShredCode::SIZE_OF_PAYLOAD;
	const SIZE_OF_HEADERS: usize = SIZE_OF_CODING_SHRED_HEADERS;

	fn from_payload(mut payload: Vec<u8>) -> Result<Self, Error> {
		let mut cursor = Cursor::new(&payload[..]);
		let common_header: ShredCommonHeader = deserialize_from_with_limit(&mut cursor)?;
		if !matches!(common_header.shred_variant, ShredVariant::MerkleCode { .. }) {
			return Err(Error::InvalidShredVariant);
		}
		let coding_header = deserialize_from_with_limit(&mut cursor)?;
		// see: https://github.com/solana-labs/solana/pull/10109
		if payload.len() < Self::SIZE_OF_PAYLOAD {
			return Err(Error::InvalidPayloadSize(payload.len()));
		}
		payload.truncate(Self::SIZE_OF_PAYLOAD);
		let shred = Self { common_header, coding_header, payload };
		shred.sanitize()?;
		Ok(shred)
	}

	fn erasure_shard_index(&self) -> Result<usize, Error> {
		shred_code::erasure_shard_index(self).ok_or_else(|| Error::InvalidErasureShardIndex)
	}

	fn erasure_shard(self) -> Result<Vec<u8>, Error> {
		if self.payload.len() != Self::SIZE_OF_PAYLOAD {
			return Err(Error::InvalidPayloadSize(self.payload.len()));
		}
		let ShredVariant::MerkleCode { proof_size, chained, resigned } =
			self.common_header.shred_variant
		else {
			return Err(Error::InvalidShredVariant);
		};
		let offset = Self::SIZE_OF_HEADERS + Self::capacity(proof_size, chained, resigned)?;
		let mut shard = self.payload;
		shard.truncate(offset);
		shard.drain(..Self::SIZE_OF_HEADERS);
		Ok(shard)
	}

	fn erasure_shard_as_slice(&self) -> Result<&[u8], Error> {
		if self.payload.len() != Self::SIZE_OF_PAYLOAD {
			return Err(Error::InvalidPayloadSize(self.payload.len()));
		}
		let ShredVariant::MerkleCode { proof_size, chained, resigned } =
			self.common_header.shred_variant
		else {
			return Err(Error::InvalidShredVariant);
		};
		let offset = Self::SIZE_OF_HEADERS + Self::capacity(proof_size, chained, resigned)?;
		self.payload
			.get(Self::SIZE_OF_HEADERS..offset)
			.ok_or(Error::InvalidPayloadSize(self.payload.len()))
	}

	fn sanitize(&self) -> Result<(), Error> {
		let shred_variant = self.common_header.shred_variant;
		if !matches!(shred_variant, ShredVariant::MerkleCode { .. }) {
			return Err(Error::InvalidShredVariant);
		}
		let _ = self.merkle_proof()?;
		shred_code::sanitize(self)
	}

	fn signed_data(&'a self) -> Result<Self::SignedData, Error> {
		self.merkle_root()
	}
}

impl ShredDataTrait for ShredData {
	#[inline]
	fn data_header(&self) -> &DataShredHeader {
		&self.data_header
	}

	fn data(&self) -> Result<&[u8], Error> {
		let ShredVariant::MerkleData { proof_size, chained, resigned } =
			self.common_header.shred_variant
		else {
			return Err(Error::InvalidShredVariant);
		};
		let data_buffer_size = Self::capacity(proof_size, chained, resigned)?;
		let size = usize::from(self.data_header.size);
		if size > self.payload.len() ||
			size < Self::SIZE_OF_HEADERS ||
			size > Self::SIZE_OF_HEADERS + data_buffer_size
		{
			return Err(Error::InvalidDataSize(self.data_header.size))
		}
		Ok(&self.payload[Self::SIZE_OF_HEADERS..size])
	}
}

impl ShredCodeTrait for ShredCode {
	#[inline]
	fn coding_header(&self) -> &CodingShredHeader {
		&self.coding_header
	}
}

// Obtains parent's hash by joining two sibiling nodes in merkle tree.
fn join_nodes<S: AsRef<[u8]>, T: AsRef<[u8]>>(node: S, other: T) -> Hash {
	let node = &node.as_ref()[..SIZE_OF_MERKLE_PROOF_ENTRY];
	let other = &other.as_ref()[..SIZE_OF_MERKLE_PROOF_ENTRY];
	hashv(&[MERKLE_HASH_PREFIX_NODE, node, other])
}

// Recovers root of the merkle tree from a leaf node
// at the given index and the respective proof.
fn get_merkle_root<'a, I>(index: usize, node: Hash, proof: I) -> Result<Hash, Error>
where
	I: IntoIterator<Item = &'a MerkleProofEntry>,
{
	let (index, root) = proof.into_iter().fold((index, node), |(index, node), other| {
		let parent = if index % 2 == 0 { join_nodes(node, other) } else { join_nodes(other, node) };
		(index >> 1, parent)
	});
	(index == 0).then_some(root).ok_or(Error::InvalidMerkleProof)
}

fn get_merkle_proof(
	shred: &[u8],
	proof_offset: usize, // Where the merkle proof starts.
	proof_size: u8,      // Number of proof entries.
) -> Result<impl Iterator<Item = &MerkleProofEntry>, Error> {
	let proof_size = usize::from(proof_size) * SIZE_OF_MERKLE_PROOF_ENTRY;
	Ok(shred
		.get(proof_offset..proof_offset + proof_size)
		.ok_or(Error::InvalidPayloadSize(shred.len()))?
		.chunks(SIZE_OF_MERKLE_PROOF_ENTRY)
		.map(<&MerkleProofEntry>::try_from)
		.map(Result::unwrap))
}

fn get_merkle_node(shred: &[u8], offsets: Range<usize>) -> Result<Hash, Error> {
	let node = shred.get(offsets).ok_or(Error::InvalidPayloadSize(shred.len()))?;
	Ok(hashv(&[MERKLE_HASH_PREFIX_LEAF, node]))
}

fn make_merkle_tree(mut nodes: Vec<Hash>) -> Vec<Hash> {
	let mut size = nodes.len();
	while size > 1 {
		let offset = nodes.len() - size;
		for index in (offset..offset + size).step_by(2) {
			let node = &nodes[index];
			let other = &nodes[(index + 1).min(offset + size - 1)];
			let parent = join_nodes(node, other);
			nodes.push(parent);
		}
		size = nodes.len() - offset - size;
	}
	nodes
}

fn make_merkle_proof(
	mut index: usize, // leaf index ~ shred's erasure shard index.
	mut size: usize,  // number of leaves ~ erasure batch size.
	tree: &[Hash],
) -> Option<Vec<&MerkleProofEntry>> {
	if index >= size {
		return None;
	}
	let mut offset = 0;
	let mut proof = Vec::<&MerkleProofEntry>::new();
	while size > 1 {
		let node = tree.get(offset + (index ^ 1).min(size - 1))?;
		let entry = &node.as_ref()[..SIZE_OF_MERKLE_PROOF_ENTRY];
		proof.push(<&MerkleProofEntry>::try_from(entry).unwrap());
		offset += size;
		size = (size + 1) >> 1;
		index >>= 1;
	}
	(offset + 1 == tree.len()).then_some(proof)
}

// Maps number of (code + data) shreds to merkle_proof.len().
fn get_proof_size(num_shreds: usize) -> u8 {
	let bits = usize::BITS - num_shreds.leading_zeros();
	let proof_size = if num_shreds.is_power_of_two() { bits.checked_sub(1).unwrap() } else { bits };
	u8::try_from(proof_size).unwrap()
}
