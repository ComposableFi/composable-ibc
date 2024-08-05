//! File source: solana/ledger/src/shred/legacy.rs

use serde::{Deserialize, Serialize};

use crate::solana::{
	packet::deserialize_from_with_limit,
	shred::{
		common::impl_shred_common,
		shred_code, shred_data,
		traits::{Shred, ShredCode as ShredCodeTrait, ShredData as ShredDataTrait},
		CodingShredHeader, DataShredHeader, ShredCommonHeader, ShredFlags, ShredVariant,
		SIZE_OF_CODING_SHRED_HEADERS, SIZE_OF_COMMON_SHRED_HEADER, SIZE_OF_DATA_SHRED_HEADERS,
		SIZE_OF_SIGNATURE,
	},
	Error,
};
use alloc::vec::Vec;
use solana_sdk::{clock::Slot, signature::Signature};
use static_assertions::const_assert_eq;
use std::{io::Cursor, ops::Range};

// All payload including any zero paddings are signed.
// Code and data shreds have the same payload size.
pub(super) const SIGNED_MESSAGE_OFFSETS: Range<usize> =
	SIZE_OF_SIGNATURE..ShredData::SIZE_OF_PAYLOAD;
const_assert_eq!(ShredData::SIZE_OF_PAYLOAD, ShredCode::SIZE_OF_PAYLOAD);
const_assert_eq!(ShredData::SIZE_OF_PAYLOAD, 1228);
const_assert_eq!(ShredData::CAPACITY, 1051);

// ShredCode::SIZE_OF_HEADERS bytes at the end of data shreds
// is never used and is not part of erasure coding.
const_assert_eq!(SIZE_OF_ERASURE_ENCODED_SLICE, 1139);
pub(super) const SIZE_OF_ERASURE_ENCODED_SLICE: usize =
	ShredCode::SIZE_OF_PAYLOAD - ShredCode::SIZE_OF_HEADERS;

// Layout: {common, data} headers | data | zero padding
// Everything up to ShredCode::SIZE_OF_HEADERS bytes at the end (which is part
// of zero padding) is erasure coded.
// All payload past signature, including the entirety of zero paddings, is
// signed.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ShredData {
	pub(crate) common_header: ShredCommonHeader,
	pub(crate) data_header: DataShredHeader,
	pub(crate) payload: Vec<u8>,
}

// Layout: {common, coding} headers | erasure coded shard
// All payload past signature is singed.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ShredCode {
	pub(crate) common_header: ShredCommonHeader,
	pub(crate) coding_header: CodingShredHeader,
	pub(crate) payload: Vec<u8>,
}

impl<'a> Shred<'a> for ShredData {
	type SignedData = &'a [u8];

	impl_shred_common!();
	// Legacy data shreds are always zero padded and
	// the same size as coding shreds.
	const SIZE_OF_PAYLOAD: usize = shred_code::ShredCode::SIZE_OF_PAYLOAD;
	const SIZE_OF_HEADERS: usize = SIZE_OF_DATA_SHRED_HEADERS;

	fn from_payload(mut payload: Vec<u8>) -> Result<Self, Error> {
		let mut cursor = Cursor::new(&payload[..]);
		let common_header: ShredCommonHeader = deserialize_from_with_limit(&mut cursor)?;
		if common_header.shred_variant != ShredVariant::LegacyData {
			return Err(Error::InvalidShredVariant);
		}
		let data_header = deserialize_from_with_limit(&mut cursor)?;
		// Shreds stored to blockstore may have trailing zeros trimmed.
		// Repair packets have nonce at the end of packet payload; see:
		// https://github.com/solana-labs/solana/pull/10109
		// https://github.com/solana-labs/solana/pull/16602
		if payload.len() < Self::SIZE_OF_HEADERS {
			return Err(Error::InvalidPayloadSize(payload.len()));
		}
		payload.resize(Self::SIZE_OF_PAYLOAD, 0u8);
		let shred = Self { common_header, data_header, payload };
		shred.sanitize().map(|_| shred)
	}

	fn erasure_shard_index(&self) -> Result<usize, Error> {
		shred_data::erasure_shard_index(self).ok_or_else(|| Error::InvalidErasureShardIndex)
	}

	fn erasure_shard(self) -> Result<Vec<u8>, Error> {
		if self.payload.len() != Self::SIZE_OF_PAYLOAD {
			return Err(Error::InvalidPayloadSize(self.payload.len()));
		}
		let mut shard = self.payload;
		shard.truncate(SIZE_OF_ERASURE_ENCODED_SLICE);
		Ok(shard)
	}

	fn erasure_shard_as_slice(&self) -> Result<&[u8], Error> {
		if self.payload.len() != Self::SIZE_OF_PAYLOAD {
			return Err(Error::InvalidPayloadSize(self.payload.len()));
		}
		Ok(&self.payload[..SIZE_OF_ERASURE_ENCODED_SLICE])
	}

	fn sanitize(&self) -> Result<(), Error> {
		match self.common_header.shred_variant {
			ShredVariant::LegacyData => (),
			_ => return Err(Error::InvalidShredVariant),
		}
		shred_data::sanitize(self)
	}

	fn signed_data(&'a self) -> Result<Self::SignedData, Error> {
		debug_assert_eq!(self.payload.len(), Self::SIZE_OF_PAYLOAD);
		Ok(&self.payload[SIZE_OF_SIGNATURE..])
	}
}

impl<'a> Shred<'a> for ShredCode {
	type SignedData = &'a [u8];

