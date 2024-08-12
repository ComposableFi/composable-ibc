//! File source: solana/ledger/src/shredder.rs

use crate::solana::{
	shred::{Shred, ShredData},
	Error,
};
use std::{prelude::rust_2015::Vec, vec};

#[derive(Debug)]
pub struct Shredder {}

impl Shredder {
	/// Combines all shreds to recreate the original buffer
	pub fn deshred(shreds: &[&Shred]) -> Result<Vec<u8>, Error> {
		let index = shreds.first().ok_or(Error::TooFewDataShards)?.index();
		let aligned = shreds.iter().zip(index..).all(|(s, i)| s.index() == i);
		let data_complete = {
			let shred = shreds.last().unwrap();
			shred.data_complete() || shred.last_in_slot()
		};
		if !data_complete || !aligned {
			return Err(Error::TooFewDataShards);
		}
		let data: Vec<_> = shreds.iter().map(|s| s.data()).collect::<Result<_, _>>()?;
		let data: Vec<_> = data.into_iter().flatten().copied().collect();
		if data.is_empty() {
			// For backward compatibility. This is needed when the data shred
			// payload is None, so that deserializing to Vec<Entry> results in
			// an empty vector.
			let data_buffer_size = ShredData::capacity(/* merkle_proof_size: */ None).unwrap();
			Ok(vec![0u8; data_buffer_size])
		} else {
			Ok(data)
		}
	}
}
