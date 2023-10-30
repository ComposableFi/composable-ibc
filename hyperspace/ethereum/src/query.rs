use crate::client::{ClientError, EthereumClient};
use serde::Deserialize;
use sqlx::{database::HasArguments, query::QueryAs, Postgres};

impl EthereumClient {
	pub fn db(&self) -> &sqlx::Pool<sqlx::Postgres> {
		&self.db_conn
	}

	pub async fn get_logs_for_event_name<T: Deserialize>(
		&self,
		from_block: u64,
		to_block: u64,
		name: &str,
		query: &str,
		additional_query: Option<&str>,
	) {
		// sqlx::query_as::<_, (serde_json::Value,)>(query)
		// 	.bind(from_block)
		// 	.bind(to_block)
	}

	// let select event_data from ibc_event where block_number >= {from_block} AND block_number <=
	// to_block AND event_data ->> 'sourceChannel' = 'channel-57';
	pub async fn query<T: Deserialize>(
		&self,
		query: QueryAs<'_, Postgres, T, <Postgres as HasArguments<'_>>::Arguments>,
	) -> Result<Vec<T>, ClientError> {
		let conn = self.db();
		// sqlx::query_as
		let logs = query.fetch_all(conn).await.unwrap();
		logs.into_iter()
			.map(|(log,)| serde_json::from_value(log).map_err(|e| ClientError::from(e)))
			.collect::<Result<Vec<_>, _>>()
		// sqlx::sqlx_macros::expand_query!()
		// let logs = self
		//     .web3
		//     .eth()
		//     .logs(FilterBuilder::default().from_block(from_block.into()).to_block(to_block.
		// into()).build())     .wait()
		//     .unwrap();
		// logs.into_iter().map(|log| log.into()).collect()
	}
}
