// use base2::Base2;
use crate::client::{ClientError, EthereumClient};
use anyhow::{anyhow, Error};
use ethers::{
	core::{rand, rand::Rng},
	prelude::{EthCall, H256},
	types::BlockNumber,
};
use ethers_providers::Middleware;
use icsxx_ethereum::{client_message::Header, client_state::ClientState};
use log::{debug, error, info};
use pallet_ibc::light_clients::HostFunctionsManager;
use primitives::mock::LocalClientTypes;
use ssz_rs::{
	calculate_multi_merkle_root, is_valid_merkle_branch, GeneralizedIndex, Merkleized, Node,
};
use std::time::Duration;
use sync_committee_primitives::{
	consensus_types::{BeaconBlockHeader, Checkpoint},
	constants::{Root, EPOCHS_PER_SYNC_COMMITTEE_PERIOD, SLOTS_PER_EPOCH, SYNC_COMMITTEE_SIZE},
	types::{
		ExecutionPayloadProof, FinalityProof, SyncCommitteeUpdate, VerifierState,
		VerifierStateUpdate as LightClientUpdate, VerifierStateUpdate,
	},
	util::{
		compute_epoch_at_slot, compute_sync_committee_period_at_slot,
		should_have_sync_committee_update,
	},
};
use sync_committee_prover::{
	prove_execution_payload, prove_finalized_header, prove_sync_committee_update,
	SyncCommitteeProver,
};
use tokio::{task::JoinSet, time, time::sleep};

