use anyhow::Error;
use ibc::events::IbcEvent;
use ibc_proto::google::protobuf::Any;
use primitives::{Chain, LightClientSync};

use crate::client::EthereumClient;

#[async_trait::async_trait]
impl LightClientSync for EthereumClient {
	async fn is_synced<C: Chain>(&self, counterparty: &C) -> Result<bool, Error> {
		Ok(true)
	}

	async fn fetch_mandatory_updates<C: Chain>(&self, counterparty: &C) -> Result<(Vec<Any>, Vec<IbcEvent>), Error> {
		todo!()
	}
}
