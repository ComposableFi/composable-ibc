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

use crate::verify_parachain_headers_with_grandpa_finality_proof;
use codec::{Decode, Encode};
use futures::StreamExt;
use grandpa_prover::{host_functions::HostFunctionsProvider, GrandpaProver};
use polkadot_core_primitives::Header;
use primitives::{justification::GrandpaJustification, ParachainHeadersWithFinalityProof};
use serde::{Deserialize, Serialize};
use subxt::{
	config::substrate::{BlakeTwo256, SubstrateHeader},
	rpc_params, PolkadotConfig,
};

pub type Justification = GrandpaJustification<Header>;

/// An encoded justification proving that the given header has been finalized
#[derive(Clone, Serialize, Deserialize)]
pub struct JustificationNotification(sp_core::Bytes);

#[tokio::test]
async fn follow_grandpa_justifications() {
	env_logger::builder()
		.filter_module("grandpa", log::LevelFilter::Trace)
		.format_module_path(false)
		.init();

	let relay = std::env::var("RELAY_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
	let para = std::env::var("PARA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

	let relay_ws_url = format!("ws://{relay}:9944");
	let para_ws_url = format!("ws://{para}:9188");

	let prover = GrandpaProver::<PolkadotConfig>::new(&relay_ws_url, &para_ws_url, 2000)
		.await
		.unwrap();

	println!("Waiting for grandpa proofs to become available");
	let session_length = prover.session_length().await.unwrap();
	prover
		.relay_client
		.blocks()
		.subscribe_finalized()
		.await
		.unwrap()
		.filter_map(|result| futures::future::ready(result.ok()))
		.skip_while(|h| futures::future::ready(h.number() < (session_length * 2) + 10))
		.take(1)
		.collect::<Vec<_>>()
		.await;

	let mut subscription = prover
		.relay_client
		.rpc()
		.subscribe::<JustificationNotification>(
			"grandpa_subscribeJustifications",
			rpc_params![],
			"grandpa_unsubscribeJustifications",
		)
		.await
		.unwrap()
		.take(100);

	let mut client_state = prover.initialize_client_state().await.unwrap();
	println!("Grandpa proofs are now available");
	while let Some(Ok(JustificationNotification(sp_core::Bytes(justification)))) =
		subscription.next().await
	{
		let justification =
			Justification::decode(&mut &justification[..]).expect("Failed to decode justification");

		let finalized_para_header = prover
			.query_latest_finalized_parachain_header(justification.commit.target_number)
			.await
			.expect("Failed to fetch finalized parachain headers");

		// notice the inclusive range
		let header_numbers = ((client_state.latest_para_height + 1)..=finalized_para_header.number)
			.collect::<Vec<_>>();

		if header_numbers.len() == 0 {
			continue
		}

		println!("current_set_id: {}", client_state.current_set_id);
		println!("latest_relay_height: {}", client_state.latest_relay_height);
		println!(
			"For relay chain header: Hash({:?}), Number({})",
			justification.commit.target_hash, justification.commit.target_number
		);

		dbg!(&client_state.latest_para_height);
		dbg!(&header_numbers);

		let proof = prover
			.query_finalized_parachain_headers_with_proof::<SubstrateHeader<u32, BlakeTwo256>>(
				&client_state,
				justification.commit.target_number,
				header_numbers,
			)
			.await
			.expect("Failed to fetch finalized parachain headers with proof");

		let proof = proof.encode();
		let proof = ParachainHeadersWithFinalityProof::<Header>::decode(&mut &*proof).unwrap();

		let new_client_state = verify_parachain_headers_with_grandpa_finality_proof::<
			Header,
			HostFunctionsProvider,
		>(client_state.clone(), proof.clone())
		.expect("Failed to verify parachain headers with grandpa finality_proof");

		if !proof.parachain_headers.is_empty() {
			assert!(new_client_state.latest_para_height > client_state.latest_para_height);
		}

		client_state = new_client_state;
		println!("========= Successfully verified grandpa justification =========");
	}
}
