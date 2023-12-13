use super::SolanaClient;
use crate::error::Error;
use core::pin::Pin;
use anchor_client::{Cluster, solana_client::{pubsub_client::PubsubClient, rpc_config::{RpcBlockSubscribeFilter, RpcBlockSubscribeConfig}}, solana_sdk::commitment_config::CommitmentConfig};
use futures::{Stream, StreamExt};
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, PrefixedCoin},
	core::ics24_host::identifier::ChannelId,
	tx_msg::Msg,
};
use primitives::{TestProvider, Chain};
use tendermint_rpc::{
	event::{Event, EventData},
	query::{EventType, Query},
};
use tokio::sync::mpsc::unbounded_channel;

#[async_trait::async_trait]
impl TestProvider for SolanaClient
{
	/// Initiate an ibc transfer on chain.
	async fn send_transfer(&self, msg: MsgTransfer<PrefixedCoin>) -> Result<(), Self::Error> {
		let hash = self.submit(vec![msg.to_any()]).await?;
		log::info!(target: "hyperspace_cosmos", "🤝 Transfer transaction confirmed with hash: {:?}", hash);
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
		let cluster = Cluster::Devnet;
		tokio::task::spawn_blocking(move || {
			let (_logs_listener, receiver) = PubsubClient::block_subscribe(
				"", /* Quicknode rpc should be used for devnet/mainnet and incase of localnet,
				     * the flag `--rpc-pubsub-enable-block-subscription` has to be passed to
				     * local validator. */
				RpcBlockSubscribeFilter::All,
				Some(RpcBlockSubscribeConfig {
					commitment: Some(CommitmentConfig::finalized()),
					..Default::default()
				}),
			)
			.unwrap();

			loop {
				match receiver.recv() {
					Ok(logs) =>
						if logs.value.block.is_some() {
							let block_info = logs.value.block.clone().unwrap();
							let block_number = block_info.block_height.unwrap();
							let _ = tx.send(block_number);
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
