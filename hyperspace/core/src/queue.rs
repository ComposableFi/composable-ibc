// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ibc_proto::google::protobuf::Any;
use metrics::handler::MetricsHandler;
use primitives::Chain;
use sha2::Digest;

const DEBUG_MESSAGES: bool = true;

/// This sends messages to the sink chain in a gas-aware manner.
pub async fn flush_message_batch(
	msgs: Vec<Any>,
	metrics: Option<&MetricsHandler>,
	sink: &impl Chain,
) -> Result<(), anyhow::Error> {
	let block_max_weight = sink.block_max_weight();
	let batch_weight = sink.estimate_weight(msgs.clone()).await?;

	if let Some(metrics) = metrics {
		metrics.handle_transaction_costs(batch_weight, &msgs).await;
	}

	if DEBUG_MESSAGES {
		let name = sink.name();
		for msg in &msgs {
			let content = format!("{}\n{}", msg.type_url, hex::encode(&msg.value));
			let mut hasher = sha2::Sha256::default();
			hasher.update(content.as_bytes());
			let hash = hex::encode(&hasher.finalize());
			let f_name = format!("messages/{}_{}_{hash}.txt", name, msg.type_url);
			log::debug!(target: "hyperspace", "Writing message to file: {}", f_name);
			if let Err(e) = std::fs::write(f_name, content) {
				log::error!(target: "hyperspace", "Failed to write message to file: {:?}", e);
			}
		}
	}

	log::debug!(target: "hyperspace", "Outgoing messages weight: {} block max weight: {}", batch_weight, block_max_weight);
	let ratio = (batch_weight / block_max_weight) as usize;
	if ratio == 0 {
		sink.submit(msgs).await?;
		return Ok(())
	}

	// whelp our batch exceeds the block max weight.
	let chunk = if ratio == 1 {
		// split the batch into ratio * 2
		ratio * 2
	} else {
		// split the batch into ratio + 2
		ratio + 2
	};

	log::info!(
		"Outgoing messages weight: {} exceeds the block max weight: {}. Chunking {} messages into {} chunks",
        batch_weight, block_max_weight, msgs.len(), chunk,
	);
	let chunk_size = (msgs.len() / chunk).max(1).min(4);
	// TODO: return number of failed messages and record it to metrics
	for batch in msgs.chunks(chunk_size) {
		// send out batches.

		if let Err(e) = sink.submit(batch.to_vec()).await {
			log::error!(target: "hyperspace", "Failed to submit batch: {:?}", e);
		}
	}

	Ok(())
}
