use super::{error::Error, CosmosClient};
use crate::finality_protocol::FinalityProtocol;
use futures::Stream;
use ibc_proto::google::protobuf::Any;
use primitives::{Chain, IbcProvider};
use prost::Message;
use std::pin::Pin;
use tendermint_rpc::Client;

#[async_trait::async_trait]
impl<H> Chain for CosmosClient<H>
where
	H: Clone + Send + Sync + 'static,
{
	fn name(&self) -> &str {
		&*self.name
	}

	fn block_max_weight(&self) -> u64 {
		todo!()
	}

	async fn estimate_weight(&self, messages: Vec<Any>) -> Result<u64, Self::Error> {
		todo!()
	}

	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>> {
		match self.finality_protocol {
			FinalityProtocol::Tendermint => {
				todo!()
			},
		}
	}

	async fn submit(&self, messages: Vec<Any>) -> Result<(sp_core::H256, Option<sp_core::H256>), Error> {
        todo!()
	}
}
