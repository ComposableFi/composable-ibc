// use base2::Base2;
use crate::client::{ClientError, EthereumClient};
use anyhow::anyhow;
use ethereum_consensus::{
	altair::mainnet::SYNC_COMMITTEE_SIZE,
	bellatrix::compute_domain,
	phase0::mainnet::SLOTS_PER_EPOCH,
	primitives::{Bytes32, Root},
	signing::compute_signing_root,
	state_transition::Context,
};
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
	types::{
		AncestorBlock, AncestryProof, BlockRootsProof, ExecutionPayloadProof, FinalityProof,
		LightClientState, LightClientUpdate, SyncCommitteeUpdate, DOMAIN_SYNC_COMMITTEE,
		GENESIS_VALIDATORS_ROOT,
	},
	util::{compute_fork_version, compute_sync_committee_period_at_slot},
};
use sync_committee_prover::{
	prove_block_roots_proof, prove_execution_payload, prove_finalized_header,
	prove_sync_committee_update, SyncCommitteeProver,
};
use sync_committee_verifier::{verify_sync_committee_attestation, SignatureVerifier};
use tokio::{task::JoinSet, time, time::sleep};
// use tokio_stream::{wrappers::IntervalStream, StreamExt};

pub async fn prove(
	client: &EthereumClient,
	eth_client_state: &ClientState<HostFunctionsManager>,
	block_number: u64,
) -> Result<LightClientUpdate<SYNC_COMMITTEE_SIZE>, ClientError> {
	let sync_committee_prover = client.prover();
	let client_state = &eth_client_state.inner;

	let block_id = format!("{block_number:?}");
	let block_id = "head";

	let finality_checkpoint = sync_committee_prover.fetch_finalized_checkpoint().await?;
	if finality_checkpoint.finalized.root == Node::default() ||
		finality_checkpoint.finalized.epoch <= client_state.latest_finalized_epoch ||
		finality_checkpoint.finalized.root ==
			client_state.finalized_header.clone().hash_tree_root()?
	{
		return Err(ClientError::Other("No new finalized checkpoint found".into()))
	}

	log::debug!(target: "hyperspace_ethereum", "A new epoch has been finalized {}", finality_checkpoint.finalized.epoch);

	let block_id = {
		let mut block_id = hex::encode(finality_checkpoint.finalized.root.as_bytes());
		block_id.insert_str(0, "0x");
		block_id
	};

	let finalized_header = sync_committee_prover.fetch_header(&block_id).await?;
	let finalized_state = sync_committee_prover
		.fetch_beacon_state(finalized_header.slot.to_string().as_str())
		.await?;
	let execution_payload_proof = prove_execution_payload(finalized_state.clone())?;

	let mut attested_epoch = finality_checkpoint.finalized.epoch + 2;
	// Get attested header and the signature slot

	let mut attested_slot = attested_epoch * SLOTS_PER_EPOCH;
	// Due to the fact that all slots in an epoch can be missed we are going to try and fetch
	// the attested block from four possible epochs.
	let mut attested_epoch_loop_count = 0;
	let (attested_block_header, signature_block) = loop {
		if attested_epoch_loop_count >= 4 {
			panic!("Could not fetch any block from the attested epoch after going through four epochs, your Eth devnet is fucked")
		}
		// If we have maxed out the slots in the current epoch and still didn't find any block,
		// we move to the next epoch
		if (attested_epoch * SLOTS_PER_EPOCH).saturating_add(SLOTS_PER_EPOCH - 1) == attested_slot {
			// No block was found in attested epoch we move to the next possible attested epoch
			log::debug!(target: "hyperspace_ethereum",
				"No slots found in epoch {attested_epoch} Moving to the next possible epoch {}",
				attested_epoch + 1
			);
			std::thread::sleep(Duration::from_secs(24));
			attested_epoch += 1;
			attested_slot = attested_epoch * SLOTS_PER_EPOCH;
			attested_epoch_loop_count += 1;
		}

		if let Ok(header) =
			sync_committee_prover.fetch_header(attested_slot.to_string().as_str()).await
		{
			let mut signature_slot = header.slot + 1;
			let mut loop_count = 0;
			let signature_block = loop {
				if loop_count == 2 {
					break None
				}
				if (attested_epoch * SLOTS_PER_EPOCH).saturating_add(SLOTS_PER_EPOCH - 1) ==
					signature_slot
				{
					log::debug!(target: "hyperspace_ethereum", "Waiting for signature block for attested header");
					std::thread::sleep(Duration::from_secs(24));
					signature_slot = header.slot + 1;
					loop_count += 1;
				}
				if let Ok(signature_block) =
					sync_committee_prover.fetch_block(signature_slot.to_string().as_str()).await
				{
					break Some(signature_block)
				}
				signature_slot += 1;
			};
			// If the next block does not have sufficient sync committee participants
			if let Some(signature_block) = signature_block {
				if signature_block
					.body
					.sync_aggregate
					.sync_committee_bits
					.as_bitslice()
					.count_ones() < (2 * SYNC_COMMITTEE_SIZE) / 3
				{
					attested_slot += 1;
					log::debug!(target: "hyperspace_ethereum", "Signature block does not have sufficient sync committee participants -> participants {}", signature_block.body.sync_aggregate.sync_committee_bits.as_bitslice().count_ones());
					continue
				}
				break (header, signature_block)
			} else {
				log::debug!(target: "hyperspace_ethereum", "No signature block found in {attested_epoch} Moving to the next possible epoch {}", attested_epoch + 1);
				std::thread::sleep(Duration::from_secs(24));
				attested_epoch += 1;
				attested_slot = attested_epoch * SLOTS_PER_EPOCH;
				attested_epoch_loop_count += 1;
				continue
			}
		}
		attested_slot += 1
	};

	let attested_state = sync_committee_prover
		.fetch_beacon_state(attested_block_header.slot.to_string().as_str())
		.await?;

	let finalized_hash_tree_root = finalized_header.clone().hash_tree_root()?;
	log::debug!(target: "hyperspace_ethereum", "{:?}, {}", attested_state.finalized_checkpoint, attested_state.slot);
	log::debug!(target: "hyperspace_ethereum", "{:?}, {}", finalized_hash_tree_root, finalized_header.slot);

	assert_eq!(finalized_hash_tree_root, attested_state.finalized_checkpoint.root);

	let finality_proof = FinalityProof {
		epoch: finality_checkpoint.finalized.epoch,
		finality_branch: prove_finalized_header(attested_state.clone())?,
	};

	let state_period = compute_sync_committee_period_at_slot(finalized_header.slot);

	let update_attested_period = compute_sync_committee_period_at_slot(attested_block_header.slot);

	let sync_committee_update = if state_period == update_attested_period {
		let sync_committee_proof = prove_sync_committee_update(attested_state.clone())?;

		let sync_committee_proof = sync_committee_proof
			.into_iter()
			.map(|node| Bytes32::try_from(node.as_bytes()).expect("Node is always 32 byte slice"))
			.collect::<Vec<_>>();

		Some(SyncCommitteeUpdate {
			next_sync_committee: attested_state.next_sync_committee,
			next_sync_committee_branch: sync_committee_proof,
		})
	} else {
		None
	};

	let mut i = finalized_header.slot - 1;
	let mut ancestor_blocks = vec![];
	while ancestor_blocks.len() < 5 {
		if (finalized_header.slot - i) > 100 {
			break
		}
		if let Ok(ancestor_header) =
			sync_committee_prover.fetch_header(i.to_string().as_str()).await
		{
			let ancestry_proof =
				prove_block_roots_proof(finalized_state.clone(), ancestor_header.clone())?;
			let header_state =
				sync_committee_prover.fetch_beacon_state(i.to_string().as_str()).await?;
			let execution_payload_proof = prove_execution_payload(header_state)?;
			ancestor_blocks.push(AncestorBlock {
				header: ancestor_header,
				execution_payload: execution_payload_proof,
				ancestry_proof,
			})
		}
		i -= 1;
	}

	log::debug!(target: "hyperspace_ethereum", "\nAncestor blocks count: \n {:?} \n", ancestor_blocks.len());

	// construct light client
	let light_client_update = LightClientUpdate {
		attested_header: attested_block_header,
		sync_committee_update,
		finalized_header,
		execution_payload: execution_payload_proof,
		finality_proof,
		sync_aggregate: signature_block.body.sync_aggregate,
		signature_slot: signature_block.slot,
		ancestor_blocks: vec![], // TODO: add ancestor blocks?
	};

	// client_state = verify_sync_committee_attestation::<SignatureVerifier>(
	// 	client_state.clone(),
	// 	light_client_update,
	// )
	// .unwrap();
	// println!(
	// 	"Sucessfully verified Ethereum block at slot {:?}",
	// 	client_state.finalized_header.slot
	// );
	Ok(light_client_update)
}

