// use base2::Base2;
use crate::client::{ClientError, EthereumClient};
use anyhow::{anyhow, Error};
use ethers::{
	core::{rand, rand::Rng},
	prelude::{EthCall, H256},
};
use ethers_providers::Middleware;
use icsxx_ethereum::{client_message::Header, client_state::ClientState};
use log::{error, info};
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

#[cfg(not(feature = "no_beacon"))]
pub async fn prove_fast(
	client: &EthereumClient,
	eth_client_state: &ClientState<HostFunctionsManager>,
	mut block_number: u64,
	up_to: u64,
) -> Result<Header, ClientError> {
	info!("prove_fast up to {up_to}");
	let sync_committee_prover = client.prover();

	let mut block_id = format!("{block_number:?}");
	let client_state = &eth_client_state.inner;

	// sync_committee_prover.fetch

	let block = loop {
		let block = sync_committee_prover.fetch_block(&block_id).await?;
		if block.body.execution_payload.block_number <= up_to {
			break block
		}

		let diff = block.body.execution_payload.block_number.saturating_sub(up_to);
		block_number -= diff;
		if block_number == 0 {
			return Err(ClientError::Other("Block number is 0".to_string()))
		}
		info!("{} > {}, proving {block_number}", block.body.execution_payload.block_number, up_to);
		block_id = format!("{block_number:?}");
	};
	let block_header = sync_committee_prover.fetch_header(&block_id).await?;
	// let state = sync_committee_prover.fetch_beacon_state(&block_header.slot.to_string()).await?;

	let from = client_state.finalized_header.slot + 1;
	let to = block_header.slot;
	let mut join_set: JoinSet<Result<_, anyhow::Error>> = JoinSet::new();
	let range = vec![to];
	// let range = (from..to).collect::<Vec<_>>();
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
						// execution_payload_proof.block_number
						if header_state.latest_execution_payload_header.block_number > up_to {
							return Ok(None)
						}
						log::info!("UPDATE ANC: {}", header_state.slot);
						return Ok(Some(AncestorBlock {
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
						}))
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
				Ok(Some(out)) => {
					ancestor_blocks.push(out);
				},
				Ok(None) => {},
				Err(e) => {
					log::warn!("Error fetching ancestor block: {:?}", e)
				},
			}
		}
	}

	ancestor_blocks.sort_by_key(|ancestor_block| ancestor_block.header.slot);

	let ep = block.body.execution_payload;
	ancestor_blocks.iter().for_each(|e| {
		info!("ADD block {}", e.execution_payload.block_number);
	});
	info!("ADD block {}", ep.block_number);
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

#[cfg(feature = "no_beacon")]
pub async fn prove_fast(
	client: &EthereumClient,
	eth_client_state: &ClientState<HostFunctionsManager>,
	_block_number: u64,
	_up_to: u64,
) -> Result<Header, ClientError> {
	let client = client.client();
	let to_block = client
		.get_block_number()
		.await
		.map_err(|e| ClientError::Other(format!("failed to get block number: {:?}", e)))?
		.as_u64();
	let from_block = eth_client_state.latest_height as u64 + 1;
	// let ep = block.body.execution_payload;
	let latest_block = client
		.get_block(to_block)
		.await
		.map_err(|e| ClientError::Other(format!("failed to get block {}: {:?}", to_block, e)))?
		.expect("block not found");
	// let block_header = latest_block.header;
	let execution_payload_proof = ExecutionPayloadProof {
		state_root: latest_block.state_root,
		block_number: latest_block.number.unwrap().as_u64(),
		multi_proof: vec![],
		execution_payload_branch: vec![],
		timestamp: latest_block.timestamp.as_u64(),
	};
	let mut ancestor_blocks = vec![];
	for n in from_block..to_block {
		let block = client
			.get_block(n)
			.await
			.map_err(|e| ClientError::Other(format!("failed to get block {}: {:?}", n, e)))
			.expect("block not found")
			.expect("block not found");
		ancestor_blocks.push(AncestorBlock {
			header: BeaconBlockHeader {
				slot: block.number.unwrap().as_u64(),
				proposer_index: 0,
				parent_root: Node(block.parent_hash.0),
				state_root: Node(block.state_root.0),
				body_root: Node::default(),
			},
			execution_payload: ExecutionPayloadProof {
				state_root: block.state_root,
				block_number: block.number.unwrap().as_u64(),
				multi_proof: vec![],
				execution_payload_branch: vec![],
				timestamp: block.timestamp.as_u64(),
			},
			ancestry_proof: AncestryProof::BlockRoots {
				block_roots_proof: BlockRootsProof {
					block_header_index: 0,
					block_header_branch: vec![],
				},
				block_roots_branch: vec![],
			},
		});
	}
	let mut light_client_update = LightClientUpdate {
		attested_header: Default::default(),
		sync_committee_update: Default::default(),
		finalized_header: BeaconBlockHeader {
			slot: latest_block.number.unwrap().as_u64(),
			proposer_index: 0,
			parent_root: Node(latest_block.parent_hash.0),
			state_root: Node(latest_block.state_root.0),
			body_root: Node::default(),
		},
		execution_payload: execution_payload_proof,
		finality_proof: Default::default(),
		sync_aggregate: Default::default(),
		signature_slot: Default::default(),
	};
	light_client_update.attested_header.slot = latest_block.number.unwrap().as_u64();

	Ok(Header { inner: light_client_update, ancestor_blocks })
}
