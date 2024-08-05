//! File source: solana/ledger/src/blockstore.rs

use crate::solana::{entry::Entry, shred::Shred, shredder::Shredder, Error};
use alloc::{format, vec::Vec};
use itertools::Itertools;
use solana_sdk::clock::Slot;
use std::collections::BTreeSet;

// An upper bound on maximum number of data shreds we can handle in a slot
// 32K shreds would allow ~320K peak TPS
// (32K shreds per slot * 4 TX per shred * 2.5 slots per sec)
pub const MAX_DATA_SHREDS_PER_SLOT: usize = 32_768;

pub type CompletedRanges = Vec<(u32, u32)>;

// Get the range of indexes [start_index, end_index] of every completed data block
pub fn get_completed_data_ranges(
	start_index: u32,
	completed_data_indexes: &BTreeSet<u32>,
	consumed: u32,
) -> CompletedRanges {
	// `consumed` is the next missing shred index, but shred `i` existing in
	// completed_data_end_indexes implies it's not missing
	assert!(!completed_data_indexes.contains(&consumed));
	completed_data_indexes
		.range(start_index..consumed)
		.scan(start_index, |begin, index| {
			let out = (*begin, *index);
			*begin = index + 1;
			Some(out)
		})
		.collect()
}

/// Fetch the entries corresponding to all of the shred indices in `completed_ranges`
/// This function takes advantage of the fact that `completed_ranges` are both
/// contiguous and in sorted order. To clarify, suppose completed_ranges is as follows:
///   completed_ranges = [..., (s_i, e_i), (s_i+1, e_i+1), ...]
/// Then, the following statements are true:
///   s_i < e_i < s_i+1 < e_i+1
///   e_i == s_i+1 + 1
pub fn get_slot_entries_in_block(
	_slot: Slot,
	completed_ranges: CompletedRanges,
	data_shreds: &[&Shred],
) -> Result<Vec<Entry>, Error> {
	assert!(!completed_ranges.is_empty());

	let (all_ranges_start_index, _) = *completed_ranges.first().unwrap();

	completed_ranges
		.into_iter()
		.map(|(start_index, end_index)| {
			// The indices from completed_ranges refer to shred indices in the
			// entire block; map those indices to indices within data_shreds
			let range_start_index = (start_index - all_ranges_start_index) as usize;
			let range_end_index = (end_index - all_ranges_start_index) as usize;
			let range_shreds = &data_shreds[range_start_index..=range_end_index];

			let last_shred = range_shreds.last().unwrap();

			assert!(last_shred.data_complete() || last_shred.last_in_slot());

			Shredder::deshred(range_shreds)
				.map_err(|e| {
					Error::Bincode(format!(
						"could not reconstruct entries buffer from shreds: {e:?}"
					))
				})
				.and_then(|payload| {
					bincode::deserialize::<Vec<Entry>>(&payload).map_err(|e| {
						Error::Bincode(format!("could not reconstruct entries: {e:?}"))
					})
				})
		})
		.flatten_ok()
		.collect()
}
