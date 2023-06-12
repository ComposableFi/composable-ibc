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
#![allow(clippy::all)]
#![deny(missing_docs)]

//! GRANDPA prover utilities

use anyhow::anyhow;
pub use beefy_prover;
use beefy_prover::helpers::{
	fetch_timestamp_extrinsic_with_proof, unsafe_arc_cast, TimeStampExtWithProof,
};
use codec::{Decode, Encode};
use finality_grandpa_rpc::GrandpaApiClient;
use jsonrpsee::{async_client::Client, tracing::log, ws_client::WsClientBuilder};
use light_client_common::config::{AsInner, RuntimeStorage};
use primitives::{
	parachain_header_storage_key, ClientState, FinalityProof, ParachainHeaderProofs,
	ParachainHeadersWithFinalityProof,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_finality_grandpa::{AuthorityId, AuthoritySignature};
use sp_runtime::traits::{One, Zero};
use std::{
	collections::{BTreeMap, BTreeSet},
	sync::{
		atomic::{AtomicU32, Ordering},
		Arc,
	},
	time::Duration,
};
use subxt::{config::Header, rpc::types::StorageChangeSet, Config, OnlineClient};
use tokio::{task::JoinSet, time::sleep};

/// The maximum number of authority set changes to request at once
pub const PROCESS_CHANGES_SET_BATCH_SIZE: usize = 100;
/// The maximum number of blocks to request at once
pub const PROCESS_BLOCKS_BATCH_SIZE: usize = 100;

/// Host function implementation for the verifier
pub mod host_functions;

/// Contains methods useful for proving parachain header finality using GRANDPA
pub struct GrandpaProver<T: Config> {
	/// Subxt client for the relay chain
	pub relay_client: OnlineClient<T>,
	/// Relay chain jsonrpsee client for typed rpc requests, which subxt lacks support for.
	pub relay_ws_client: Arc<Client>,
	/// Subxt client for the parachain
	pub para_client: OnlineClient<T>,
	/// Parachain jsonrpsee client for typed rpc requests, which subxt lacks support for.
	pub para_ws_client: Arc<Client>,
	/// ParaId of the associated parachain
	pub para_id: u32,
	/// Delay between rpc calls to the RPC
	pub rpc_call_delay: Duration,
}

// We redefine these here because we want the header to be bounded by subxt::config::Header in the
// prover
/// Commit
pub type Commit = finality_grandpa::Commit<H256, u32, AuthoritySignature, AuthorityId>;

/// Justification
#[cfg_attr(any(feature = "std", test), derive(Debug))]
#[derive(Clone, Encode, Decode)]
pub struct GrandpaJustification<H: Header + codec::Decode> {
	/// Current voting round number, monotonically increasing
	pub round: u64,
	/// Contains block hash & number that's being finalized and the signatures.
	pub commit: Commit,
	/// Contains the path from a [`PreCommit`]'s target hash to the GHOST finalized block.
	pub votes_ancestries: Vec<H>,
}

/// An encoded justification proving that the given header has been finalized
#[derive(Clone, Serialize, Deserialize)]
pub struct JustificationNotification(pub sp_core::Bytes);

impl<T: Config> Clone for GrandpaProver<T> {
	fn clone(&self) -> Self {
		Self {
			relay_client: self.relay_client.clone(),
			relay_ws_client: self.relay_ws_client.clone(),
			para_client: self.para_client.clone(),
			para_ws_client: self.para_ws_client.clone(),
			para_id: self.para_id,
			rpc_call_delay: self.rpc_call_delay,
		}
	}
}

impl<T> GrandpaProver<T>
where
	T: light_client_common::config::Config + Send + Sync,
	<<T as subxt::Config>::Header as Header>::Number: Ord + Zero,
	u32: From<<<T as subxt::Config>::Header as Header>::Number>,
	sp_core::H256: From<T::Hash>,
{
	/// Initializes the parachain and relay chain clients given the ws urls.
	pub async fn new(
		relay_ws_url: &str,
		para_ws_url: &str,
		para_id: u32,
		rpc_call_delay: Duration,
	) -> Result<Self, anyhow::Error> {
		let relay_ws_client = Arc::new(WsClientBuilder::default().build(relay_ws_url).await?);
		let relay_client = OnlineClient::<T>::from_rpc_client(relay_ws_client.clone()).await?;
		let para_ws_client = Arc::new(WsClientBuilder::default().build(para_ws_url).await?);
		let para_client = OnlineClient::<T>::from_rpc_client(para_ws_client.clone()).await?;

		Ok(Self {
			relay_ws_client,
			relay_client,
			para_ws_client,
			para_client,
			para_id,
			rpc_call_delay,
		})
	}

	/// Construct the inital client state.
	pub async fn initialize_client_state(&self) -> Result<ClientState, anyhow::Error>
	where
		<T as subxt::Config>::Header: Decode,
	{
		use sp_finality_grandpa::AuthorityList;
		let latest_relay_hash = self.relay_client.rpc().finalized_head().await.unwrap();
		log::debug!(target: "hyperspace", "Latest relay hash: {:?}", latest_relay_hash);
		let header = self
			.relay_client
			.rpc()
			.header(Some(latest_relay_hash))
			.await
			.unwrap()
			.ok_or_else(|| anyhow!("Header not found for hash: {latest_relay_hash:?}"))
			.unwrap();

		let current_set_id = {
			let key = T::Storage::grandpa_current_set_id();
			self.relay_client
				.storage()
				.at(latest_relay_hash)
				.fetch(&key)
				.await
				.unwrap()
				.expect("Failed to fetch current set id")
		};

		let current_authorities = {
			let bytes = self
				.relay_client
				.rpc()
				.request::<String>(
					"state_call",
					subxt::rpc_params!(
						"GrandpaApi_grandpa_authorities",
						"0x",
						Some(format!("{:?}", latest_relay_hash))
					),
				)
				.await
				.map(|res| hex::decode(&res[2..]))
				.unwrap()
				.unwrap();

			AuthorityList::decode(&mut &bytes[..]).expect("Failed to scale decode authorities")
		};

		// Ensure there are no duplicates in authority list
		let mut set = BTreeSet::new();
		for (id, ..) in &current_authorities {
			if !set.insert(id) {
				Err(anyhow!("Duplicate entries found in current authority set"))?
			}
		}

		let latest_relay_height = u32::from(header.number());
		let finalized_para_header =
			self.query_latest_finalized_parachain_header(latest_relay_height).await.unwrap();

		Ok(ClientState {
			current_authorities,
			current_set_id,
			latest_relay_height,
			latest_relay_hash: latest_relay_hash.into(),
			para_id: self.para_id,
			// we'll set this below
			latest_para_height: u32::from(finalized_para_header.number()),
		})
	}

	/// Returns the latest finalized parachain header at the given finalized relay chain height.
	pub async fn query_latest_finalized_parachain_header(
		&self,
		latest_finalized_height: u32,
	) -> Result<T::Header, anyhow::Error>
	where
		<T as subxt::Config>::Header: Decode,
	{
		let latest_finalized_hash = self
			.relay_client
			.rpc()
			.block_hash(Some(latest_finalized_height.into()))
			.await?
			.ok_or_else(|| anyhow!("Block hash not found for number: {latest_finalized_height}"))?;
		let key = T::Storage::paras_heads(self.para_id);
		let header = <T::Storage as RuntimeStorage>::HeadData::from_inner(
			self.relay_client
				.storage()
				.at(latest_finalized_hash)
				.fetch(&key)
				.await?
				.ok_or_else(|| {
					anyhow!("parachain header not found for para id: {}", self.para_id)
				})?,
		);
		let header = T::Header::decode(&mut header.as_ref())
			.map_err(|_| anyhow!("Failed to decode header"))?;

		Ok(header)
	}

	/// Returns a tuple of the finality proof for the given parachain `header_numbers` finalized by
	/// `latest_finalized_height`.
	pub async fn query_finalized_parachain_headers_with_proof<H>(
		&self,
		previous_finalized_height: u32,
		mut latest_finalized_height: u32,
		latest_justification: Option<Vec<u8>>,
		header_numbers: Vec<<<T as subxt::Config>::Header as Header>::Number>,
	) -> Result<ParachainHeadersWithFinalityProof<H>, anyhow::Error>
	where
		H: Header + codec::Decode + Send + 'static,
		u32: From<<H as Header>::Number>,
		<H::Hasher as subxt::config::Hasher>::Output: From<T::Hash>,
		T::Hash: From<<H::Hasher as subxt::config::Hasher>::Output>,
		H::Number: finality_grandpa::BlockNumberOps,
		<<T as subxt::Config>::Header as Header>::Number: One + Clone + Sync + Send,
		<T as subxt::Config>::Header: Decode + Sync,
	{
		let mut finality_proof = if let Some(justification) = latest_justification {
			let justification = GrandpaJustification::<H>::decode(&mut &*justification)?;

			FinalityProof::<H> {
				block: justification.commit.target_hash,
				justification: justification.encode(),
				unknown_headers: vec![],
			}
		} else {
			let encoded = GrandpaApiClient::<JustificationNotification, H256, u32>::prove_finality(
				// we cast between the same type but different crate versions.
				&*unsafe {
					unsafe_arc_cast::<_, jsonrpsee_ws_client::WsClient>(
						self.relay_ws_client.clone(),
					)
				},
				latest_finalized_height,
			)
			.await?
			.ok_or_else(|| {
				anyhow!("No justification found for block: {:?}", latest_finalized_height)
			})?
			.0;

			let mut finality_proof = FinalityProof::<H>::decode(&mut &encoded[..])?;

			let justification =
				GrandpaJustification::<H>::decode(&mut &finality_proof.justification[..])?;

			finality_proof.block = justification.commit.target_hash;

			latest_finalized_height = u32::from(justification.commit.target_number);
			finality_proof
		};

		let start = self
			.relay_client
			.rpc()
			.block_hash(Some(previous_finalized_height.into()))
			.await?
			.ok_or_else(|| anyhow!("Failed to fetch previous finalized hash + 1"))?;

		let latest_finalized_hash = self
			.relay_client
			.rpc()
			.block_hash(Some(latest_finalized_height.into()))
			.await?
			.ok_or_else(|| anyhow!("Failed to fetch previous finalized hash + 1"))?;

		let mut unknown_headers = vec![];
		let mut unknown_headers_join_set: JoinSet<Result<_, anyhow::Error>> = JoinSet::new();
		let heights = (previous_finalized_height..=latest_finalized_height).collect::<Vec<_>>();
		for heights in heights.chunks(PROCESS_BLOCKS_BATCH_SIZE) {
			for height in heights.to_owned() {
				log::trace!(target: "hyperspace", "Processing height: {height}");

				let prover = self.clone();
				let to = self.rpc_call_delay.as_millis();
				let duration = Duration::from_millis(rand::thread_rng().gen_range(1..to) as u64);
				unknown_headers_join_set.spawn(async move {
					sleep(duration).await;
					let hash = prover
						.relay_client
						.rpc()
						.block_hash(Some(height.into()))
						.await?
						.ok_or_else(|| anyhow!("Failed to fetch block has for height {height}"))?;

					let header = prover
						.relay_client
						.rpc()
						.header(Some(hash))
						.await?
						.ok_or_else(|| anyhow!("Header with hash: {hash:?} not found!"))?;

					H::decode(&mut &header.encode()[..]).map_err(|e| e.into())
				});
			}

			// TODO: change this to a binary tree with sorting over the header height
			while let Some(header) = unknown_headers_join_set.join_next().await {
				unknown_headers.push(header??);
			}
		}

		// we are interested only in the blocks where our parachain header changes.
		let para_storage_key = parachain_header_storage_key(self.para_id);
		let keys = vec![para_storage_key.as_ref()];

		let change_set = self
			.relay_client
			.rpc()
			.query_storage(keys.clone(), start, Some(latest_finalized_hash))
			.await?;

		let mut change_set_join_set: JoinSet<Result<Option<_>, anyhow::Error>> = JoinSet::new();
		let mut parachain_headers_with_proof = BTreeMap::<H256, ParachainHeaderProofs>::default();
		log::debug!(target:"hyperspace", "Got {} authority set changes", change_set.len());

		fn clone_storage_change_sets<T: light_client_common::config::Config + Send + Sync>(
			changes: &[StorageChangeSet<T::Hash>],
		) -> Vec<StorageChangeSet<T::Hash>> {
			changes
				.iter()
				.map(|change| StorageChangeSet {
					block: change.block.clone(),
					changes: change.changes.clone(),
				})
				.collect()
		}
		let latest_para_height = Arc::new(AtomicU32::new(0u32));
		for changes in change_set.chunks(PROCESS_CHANGES_SET_BATCH_SIZE) {
			for change in clone_storage_change_sets::<T>(changes) {
				let header_numbers = header_numbers.clone();
				let keys = vec![para_storage_key.clone()];
				let client = self.clone();
				let to = self.rpc_call_delay.as_millis();
				let duration1 = Duration::from_millis(rand::thread_rng().gen_range(1..to) as u64);
				let latest_para_height = latest_para_height.clone();
				change_set_join_set.spawn(async move {
					sleep(duration1).await;
					let header = client
						.relay_client
						.rpc()
						.header(Some(change.block))
						.await?
						.ok_or_else(|| anyhow!("block not found {:?}", change.block))?;

					let parachain_header_bytes = {
						let key = T::Storage::paras_heads(client.para_id);
						let data = client
							.relay_client
							.storage()
							.at(header.hash())
							.fetch(&key)
							.await?
							.expect("Header exists in its own changeset; qed");
						<T::Storage as RuntimeStorage>::HeadData::from_inner(data)
					};

					let para_header: T::Header =
						Decode::decode(&mut parachain_header_bytes.as_ref())?;
					let para_block_number = para_header.number();
					// skip genesis header or any unknown headers
					if para_block_number == Zero::zero() ||
						!header_numbers.contains(&para_block_number)
					{
						return Ok(None)
					}

					let state_proof = client
						.relay_client
						.rpc()
						.read_proof(keys.iter().map(AsRef::as_ref), Some(header.hash()))
						.await?
						.proof
						.into_iter()
						.map(|p| p.0)
						.collect();

					let TimeStampExtWithProof { ext: extrinsic, proof: extrinsic_proof } =
						fetch_timestamp_extrinsic_with_proof(
							&client.para_client,
							Some(para_header.hash()),
						)
						.await
						.map_err(|err| anyhow!("Error fetching timestamp with proof: {err:?}"))?;
					let proofs = ParachainHeaderProofs { state_proof, extrinsic, extrinsic_proof };
					latest_para_height.fetch_max(u32::from(para_block_number), Ordering::SeqCst);
					Ok(Some((H256::from(header.hash()), proofs)))
				});
			}

			while let Some(res) = change_set_join_set.join_next().await {
				if let Some((hash, proofs)) = res?? {
					parachain_headers_with_proof.insert(hash, proofs);
				}
			}
		}

		unknown_headers.sort_by_key(|header| header.number());
		// overwrite unknown headers
		finality_proof.unknown_headers = unknown_headers;

		Ok(ParachainHeadersWithFinalityProof {
			finality_proof,
			parachain_headers: parachain_headers_with_proof,
			latest_para_height: latest_para_height.load(Ordering::SeqCst),
		})
	}

	/// Queries the block at which the epoch for the given block belongs to ends.
	pub async fn session_start_and_end_for_block(
		&self,
		block: u32,
	) -> Result<(u32, u32), anyhow::Error> {
		let epoch_addr = T::Storage::babe_epoch_start();
		let block_hash = self
			.relay_client
			.rpc()
			.block_hash(Some(block.into()))
			.await?
			.ok_or_else(|| anyhow!("Failed to fetch block hash for block number {}", block))?;
		let (previous_epoch_start, current_epoch_start) = self
			.relay_client
			.storage()
			.at(block_hash)
			.fetch(&epoch_addr)
			.await?
			.ok_or_else(|| anyhow!("Failed to fetch epoch information"))?;
		Ok((
			current_epoch_start,
			current_epoch_start + (current_epoch_start - previous_epoch_start),
		))
	}

	/// Returns the session length in blocks
	pub async fn session_length(&self) -> Result<u32, anyhow::Error> {
		let metadata = self.relay_client.rpc().metadata(None).await?;
		let metadata = metadata.pallet("Babe")?.constant("EpochDuration")?;
		Ok(Decode::decode(&mut &metadata.value[..])?)
	}
}
