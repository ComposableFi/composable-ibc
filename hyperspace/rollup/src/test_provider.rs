use crate::{client::RollupClient, error::Error, events};
use anchor_client::{
	solana_client::{
		pubsub_client::PubsubClient,
		rpc_config::{
			RpcBlockSubscribeConfig, RpcBlockSubscribeFilter, RpcTransactionLogsConfig,
			RpcTransactionLogsFilter,
		},
	},
	solana_sdk::commitment_config::CommitmentConfig,
};
use core::pin::Pin;
use futures::Stream;
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, PrefixedCoin},
	core::ics24_host::identifier::ChannelId,
};
use primitives::TestProvider;
use tokio::sync::mpsc::unbounded_channel;

#[async_trait::async_trait]
impl TestProvider for RollupClient {
	/// Initiate an ibc transfer on chain.
	async fn send_transfer(&self, msg: MsgTransfer<PrefixedCoin>) -> Result<(), Self::Error> {
		let hash = self.send_transfer_inner(msg).await?;
		log::info!(target: "hyperspace_cosmos", "🤝 Transfer transaction confirmed with hash: {}", hash);
		Ok(())
	}

	/// Send a packet on an ordered channel
	async fn send_ordered_packet(
		&self,
		_channel_id: ChannelId,
		_timeout: pallet_ibc::Timeout,
	) -> Result<(), Self::Error> {
		Err(Error::Custom("send_ordered_packet is not implemented yet".to_string()))
	}

	/// Returns a stream that yields chain Block number
	async fn subscribe_blocks(&self) -> Pin<Box<dyn Stream<Item = u64> + Send + Sync>> {
		let (tx, rx) = unbounded_channel();
		let ws_url = self.ws_url.clone();
		let program_id = self.solana_ibc_program_id;
		tokio::task::spawn_blocking(move || {
			let (_block_subscription, receiver) = PubsubClient::block_subscribe(
				&ws_url,
				// RpcBlockSubscribeFilter::MentionsAccountOrProgram(program_id.to_string()),
				RpcBlockSubscribeFilter::All,
				Some(RpcBlockSubscribeConfig {
					commitment: Some(CommitmentConfig::finalized()),
					..Default::default()
				}),
			)
			.unwrap();

			loop {
				match receiver.recv() {
					Ok(logs) => {
						let mut broke = false;
						let _ = tx.send(u64::from(logs.value.slot)).map_err(|_| broke = true);
						if broke {
							break;
						}
					},
					Err(err) => {
						panic!("{}", format!("Disconnected: {err}"));
					},
				}
			}
		});

		let streams = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);
		Box::pin(streams)
	}

	async fn increase_counters(&mut self) -> Result<(), Self::Error> {
		unimplemented!()
	}
}
