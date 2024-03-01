use crate::{
	chains::chains::Chain,
	configs::indexer_config::EVMIndexerConfig,
	db::{
		db::Database,
		models::models::{
			DatabaseBlock, DatabaseChainIndexedState, DatabaseContract, DatabaseIBCEventData,
			DatabaseLog, DatabaseReceipt, DatabaseTransaction,
		},
	},
	rpc::rpc::Rpc,
};
use ethabi::ethereum_types::Address;
use futures::future::join_all;
use log::{debug, error, info};
use std::collections::HashSet;

pub async fn sync_chain(
	rpc: &Rpc,
	db: &Database,
	config: &EVMIndexerConfig,
	indexed_blocks: &mut HashSet<i64>,
	from_block: i64,
) {
	let db_state = DatabaseChainIndexedState {
		chain: config.chain.name.to_string(),
		indexed_blocks_amount: indexed_blocks.len() as i64,
	};

	db.update_indexed_blocks_number(&db_state).await.unwrap();

	let last_block = rpc.get_last_block().await.unwrap();

	let full_block_range = HashSet::<i64>::from_iter(from_block..last_block);

	let mut missing_blocks: Vec<i64> = (&full_block_range - &indexed_blocks).into_iter().collect();
	missing_blocks.sort();

	// Re-fetch previous `config.block_confirmation_length` blocks also because of a potential
	// chain reorganization
	let min_missing = *missing_blocks.first().unwrap_or(&0);
	let new_min_missing =
		min_missing.saturating_sub(config.block_confirmation_length as i64).max(0);
	if new_min_missing != min_missing {
		for n in (new_min_missing..min_missing).rev() {
			missing_blocks.insert(0, n);
		}
	}

	let total_missing_blocks = missing_blocks.len();

	if total_missing_blocks == 0 {
		debug!("Chain is synced");
		return
	}

	let max_missing = *missing_blocks.last().unwrap_or(&0);

	info!("Syncing {} blocks {new_min_missing}..{max_missing}.", total_missing_blocks);

	let missing_blocks_chunks = missing_blocks.chunks(config.batch_size);

	for missing_blocks_chunk in missing_blocks_chunks {
		let mut work = vec![];
		let mut work2 = vec![];

		for block_number in missing_blocks_chunk {
			work2.push(fetch_block(&rpc, &block_number, &config.chain))
		}

		work.push(fetch_ibc_events(&rpc, &missing_blocks_chunk, &config.contract_addresses));

		let results = join_all(work).await;
		let results2 = join_all(work2).await;

		let mut db_blocks: Vec<DatabaseBlock> = Vec::new();
		let db_transactions: Vec<DatabaseTransaction> = Vec::new();
		let db_receipts: Vec<DatabaseReceipt> = Vec::new();
		let db_logs: Vec<DatabaseLog> = Vec::new();
		let db_contracts: Vec<DatabaseContract> = Vec::new();
		let mut db_ibc_events: Vec<DatabaseIBCEventData> = Vec::new();

		for result in results {
			match result {
				Some(
					// block,
					// mut transactions,
					// mut receipts,
					// mut logs,
					// mut contracts,
					mut ibc_events,
				) => {
					// db_blocks.push(block);
					// db_transactions.append(&mut transactions);
					// db_receipts.append(&mut receipts);
					// db_logs.append(&mut logs);
					// db_contracts.append(&mut contracts);
					db_ibc_events.append(&mut ibc_events);
				},
				None => continue,
			}
		}

		for result in results2 {
			match result {
				Some((block, _transactions, _receipts, _logs, _contracts, _ibc_events)) => {
					db_blocks.push(block);
					// db_transactions.append(&mut transactions);
					// db_receipts.append(&mut receipts);
					// db_logs.append(&mut logs);
					// db_contracts.append(&mut contracts);
					// db_ibc_events.append(&mut ibc_events);
				},
				None => continue,
			}
		}

		db.store_data(
			&db_blocks,
			&db_transactions,
			&db_receipts,
			&db_logs,
			&db_contracts,
			&db_ibc_events,
		)
		.await;

		// for number in missing_blocks_chunk.into_iter() {
		// 	indexed_blocks.insert(*number);
		// }
		for block in db_blocks.into_iter() {
			indexed_blocks.insert(block.number);
		}

		let indexed_blocks_vector: Vec<i64> = indexed_blocks.clone().into_iter().collect();

		db.store_indexed_blocks(&indexed_blocks_vector).await.unwrap();
	}
}