	impl_shred_common!();
	const SIZE_OF_PAYLOAD: usize = shred_code::ShredCode::SIZE_OF_PAYLOAD;
	const SIZE_OF_HEADERS: usize = SIZE_OF_CODING_SHRED_HEADERS;

	fn from_payload(mut payload: Vec<u8>) -> Result<Self, Error> {
		let mut cursor = Cursor::new(&payload[..]);
		let common_header: ShredCommonHeader = deserialize_from_with_limit(&mut cursor)?;
		if common_header.shred_variant != ShredVariant::LegacyCode {
			return Err(Error::InvalidShredVariant);
		}
		let coding_header = deserialize_from_with_limit(&mut cursor)?;
		// Repair packets have nonce at the end of packet payload:
		// https://github.com/solana-labs/solana/pull/10109
		payload.truncate(Self::SIZE_OF_PAYLOAD);
		let shred = Self { common_header, coding_header, payload };
		shred.sanitize().map(|_| shred)
	}

	fn erasure_shard_index(&self) -> Result<usize, Error> {
		shred_code::erasure_shard_index(self).ok_or_else(|| Error::InvalidErasureShardIndex)
	}

	fn erasure_shard(self) -> Result<Vec<u8>, Error> {
		if self.payload.len() != Self::SIZE_OF_PAYLOAD {
			return Err(Error::InvalidPayloadSize(self.payload.len()));
		}
		let mut shard = self.payload;
		// ShredCode::SIZE_OF_HEADERS bytes at the beginning of the coding
		// shreds contains the header and is not part of erasure coding.
		shard.drain(..Self::SIZE_OF_HEADERS);
		Ok(shard)
	}

	fn erasure_shard_as_slice(&self) -> Result<&[u8], Error> {
		if self.payload.len() != Self::SIZE_OF_PAYLOAD {
			return Err(Error::InvalidPayloadSize(self.payload.len()));
		}
		Ok(&self.payload[Self::SIZE_OF_HEADERS..])
	}

	fn sanitize(&self) -> Result<(), Error> {
		match self.common_header.shred_variant {
			ShredVariant::LegacyCode => (),
			_ => return Err(Error::InvalidShredVariant),
		}
		shred_code::sanitize(self)
	}

	fn signed_data(&'a self) -> Result<Self::SignedData, Error> {
		debug_assert_eq!(self.payload.len(), Self::SIZE_OF_PAYLOAD);
		Ok(&self.payload[SIZE_OF_SIGNATURE..])
	}
}

impl ShredDataTrait for ShredData {
	#[inline]
	fn data_header(&self) -> &DataShredHeader {
		&self.data_header
	}

	fn data(&self) -> Result<&[u8], Error> {
		let size = usize::from(self.data_header.size);
		#[allow(clippy::manual_range_contains)]
		if size > self.payload.len() ||
			size < Self::SIZE_OF_HEADERS ||
			size > Self::SIZE_OF_HEADERS + Self::CAPACITY
		{
			return Err(Error::InvalidDataSize(self.data_header.size));
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

impl ShredData {
	// Maximum size of ledger data that can be embedded in a data-shred.
	pub(super) const CAPACITY: usize =
		Self::SIZE_OF_PAYLOAD - Self::SIZE_OF_HEADERS - ShredCode::SIZE_OF_HEADERS;

	pub(super) fn new_from_data(
		slot: Slot,
		index: u32,
		parent_offset: u16,
		data: &[u8],
		flags: ShredFlags,
		reference_tick: u8,
		version: u16,
		fec_set_index: u32,
	) -> Self {
		let mut payload = alloc::vec![0; Self::SIZE_OF_PAYLOAD];
		let common_header = ShredCommonHeader {
			signature: Signature::default(),
			shred_variant: ShredVariant::LegacyData,
			slot,
			index,
			version,
			fec_set_index,
		};
		let size = (data.len() + Self::SIZE_OF_HEADERS) as u16;
		let flags = flags |
			ShredFlags::from_bits_retain(
				ShredFlags::SHRED_TICK_REFERENCE_MASK.bits().min(reference_tick),
			);
		let data_header = DataShredHeader { parent_offset, flags, size };
		let mut cursor = Cursor::new(&mut payload[..]);
		bincode::serialize_into(&mut cursor, &common_header).unwrap();
		bincode::serialize_into(&mut cursor, &data_header).unwrap();
		// TODO: Need to check if data is too large!
		let offset = cursor.position() as usize;
		debug_assert_eq!(offset, Self::SIZE_OF_HEADERS);
		payload[offset..offset + data.len()].copy_from_slice(data);
		Self { common_header, data_header, payload }
	}

	pub(super) fn bytes_to_store(&self) -> &[u8] {
		// Payload will be padded out to Self::SIZE_OF_PAYLOAD.
		// But only need to store the bytes within data_header.size.
		&self.payload[..self.data_header.size as usize]
	}

	pub(super) fn resize_stored_shred(mut shred: Vec<u8>) -> Result<Vec<u8>, Error> {
		// Old shreds might have been extra zero padded.
		if !(Self::SIZE_OF_HEADERS..=Self::SIZE_OF_PAYLOAD).contains(&shred.len()) {
			return Err(Error::InvalidPayloadSize(shred.len()));
		}
		shred.resize(Self::SIZE_OF_PAYLOAD, 0u8);
		Ok(shred)
	}

	// Only for tests.
	pub(crate) fn set_last_in_slot(&mut self) {
		self.data_header.flags |= ShredFlags::LAST_SHRED_IN_SLOT;
		let buffer = &mut self.payload[SIZE_OF_COMMON_SHRED_HEADER..];
		bincode::serialize_into(buffer, &self.data_header).unwrap();
	}
}
