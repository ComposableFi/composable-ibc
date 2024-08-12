use crate::{
	error::Error,
	proto,
	solana::{
		blockstore::{get_completed_data_ranges, get_slot_entries_in_block},
		shred::{shred_code::ShredCode, shred_data::ShredData, *},
	},
};
use alloc::vec::Vec;
use ibc::Height;
use proto_utils::BadMessage;
use solana_sdk::{clock::Slot, hash::Hash, signature::Signature};
use std::{collections::BTreeSet, convert::From, ops::Deref};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Header {
	pub shreds: PreCheckedShreds,
}

impl Header {
	pub(crate) fn slot(&self) -> Slot {
		assert!(!self.shreds.is_empty(), "Header must contain at least one shred");
		self.shreds[0].slot()
	}

	pub(crate) fn height(&self) -> Height {
		Height::new(1, self.slot() as u64)
	}

	/// Calculate the block hash of the block represented by its shreds.
	///
	/// The block hash is the hash of the last entry formed from data shreds.
	/// If there are no data shreds, no last data shred in slot or it's not possible to form the
	/// last entry an error is returned.
	///
	/// TODO: since only the last entry is needed to calculate hash, consider filtering out all the
	/// other shreds
	pub fn calculate_hash(&self) -> Result<Hash, Error> {
		let data_shreds = self.shreds.iter().filter(|s| s.is_data()).collect::<Vec<_>>();
		if data_shreds.is_empty() {
			return Err(Error::NoDataShreds);
		}

		let last_data_shred = data_shreds.last().unwrap();
		if !last_data_shred.last_in_slot() {
			return Err(Error::LastShredNotLastInSlot);
		}

		let completed_data_indexes = data_shreds
			.iter()
			.filter_map(|s| if s.data_complete() { Some(s.index()) } else { None })
			.collect::<BTreeSet<_>>();
		let consumed = last_data_shred.index() + 1;
		let completed_ranges = get_completed_data_ranges(0, &completed_data_indexes, consumed);
		let entries = get_slot_entries_in_block(self.slot(), completed_ranges, &data_shreds)?;
		let blockhash = entries.last().map(|entry| entry.hash).ok_or(Error::EntriesAreEmpty)?;

		// FIXME: verify the hash

		Ok(blockhash)
	}
}

/// An immutable array of shreds that have the following properties:
/// 1. Is non-empty
/// 2. Is sorted
/// 3. Doesn't contain duplicates
/// 4. All shreds have the same slot
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PreCheckedShreds(Vec<Shred>);

impl PreCheckedShreds {
	/// Returns the (common) shreds' slot. Since the shreds are pre-checked, the array contains at
	/// least one shred and all the shreds have same slot.
	pub(crate) fn slot(&self) -> Slot {
		self.0[0].slot()
	}
}

