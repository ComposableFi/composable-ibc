#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use crate::justification::{
	find_scheduled_change, AncestryChain, FinalityProof, GrandpaJustification,
};
use alloc::collections::BTreeMap;
use anyhow::anyhow;
use codec::{Decode, Encode};
use finality_grandpa::Chain;
use sp_core::storage::StorageKey;
use sp_finality_grandpa::AuthorityList;
use sp_runtime::traits::{Block, Header, NumberFor};
use sp_state_machine::StorageProof;
use sp_trie::LayoutV0;
use sp_std::prelude::*;

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

/// Verify a new grandpa justification, given the old state.
pub fn verify_parachain_headers_with_grandpa_finality_proof<B, H>(
	mut client_state: ClientState<B::Hash>,
	proof: ParachainHeadersWithFinalityProof<B>,
) -> Result<ClientState<B::Hash>, anyhow::Error>
where
	B: Block,  // relay chain block type
	H: Header, // parachain header type
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
	justification.verify(client_state.current_set_id, &client_state.current_authorities)?;

	// 4. verify state proofs of parachain headers in finalized relay chain headers.
	for hash in finalized {
		let relay_chain_header =
			headers.header(&hash).expect("Headers have been checked by AncestryChain; qed");
		if let Some(proofs) = parachain_headers.remove(&hash) {
			let ParachainHeaderProofs { extrinsic_proof, extrinsic, state_proof } = proofs;
			let proof = StorageProof::new(state_proof);
			let key = parachain_header_storage_key(client_state.para_id);
			let header = sp_state_machine::read_proof_check::<<B::Header as Header>::Hashing, _>(
				relay_chain_header.state_root().clone(),
				proof,
				&[key.as_ref()],
			)
			.map_err(|err| anyhow!("error verifying parachain header state proof: {err}"))?
			.remove(key.as_ref())
			.flatten()
			.ok_or_else(|| anyhow!("Invalid proof, parachain header not found"))?;
			let parachain_header = H::decode(&mut &header[..])?;
			// Timestamp extrinsic should be the first inherent and hence the first extrinsic
			let key = codec::Compact(0u32).encode();
			// verify timestamp extrinsic proof
			sp_trie::verify_trie_proof::<LayoutV0<H::Hashing>, _, _, _>(
				parachain_header.state_root(),
				&extrinsic_proof,
				&vec![(key, Some(&extrinsic[..]))],
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
	storage_key.extend_from_slice(sp_core::hashing::twox_64(&encoded_para_id).as_slice());
	storage_key.extend_from_slice(&encoded_para_id);
	StorageKey(storage_key)
}
