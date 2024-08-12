//! File source: solana/perf/src/packet.rs

use bincode::Options;
use serde::de::DeserializeOwned;
use solana_sdk::packet::PACKET_DATA_SIZE;
use std::io::Read;

pub fn deserialize_from_with_limit<R, T>(reader: R) -> bincode::Result<T>
where
	R: Read,
	T: DeserializeOwned,
{
	// with_limit causes pre-allocation size to be limited
	// to prevent against memory exhaustion attacks.
	bincode::options()
		.with_limit(PACKET_DATA_SIZE as u64)
		.with_fixint_encoding()
		.allow_trailing_bytes()
		.deserialize_from(reader)
}
