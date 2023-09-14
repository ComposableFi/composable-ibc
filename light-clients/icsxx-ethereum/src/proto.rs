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

include!(concat!(env!("OUT_DIR"), "/ibc.lightclients.ethereum.v1.rs"));

mod convert {
	use super::{
		ancestry_proof, AncestorBlock as RawAncestorBlock, AncestryProof as RawAncestryProof,
		BeaconBlockHeader as RawBeaconBlockHeader, BlockRoots as RawBlockRoots,
		BlockRootsProof as RawBlockRootsProof, ExecutionPayloadProof as RawExecutionPayloadProof,
		FinalityProof as RawFinalityProof, HistoricalRoots as RawHistoricalRoots,
		LightClientState as RawLightClientState, SyncAggregate as RawSyncAggregate,
		SyncCommittee as RawSyncCommittee, SyncCommitteeUpdate as RawSyncCommitteeUpdate,
	};
	use crate::{alloc::string::ToString, error::Error};
	use alloc::vec::Vec;
	use primitive_types::H256;
	use ssz_rs::{Bitvector, Deserialize, Serialize};
	use sync_committee_primitives::{
		consensus_types::{BeaconBlockHeader, SyncAggregate, SyncCommittee},
		constants::SYNC_COMMITTEE_SIZE,
		types::{
			AncestorBlock, AncestryProof, BlockRootsProof, ExecutionPayloadProof, FinalityProof,
			SyncCommitteeUpdate,
		},
	};
	use sync_committee_verifier::LightClientState;

	impl TryFrom<RawBeaconBlockHeader> for BeaconBlockHeader {
		type Error = Error;

		fn try_from(raw: RawBeaconBlockHeader) -> Result<Self, Self::Error> {
			let slot = raw.slot;
			let proposer_index = raw.proposer_index;
			let parent_root = raw.parent_root.as_slice().try_into()?;
			let state_root = raw.state_root.as_slice().try_into()?;
			let body_root = raw.body_root.as_slice().try_into()?;
			Ok(Self { slot, parent_root, state_root, body_root, proposer_index })
		}
	}

	impl From<BeaconBlockHeader> for RawBeaconBlockHeader {
		fn from(header: BeaconBlockHeader) -> Self {
			Self {
				slot: header.slot.into(),
				proposer_index: header.proposer_index as u64,
				parent_root: header.parent_root.as_bytes().to_vec(),
				state_root: header.state_root.as_bytes().to_vec(),
				body_root: header.body_root.as_bytes().to_vec(),
			}
		}
	}

	impl TryFrom<RawSyncCommitteeUpdate> for SyncCommitteeUpdate {
		type Error = Error;

		fn try_from(raw: RawSyncCommitteeUpdate) -> Result<Self, Self::Error> {
			let next_sync_committee = raw
				.next_sync_committee
				.ok_or_else(|| {
					anyhow::anyhow!("SyncCommitteeUpdate.next_sync_committee is required")
				})?
				.try_into()?;
			let next_sync_committee_branch = raw
				.next_sync_committee_branch
				.into_iter()
				.map(|root| root.as_slice().try_into())
				.collect::<Result<Vec<_>, _>>()?;
			Ok(Self { next_sync_committee, next_sync_committee_branch })
		}
	}

	impl From<SyncCommitteeUpdate> for RawSyncCommitteeUpdate {
		fn from(update: SyncCommitteeUpdate) -> Self {
			Self {
				next_sync_committee: Some(update.next_sync_committee.into()),
				next_sync_committee_branch: update
					.next_sync_committee_branch
					.into_iter()
					.map(|root| root.0.to_vec())
					.collect(),
			}
		}
	}

	impl TryFrom<RawSyncCommittee> for SyncCommittee<SYNC_COMMITTEE_SIZE> {
		type Error = Error;

		fn try_from(raw: RawSyncCommittee) -> Result<Self, Self::Error> {
			let public_keys = raw
				.public_keys
				.into_iter()
				.map(|pubkey| pubkey.as_slice().try_into())
				.collect::<Result<Vec<_>, _>>()?
				.try_into()
				.map_err(|(_, e)| e)?;
			let aggregate_public_key = raw.aggregate_public_key.as_slice().try_into()?;

			Ok(Self { public_keys, aggregate_public_key })
		}
	}

	impl From<SyncCommittee<SYNC_COMMITTEE_SIZE>> for RawSyncCommittee {
		fn from(committee: SyncCommittee<SYNC_COMMITTEE_SIZE>) -> Self {
			Self {
				public_keys: committee.public_keys.iter().map(|pubkey| pubkey.to_vec()).collect(),
				aggregate_public_key: committee.aggregate_public_key.to_vec(),
			}
		}
	}

	impl TryFrom<RawExecutionPayloadProof> for ExecutionPayloadProof {
		type Error = Error;

