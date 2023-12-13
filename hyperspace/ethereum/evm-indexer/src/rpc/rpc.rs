use crate::{
	chains::chains::Chain,
	configs::indexer_config::EVMIndexerConfig,
	db::models::models::{
		DatabaseBlock, DatabaseContract, DatabaseIBCEventData, DatabaseLog, DatabaseReceipt,
		DatabaseTransaction,
	},
	utils::format_small_number,
};
use anyhow::Result;
use ethabi::Address;
use ethers::{
	abi::RawLog,
	contract::EthEvent,
	prelude::Log,
	types::{Block, Transaction, TransactionReceipt, U256},
};
use jsonrpsee::core::{client::ClientT, rpc_params};
use jsonrpsee_http_client::{HttpClient, HttpClientBuilder};
use log::{debug, info, warn};
use rand::seq::SliceRandom;
use serde_json::{json, Error};
use sqlx::types::Json;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Rpc {
	pub clients: Vec<HttpClient>,
	pub chain: Chain,
}

impl Rpc {
	pub async fn new(config: &EVMIndexerConfig) -> Result<Self> {
		info!("Starting EVM rpc service");

		let timeout = Duration::from_secs(5);

		let mut clients = Vec::new();

		for rpc in config.rpcs.clone() {
			let client = HttpClientBuilder::default()
				.max_concurrent_requests(100000)
				.request_timeout(timeout)
				.build(rpc)
				.unwrap();

			let client_id = client.request("eth_chainId", rpc_params![]).await;

			match client_id {
				Ok(value) => {
					let chain_id: U256 = match serde_json::from_value(value) {
						Ok(value) => value,
						Err(_) => continue,
					};

					if chain_id.as_u64() as i64 != config.chain.id {
						warn!(
							"RPC client chain id does not match the configured chain id: {} != {}",
							chain_id.as_u64() as i64,
							config.chain.id
						);
						continue
					}

					clients.push(client);
				},
				Err(_) => continue,
			}
		}

		if clients.len() == 0 {
			panic!("No valid RPC client found");
		}

		Ok(Self { clients, chain: config.chain })
	}

	pub async fn get_last_block(&self) -> Result<i64> {
		let client = self.get_client();

		let last_block = client.request("eth_blockNumber", rpc_params![]).await;

		match last_block {
			Ok(value) => {
				let block_number: U256 = serde_json::from_value(value)
					.expect("Unable to deserialize eth_blockNumber response");

				Ok(block_number.as_u64() as i64)
			},
			Err(_) => Ok(0),
		}
	}

	pub async fn get_block(
		&self,
		block_number: &i64,
	) -> Result<Option<(DatabaseBlock, Vec<DatabaseTransaction>)>> {
		let client = self.get_client();

		let raw_block = client
			.request("eth_getBlockByNumber", rpc_params![format!("0x{:x}", block_number), true])
			.await;

		match raw_block {
			Ok(value) => {
				let block: Result<Block<Transaction>, Error> = serde_json::from_value(value);

				match block {
					Ok(block) => {
						let db_block = DatabaseBlock::from_rpc(&block, self.chain.name);

						let mut db_transactions = Vec::new();

						for transaction in block.transactions {
							let db_transaction = DatabaseTransaction::from_rpc(
								transaction,
								self.chain.name,
								db_block.timestamp.clone(),
							);

							db_transactions.push(db_transaction)
						}

						Ok(Some((db_block, db_transactions)))
					},
					Err(_) => Ok(None),
				}
			},
			Err(_) => Ok(None),
		}
	}

	pub async fn get_transaction_receipt(
		&self,
		transaction: String,
	) -> Result<Option<(DatabaseReceipt, Vec<DatabaseLog>, Option<DatabaseContract>)>> {
		let client = self.get_client();

		let raw_receipt =
			client.request("eth_getTransactionReceipt", rpc_params![transaction]).await;

		match raw_receipt {
			Ok(value) => {
				let receipt: Result<TransactionReceipt, Error> = serde_json::from_value(value);

				match receipt {
					Ok(receipt) => {
						let db_receipt = DatabaseReceipt::from_rpc(&receipt);

						let mut db_transaction_logs: Vec<DatabaseLog> = Vec::new();

						let status: String = match receipt.status {
							None => String::from("-1"),
							Some(status) => format_small_number(status),
						};

						let mut db_contract: Option<DatabaseContract> = None;

						if status == "1" {
							db_contract = match receipt.contract_address {
								Some(_) => Some(DatabaseContract::from_rpc(
									receipt.clone(),
									self.chain.name,
								)),
								None => None,
							};
						}

						for log in receipt.logs {
							let db_log = DatabaseLog::from_rpc(log, self.chain.name.to_owned());

							db_transaction_logs.push(db_log)
						}

						return Ok(Some((db_receipt, db_transaction_logs, db_contract)))
					},
					Err(_) => return Ok(None),
				}
			},
			Err(_) => return Ok(None),
		}
	}

