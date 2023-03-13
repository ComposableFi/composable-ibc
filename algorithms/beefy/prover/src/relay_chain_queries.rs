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

use crate::error::Error;
use beefy_primitives::{SignedCommitment, VersionedFinalityProof};
use codec::{Decode, Encode};
use light_client_common::config::{ParaLifecycleT, RuntimeStorage};
use pallet_mmr_rpc::LeavesProof;
use sp_core::{hexdisplay::AsBytesRef, storage::StorageKey, H256};
use sp_runtime::traits::Zero;
use std::collections::{BTreeMap, BTreeSet};
use subxt::{config::Header, rpc::rpc_params, Config, OnlineClient};

/// This contains the leaf indices of the relay chain blocks and a map of relay chain heights to a
/// map of all parachain headers at those heights Used for generating [`ParaHeadsProof`]
pub struct FinalizedParaHeads {
	/// Block numbers
	pub block_numbers: Vec<u32>,
	/// Map of relay chain heights to map of para ids and parachain headers SCALE-encoded
	pub raw_finalized_heads: BTreeMap<u64, BTreeMap<u32, Vec<u8>>>,
}

/// Get the raw parachain heads finalized in the provided block
pub async fn fetch_finalized_parachain_heads<T: light_client_common::config::Config>(
	client: &OnlineClient<T>,
	commitment_block_number: u32,
	latest_beefy_height: u32,
	para_id: u32,
	header_numbers: &BTreeSet<T::BlockNumber>,
) -> Result<FinalizedParaHeads, Error>
where
	u32: From<<T as subxt::Config>::BlockNumber>,
	T::BlockNumber: Ord + Zero,
{
	let subxt_block_number: subxt::rpc::types::BlockNumber = commitment_block_number.into();
	let block_hash = client.rpc().block_hash(Some(subxt_block_number)).await?;

	let mut para_ids = vec![];
	let key = T::Storage::paras_parachains();
	let ids = client
		.storage()
		.at(block_hash)
		.await
		.expect("Storage client")
		.fetch(&key)
		.await?
		.ok_or_else(|| Error::Custom(format!("No ParaIds on relay chain?")))?;
	for id in ids {
		let id = id.into();
		let key = T::Storage::paras_para_lifecycles(id);
		let lifecycle = client
			.storage()
			.at(block_hash)
			.await
			.expect("Storage client")
			.fetch(&key)
			.await?
			.expect("ParaId is known");
		// only care about active parachains.
		if lifecycle.is_parachain() {
			para_ids.push(id);
		}
	}
	let previous_finalized_block_number: subxt::rpc::types::BlockNumber =
		(latest_beefy_height + 1).into();
	let previous_finalized_hash = client
		.rpc()
		.block_hash(Some(previous_finalized_block_number))
		.await?
		.ok_or_else(|| {
			Error::Custom(
				"Failed to get previous finalized beefy block hash from block number".to_string(),
			)
		})?;

	let change_set = client
		.rpc()
		.query_storage(
			// we are interested only in the blocks where our parachain header changes.
			vec![parachain_header_storage_key(para_id).as_bytes_ref()],
			previous_finalized_hash,
			block_hash,
		)
		.await?;
	let mut finalized_blocks = BTreeMap::new();
	let mut block_numbers = vec![];

	for changes in change_set {
		let header = client.rpc().header(Some(changes.block)).await?.ok_or_else(|| {
			Error::Custom(format!("[get_parachain_headers] block not found {:?}", changes.block))
		})?;

		let mut heads = BTreeMap::new();
		for id in para_ids.iter() {
			let id: u32 = (*id).into();
			let key = T::Storage::paras_heads(id);
			if let Some(head) = client
				.storage()
				.at(Some(header.hash()))
				.await
				.expect("Storage client")
				.fetch(&key)
				.await?
			{
				heads.insert(id, Into::<Vec<u8>>::into(head));
			}
		}

		let para_header: T::Header = Decode::decode(&mut &heads[&para_id][..])
			.map_err(|_| Error::Custom(format!("Failed to decode header for {para_id}")))?;
		let para_block_number = para_header.number();
		// skip genesis header or any unknown headers
		if para_block_number == Zero::zero() || !header_numbers.contains(&para_block_number) {
			continue
		}

		let block_number = u32::from(header.number());
		finalized_blocks.insert(block_number as u64, heads);
		block_numbers.push(block_number);
	}

	Ok(FinalizedParaHeads { raw_finalized_heads: finalized_blocks, block_numbers })
}

/// Get beefy justification for latest finalized beefy block
pub async fn fetch_beefy_justification<T: Config>(
	client: &OnlineClient<T>,
) -> Result<(SignedCommitment<u32, beefy_primitives::crypto::Signature>, T::Hash), Error> {
	let latest_beefy_finalized: <T as Config>::Hash =
		client.rpc().request("beefy_getFinalizedHead", rpc_params!()).await?;
	let block = client
		.rpc()
		.block(Some(latest_beefy_finalized))
		.await
		.ok()
		.flatten()
		.expect("Should find a valid block");

	let justifications = block.justifications.expect("Block should have valid justifications");

	let beefy_justification = justifications
		.into_iter()
		.find_map(|justfication| {
			(justfication.0 == beefy_primitives::BEEFY_ENGINE_ID).then(|| justfication.1)
		})
		.expect("Should have valid beefy justification");
	let VersionedFinalityProof::V1(signed_commitment) = VersionedFinalityProof::<
		u32,
		beefy_primitives::crypto::Signature,
	>::decode(&mut &*beefy_justification)
	.expect("Beefy justification should decode correctly");

	Ok((signed_commitment, latest_beefy_finalized))
}

/// Query a mmr  proof
pub async fn fetch_mmr_proof<T: Config>(
	client: &OnlineClient<T>,
	block_numbers: Vec<u32>,
	block_hash: Option<T::Hash>,
) -> Result<LeavesProof<H256>, Error> {
	let proof: LeavesProof<H256> = client
		.rpc()
		.request(
			"mmr_generateProof",
			rpc_params!(block_numbers, Option::<T::Hash>::None, block_hash),
		)
		.await?;
	Ok(proof)
}

/// This returns the storage key under which the parachain header with a given para_id is stored.
pub fn parachain_header_storage_key(para_id: u32) -> StorageKey {
	let mut storage_key = frame_support::storage::storage_prefix(b"Paras", b"Heads").to_vec();
	let encoded_para_id = para_id.encode();
	storage_key.extend_from_slice(sp_core::hashing::twox_64(&encoded_para_id).as_slice());
	storage_key.extend_from_slice(&encoded_para_id);
	StorageKey(storage_key)
}