		fn try_from(raw: RawExecutionPayloadProof) -> Result<Self, Self::Error> {
			let state_root = H256::from_slice(&raw.state_root);
			let block_number = raw.block_number;
			let multi_proof = raw
				.multi_proof
				.into_iter()
				.map(|root| root.as_slice().try_into())
				.collect::<Result<Vec<_>, _>>()?;
			let execution_payload_branch = raw
				.execution_payload_branch
				.into_iter()
				.map(|root| root.as_slice().try_into())
				.collect::<Result<Vec<_>, _>>()?;
			let timestamp = raw.timestamp;
			Ok(Self { state_root, block_number, multi_proof, execution_payload_branch, timestamp })
		}
	}

	impl From<ExecutionPayloadProof> for RawExecutionPayloadProof {
		fn from(proof: ExecutionPayloadProof) -> Self {
			Self {
				state_root: proof.state_root.0.to_vec(),
				block_number: proof.block_number,
				multi_proof: proof.multi_proof.iter().map(|root| root.0.to_vec()).collect(),
				execution_payload_branch: proof
					.execution_payload_branch
					.iter()
					.map(|root| root.0.to_vec())
					.collect(),
				timestamp: proof.timestamp,
			}
		}
	}

	impl TryFrom<RawFinalityProof> for FinalityProof {
		type Error = Error;

		fn try_from(raw: RawFinalityProof) -> Result<Self, Self::Error> {
			let epoch = raw.epoch;
			let finality_branch = raw
				.finality_branch
				.into_iter()
				.map(|root| root.as_slice().try_into())
				.collect::<Result<Vec<_>, _>>()?;
			Ok(Self { epoch, finality_branch })
		}
	}

	impl From<FinalityProof> for RawFinalityProof {
		fn from(proof: FinalityProof) -> Self {
			Self {
				epoch: proof.epoch,
				finality_branch: proof.finality_branch.iter().map(|root| root.0.to_vec()).collect(),
			}
		}
	}

	impl TryFrom<RawSyncAggregate> for SyncAggregate<SYNC_COMMITTEE_SIZE> {
		type Error = Error;

		fn try_from(raw: RawSyncAggregate) -> Result<Self, Self::Error> {
			let sync_committee_bits = Bitvector::deserialize(&raw.sync_committee_bits.as_slice())?;
			let sync_committee_signature = raw.sync_committee_signature.as_slice().try_into()?;
			Ok(Self { sync_committee_bits, sync_committee_signature })
		}
	}

	impl From<SyncAggregate<SYNC_COMMITTEE_SIZE>> for RawSyncAggregate {
		fn from(aggregate: SyncAggregate<SYNC_COMMITTEE_SIZE>) -> Self {
			let mut sync_committee_bits = Vec::new();
			aggregate
				.sync_committee_bits
				.serialize(&mut sync_committee_bits)
				.expect("Should serialize");
			Self {
				sync_committee_bits,
				sync_committee_signature: aggregate.sync_committee_signature.to_vec(),
			}
		}
	}

	impl TryFrom<RawAncestorBlock> for AncestorBlock {
		type Error = Error;

		fn try_from(raw: RawAncestorBlock) -> Result<Self, Self::Error> {
			let header = raw
				.header
				.ok_or_else(|| Error::Custom("Missing header in ancestor block".to_string()))?
				.try_into()?;
			let execution_payload = raw
				.execution_payload
				.ok_or_else(|| {
					Error::Custom("Missing execution payload in ancestor block".to_string())
				})?
				.try_into()?;
			let ancestry_proof = raw
				.ancestry_proof
				.ok_or_else(|| {
					Error::Custom("Missing ancestry proof in ancestor block".to_string())
				})?
				.try_into()?;
			Ok(Self { header, execution_payload, ancestry_proof })
		}
	}

	impl From<AncestorBlock> for RawAncestorBlock {
		fn from(block: AncestorBlock) -> Self {
			Self {
				header: Some(block.header.into()),
				execution_payload: Some(block.execution_payload.into()),
				ancestry_proof: Some(block.ancestry_proof.into()),
			}
		}
	}

	impl TryFrom<RawAncestryProof> for AncestryProof {
		type Error = Error;