// #[cfg(not(feature = "no_beacon"))]
pub async fn prove_fast(
	client: &EthereumClient,
	_eth_client_state: &ClientState<HostFunctionsManager>,
	mut block_number: u64,
	up_to: u64,
) -> Result<Header, ClientError> {
	info!("prove_fast up to {up_to}");
	let sync_committee_prover = client.prover();

	let mut block_id = format!("{block_number:?}");

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

	Ok(Header { inner: light_client_update })
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EventResponse {
	pub block: Root,
	pub state: Root,
	pub epoch: String,
	pub execution_optimistic: bool,
}

pub async fn fetch_light_client_update2(
	prover: &SyncCommitteeProver,
	mut client_state: VerifierState,
	finality_checkpoint: Checkpoint,
	latest_block_id: Option<&str>,
	debug_target: &str,
) -> Result<Option<VerifierStateUpdate>, anyhow::Error> {
	if finality_checkpoint.root == Node::default() ||
		client_state.latest_finalized_epoch >= finality_checkpoint.epoch
	{
		return Ok(None)
	}

	debug!(target: debug_target, "A new epoch has been finalized {}", finality_checkpoint.epoch);
	// Find the highest block with the a threshhold number of sync committee signatures
	let latest_header = prover.fetch_header(latest_block_id.unwrap_or("head")).await?;
	let state_period = client_state.state_period;
	let max_slot = (state_period + 2) * EPOCHS_PER_SYNC_COMMITTEE_PERIOD * SLOTS_PER_EPOCH - 1;
	let slot = latest_header.slot.min(max_slot);
	debug!(target: debug_target, "Using slot {slot}, max: {max_slot}");
	// let latest_root = latest_header.clone().hash_tree_root()?;
	let get_block_id = |root: Root| {
		let mut block_id = hex::encode(root.0.to_vec());
		block_id.insert_str(0, "0x");
		block_id
	};
	let mut block = prover.fetch_block(&slot.to_string()).await?;
	let min_signatures = (2 * SYNC_COMMITTEE_SIZE) / 3;
	loop {
		debug!(target: debug_target, "Trying block: {}, current trusted: {}", block.slot, client_state.finalized_header.slot);
		// Some checks on the epoch finalized by the signature block
		let parent_root = block.parent_root;
		let parent_block_id = get_block_id(parent_root);

		let num_signatures = block.body.sync_aggregate.sync_committee_bits.count_ones();
		if num_signatures < min_signatures {
			let parent_block = prover.fetch_block(&parent_block_id).await?;
			block = parent_block;
			continue
		}
		let parent_block = prover.fetch_block(&parent_block_id).await?;
		let parent_state_id = get_block_id(parent_block.state_root);
		let parent_block_finality_checkpoint =
			prover.fetch_finalized_checkpoint(Some(&parent_state_id)).await?.finalized;
		if parent_block_finality_checkpoint.epoch <= client_state.latest_finalized_epoch {
			debug!(target: "prover", "Signature block search has reached an invalid epoch {} latest finalized_block_epoch {}", parent_block_finality_checkpoint.epoch, client_state.latest_finalized_epoch);
			return Ok(None)
		}

		let signature_period = compute_sync_committee_period_at_slot(block.slot);

		debug!(target: debug_target, "Required {num_signatures} >= {min_signatures}, {signature_period} in [{state_period}, {}], {} > {}",  state_period + 1, parent_block_finality_checkpoint.epoch, client_state.latest_finalized_epoch);

		if num_signatures >= min_signatures &&
			(state_period..=state_period + 1).contains(&signature_period) &&
			parent_block_finality_checkpoint.epoch > client_state.latest_finalized_epoch
		{
			debug!(target: debug_target, "Found block: {}", parent_block.slot);
			break
		}
		block = parent_block;
	}

	let attested_block_id = get_block_id(block.parent_root);
	let attested_header = prover.fetch_header(&attested_block_id).await?;
	let mut attested_state =
		prover.fetch_beacon_state(&get_block_id(attested_header.state_root)).await?;
	if attested_state.finalized_checkpoint.root == Node::default() {
		debug!(target: debug_target, "No checkpoint root");
		return Ok(None)
	}
	debug!(target: debug_target, "Fetching update #1");
	let finalized_block_id = get_block_id(attested_state.finalized_checkpoint.root);
	let finalized_header = prover.fetch_header(&finalized_block_id).await?;
	let mut finalized_state =
		prover.fetch_beacon_state(&get_block_id(finalized_header.state_root)).await?;
	let finality_proof = FinalityProof {
		epoch: attested_state.finalized_checkpoint.epoch,
		finality_branch: prove_finalized_header(&mut attested_state)?,
	};

	debug!(target: debug_target, "Fetching update #2");
	let execution_payload_proof = prove_execution_payload(&mut finalized_state)?;

	let signature_period = compute_sync_committee_period_at_slot(block.slot);
	let client_state_next_sync_committee_root =
		client_state.next_sync_committee.hash_tree_root()?;
	let attested_state_current_sync_committee_root =
		attested_state.current_sync_committee.hash_tree_root()?;
	debug!(target: debug_target, "Fetching update #3");
	let sync_committee_update =
		// We must make sure we switch the sync comittee only when the finalized header has changed sync committees
		if should_have_sync_committee_update(state_period, signature_period) && client_state_next_sync_committee_root == attested_state_current_sync_committee_root {
			debug!(target: debug_target, "Fetching update #4");
			let sync_committee_proof = prove_sync_committee_update(&mut attested_state)?;
			Some(SyncCommitteeUpdate {
				next_sync_committee: attested_state.next_sync_committee,
				next_sync_committee_branch: sync_committee_proof,
			})
		} else {
			None
		};

	// construct light client
	let light_client_update = VerifierStateUpdate {
		attested_header,
		sync_committee_update,
		finalized_header,
		execution_payload: execution_payload_proof,
		finality_proof,
		sync_aggregate: block.body.sync_aggregate,
		signature_slot: block.slot,
	};

	Ok(Some(light_client_update))
}

pub async fn fetch_light_client_update(
	prover: &SyncCommitteeProver,
	mut client_state: VerifierState,
	finality_checkpoint: Checkpoint,
	latest_block_id: Option<&str>,
	debug_target: &str,
) -> Result<Option<VerifierStateUpdate>, anyhow::Error> {
	if finality_checkpoint.root == Node::default() ||
		client_state.latest_finalized_epoch >= finality_checkpoint.epoch
	{
		return Ok(None)
	}
	debug!(target: debug_target, "A new epoch has been finalized {}", finality_checkpoint.epoch);
	// Find the highest block with the a threshhold number of sync committee signatures
	let latest_header = prover.fetch_header(latest_block_id.unwrap_or("head")).await?;
	let latest_root = latest_header.clone().hash_tree_root()?;
	let get_block_id = |root: Root| {
		let mut block_id = hex::encode(root.0.to_vec());
		block_id.insert_str(0, "0x");
		block_id
	};
	let finalized_block = prover.fetch_block(&get_block_id(finality_checkpoint.root)).await?;
	let mut block = prover
		.fetch_block(&(client_state.finalized_header.slot + 32).to_string())
		.await?;
	let mut next_block = block.clone();
	let min_signatures = (2 * SYNC_COMMITTEE_SIZE) / 3;
	let state_period = client_state.state_period;
	let mut found = false;
	loop {
		debug!(target: debug_target, "Trying block: {}, current trusted: {}", next_block.slot, client_state.finalized_header.slot);
		// Some checks on the epoch finalized by the signature block
		let state_id = get_block_id(next_block.state_root);
		let block_finality_checkpoint =
			prover.fetch_finalized_checkpoint(Some(&state_id)).await?.finalized;
		let epoch = compute_epoch_at_slot(next_block.slot);

		if block_finality_checkpoint.epoch <= client_state.latest_finalized_epoch {
			debug!(target: "hyperspace_ethereum", "Signature block search has reached an invalid epoch {} latest finalized_block_epoch {}", block_finality_checkpoint.epoch, client_state.latest_finalized_epoch);
			next_block = prover.fetch_block(&(next_block.slot + 32).to_string()).await?;
			continue
			// return Ok(None)
		}

		let num_signatures = next_block.body.sync_aggregate.sync_committee_bits.count_ones();

		let signature_period = compute_sync_committee_period_at_slot(next_block.slot);

		debug!(target: debug_target, "Required {num_signatures} >= {min_signatures}, {signature_period} in [{state_period}, {}], {} > {}",  state_period + 1, block_finality_checkpoint.epoch, client_state.latest_finalized_epoch);

		if num_signatures >= min_signatures &&
			(state_period..=state_period + 1).contains(&signature_period) &&
			block_finality_checkpoint.epoch > client_state.latest_finalized_epoch
		{
			debug!(target: debug_target, "Found block: {}", next_block.slot);
			found = true;
		} else {
			if found {
				debug!(target: debug_target, "Stopped at block: {}", block.slot);
				break
			}
		}
		block = next_block.clone();
		next_block = prover.fetch_block(&(next_block.slot + 1).to_string()).await?;

		// block = parent_block;
	}

	let attested_block_id = get_block_id(block.parent_root);
	let attested_header = prover.fetch_header(&attested_block_id).await?;
	let mut attested_state =
		prover.fetch_beacon_state(&get_block_id(attested_header.state_root)).await?;
	if attested_state.finalized_checkpoint.root == Node::default() {
		debug!(target: debug_target, "No checkpoint root");
		return Ok(None)
	}
	debug!(target: debug_target, "Fetching update #1");

	let finalized_block_id = get_block_id(attested_state.finalized_checkpoint.root);
	let finalized_header = prover.fetch_header(&finalized_block_id).await?;
	let mut finalized_state =
		prover.fetch_beacon_state(&get_block_id(finalized_header.state_root)).await?;
	let finality_proof = FinalityProof {
		epoch: attested_state.finalized_checkpoint.epoch,
		finality_branch: prove_finalized_header(&mut attested_state)?,
	};
	debug!(target: debug_target, "Fetching update #2");

	let execution_payload_proof = prove_execution_payload(&mut finalized_state)?;

	let signature_period = compute_sync_committee_period_at_slot(block.slot);
	let client_state_next_sync_committee_root =
		client_state.next_sync_committee.hash_tree_root()?;
	let attested_state_current_sync_committee_root =
		attested_state.current_sync_committee.hash_tree_root()?;
	debug!(target: debug_target, "Fetching update #3");
	let sync_committee_update =
		// We must make sure we switch the sync comittee only when the finalized header has changed sync committees
		if should_have_sync_committee_update(state_period, signature_period) && client_state_next_sync_committee_root == attested_state_current_sync_committee_root {
			debug!(target: debug_target, "Fetching update #4");
			let sync_committee_proof = prove_sync_committee_update(&mut attested_state)?;
			Some(SyncCommitteeUpdate {
				next_sync_committee: attested_state.next_sync_committee,
				next_sync_committee_branch: sync_committee_proof,
			})
		} else {
			None
		};

	// construct light client
	let light_client_update = VerifierStateUpdate {
		attested_header,
		sync_committee_update,
		finalized_header,
		execution_payload: execution_payload_proof,
		finality_proof,
		sync_aggregate: block.body.sync_aggregate,
		signature_slot: block.slot,
	};

	Ok(Some(light_client_update))
}

#[cfg(not(feature = "no_beacon"))]
pub async fn prove(
	_client: &EthereumClient,
	sync_committee_prover: &SyncCommitteeProver,
	eth_client_state: &ClientState<HostFunctionsManager>,
	_block_number: u64,
	_up_to: u64,
) -> Result<Option<Header>, ClientError> {
	info!("waiting for the checkpoint");
	info!("state_period = {}", eth_client_state.inner.state_period);
	info!("state = {}", eth_client_state.inner.finalized_header.slot);

	let mut upd = None;

	let Ok(checkpoint) = sync_committee_prover.fetch_finalized_checkpoint(None).await else {
		info!("no checkpoint");
		return Err(ClientError::Other("No checkpoint".to_string()))
	};
	info!("checkpoint: {checkpoint:?}");
	if let Some(update) = fetch_light_client_update2(
		&sync_committee_prover,
		eth_client_state.inner.clone(),
		checkpoint.finalized,
		None, // Some(&eth_client_state.inner.finalized_header.slot.to_string()),
		"hyperspace_ethereum",
	)
	.await?
	{
		info!("got update");
		if update.execution_payload.block_number > eth_client_state.latest_height as u64 {
			upd = Some(update);
		} else {
			info!(
				"exec payload block number {} <= {}",
				update.execution_payload.block_number, eth_client_state.latest_height as u64
			);
			// return Err(ClientError::Other("No update 2".to_string()))
			return Ok(None)
		}
	} else {
		info!("no update 3");
		// return Err(ClientError::Other("No update 3".to_string()))
		return Ok(None)
	}
	if upd.is_none() {
		info!("no update 4");
		return Ok(None)
		// return Err(ClientError::Other("No update 4".to_string()))
	}

	let light_client_update = upd.unwrap();

	let latest_cp_client_height = eth_client_state.latest_height().revision_height;
	let from = latest_cp_client_height + 1;
	let to = light_client_update.execution_payload.block_number;
	if to < from {
		return Ok(None)
	}

	info!(target: "hyperspace_ethereum", "Getting blocks {}..{}", from, to);

	let update = &light_client_update;

	info!(target: "hyperspace_ethereum",
		"proven: state root = {}, body root = {}, slot = {}, block number = {}",
		update.finalized_header.state_root,
		update.finalized_header.body_root,
		update.finalized_header.slot,
		update.execution_payload.block_number
	);

	if update.execution_payload.block_number <= eth_client_state.latest_height().revision_height ||
		update.attested_header.slot <= eth_client_state.inner.finalized_header.slot ||
		update.finality_proof.epoch <= eth_client_state.inner.latest_finalized_epoch
	{
		info!(target: "hyperspace_ethereum", "no new events");
		return Ok(None)
	}

	Ok(Some(Header { inner: light_client_update }))
}

#[cfg(feature = "no_beacon")]
pub async fn prove(
	client: &EthereumClient,
	sync_committee_prover: &SyncCommitteeProver,
	eth_client_state: &ClientState<HostFunctionsManager>,
	_block_number: u64,
	up_to: u64,
) -> Result<Option<Header>, ClientError> {
	tokio::time::sleep(Duration::from_secs(15)).await;
	let client = client.client();
	let latest_block = client
		.get_block(BlockNumber::Latest)
		.await
		.map_err(|e| ClientError::Other(format!("failed to get latest block: {:?}", e)))?
		.expect("block not found");
	let execution_payload_proof = ExecutionPayloadProof {
		state_root: latest_block.state_root,
		block_number: latest_block.number.unwrap().as_u64(),
		multi_proof: vec![],
		execution_payload_branch: vec![],
		timestamp: latest_block.timestamp.as_u64(),
	};
	let mut light_client_update = LightClientUpdate {
		attested_header: BeaconBlockHeader {
			slot: eth_client_state.inner.finalized_header.slot + 1,
			..Default::default()
		},
		sync_committee_update: Default::default(),
		finalized_header: BeaconBlockHeader {
			slot: latest_block.number.unwrap().as_u64(),
			proposer_index: 0,
			parent_root: Node(latest_block.parent_hash.0),
			state_root: Node(latest_block.state_root.0),
			body_root: Node::default(),
		},
		execution_payload: execution_payload_proof,
		finality_proof: FinalityProof {
			epoch: eth_client_state.inner.latest_finalized_epoch + 1,
			..Default::default()
		},
		sync_aggregate: Default::default(),
		signature_slot: Default::default(),
	};
	light_client_update.attested_header.slot = latest_block.number.unwrap().as_u64();

	Ok(Some(Header { inner: light_client_update }))
}