async fn fetch_ibc_events(
	rpc: &Rpc,
	block_numbers: &[i64],
	contract_addresses: &[Address],
) -> Option<Vec<DatabaseIBCEventData>> {
	//  `from_tos` is `block_numbers` shrinked to pairs of `(from_block, to_block)`
	// e.g. block numbers = [1, 2, 3, 5, 6, 7, 8, 10], from_tos = [(1,3), (5,8), (10, 10)]
	let from_tos: Vec<(i64, i64)> = block_numbers.iter().fold(vec![], |mut acc, block_number| {
		if acc.is_empty() {
			acc.push((*block_number, *block_number));
			return acc
		}

		let last_from_to = acc.last_mut().unwrap();

		if last_from_to.1 + 1 == *block_number {
			last_from_to.1 = *block_number;
		} else {
			acc.push((*block_number, *block_number));
		}

		acc
	});
	// dbg!(&from_tos);
	let mut events = vec![];
	for (from, to) in from_tos {
		let result = rpc.get_ibc_logs(from, to, contract_addresses).await;
		match result {
			Ok(Some(r)) => events.extend(r),
			Err(e) => {
				error!("Error fetching IBC events: {:?}", e);
				return None
			},
			_ => {},
		}
	}
	// match receipts_data {
	// 	Some((mut receipts, mut logs, mut contracts, mut ibc_events)) => {
	// 		db_receipts.append(&mut receipts);
	// 		db_logs.append(&mut logs);
	// 		db_contracts.append(&mut contracts);
	// 		db_ibc_events.append(&mut ibc_events);
	// 	},
	// 	None => return None,
	// }
	return Some(events)
}

async fn fetch_block(
	rpc: &Rpc,
	block_number: &i64,
	_chain: &Chain,
) -> Option<(
	DatabaseBlock,
	Vec<DatabaseTransaction>,
	Vec<DatabaseReceipt>,
	Vec<DatabaseLog>,
	Vec<DatabaseContract>,
	Vec<DatabaseIBCEventData>,
)> {
	let block_data = rpc.get_block(block_number).await.unwrap();

	match block_data {
		Some((db_block, db_transactions)) => {
			let _total_block_transactions = db_transactions.len();

			// Make sure all the transactions are correctly formatted.
			// if db_block.transactions != total_block_transactions as i64 {
			// 	warn!(
			// 		"Missing {} transactions for block {}.",
			// 		db_block.transactions - total_block_transactions as i64,
			// 		db_block.number
			// 	);
			// 	return None
			// }

			let db_receipts: Vec<DatabaseReceipt> = Vec::new();
			let db_logs: Vec<DatabaseLog> = Vec::new();
			let db_contracts: Vec<DatabaseContract> = Vec::new();
			let db_ibc_events: Vec<DatabaseIBCEventData> = Vec::new();

			// if chain.supports_blocks_receipts {
			// 	let receipts_data = rpc.get_block_receipts(block_number).await.unwrap();
			// 	match receipts_data {
			// 		Some((mut receipts, mut logs, mut contracts, mut ibc_events)) => {
			// 			db_receipts.append(&mut receipts);
			// 			db_logs.append(&mut logs);
			// 			db_contracts.append(&mut contracts);
			// 			db_ibc_events.append(&mut ibc_events);
			// 		},
			// 		None => return None,
			// 	}
			// } else {
			// 	for transaction in db_transactions.iter_mut() {
			// 		let receipt_data =
			// 			rpc.get_transaction_receipt(transaction.hash.clone()).await.unwrap();
			//
			// 		match receipt_data {
			// 			Some((receipt, mut logs, contract)) => {
			// 				db_receipts.push(receipt);
			// 				db_logs.append(&mut logs);
			// 				match contract {
			// 					Some(contract) => db_contracts.push(contract),
			// 					None => continue,
			// 				}
			// 			},
			// 			None => continue,
			// 		}
			// 	}
			// }

			// if total_block_transactions != db_receipts.len() {
			// 	warn!(
			// 		"Missing receipts for block {}. Transactions {} receipts {}",
			// 		db_block.number,
			// 		total_block_transactions,
			// 		db_receipts.len()
			// 	);
			// 	return None
			// }

			// info!(
			// 	"Found transactions {} receipts {} logs {} contracts {} ibc events: {} for block
			// {}.", 	total_block_transactions,
			// 	db_receipts.len(),
			// 	db_logs.len(),
			// 	db_contracts.len(),
			// 	db_ibc_events.len(),
			// 	block_number
			// );

			return Some((
				db_block,
				db_transactions,
				db_receipts,
				db_logs,
				db_contracts,
				db_ibc_events,
			))
		},
		None => return None,
	}
}
