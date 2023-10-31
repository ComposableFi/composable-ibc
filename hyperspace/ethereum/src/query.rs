use crate::client::{ClientError, EthereumClient};
use ethers::{
	prelude::{EthEvent, Log},
	utils::keccak256,
};
use futures::TryFutureExt;
use log::info;
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
		from_block: u64,
		to_block: u64,
		query: &str,
		additional_query: Option<&str>,
	) -> Result<Vec<(T, Log)>, ClientError> {
		let signature = T::abi_signature();
		info!(target: "hyperspace_ethereum", "get_logs_for_event_name: from_block: {}, to_block: {}, signature: {}", from_block, to_block, signature);
		let mut string = format!("SELECT event_data, raw_log FROM ibc_events WHERE block_number >= {from_block} AND block_number <= {to_block} AND topic0 = {} AND {query}", format!("decode('{}', 'hex')", hex::encode(&keccak256(signature))));
		if let Some(additional_query) = additional_query {
			string.push_str(&format!(" AND {additional_query}"));
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

	// let select event_data from ibc_event where block_number >= {from_block} AND block_number <=
	// to_block AND event_data ->> 'sourceChannel' = 'channel-57';
	pub async fn query<'a, T: Send + Unpin + for<'r> FromRow<'r, PgRow>>(
		&self,
		query: QueryAs<'a, Postgres, T, <Postgres as HasArguments<'a>>::Arguments>,
	) -> Result<Vec<T>, ClientError> {
		let conn = self.db();
		info!(target: "hyperspace_ethereum", "query: {}", query.sql());
		let logs = query.fetch_all(conn).await?;
		Ok(logs)
	}
}
