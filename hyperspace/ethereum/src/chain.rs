use std::time::Duration;

use ethers::providers::Middleware;
use futures::{Stream, StreamExt};
use primitives::{Chain, LightClientSync, MisbehaviourHandler};

use crate::{client::Client, ibc_provider::BlockHeight};

#[async_trait::async_trait]
impl MisbehaviourHandler for Client {
	async fn check_for_misbehaviour<C>(
		&self,
		counterparty: &C,
		client_message: pallet_ibc::light_clients::AnyClientMessage,
	) -> Result<(), anyhow::Error> {
		// assume no misbehaviour occurs for now.
		Ok(())
	}
}

#[async_trait::async_trait]
impl Chain for Client {
	#[inline]
	fn name(&self) -> &str {
		&self.config.name
	}

	#[inline]
	fn block_max_weight(&self) -> u64 {
		self.config.max_block_weight
	}

	async fn estimate_weight(
		&self,
		msg: Vec<ibc_proto::google::protobuf::Any>,
	) -> Result<u64, Self::Error> {
		Ok(1)
	}

	async fn finality_notifications(
		&self,
	) -> Result<std::pin::Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send>>, Self::Error> {
		let ws = crate::client::WsEth::connect(self.ws_uri.to_string())
			.await
			.map_err(|err| crate::client::ClientError::ProviderError(self.ws_uri.clone(), err))?;

		let stream = async_stream::stream! {
			let mut stream = ws.subscribe_blocks().await.expect("fuck");

			while let Some(block) = stream.next().await {
				if let Some(hash) = block.hash.clone() {
					yield Self::FinalityEvent::Ethereum { hash };
				}
			}
		};

		Ok(stream.boxed())
	}

	async fn submit(
		&self,
		messages: Vec<ibc_proto::google::protobuf::Any>,
	) -> Result<Self::TransactionId, Self::Error> {
		todo!()
	}

	fn query_client_message<'life0, 'async_trait>(
		&'life0 self,
		update: ibc::core::ics02_client::events::UpdateClient,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<pallet_ibc::light_clients::AnyClientMessage, Self::Error>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn get_proof_height<'life0, 'async_trait>(
		&'life0 self,
		block_height: ibc::Height,
	) -> core::pin::Pin<
		Box<dyn core::future::Future<Output = ibc::Height> + core::marker::Send + 'async_trait>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	async fn handle_error(&mut self, error: &anyhow::Error) -> Result<(), anyhow::Error> {
		tracing::error!(?error, "handle-error");
		Ok(())
	}

	fn rpc_call_delay(&self) -> std::time::Duration {
		Duration::from_millis(100)
	}

	fn set_rpc_call_delay(&mut self, delay: std::time::Duration) {}
}
