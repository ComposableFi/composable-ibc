use super::*;
// use base2::Base2;
use ethereum_consensus::{
	bellatrix::compute_domain, primitives::Root, signing::compute_signing_root,
	state_transition::Context,
};
use ssz_rs::{calculate_multi_merkle_root, is_valid_merkle_branch, GeneralizedIndex, Merkleized};
use std::time::Duration;
use sync_committee_primitives::{
	types::{
		AncestorBlock, FinalityProof, LightClientState, LightClientUpdate, SyncCommitteeUpdate,
		DOMAIN_SYNC_COMMITTEE, GENESIS_VALIDATORS_ROOT,
	},
	util::{compute_fork_version, compute_sync_committee_period_at_slot},
};
use sync_committee_prover::SyncCommitteeProver;
use sync_committee_verifier::verify_sync_committee_attestation;
use tokio::time;
// use tokio_stream::{wrappers::IntervalStream, StreamExt};

const NODE_URL: &'static str = "http://127.0.0.1:3500";

#[cfg(test)]
#[actix_rt::test]
async fn test_prover() {
	env_logger::init();
	// let mut stream = IntervalStream::new(time::interval(Duration::from_secs(12 * 12)));

	let sync_committee_prover = SyncCommitteeProver::new(NODE_URL.to_string());
	let block_id = "head";
	let block_header = sync_committee_prover.fetch_header(&block_id).await.unwrap();
	// println!("block_header: {:?}", block_header);
	let state = sync_committee_prover
		.fetch_beacon_state(&block_header.slot.to_string())
		.await
		.unwrap();
	// sync_committee_prover.
	// println!("state: {:?}", state);
	// println!("state: {:?}", state.validators);
}
