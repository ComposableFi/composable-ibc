#![cfg_attr(not(feature = "std"), no_std)]

//! Common utilities for light clients.

extern crate alloc;

use alloc::{string::ToString, vec, vec::Vec};
use anyhow::anyhow;
use core::fmt::Debug;
use ibc::core::{
	ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
	ics24_host::Path,
};
use primitive_types::H256;

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
