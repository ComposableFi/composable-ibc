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
use log::{error, info};
use pallet_ibc::light_clients::HostFunctionsManager;
use primitives::mock::LocalClientTypes;
use ssz_rs::{
	calculate_multi_merkle_root, is_valid_merkle_branch, GeneralizedIndex, Merkleized, Node,
};
use std::time::Duration;
use sync_committee_primitives::{
	consensus_types::BeaconBlockHeader,
	constants::Root,
	types::{ExecutionPayloadProof, FinalityProof, VerifierStateUpdate as LightClientUpdate},
};
use sync_committee_prover::{prove_execution_payload, SyncCommitteeProver};
use tokio::{task::JoinSet, time, time::sleep};

#[cfg(not(feature = "no_beacon"))]
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

#[cfg(not(feature = "no_beacon"))]
pub async fn prove(
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
	if let Some(update) = sync_committee_prover
		.fetch_light_client_update(
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
pub async fn prove_fast(
	client: &EthereumClient,
	eth_client_state: &ClientState<HostFunctionsManager>,
	_block_number: u64,
	up_to: u64,
) -> Result<Option<Header>, ClientError> {
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
