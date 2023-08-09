use std::time::Duration;

use ethers::providers::Middleware;
use futures::{Stream, StreamExt};
use ibc::core::ics02_client::events::UpdateClient;
use ibc::Height;
use pallet_ibc::light_clients::AnyClientMessage;
use primitives::{Chain, CommonClientState, LightClientSync, MisbehaviourHandler};

use crate::{client::EthereumClient, ibc_provider::BlockHeight};

#[async_trait::async_trait]
impl MisbehaviourHandler for EthereumClient {
	async fn check_for_misbehaviour<C>(
		&self,
		counterparty: &C,
		client_message: AnyClientMessage,
	) -> Result<(), anyhow::Error> {
		// assume no misbehaviour occurs for now.
		Ok(())
	}
}

#[async_trait::async_trait]
impl Chain for EthereumClient {
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
		// TODO: estimate gas for the tx. Convert any to another type (see `wrap_any_msg_into_wasm` for an example)
		Ok(1)
	}

	async fn finality_notifications(
		&self,
	) -> Result<std::pin::Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send>>, Self::Error> {
		let ws = crate::client::WsEth::connect(self.ws_uri.to_string())
			.await
			.map_err(|err| crate::client::ClientError::ProviderError(self.ws_uri.clone(), err))?;

		let stream = async_stream::stream! {
			// TODO: is it really finalized blocks stream?
			// - PoW: probabilistic finality (wait for ~30 blocks)
			// - PoS: finality is deterministic
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
		todo!("submit to ethereum")
	}

	async fn query_client_message(&self, update: UpdateClient) -> Result<AnyClientMessage, Self::Error> {
		todo!("used for misbehaviour; skip for now")
	}

	async fn get_proof_height(&self, block_height: Height) -> Height {
		block_height
	}


	async fn handle_error(&mut self, error: &anyhow::Error) -> Result<(), anyhow::Error> {
		tracing::error!(?error, "handle-error");
		Ok(())
	}

	fn common_state(&self) -> &CommonClientState {
		&self.common_state
	}

	fn common_state_mut(&mut self) -> &mut CommonClientState {
		&mut self.common_state
	}

	fn rpc_call_delay(&self) -> Duration {
		Duration::from_millis(100)
	}

	fn set_rpc_call_delay(&mut self, delay: Duration) {}

	async fn reconnect(&mut self) -> anyhow::Result<()> {
		// TODO: reconnection logic
		Ok(())
	}
}