pub async fn prove_fast(
	client: &EthereumClient,
	eth_client_state: &ClientState<HostFunctionsManager>,
	block_number: u64,
) -> Result<LightClientUpdate<SYNC_COMMITTEE_SIZE>, ClientError> {
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
	for heights in range.chunks(100) {
		for i in heights.iter().copied() {
			let duration = Duration::from_millis(rand::thread_rng().gen_range(1..delay) as u64);
			let sync_committee_prover = sync_committee_prover.clone();
			join_set.spawn(async move {
				sleep(duration).await;

				if let Ok(ancestor_header) =
					sync_committee_prover.fetch_header(i.to_string().as_str()).await
				{
					// let ancestry_proof =
					// 	prove_block_roots_proof(finalized_state.clone(), ancestor_header.clone())?;
					let header_state =
						sync_committee_prover.fetch_beacon_state(i.to_string().as_str()).await?;
					let execution_payload_proof = prove_execution_payload(header_state)?;
					return Ok(AncestorBlock {
						header: ancestor_header,
						execution_payload: execution_payload_proof,
						ancestry_proof: AncestryProof::BlockRoots {
							block_roots_proof: BlockRootsProof {
								block_header_index: 0,
								block_header_branch: vec![],
							},
							block_roots_branch: vec![],
						},
					})
				}
				Err(anyhow::anyhow!("Could not fetch ancestor block"))
			});
		}
		while let Some(res) = join_set.join_next().await {
			let out = res.map_err(|e| anyhow!("{e}"))??;
			ancestor_blocks.push(out);
		}
	}
	ancestor_blocks.sort_by_key(|ancestor_block| ancestor_block.header.slot);

	let ep = block.body.execution_payload;
	let execution_payload_proof = ExecutionPayloadProof {
		state_root: ep.state_root,
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
		ancestor_blocks,
	};
	light_client_update.attested_header.slot = block_header.slot;
	light_client_update.finalized_header.slot = block_header.slot;

	Ok(light_client_update)
}
