// use base2::Base2;
use crate::client::{ClientError, EthereumClient};
use anyhow::anyhow;
use ethers::{
	core::{rand, rand::Rng},
	prelude::{EthCall, H256},
};
use icsxx_ethereum::client_state::ClientState;
use pallet_ibc::light_clients::HostFunctionsManager;
use primitives::mock::LocalClientTypes;
use ssz_rs::{
	calculate_multi_merkle_root, is_valid_merkle_branch, GeneralizedIndex, Merkleized, Node,
};
use std::time::Duration;
use sync_committee_primitives::{
	consensus_types::BeaconBlockHeader,
	constants::{Bytes32, SLOTS_PER_EPOCH},
	types::{
		AncestorBlock, AncestryProof, BlockRootsProof, ExecutionPayloadProof, FinalityProof,
		LightClientState, LightClientUpdate, SyncCommitteeUpdate,
	},
	util::{compute_fork_version, compute_sync_committee_period_at_slot},
};
use sync_committee_prover::{
	prove_block_roots_proof, prove_execution_payload, prove_finalized_header,
	prove_sync_committee_update, SyncCommitteeProver,
};
use sync_committee_verifier::verify_sync_committee_attestation;
use tokio::{task::JoinSet, time, time::sleep};
// use tokio_stream::{wrappers::IntervalStream, StreamExt};
use icsxx_ethereum::client_message::Header;
use sync_committee_primitives::constants::SYNC_COMMITTEE_SIZE;

pub async fn prove_fast(
	client: &EthereumClient,
	eth_client_state: &ClientState<HostFunctionsManager>,
	block_number: u64,
) -> Result<Header, ClientError> {
	let sync_committee_prover = client.prover();

	let block_id = format!("{block_number:?}");
	// let block_id = "head";

	let client_state = &eth_client_state.inner;
	// let block_id = "head";

	let block_header = sync_committee_prover.fetch_header(&block_id).await?;

	let block = sync_committee_prover.fetch_block(&block_header.slot.to_string()).await?;
	let state = sync_committee_prover.fetch_beacon_state(&block_header.slot.to_string()).await?;

	let from = client_state.finalized_header.slot + 1;
	let to = block_header.slot;
	let mut join_set: JoinSet<Result<_, anyhow::Error>> = JoinSet::new();
	let range = (from..to).collect::<Vec<_>>();
	let delay = 5000;
	let mut ancestor_blocks = vec![];
	for heights in range.chunks(10) {
		for i in heights.iter().copied() {
			let duration = Duration::from_millis(rand::thread_rng().gen_range(1..delay) as u64);
			let sync_committee_prover = sync_committee_prover.clone();
			join_set.spawn(async move {
				sleep(duration).await;

				match sync_committee_prover.fetch_beacon_state(i.to_string().as_str()).await {
					Ok(mut header_state) => {
						let execution_payload_proof = prove_execution_payload(&mut header_state)?;
						log::info!("UPDATE ANC: {}", header_state.slot);
						return Ok(AncestorBlock {
							header: BeaconBlockHeader {
								slot: header_state.slot,
								proposer_index: 0,
								parent_root: Default::default(),
								state_root: Default::default(),
								body_root: Default::default(),
							},
							execution_payload: execution_payload_proof,
							ancestry_proof: AncestryProof::BlockRoots {
								block_roots_proof: BlockRootsProof {
									block_header_index: 0,
									block_header_branch: vec![],
								},
								block_roots_branch: vec![],
							},
						})
					},
					Err(e) => {
						log::info!("UPDATE cannot fetch: {} {e}", i);
					},
				}
				Err(anyhow::anyhow!("Could not fetch ancestor block"))
			});
		}
		while let Some(res) = join_set.join_next().await {
			match res.map_err(|e| anyhow!("{e}"))? {
				Ok(out) => {
					ancestor_blocks.push(out);
				},
				Err(e) => {
					log::warn!("Error fetching ancestor block: {:?}", e)
				},
			}
		}
	}

	ancestor_blocks.sort_by_key(|ancestor_block| ancestor_block.header.slot);

	let ep = block.body.execution_payload;
	let execution_payload_proof = ExecutionPayloadProof {
		state_root: H256::from_slice(&ep.state_root.to_vec()),
		block_number: ep.block_number,
		multi_proof: vec![],
		execution_payload_branch: vec![],
		timestamp: ep.timestamp,
	};
	let mut light_client_update = LightClientUpdate {
		attested_header: Default::default(),
		sync_committee_update: Default::default(),
		finalized_header: Default::default(),
		execution_payload: execution_payload_proof,
		finality_proof: Default::default(),
		sync_aggregate: Default::default(),
		signature_slot: Default::default(),
	};
	light_client_update.attested_header.slot = block_header.slot;
	light_client_update.finalized_header.slot = block_header.slot;

	Ok(Header { inner: light_client_update, ancestor_blocks })
}
