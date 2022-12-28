use super::client::CosmosClient;
use core::pin::Pin;
use futures::Stream;
// use ibc::{applications::transfer::msgs::transfer::MsgTransfer, tx_msg::Msg};
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, PrefixedCoin},
	core::ics24_host::identifier::{ChannelId, PortId},
	tx_msg::Msg,
};
use primitives::TestProvider;

#[async_trait::async_trait]
impl<H> TestProvider for CosmosClient<H>
where
	H: Clone + Send + Sync + 'static,
{
	/// Initiate an ibc transfer on chain.
	async fn send_transfer(&self, msg: MsgTransfer<PrefixedCoin>) -> Result<(), Self::Error> {
		let hash = self.submit_call(vec![msg.to_any()]).await?;
		log::info!(target: "hyperspace-light", "🤝 Transfer transaction confirmed with hash: {:?}", hash);
		Ok(())
	}

	/// Send a packet on an ordered channel
	async fn send_ordered_packet(
		&self,
		_channel_id: ChannelId,
		_timeout: pallet_ibc::Timeout,
	) -> Result<(), Self::Error> {
		todo!()
	}

	/// Returns a stream that yields chain Block number
	async fn subscribe_blocks(&self) -> Pin<Box<dyn Stream<Item = u64> + Send + Sync>> {
		todo!()
	}

	/// Set the channel whitelist for the relayer task.
	fn set_channel_whitelist(&mut self, channel_whitelist: Vec<(ChannelId, PortId)>) {
		self.channel_whitelist = channel_whitelist;
	}
}