impl Deref for PreCheckedShreds {
	type Target = [Shred];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl IntoIterator for PreCheckedShreds {
	type Item = Shred;
	type IntoIter = alloc::vec::IntoIter<Shred>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl TryFrom<Vec<Shred>> for PreCheckedShreds {
	type Error = Error;

	/// Creates a new `PreCheckedShreds` from a `Vec<Shred>`. The input shreds must satisfy the
	/// properties of the `PreCheckedShreds` type (see the `PreCheckedShreds`'s documentation).
	fn try_from(shreds: Vec<Shred>) -> Result<Self, Self::Error> {
		if shreds.is_empty() {
			return Err(Error::ShardsAreEmpty);
		}

		let mut shreds_set = BTreeSet::new();
		let mut prev_index = None;
		let slot = shreds[0].slot();

		for shred in &shreds {
			let index = shred.index();

			// Ensure the shreds are from the same slot.
			if shred.slot() != slot {
				return Err(Error::ShredsFromDifferentSlots);
			}

			// Ensure the shreds are sorted.
			if let Some(prev_index) = prev_index {
				if prev_index >= index {
					return Err(Error::ShredsNotSorted);
				}
			}

			// Ensure the shreds don't contain duplicates.
			if !shreds_set.insert(index) {
				return Err(Error::ShredsContainDuplicates);
			}

			prev_index = Some(index);
		}

		Ok(Self(shreds))
	}
}

impl From<Header> for proto::Header {
	fn from(value: Header) -> Self {
		Self::from(&value)
	}
}

impl TryFrom<proto::Header> for Header {
	type Error = BadMessage;

	fn try_from(msg: proto::Header) -> Result<Self, Self::Error> {
		Self::try_from(&msg)
	}
}

impl TryFrom<&proto::ShredCommonHeader> for ShredCommonHeader {
	type Error = BadMessage;

	fn try_from(value: &proto::ShredCommonHeader) -> Result<Self, Self::Error> {
		Ok(ShredCommonHeader {
			signature: Signature::try_from(value.signature.as_ref()).map_err(|_| BadMessage)?,
			shred_variant: value
				.shred_variant
				.as_ref()
				.ok_or(BadMessage)?
				.try_into()
				.map_err(|_| BadMessage)?,
			slot: value.slot,
			index: value.index,
			version: value.version.try_into().map_err(|_| BadMessage)?,
			fec_set_index: value.fec_set_index,
		})
	}
}

impl TryFrom<&proto::CodingShredHeader> for CodingShredHeader {
	type Error = BadMessage;

	fn try_from(value: &proto::CodingShredHeader) -> Result<Self, Self::Error> {
		Ok(CodingShredHeader {
			num_data_shreds: value.num_data_shreds.try_into().map_err(|_| BadMessage)?,
			num_coding_shreds: value.num_coding_shreds.try_into().map_err(|_| BadMessage)?,
			position: value.position.try_into().map_err(|_| BadMessage)?,
		})
	}
}

impl TryFrom<&proto::DataShredHeader> for DataShredHeader {
	type Error = BadMessage;

	fn try_from(value: &proto::DataShredHeader) -> Result<Self, Self::Error> {
		Ok(DataShredHeader {
			parent_offset: value.parent_offset.try_into().map_err(|_| BadMessage)?,
			flags: value.flags.as_ref().ok_or(BadMessage)?.try_into()?,
			size: value.size.try_into().map_err(|_| BadMessage)?,
		})
	}
}

// Implement TryFrom for ShredVariant if not already implemented
impl TryFrom<&proto::ShredVariant> for ShredVariant {
	type Error = BadMessage;

	fn try_from(value: &proto::ShredVariant) -> Result<Self, Self::Error> {
		match &value.variant {
			Some(proto::shred_variant::Variant::LegacyCode(_)) => Ok(ShredVariant::LegacyCode),
			Some(proto::shred_variant::Variant::LegacyData(_)) => Ok(ShredVariant::LegacyData),
			Some(proto::shred_variant::Variant::MerkleCode(proto_merkle_code)) =>
				Ok(ShredVariant::MerkleCode {
					proof_size: proto_merkle_code.proof_size as u8,
					chained: proto_merkle_code.chained,
					resigned: proto_merkle_code.resigned,
				}),
			Some(proto::shred_variant::Variant::MerkleData(proto_merkle_data)) =>
				Ok(ShredVariant::MerkleData {
					proof_size: proto_merkle_data.proof_size as u8,
					chained: proto_merkle_data.chained,
					resigned: proto_merkle_data.resigned,
				}),
			None => Err(BadMessage),
		}
	}
}
// Implement TryFrom for ShredFlags if not already implemented
impl TryFrom<&proto::ShredFlags> for ShredFlags {
	type Error = BadMessage;

	fn try_from(value: &proto::ShredFlags) -> Result<Self, Self::Error> {
		Ok(ShredFlags::from_bits(value.bits.try_into().map_err(|_| BadMessage)?)
			.ok_or(BadMessage)?)
	}
}

impl TryFrom<&proto::Header> for Header {
	type Error = BadMessage;

	fn try_from(msg: &proto::Header) -> Result<Self, Self::Error> {
		let shreds = msg
			.shreds
			.iter()
			.map(|shred| {
				Ok(match &shred.message {
					Some(proto::shred::Message::ShredCode(proto_shred_code)) => {
						match &proto_shred_code.message {
							Some(proto::shred_code::Message::LegacyShredCode(
								proto_legacy_shred_code,
							)) => {
								// Convert LegacyShredCode
								let common_header: ShredCommonHeader = proto_legacy_shred_code
									.common_header
									.as_ref()
									.ok_or(BadMessage)?
									.try_into()?;
								let coding_header: CodingShredHeader = proto_legacy_shred_code
									.coding_header
									.as_ref()
									.ok_or(BadMessage)?
									.try_into()?;
								let payload = proto_legacy_shred_code.payload.clone();
								Shred::ShredCode(ShredCode::Legacy(legacy::ShredCode {
									common_header,
									coding_header,
									payload,
								}))
							},
							Some(proto::shred_code::Message::MerkleShredCode(
								proto_merkle_shred_code,
							)) => {
								// Convert MerkleShredCode
								let common_header: ShredCommonHeader = proto_merkle_shred_code
									.common_header
									.as_ref()
									.ok_or(BadMessage)?
									.try_into()?;
								let coding_header: CodingShredHeader = proto_merkle_shred_code
									.coding_header
									.as_ref()
									.ok_or(BadMessage)?
									.try_into()?;
								let payload = proto_merkle_shred_code.payload.clone();
								Shred::ShredCode(ShredCode::Merkle(merkle::ShredCode {
									common_header,
									coding_header,
									payload,
								}))
							},
							None => return Err(BadMessage),
						}
					},
					Some(proto::shred::Message::ShredData(proto_shred_data)) => {
						match &proto_shred_data.message {
							Some(proto::shred_data::Message::LegacyShredData(
								proto_legacy_shred_data,
							)) => {
								// Convert LegacyShredData
								let common_header: ShredCommonHeader = proto_legacy_shred_data
									.common_header
									.as_ref()
									.ok_or(BadMessage)?
									.try_into()?;
								let data_header: DataShredHeader = proto_legacy_shred_data
									.data_header
									.as_ref()
									.ok_or(BadMessage)?
									.try_into()?;
								let payload = proto_legacy_shred_data.payload.clone();
								Shred::ShredData(ShredData::Legacy(legacy::ShredData {
									common_header,
									data_header,
									payload,
								}))
							},
							Some(proto::shred_data::Message::MerkleShredData(
								proto_merkle_shred_data,
							)) => {
								// Convert MerkleShredData
								let common_header: ShredCommonHeader = proto_merkle_shred_data
									.common_header
									.as_ref()
									.ok_or(BadMessage)?
									.try_into()?;
								let data_header: DataShredHeader = proto_merkle_shred_data
									.data_header
									.as_ref()
									.ok_or(BadMessage)?
									.try_into()?;
								let payload = proto_merkle_shred_data.payload.clone();
								Shred::ShredData(ShredData::Merkle(merkle::ShredData {
									common_header,
									data_header,
									payload,
								}))
							},
							None => return Err(BadMessage),
						}
					},
					None => return Err(BadMessage),
				})
			})
			.collect::<Result<Vec<_>, _>>()?;

		let shreds = PreCheckedShreds::try_from(shreds).map_err(|_| BadMessage)?;
		Ok(Header { shreds })
	}
}

impl From<&ShredCommonHeader> for proto::ShredCommonHeader {
	fn from(header: &ShredCommonHeader) -> Self {
		proto::ShredCommonHeader {
			signature: header.signature.as_ref().into(),
			shred_variant: Some((&header.shred_variant).into()),
			slot: header.slot,
			index: header.index,
			version: header.version as _,
			fec_set_index: header.fec_set_index,
		}
	}
}

impl From<&CodingShredHeader> for proto::CodingShredHeader {
	fn from(header: &CodingShredHeader) -> Self {
		proto::CodingShredHeader {
			num_data_shreds: header.num_data_shreds as _,
			num_coding_shreds: header.num_coding_shreds as _,
			position: header.position as _,
		}
	}
}

impl From<&DataShredHeader> for proto::DataShredHeader {
	fn from(header: &DataShredHeader) -> Self {
		proto::DataShredHeader {
			parent_offset: header.parent_offset as _,
			flags: Some((&header.flags).into()),
			size: header.size as _,
		}
	}
}

impl From<&ShredVariant> for proto::ShredVariant {
	fn from(variant: &ShredVariant) -> Self {
		match variant {
			ShredVariant::LegacyCode => proto::ShredVariant {
				variant: Some(proto::shred_variant::Variant::LegacyCode(proto::LegacyCode {})),
			},
			ShredVariant::LegacyData => proto::ShredVariant {
				variant: Some(proto::shred_variant::Variant::LegacyData(proto::LegacyData {})),
			},
			ShredVariant::MerkleCode { proof_size, chained, resigned } => proto::ShredVariant {
				variant: Some(proto::shred_variant::Variant::MerkleCode(proto::MerkleCode {
					proof_size: *proof_size as u32,
					chained: *chained,
					resigned: *resigned,
				})),
			},
			ShredVariant::MerkleData { proof_size, chained, resigned } => proto::ShredVariant {
				variant: Some(proto::shred_variant::Variant::MerkleData(proto::MerkleData {
					proof_size: *proof_size as u32,
					chained: *chained,
					resigned: *resigned,
				})),
			},
		}
	}
}

impl From<&ShredFlags> for proto::ShredFlags {
	fn from(flags: &ShredFlags) -> Self {
		proto::ShredFlags { bits: flags.bits() as _ }
	}
}

impl From<&Header> for proto::Header {
	fn from(header: &Header) -> Self {
		let proto_shreds = header
			.shreds
			.iter()
			.map(|shred| match shred {
				Shred::ShredCode(shred_code) => {
					let proto_shred_code = match shred_code {
						ShredCode::Legacy(legacy_shred_code) =>
							proto::shred_code::Message::LegacyShredCode(proto::LegacyShredCode {
								common_header: Some((&legacy_shred_code.common_header).into()),
								coding_header: Some((&legacy_shred_code.coding_header).into()),
								payload: legacy_shred_code.payload.clone(),
							}),
						ShredCode::Merkle(merkle_shred_code) =>
							proto::shred_code::Message::MerkleShredCode(proto::MerkleShredCode {
								common_header: Some((&merkle_shred_code.common_header).into()),
								coding_header: Some((&merkle_shred_code.coding_header).into()),
								payload: merkle_shred_code.payload.clone(),
							}),
					};

					proto::Shred {
						message: Some(proto::shred::Message::ShredCode(proto::ShredCode {
							message: Some(proto_shred_code),
						})),
					}
				},
				Shred::ShredData(shred_data) => {
					let proto_shred_data = match shred_data {
						ShredData::Legacy(legacy_shred_data) =>
							proto::shred_data::Message::LegacyShredData(proto::LegacyShredData {
								common_header: Some((&legacy_shred_data.common_header).into()),
								data_header: Some((&legacy_shred_data.data_header).into()),
								payload: legacy_shred_data.payload.clone(),
							}),
						ShredData::Merkle(merkle_shred_data) =>
							proto::shred_data::Message::MerkleShredData(proto::MerkleShredData {
								common_header: Some((&merkle_shred_data.common_header).into()),
								data_header: Some((&merkle_shred_data.data_header).into()),
								payload: merkle_shred_data.payload.clone(),
							}),
					};

					proto::Shred {
						message: Some(proto::shred::Message::ShredData(proto::ShredData {
							message: Some(proto_shred_data),
						})),
					}
				},
			})
			.collect();

		proto::Header { shreds: proto_shreds }
	}
}

proto_utils::define_wrapper! {
	proto: crate::proto::Header,
	wrapper: Header,
}

// super::impls!( Header);
// super::impls!(impl proto for Header);