	pub async fn get_block_receipts(
		&self,
		block_number: &i64,
	) -> Result<
		Option<(
			Vec<DatabaseReceipt>,
			Vec<DatabaseLog>,
			Vec<DatabaseContract>,
			Vec<DatabaseIBCEventData>,
		)>,
	> {
		let client = self.get_client();

		let raw_receipts = client
			.request("eth_getBlockReceipts", rpc_params![format!("0x{:x}", block_number)])
			.await;

		match raw_receipts {
			Ok(value) => {
				let receipts: Result<Vec<TransactionReceipt>, Error> =
					serde_json::from_value(value);

				match receipts {
					Ok(receipts) => {
						let mut db_receipts: Vec<DatabaseReceipt> = Vec::new();

						let mut db_transaction_logs: Vec<DatabaseLog> = Vec::new();

						let mut db_contracts: Vec<DatabaseContract> = Vec::new();

						let db_ibc_events: Vec<DatabaseIBCEventData> = Vec::new();

						for receipt in receipts {
							let db_receipt = DatabaseReceipt::from_rpc(&receipt);

							db_receipts.push(db_receipt);

							let db_contract = match receipt.contract_address {
								Some(_) => Some(DatabaseContract::from_rpc(
									receipt.clone(),
									self.chain.name,
								)),
								None => None,
							};

							if db_contract.is_some() {
								db_contracts.push(db_contract.unwrap())
							}

							for log in receipt.logs {
								let db_log =
									DatabaseLog::from_rpc(log.clone(), self.chain.name.to_owned());
								if let Ok(Some(_s)) = parse_log(log.clone()) {
									// db_ibc_events.push(DatabaseIBCEventData {
									// 	block_number: *block_number,
									// 	event_data: serde_json::from_str(s.as_str()).unwrap(),
									// })
								}

								db_transaction_logs.push(db_log)
							}
						}

						return Ok(Some((
							db_receipts,
							db_transaction_logs,
							db_contracts,
							db_ibc_events,
						)))
					},
					Err(_) => return Ok(None),
				}
			},
			Err(_) => return Ok(None),
		}
	}

	pub async fn get_ibc_logs(
		&self,
		from_block: i64,
		to_block: i64,
		contract_addresses: &[Address],
	) -> Result<Option<Vec<DatabaseIBCEventData>>> {
		let client = self.get_client();
		debug!("get_ibc_logs: {from_block}..{to_block} of {contract_addresses:?}");

		let value = client
			.request(
				"eth_getLogs",
				rpc_params![json!({
					"fromBlock": format!("0x{:x}", from_block),
					"toBlock": format!("0x{:x}", to_block),
					"address": contract_addresses.iter().map(|x| format!("0x{:x}", x)).collect::<Vec<String>>(),
				})],
			)
			.await?;
		let logs: Vec<Log> = serde_json::from_value(value)?;

		let mut db_ibc_events: Vec<DatabaseIBCEventData> = Vec::new();

		for log in logs {
			if log.block_number.is_none() ||
				log.log_index.is_none() ||
				log.transaction_index.is_none()
			{
				continue
			}

			if log.removed == Some(true) {
				warn!("log was removed: {:?}", log);
				continue
			}

			if let Ok(Some(s)) = parse_log(log.clone()) {
				db_ibc_events.push(DatabaseIBCEventData {
					block_number: log.block_number.unwrap().as_u64() as i64,
					event_data: serde_json::from_str(s.as_str()).unwrap(),
					address: log.address.0.to_vec(),
					topic0: log.topics[0].0.to_vec(),
					topics: log.topics.iter().map(|x| x.0.to_vec()).collect(),
					data: log.data.0.to_vec(),
					tx_index: log.transaction_index.unwrap().as_u64() as i64,
					event_index: log.log_index.unwrap().as_u64() as i64,
					raw_log: Json(log),
				})
			} else {
				warn!("failed to parse a log at {}", log.block_number.unwrap().as_u64());
			}
		}

		return Ok(Some(db_ibc_events))
	}

	fn get_client(&self) -> &HttpClient {
		let client = self.clients.choose(&mut rand::thread_rng()).unwrap();
		return client
	}
}

fn parse_log(log: Log) -> Result<Option<String>, Error> {
	use crate::utils::*;

	let raw_log = RawLog::from(log.clone());
	let topic0 = log.topics[0];

	macro_rules! handle_events {
        ($topic0:ident, $events:ident, $log:ident, $raw_log:ident, $height:ident, $($ty:ty),+) => {
            $(if $topic0 == <$ty>::signature() {
                 let ev = <$ty>::decode_log(&$raw_log).expect("decode event");
                //  log::debug!(target: "hyperspace_ethereum", "encountered event: {:?} at {}", ev.event_type(), ev.height());
                    return Ok(Some(serde_json::to_string(&ev)?));
            } else )+ {
                 log::warn!(
                     target: "hyperspace_ethereum", "unknown event: {}",
                       log.log_type.unwrap_or(format!("{:?}", $topic0))
                 );
                 return Ok(None)
            }
        };
    }

	handle_events!(
		topic0,
		event,
		log,
		raw_log,
		height,
		// Connection
		OpenInitConnectionFilter,
		OpenTryConnectionFilter,
		OpenAckConnectionFilter,
		OpenConfirmConnectionFilter,
		// Channel
		OpenInitChannelFilter,
		OpenTryChannelFilter,
		OpenAckChannelFilter,
		OpenConfirmChannelFilter,
		// Channel close
		CloseInitChannelFilter,
		CloseConfirmChannelFilter,
		// Packet
		SendPacketFilter,
		RecvPacketFilter,
		WriteAcknowledgementFilter,
		AcknowledgePacketFilter,
		TimeoutPacketFilter,
		// TimeoutOnClosePacketFilter,
		// Client
		CreateClientFilter,
		UpdateClientHeightFilter,
		UpdateClientFilter,
		// Custom
		RegisterClientFilter,
		GeneratedClientIdentifierFilter,
		GeneratedConnectionIdentifierFilter,
		GeneratedChannelIdentifierFilter,
		OwnershipTransferredFilter
	)
}
