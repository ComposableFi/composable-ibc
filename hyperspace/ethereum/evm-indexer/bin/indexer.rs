use dotenv::dotenv;
use evm_indexer::{
	configs::indexer_config::EVMIndexerConfig, db::db::Database, indexer, rpc::rpc::Rpc,
};
use log::*;
use simple_logger::SimpleLogger;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main()]
async fn main() {
	dotenv().ok();

	let log = SimpleLogger::new().with_level(LevelFilter::Info);

	let mut config = EVMIndexerConfig::new();

	if config.debug {
		log.with_level(LevelFilter::Debug).init().unwrap();
	} else {
		log.init().unwrap();
	}

	info!("Starting EVM Indexer.");

	if !config.reset {
		info!("Syncing chain {}.", config.chain.name.clone());
	}

	let rpc = Rpc::new(&config).await.expect("Unable to start RPC client.");

	let db = Database::new(config.db_url.clone(), config.redis_url.clone(), config.chain.clone())
		.await
		.expect("Unable to start DB connection.");

	if config.recalc_blocks_indexer {
		info!("Updating state of indexed blocks.");
		db.update_indexed_blocks().await.unwrap();
	}

	if !config.reset {
		let mut indexed_blocks = db.get_indexed_blocks().await.unwrap();

		let from_block = match config.start_block {
			Some(n) => n,
			None => rpc.get_last_block().await.unwrap(),
		};
		loop {
			indexer::sync_chain(&rpc, &db, &mut config, &mut indexed_blocks, from_block).await;
			sleep(Duration::from_millis(50)).await;
		}
	} else {
		db.delete_indexed_blocks().await.unwrap();
	}
}
