use crate::client::{ClientError, EthereumClient};
use ethers::{
	prelude::{EthEvent, Log},
	types::BlockNumber,
	utils::keccak256,
};
use ethers_providers::Middleware;
use futures::TryFutureExt;
use log::trace;
use serde::de::DeserializeOwned;
use sqlx::{
	database::HasArguments, postgres::PgRow, query::QueryAs, types::Json, Execute, FromRow,
	Postgres,
};

impl EthereumClient {
	pub fn db(&self) -> &sqlx::Pool<sqlx::Postgres> {
		&self.db_conn
	}

	pub async fn get_logs_for_event_name<T: DeserializeOwned + EthEvent>(
		&self,
		from_block: impl Into<BlockNumber>,
		to_block: impl Into<BlockNumber>,
		query: &str,
		additional_query: Option<&str>,
	) -> Result<Vec<(T, Log)>, ClientError> {
		let from_block = from_block.into();
		let to_block = to_block.into();
		let to_block = match to_block {
			BlockNumber::Number(n) => n.as_u64(),
			BlockNumber::Latest => self
				.client()
				.get_block_number()
				.await
				.map_err(|e| {
					ClientError::from(format!("Unable to get latest block number: {}", e))
				})?
				.as_u64(),
			_ =>
				return Err(ClientError::Other(
					"The given BlockNumber variant is not supported".to_owned(),
				)),
		};
		let signature = T::abi_signature();
		let contract_address = self.yui.diamond.address();
		let address_val = format!("decode('{}', 'hex')", hex::encode(contract_address.as_bytes()));
		trace!(target: "hyperspace_ethereum", "get_logs_for_event_name: from_block: {}, to_block: {}, signature: {}", from_block, to_block, signature);
		let topic0 = format!("decode('{}', 'hex')", hex::encode(&keccak256(signature.as_bytes())));
		let mut string = format!("SELECT event_data, raw_log FROM ibc_events WHERE block_number >= {from_block} AND block_number <= {to_block} AND topic0 = {topic0} AND address = {address_val} AND {query}");
		if let Some(additional_query) = additional_query {
			string.push_str(&format!(" {additional_query}"));
		}
		let query = sqlx::query_as::<_, (serde_json::Value, Json<Log>)>(&string);
		self.query(query)
			.await?
			.into_iter()
			.map(|(val, raw_log)| {
				Ok((serde_json::from_value(val).map_err(ClientError::from)?, raw_log.0))
			})
			.collect::<Result<Vec<_>, _>>()
	}

	pub async fn get_logs(
		&self,
		from_block: u64,
		to_block: BlockNumber,
		additional_query: Option<&str>,
	) -> Result<Vec<Log>, ClientError> {
		let to_block = match to_block {
			BlockNumber::Number(n) => n.as_u64(),
			BlockNumber::Latest => self
				.client()
				.get_block_number()
				.await
				.map_err(|e| {
					ClientError::from(format!("Unable to get latest block number: {}", e))
				})?
				.as_u64(),
			_ =>
				return Err(ClientError::Other(
					"The given BlockNumber variant is not supported".to_owned(),
				)),
		};
		let contract_address = self.yui.diamond.address();
		let address_val = format!("decode('{}', 'hex')", hex::encode(contract_address.as_bytes()));
		trace!(target: "hyperspace_ethereum", "get_logs: from_block: {}, to_block: {}", from_block, to_block);
		let mut string = format!("SELECT raw_log FROM ibc_events WHERE block_number >= {from_block} AND block_number <= {to_block} AND address = {address_val}");
		if let Some(additional_query) = additional_query {
			string.push_str(&format!(" {additional_query}"));
		}
		let query = sqlx::query_as::<_, (Json<Log>,)>(&string);
		self.query(query)
			.await?
			.into_iter()
			.map(|(raw_log,)| Ok(raw_log.0))
			.collect::<Result<Vec<_>, _>>()
	}
	pub async fn query<'a, T: Send + Unpin + for<'r> FromRow<'r, PgRow>>(
		&self,
		query: QueryAs<'a, Postgres, T, <Postgres as HasArguments<'a>>::Arguments>,
	) -> Result<Vec<T>, ClientError> {
		let conn = self.db();
		trace!(target: "hyperspace_ethereum", "query: {}", query.sql());
		let logs = query.fetch_all(conn).await?;
		Ok(logs)
	}
}
