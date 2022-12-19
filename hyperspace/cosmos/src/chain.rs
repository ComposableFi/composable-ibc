use super::{error::Error, CosmosClient};
use crate::provider::{FinalityEvent, TransactionId};
use futures::Stream;
use ibc_proto::{
	cosmos::tx::v1beta1::{
		mode_info::{Single, Sum},
		AuthInfo, ModeInfo, SignDoc, SignerInfo, TxBody, TxRaw,
	},
	google::protobuf::Any,
};
use k256::ecdsa::{signature::Signer as _, Signature, SigningKey};
use primitives::{Chain, IbcProvider};
use prost::Message;
use std::pin::Pin;
use tendermint_rpc::{
	event::{Event, EventData},
	query::{EventType, Query},
	Client, SubscriptionClient, WebSocketClient,
};

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

	async fn estimate_weight(&self, _messages: Vec<Any>) -> Result<u64, Self::Error> {
		Ok(0)
	}

	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>> {
		let (ws_client, ws_driver) = WebSocketClient::new(self.websocket_url.clone())
			.await
			.map_err(|e| Error::from(format!("Web Socket Client Error {:?}", e)))
			.unwrap();

		let subscription = ws_client
			.subscribe(Query::from(EventType::NewBlock))
			.await
			.map_err(|e| Error::from(format!("failed to subscribe to new blocks {:?}", e)))
			.unwrap();

		let stream = subscription.map(|event| {
			let Event { data, events, query } = event;
			let header = match data {
				EventData::NewBlock { block, .. } => block.unwrap().header,
				_ => unreachable!(),
			};
			futures::future::ready(Some(FinalityEvent::Tendermint(header)))
		});

		Box::pin(Box::new(stream))
	}

	async fn submit(&self, messages: Vec<Any>) -> Result<Self::TransactionId, Error> {
		let hash = self.submit_call(messages).await?;
		Ok(Self::TransactionId { hash })
	}
}