		fn try_from(raw: RawAncestryProof) -> Result<Self, Self::Error> {
			let msg = raw
				.message
				.ok_or_else(|| Error::Custom("Missing message in ancestry proof".to_string()))?;
			match msg {
				ancestry_proof::Message::BlockRoots(br) => {
					let block_roots_proof = br
						.block_roots_proof
						.ok_or_else(|| {
							Error::Custom("Missing block roots proof in ancestry proof".to_string())
						})?
						.try_into()?;
					let block_roots_branch = br
						.block_roots_branch
						.into_iter()
						.map(|root| root.as_slice().try_into())
						.collect::<Result<Vec<_>, _>>()?;
					Ok(Self::BlockRoots { block_roots_proof, block_roots_branch })
				},
				ancestry_proof::Message::HistoricalRoots(hr) => {
					let block_roots_proof = hr
						.block_roots_proof
						.ok_or_else(|| {
							Error::Custom("Missing block roots proof in ancestry proof".to_string())
						})?
						.try_into()?;
					let historical_batch_proof = hr
						.historical_batch_proof
						.into_iter()
						.map(|root| root.as_slice().try_into())
						.collect::<Result<Vec<_>, _>>()?;
					let historical_roots_proof = hr
						.historical_roots_proof
						.into_iter()
						.map(|root| root.as_slice().try_into())
						.collect::<Result<Vec<_>, _>>()?;
					let historical_roots_index = hr.historical_roots_index;
					let historical_roots_branch = hr
						.historical_roots_branch
						.into_iter()
						.map(|root| root.as_slice().try_into())
						.collect::<Result<Vec<_>, _>>()?;
					Ok(Self::HistoricalRoots {
						block_roots_proof,
						historical_batch_proof,
						historical_roots_proof,
						historical_roots_index,
						historical_roots_branch,
					})
				},
			}
		}
	}

	impl From<AncestryProof> for RawAncestryProof {
		fn from(proof: AncestryProof) -> Self {
			match proof {
				AncestryProof::BlockRoots { block_roots_proof, block_roots_branch } => {
					let block_roots_branch =
						block_roots_branch.into_iter().map(|root| root.0.to_vec()).collect();
					Self {
						message: Some(ancestry_proof::Message::BlockRoots(RawBlockRoots {
							block_roots_proof: Some(block_roots_proof.into()),
							block_roots_branch,
						})),
					}
				},
				AncestryProof::HistoricalRoots {
					block_roots_proof,
					historical_batch_proof,
					historical_roots_proof,
					historical_roots_index,
					historical_roots_branch,
				} => {
					let historical_batch_proof =
						historical_batch_proof.into_iter().map(|root| root.0.to_vec()).collect();
					let historical_roots_proof =
						historical_roots_proof.into_iter().map(|root| root.0.to_vec()).collect();
					let historical_roots_branch =
						historical_roots_branch.into_iter().map(|root| root.0.to_vec()).collect();
					Self {
						message: Some(ancestry_proof::Message::HistoricalRoots(
							RawHistoricalRoots {
								block_roots_proof: Some(block_roots_proof.into()),
								historical_batch_proof,
								historical_roots_proof,
								historical_roots_index,
								historical_roots_branch,
							},
						)),
					}
				},
			}
		}
	}

	impl TryFrom<RawBlockRootsProof> for BlockRootsProof {
		type Error = Error;

		fn try_from(raw: RawBlockRootsProof) -> Result<Self, Self::Error> {
			let block_header_index = raw.block_header_index;
			let block_header_branch = raw
				.block_header_branch
				.into_iter()
				.map(|root| root.as_slice().try_into())
				.collect::<Result<Vec<_>, _>>()?;
			Ok(Self { block_header_index, block_header_branch })
		}
	}

	impl From<BlockRootsProof> for RawBlockRootsProof {
		fn from(proof: BlockRootsProof) -> Self {
			let block_header_branch =
				proof.block_header_branch.into_iter().map(|root| root.0.to_vec()).collect();
			Self { block_header_index: proof.block_header_index, block_header_branch }
		}
	}

	impl TryFrom<RawLightClientState> for LightClientState {
		type Error = Error;

		fn try_from(raw: RawLightClientState) -> Result<Self, Self::Error> {
			let finalized_header = raw
				.finalized_header
				.ok_or_else(|| {
					Error::Custom("Missing finalized header in light client state".to_string())
				})?
				.try_into()?;
			let current_sync_committee = raw
				.current_sync_committee
				.ok_or_else(|| {
					Error::Custom(
						"Missing current sync committee in light client state".to_string(),
					)
				})?
				.try_into()?;
			let next_sync_committee = raw
				.next_sync_committee
				.ok_or_else(|| {
					Error::Custom("Missing next sync committee in light client state".to_string())
				})?
				.try_into()?;
			Ok(LightClientState {
				finalized_header,
				latest_finalized_epoch: raw.latest_finalized_epoch,
				current_sync_committee,
				next_sync_committee,
			})
		}
	}

	impl From<LightClientState> for RawLightClientState {
		fn from(client_state: LightClientState) -> Self {
			Self {
				finalized_header: Some(client_state.finalized_header.into()),
				latest_finalized_epoch: client_state.latest_finalized_epoch,
				current_sync_committee: Some(client_state.current_sync_committee.into()),
				next_sync_committee: Some(client_state.next_sync_committee.into()),
			}
		}
	}
}
