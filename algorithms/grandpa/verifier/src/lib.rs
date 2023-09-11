// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! GRANDPA light client verification function

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::all)]
#![deny(missing_docs)]

extern crate alloc;

use alloc::vec;
use anyhow::anyhow;
use codec::{Decode, Encode};
use finality_grandpa::Chain;
use hash_db::Hasher;
use light_client_common::state_machine;
use sp_runtime::traits::BlakeTwo256;
use primitives::{
	error,
	justification::{find_scheduled_change, AncestryChain, GrandpaJustification},
	parachain_header_storage_key, ClientState, HostFunctions, ParachainHeaderProofs,
	ParachainHeadersWithFinalityProof,
};
use sp_core::H256;
use sp_runtime::traits::Header;
use sp_trie::{LayoutV0, StorageProof, TrieDBBuilder, LayoutV1};

#[cfg(test)]
mod tests;

/// This function verifies the GRANDPA finality proof for relay chain headers.
///
/// Next, we prove the finality of parachain headers, by verifying patricia-merkle trie state proofs
/// of these headers, stored at the recently finalized relay chain heights.
pub fn verify_parachain_headers_with_grandpa_finality_proof<H, Host>(
	mut client_state: ClientState,
	proof: ParachainHeadersWithFinalityProof<H>,
) -> Result<ClientState, error::Error>
where
	H: Header<Hash = H256, Number = u32>,
	H::Number: finality_grandpa::BlockNumberOps + Into<u32>,
	Host: HostFunctions,
	Host::BlakeTwo256: Hasher<Out = H256>,
{
	let ParachainHeadersWithFinalityProof { finality_proof, parachain_headers, latest_para_height } =
		proof;

	// 1. First validate unknown headers.
	let headers = AncestryChain::<H>::new(&finality_proof.unknown_headers);

	let target = finality_proof
		.unknown_headers
		.iter()
		.max_by_key(|h| *h.number())
		.ok_or_else(|| anyhow!("Unknown headers can't be empty!"))?;

	// this is illegal
	if target.hash() != finality_proof.block {
		Err(anyhow!("Latest finalized block should be highest block in unknown_headers"))?;
	}

	let justification = GrandpaJustification::<H>::decode(&mut &finality_proof.justification[..])?;

	if justification.commit.target_hash != finality_proof.block {
		Err(anyhow!("Justification target hash and finality proof block hash mismatch"))?;
	}

	let from = client_state.latest_relay_hash;

	let base = finality_proof
		.unknown_headers
		.iter()
		.min_by_key(|h| *h.number())
		.ok_or_else(|| anyhow!("Unknown headers can't be empty!"))?;

	if base.number() < &client_state.latest_relay_height {
		headers.ancestry(base.hash(), client_state.latest_relay_hash).map_err(|_| {
			anyhow!(
				"[verify_parachain_headers_with_grandpa_finality_proof] Invalid ancestry (base -> latest relay block)!"
			)
		})?;
	}

	let mut finalized = headers.ancestry(from, target.hash()).map_err(|_| {
		anyhow!("[verify_parachain_headers_with_grandpa_finality_proof] Invalid ancestry!")
	})?;
	finalized.sort();

	// 2. verify justification.
	justification.verify::<Host>(client_state.current_set_id, &client_state.current_authorities)?;

	// 3. verify state proofs of parachain headers in finalized relay chain headers.
	let mut para_heights = vec![];
	for (hash, proofs) in parachain_headers {
		if finalized.binary_search(&hash).is_err() {
			// seems relay hash isn't in the finalized chain.
			continue
		}
		let relay_chain_header =
			headers.header(&hash).expect("Headers have been checked by AncestryChain; qed");

		let ParachainHeaderProofs { extrinsic_proof, extrinsic, state_proof } = proofs;
		let proof = StorageProof::new(state_proof);
		let key = parachain_header_storage_key(client_state.para_id);
		// verify patricia-merkle state proofs
		let header = state_machine::read_proof_check::<Host::BlakeTwo256, _>(
			relay_chain_header.state_root(),
			proof,
			&[key.as_ref()],
		)
		.map_err(|err| anyhow!("error verifying parachain header state proof: {err}"))?
		.remove(key.as_ref())
		.flatten()
		.ok_or_else(|| anyhow!("Invalid proof, parachain header not found"))?;
		let parachain_header = H::decode(&mut &header[..])?;
		para_heights.push(parachain_header.number().clone().into());
		// Timestamp extrinsic should be the first inherent and hence the first extrinsic
		// https://github.com/paritytech/substrate/blob/d602397a0bbb24b5d627795b797259a44a5e29e9/primitives/trie/src/lib.rs#L99-L101
		let key = codec::Compact(0u32).encode();
		// verify extrinsic proof for timestamp extrinsic
		// dbg!(extrinsic_proof);
		// dbg!(extrinsic_proof);
		
		use sp_trie::Trie;
		use crate::alloc::string::ToString;
		let mut s = vec!["test".to_string()];
		let trie_proof = extrinsic_proof.clone();
		let proof = StorageProof::new(trie_proof);
		use sp_trie::TrieLayout;
		use trie_db::NodeCodec;
		use trie_db::node::Node;
		for node_data in proof.iter_nodes(){
			let node = <LayoutV1<Host::BlakeTwo256> as TrieLayout>::Codec::decode(&mut &node_data[..]).unwrap();
			match node{
				Node::Empty => {
					log::info!("node: empty");
					log::info!(target: "hyperspace", "node: empty");
				}
				Node::Branch(v, Some(_)) => {
					log::info!("branch");
					log::info!(target: "hyperspace", "branch");
				}
				Node::NibbledBranch(v, _, Some(_)) => {
					log::info!("nibbled branch");
					log::info!(target: "hyperspace", "nibbled branch");
				}
				Node::Leaf(v, _) => {
					log::info!("leaf");
					log::info!(target: "hyperspace", "leaf");
				}
				_ => {
					log::info!("not empty");
					log::info!(target: "hyperspace", "not empty");
				}
			}
		}
		// let root = H256::from_slice(parachain_header.state_root().as_bytes());
		// let memory_db = proof.into_memory_db::<Host::BlakeTwo256>();
		// let trie = TrieDBBuilder::<sp_trie::LayoutV0<Host::BlakeTwo256>>::new(&memory_db, &root).build();
		// let t = trie.key_iter().unwrap();
		
		// // //iterate over t
		// for k in t{
		// 	let key = k.clone().unwrap_or_default();
		// 	let key = hex::encode(&key);
		// 	log::info!("key: {}", key);
		// 	log::info!(target: "hyperspace", "key: {}", key);
		// 	s.push(key);
		// }
		

		sp_trie::verify_trie_proof::<LayoutV0<Host::BlakeTwo256>, _, _, _>(
			parachain_header.state_root(),
			&extrinsic_proof,
			&vec![(key, Some(&extrinsic[..]))],
		)
		.map_err(|e| {
				match e {
					sp_trie::VerifyError::DuplicateKey(_) => {
						panic!("DuplicateKey extrinsic proof, duplicate key");
					}
					sp_trie::VerifyError::ExtraneousNode => {
						panic!("ExtraneousNode extrinsic proof, invalid proof");
					}
					sp_trie::VerifyError::ExtraneousValue(_) => {
						panic!("ExtraneousValue extrinsic proof, invalid value");
					}
					sp_trie::VerifyError::ExtraneousHashReference(_) => {
						panic!("ExtraneousHashReference extrinsic proof, incomplete proof");
					}
					sp_trie::VerifyError::InvalidChildReference(_) => {
						panic!("InvalidChildReference extrinsic proof, incomplete proof");
					}
					sp_trie::VerifyError::ValueMismatch(_) => {
						panic!("ValueMismatch extrinsic proof, incomplete proof");
					}
					sp_trie::VerifyError::IncompleteProof => {
						panic!("IncompleteProof extrinsic proof, incomplete proof");
					}
					sp_trie::VerifyError::RootMismatch(e) => {
						// panic!("RootMismatch extrinsic proof, invalid proof: {e}");
						panic!("RootMismatch extrinsic proof, invalid proof: {e}, {s:?}");
					}
					sp_trie::VerifyError::DecodeError(_) => {
						panic!("DecodeError extrinsic proof, invalid value");
					}
					_ => {}
				}
				anyhow!("Invalid extrinsic proof"
			)}
		)?;
	}

	// 4. set new client state, optionally rotating authorities
	client_state.latest_relay_hash = target.hash();
	client_state.latest_relay_height = (*target.number()).into();
	if let Some(max_height) = para_heights.into_iter().max() {
		if max_height != latest_para_height {
			Err(anyhow!("Latest parachain header height doesn't match the one in the proof"))?;
		}
		client_state.latest_para_height = max_height;
	}
	if let Some(scheduled_change) = find_scheduled_change::<H>(&target) {
		client_state.current_set_id += 1;
		client_state.current_authorities = scheduled_change.next_authorities;
	}

	Ok(client_state)
}
