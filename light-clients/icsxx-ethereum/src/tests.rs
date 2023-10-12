use sync_committee_prover::SyncCommitteeProver;

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
	let _state = sync_committee_prover
		.fetch_beacon_state(&block_header.slot.to_string())
		.await
		.unwrap();
}
