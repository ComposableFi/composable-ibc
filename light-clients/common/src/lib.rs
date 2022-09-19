#![cfg_attr(not(feature = "std"), no_std)]

//! Common utilities for light clients.

extern crate alloc;

use alloc::{string::ToString, vec, vec::Vec};
use anyhow::anyhow;
use codec::Compact;
use core::{
	fmt,
	fmt::{Debug, Display, Formatter},
	str::FromStr,
	time::Duration,
};
use ibc::core::{
	ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
	ics24_host::Path,
};
use primitive_types::H256;
use serde::{Deserialize, Serialize};

/// Host functions that allow the light client verify cryptographic proofs in native.
pub trait HostFunctions: Clone + Send + Sync + Eq + Debug + Default {
	/// This function should verify membership and non-membership in a trie proof using
	/// [`sp_state_machine::read_child_proof_check`]
	fn verify_child_trie_proof<I>(
		root: &[u8; 32],
		proof: &[Vec<u8>],
		items: I,
	) -> Result<(), anyhow::Error>
	where
		I: IntoIterator<Item = (Vec<u8>, Option<Vec<u8>>)>;
}

/// Membership proof verification via child trie host function
pub fn verify_membership<T, P>(
	prefix: &CommitmentPrefix,
	proof: &CommitmentProofBytes,
	root: &CommitmentRoot,
	path: P,
	value: Vec<u8>,
) -> Result<(), anyhow::Error>
where
	P: Into<Path>,
	T: HostFunctions,
{
	if root.as_bytes().len() != 32 {
		return Err(anyhow!("invalid commitment root length: {}", root.as_bytes().len()))
	}
	let path: Path = path.into();
	let path = path.to_string();
	let mut key = prefix.as_bytes().to_vec();
	key.extend(path.as_bytes());
	let trie_proof: Vec<Vec<u8>> =
		codec::Decode::decode(&mut &*proof.as_bytes()).map_err(anyhow::Error::msg)?;
	let root = H256::from_slice(root.as_bytes());
	T::verify_child_trie_proof(root.as_fixed_bytes(), &trie_proof, vec![(key, Some(value))])
}

/// Non-membership proof verification via child trie host function
pub fn verify_non_membership<T, P>(
	prefix: &CommitmentPrefix,
	proof: &CommitmentProofBytes,
	root: &CommitmentRoot,
	path: P,
) -> Result<(), anyhow::Error>
where
	P: Into<Path>,
	T: HostFunctions,
{
	if root.as_bytes().len() != 32 {
		return Err(anyhow!("invalid commitment root length: {}", root.as_bytes().len()))
	}
	let path: Path = path.into();
	let path = path.to_string();
	let mut key = prefix.as_bytes().to_vec();
	key.extend(path.as_bytes());
	let trie_proof: Vec<Vec<u8>> =
		codec::Decode::decode(&mut &*proof.as_bytes()).map_err(anyhow::Error::msg)?;
	let root = H256::from_slice(root.as_bytes());
	T::verify_child_trie_proof(root.as_fixed_bytes(), &trie_proof, vec![(key, None)])
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum RelayChain {
	Polkadot = 0,
	Kusama = 1,
	Rococo = 2,
}

impl Default for RelayChain {
	fn default() -> Self {
		RelayChain::Rococo
	}
}

impl Display for RelayChain {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

// Unbonding period for relay chains in days
const POLKADOT_UNBONDING_PERIOD: u64 = 28;
const KUSAMA_UNBONDING_PERIOD: u64 = 7;
// number of seconds in a day
const DAY: u64 = 24 * 60 * 60;

impl RelayChain {
	/// Yields the Order as a string
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Polkadot => "Polkadot",
			Self::Kusama => "Kusama",
			Self::Rococo => "Rococo",
		}
	}

	// Parses the Order out from a i32.
	pub fn from_i32(nr: i32) -> Result<Self, anyhow::Error> {
		match nr {
			0 => Ok(Self::Polkadot),
			1 => Ok(Self::Kusama),
			2 => Ok(Self::Rococo),
			id => Err(anyhow!("Unknown relay chain {id}")),
		}
	}

	pub fn unbonding_period(&self) -> Duration {
		match self {
			Self::Polkadot => Duration::from_secs(POLKADOT_UNBONDING_PERIOD * DAY),
			Self::Kusama | Self::Rococo => Duration::from_secs(KUSAMA_UNBONDING_PERIOD * DAY),
		}
	}

	pub fn trusting_period(&self) -> Duration {
		let unbonding_period = self.unbonding_period();
		// Trusting period is 1/3 of unbonding period
		unbonding_period.checked_div(3).unwrap()
	}
}

impl FromStr for RelayChain {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().trim_start_matches("order_") {
			"polkadot" => Ok(Self::Polkadot),
			"kusama" => Ok(Self::Kusama),
			"rococo" => Ok(Self::Rococo),
			_ => Err(anyhow!("Unknown relay chain {s}")),
		}
	}
}

/// Attempt to extract the timestamp extrinsic from the parachain header
pub fn decode_timestamp_extrinsic(ext: &Vec<u8>) -> Result<u64, anyhow::Error> {
	// Timestamp extrinsic should be the first inherent and hence the first extrinsic
	// https://github.com/paritytech/substrate/blob/d602397a0bbb24b5d627795b797259a44a5e29e9/primitives/trie/src/lib.rs#L99-L101
	// Decoding from the [2..] because the timestamp inmherent has two extra bytes before the call
	// that represents the call length and the extrinsic version.
	let (_, _, timestamp): (u8, u8, Compact<u64>) = codec::Decode::decode(&mut &ext[2..])
		.map_err(|err| anyhow!("Failed to decode extrinsic: {err}"))?;
	Ok(timestamp.into())
}
