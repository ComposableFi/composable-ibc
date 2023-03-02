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

use crate::polkadot::api::runtime_types::polkadot_parachain::primitives::Id;
use anyhow::anyhow;
pub use beefy_prover;
use beefy_prover::helpers::{
	fetch_timestamp_extrinsic_with_proof, unsafe_arc_cast, TimeStampExtWithProof,
};
use codec::{Decode, Encode};
use finality_grandpa_rpc::GrandpaApiClient;
use jsonrpsee::{async_client::Client, ws_client::WsClientBuilder};
use primitives::{
	parachain_header_storage_key, ClientState, FinalityProof, ParachainHeaderProofs,
	ParachainHeadersWithFinalityProof,
};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_finality_grandpa::{AuthorityId, AuthoritySignature};
use sp_runtime::traits::{One, Zero};
use std::{
	collections::{BTreeMap, BTreeSet},
	sync::Arc,
};
use subxt::{config::Header, Config, OnlineClient};

/// Host function implementation for the verifier
pub mod host_functions;
/// Subxt generated code for the parachain
pub mod parachain;
/// Subxt generated code for the relay chain
pub mod polkadot;

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

impl<T> GrandpaProver<T>
where
	T: Config,
	T::BlockNumber: Ord + Zero,
	u32: From<T::BlockNumber>,
	sp_core::H256: From<T::Hash>,
{
	/// Initializes the parachain and relay chain clients given the ws urls.
	pub async fn new(
		relay_ws_url: &str,
		para_ws_url: &str,
		para_id: u32,
	) -> Result<Self, anyhow::Error> {
		let relay_ws_client = Arc::new(WsClientBuilder::default().build(relay_ws_url).await?);
		let relay_client = OnlineClient::<T>::from_rpc_client(relay_ws_client.clone()).await?;
		let para_ws_client = Arc::new(WsClientBuilder::default().build(para_ws_url).await?);
		let para_client = OnlineClient::<T>::from_rpc_client(para_ws_client.clone()).await?;

		Ok(Self { relay_ws_client, relay_client, para_ws_client, para_client, para_id })
	}

	/// Construct the inital client state.
	pub async fn initialize_client_state(&self) -> Result<ClientState, anyhow::Error> {
		use sp_finality_grandpa::AuthorityList;
		let latest_relay_hash = self.relay_client.rpc().finalized_head().await?;
		let header = self
			.relay_client
			.rpc()
			.header(Some(latest_relay_hash))
			.await?
			.ok_or_else(|| anyhow!("Header not found for hash: {latest_relay_hash:?}"))?;

		let current_set_id = {
			let key = polkadot::api::storage().grandpa().current_set_id();
			self.relay_client
				.storage()
				.at(Some(latest_relay_hash))
				.await
				.expect("Storage client")
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
				.map(|res| hex::decode(&res[2..]))??;

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
			self.query_latest_finalized_parachain_header(latest_relay_height).await?;

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
	) -> Result<T::Header, anyhow::Error> {
		let latest_finalized_hash = self
			.relay_client
			.rpc()
			.block_hash(Some(latest_finalized_height.into()))
			.await?
			.ok_or_else(|| anyhow!("Block hash not found for number: {latest_finalized_height}"))?;
		let key = polkadot::api::storage().paras().heads(&Id(self.para_id));
		let header = self
			.relay_client
			.storage()
			.at(Some(latest_finalized_hash))
			.await
			.expect("Storage client")
			.fetch(&key)
			.await?
			.ok_or_else(|| anyhow!("parachain header not found for para id: {}", self.para_id))?;
		let header = T::Header::decode(&mut &header.0[..])
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
		header_numbers: Vec<T::BlockNumber>,
	) -> Result<ParachainHeadersWithFinalityProof<H>, anyhow::Error>
	where
		H: Header + codec::Decode,
		u32: From<<H as Header>::Number>,
		<H::Hasher as subxt::config::Hasher>::Output: From<T::Hash>,
		T::Hash: From<<H::Hasher as subxt::config::Hasher>::Output>,
		H::Number: finality_grandpa::BlockNumberOps,
		T::BlockNumber: One,
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
		for height in previous_finalized_height..=latest_finalized_height {
			let hash = self
				.relay_client
				.rpc()
				.block_hash(Some(height.into()))
				.await?
				.ok_or_else(|| anyhow!("Failed to fetch block has for height {height}"))?;

			let header = self
				.relay_client
				.rpc()
				.header(Some(hash))
				.await?
				.ok_or_else(|| anyhow!("Header with hash: {hash:?} not found!"))?;

			unknown_headers.push(H::decode(&mut &header.encode()[..])?);
		}

		// overwrite unknown headers
		finality_proof.unknown_headers = unknown_headers;

		// we are interested only in the blocks where our parachain header changes.
		let para_storage_key = parachain_header_storage_key(self.para_id);
		let keys = vec![para_storage_key.as_ref()];
		let mut parachain_headers_with_proof = BTreeMap::<H256, ParachainHeaderProofs>::default();

		let change_set = self
			.relay_client
			.rpc()
			.query_storage(keys.clone(), start, Some(latest_finalized_hash))
			.await?;

		let mut latest_para_height = 0u32;
		for changes in change_set {
			let header = self
				.relay_client
				.rpc()
				.header(Some(changes.block))
				.await?
				.ok_or_else(|| anyhow!("block not found {:?}", changes.block))?;

			let parachain_header_bytes = {
				let key = polkadot::api::storage().paras().heads(&Id(self.para_id));
				self.relay_client
					.storage()
					.at(Some(header.hash()))
					.await
					.expect("Storage client")
					.fetch(&key)
					.await?
					.expect("Header exists in its own changeset; qed")
					.0
			};

			let para_header: T::Header = Decode::decode(&mut &parachain_header_bytes[..])?;
			let para_block_number = para_header.number();
			// skip genesis header or any unknown headers
			if para_block_number == Zero::zero() || !header_numbers.contains(&para_block_number) {
				continue
			}

			let state_proof = self
				.relay_client
				.rpc()
				.read_proof(keys.clone(), Some(header.hash()))
				.await?
				.proof
				.into_iter()
				.map(|p| p.0)
				.collect();

			let TimeStampExtWithProof { ext: extrinsic, proof: extrinsic_proof } =
				fetch_timestamp_extrinsic_with_proof(&self.para_client, Some(para_header.hash()))
					.await
					.map_err(|err| anyhow!("Error fetching timestamp with proof: {err:?}"))?;
			let proofs = ParachainHeaderProofs { state_proof, extrinsic, extrinsic_proof };
			latest_para_height = latest_para_height.max(u32::from(para_block_number));
			parachain_headers_with_proof.insert(header.hash().into(), proofs);
		}

		Ok(ParachainHeadersWithFinalityProof {
			finality_proof,
			parachain_headers: parachain_headers_with_proof,
			latest_para_height,
		})
	}

	/// Queries the block at which the epoch for the given block belongs to ends.
	pub async fn session_start_and_end_for_block(
		&self,
		block: u32,
	) -> Result<(u32, u32), anyhow::Error> {
		let epoch_addr = polkadot::api::storage().babe().epoch_start();
		let block_hash = self.relay_client.rpc().block_hash(Some(block.into())).await?;
		let (previous_epoch_start, current_epoch_start) = self
			.relay_client
			.storage()
			.at(block_hash)
			.await
			.expect("Storage client")
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
