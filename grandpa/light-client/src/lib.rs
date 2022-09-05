#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use crate::justification::{
	find_scheduled_change, AncestryChain, FinalityProof, GrandpaJustification,
};
use alloc::collections::BTreeMap;
use anyhow::anyhow;
use codec::{Decode, Encode};
use finality_grandpa::Chain;
use sp_core::{ed25519, storage::StorageKey, H256};
use sp_finality_grandpa::AuthorityList;
use sp_runtime::traits::{Block, Header, NumberFor};
use sp_std::prelude::*;
use sp_trie::StorageProof;

pub mod error;
pub mod justification;

#[cfg(test)]
mod kusama;

/// Previous light client state.
pub struct ClientState<H> {
	// Current authority set
	pub current_authorities: AuthorityList,
	// Id of the current authority set.
	pub current_set_id: u64,
	// latest finalized hash on the relay chain.
	pub latest_relay_hash: H,
	// para_id of associated parachain
	pub para_id: u32,
}

/// Holds relavant parachain proofs for both header and timestamp extrinsic.
pub struct ParachainHeaderProofs {
	/// State proofs that prove a parachain header exists at a given relay chain height
	state_proof: Vec<Vec<u8>>,
	/// Timestamp extrinsic for ibc
	extrinsic: Vec<u8>,
	/// Timestamp extrinsic proof for previously proven parachain header.
	extrinsic_proof: Vec<Vec<u8>>,
}

/// Parachain headers with a Grandpa finality proof.
pub struct ParachainHeadersWithFinalityProof<B: Block> {
	/// The grandpa finality proof: contains relay chain headers from the
	/// last known finalized grandpa block.
	pub finality_proof: FinalityProof<B::Header>,
	/// Contains a map of relay chain header hashes to parachain headers
	/// finalzed at the relay chain height. We check for this parachain header finalization
	/// via state proofs. Also contains extrinsic proof for timestamp.
	pub parachain_headers: BTreeMap<B::Hash, ParachainHeaderProofs>,
}

/// Functions that this light client needs that should delegated to
/// a native implementation.
pub trait HostFunctions {
	/// Verify an ed25519 signature
	fn ed25519_verify(sig: &ed25519::Signature, msg: &[u8], pub_key: &ed25519::Public) -> bool;

	/// see [`sp_state_machine::read_proof_check`]
	fn read_proof_check<I>(
		root: &[u8; 32],
		proof: StorageProof,
		keys: I,
	) -> Result<BTreeMap<Vec<u8>, Option<Vec<u8>>>, error::Error>
	where
		I: IntoIterator,
		I::Item: AsRef<[u8]>;

	/// parity trie_db proof verification using BlakeTwo256 Hasher
	fn verify_timestamp_extrinsic(
		root: &[u8; 32],
		proof: &[Vec<u8>],
		value: &[u8],
	) -> Result<(), error::Error>;
}

type HashFor<B> = <B as Block>::Hash;

/// Verify a new grandpa justification, given the old state.
pub fn verify_parachain_headers_with_grandpa_finality_proof<B, H>(
	mut client_state: ClientState<HashFor<B>>,
	proof: ParachainHeadersWithFinalityProof<B>,
) -> Result<ClientState<HashFor<B>>, error::Error>
where
	B: Block<Hash = H256>,
	H: HostFunctions,
	NumberFor<B>: finality_grandpa::BlockNumberOps,
{
	let ParachainHeadersWithFinalityProof { finality_proof, mut parachain_headers } = proof;

	// 1. first check that target is in proof.unknown_headers.
	let headers = AncestryChain::<B>::new(&finality_proof.unknown_headers);
	let target = headers
		.header(&finality_proof.block)
		.ok_or_else(|| anyhow!("Target header not found!"))?;

	// 2. next check that there exists a route from client.latest_relay_hash to target.
	let finalized = headers.ancestry(client_state.latest_relay_hash, finality_proof.block)?;

	// 3. verify justification.
	let justification = GrandpaJustification::<B>::decode(&mut &finality_proof.justification[..])?;
	justification.verify::<H>(client_state.current_set_id, &client_state.current_authorities)?;

	// 4. verify state proofs of parachain headers in finalized relay chain headers.
	for hash in finalized {
		let relay_chain_header =
			headers.header(&hash).expect("Headers have been checked by AncestryChain; qed");
		if let Some(proofs) = parachain_headers.remove(&hash) {
			let ParachainHeaderProofs { extrinsic_proof, extrinsic, state_proof } = proofs;
			let proof = StorageProof::new(state_proof);
			let key = parachain_header_storage_key(client_state.para_id);
			let header = H::read_proof_check(
				relay_chain_header.state_root().as_fixed_bytes(),
				proof,
				&[key.as_ref()],
			)
			.map_err(|err| anyhow!("error verifying parachain header state proof: {err}"))?
			.remove(key.as_ref())
			.flatten()
			.ok_or_else(|| anyhow!("Invalid proof, parachain header not found"))?;
			let parachain_header = B::Header::decode(&mut &header[..])?;
			// verify timestamp extrinsic proof
			H::verify_timestamp_extrinsic(
				parachain_header.state_root().as_fixed_bytes(),
				&extrinsic_proof,
				&extrinsic[..],
			)?;
		}
	}

	// 5. set new client state, optionally rotating authorities
	client_state.latest_relay_hash = target.hash();
	if let Some(scheduled_change) = find_scheduled_change::<B>(&target) {
		client_state.current_set_id += 1;
		client_state.current_authorities = scheduled_change.next_authorities;
	}

	Ok(client_state)
}

/// This returns the storage key for a parachain header on the relay chain.
pub fn parachain_header_storage_key(para_id: u32) -> StorageKey {
	let mut storage_key = frame_support::storage::storage_prefix(b"Paras", b"Heads").to_vec();
	let encoded_para_id = para_id.encode();
	storage_key.extend_from_slice(sp_io::hashing::twox_64(&encoded_para_id).as_slice());
	storage_key.extend_from_slice(&encoded_para_id);
	StorageKey(storage_key)
}
