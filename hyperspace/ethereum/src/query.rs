use crate::client::{ClientError, EthereumClient};
use serde::Deserialize;
use sqlx::{database::HasArguments, query::QueryAs, Postgres};

impl EthereumClient {
	pub fn db(&self) -> &sqlx::Pool<sqlx::Postgres> {
		&self.db_conn
	}

	// pub async fn get_logs_for_event_name<T: for<'a> Deserialize<'a>>(
	pub async fn get_logs_for_event_name(
		&self,
		from_block: u64,
		to_block: u64,
		name: &str,
		query: &str,
		additional_query: Option<&str>,
	) -> Result<Vec<(String,)>, ClientError> {
		sqlx::query_as::<_, (String,)>(
			"SELECT event_name FROM asd WHERE block_number > $1 AND block_number < $2",
		)
		.bind(from_block as i64)
		.bind(to_block as i64)
		.fetch_all(&self.db_conn)
		.await
		.map_err(ClientError::from)
	}

	// let select event_data from ibc_event where block_number >= {from_block} AND block_number <=
	// to_block AND event_data ->> 'sourceChannel' = 'channel-57';
	pub async fn query<T: for<'a> Deserialize<'a>>(
		&self,
		query: QueryAs<'_, Postgres, T, <Postgres as HasArguments<'_>>::Arguments>,
	) -> Result<Vec<(String,)>, ClientError> {
		let conn = self.db();
		// sqlx::query_as

		// let logs = query.fetch_all(conn).await.unwrap();
		// logs.into_iter()
		// 	.map(|(log,)| serde_json::from_value(log).map_err(ClientError::from))
		// 	.collect::<Result<Vec<_>, _>>()
		// sqlx::sqlx_macros::expand_query!()
		// let logs = self
		//     .web3
		//     .eth()
		//     .logs(FilterBuilder::default().from_block(from_block.into()).to_block(to_block.
		// into()).build())     .wait()
		//     .unwrap();
		// logs.into_iter().map(|log| log.into()).collect()

		Ok(vec![])
	}
}
