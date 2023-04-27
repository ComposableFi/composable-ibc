use primitives::IbcProvider;

use futures::Stream;

use crate::client::Client;

impl IbcProvider for Client {
	type FinalityEvent;

	type TransactionId;

	type AssetId;

	type Error;

	fn query_latest_ibc_events<'life0, 'life1, 'async_trait, T>(
		&'life0 mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &'life1 T,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						Vec<(
							ibc_proto::google::protobuf::Any,
							Vec<ibc::events::IbcEvent>,
							primitives::UpdateType,
						)>,
						anyhow::Error,
					>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		T: primitives::Chain,
		T: 'async_trait,
		'life0: 'async_trait,
		'life1: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn ibc_events<'life0, 'async_trait>(
		&'life0 self,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = std::pin::Pin<
						Box<dyn Stream<Item = ibc::events::IbcEvent> + Send + 'static>,
					>,
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

	fn query_client_consensus<'life0, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		consensus_height: ibc::Height,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						ibc_proto::ibc::core::client::v1::QueryConsensusStateResponse,
						Self::Error,
					>,
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

	fn query_client_state<'life0, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		client_id: ibc::core::ics24_host::identifier::ClientId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						ibc_proto::ibc::core::client::v1::QueryClientStateResponse,
						Self::Error,
					>,
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

	fn query_connection_end<'life0, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		connection_id: ibc::core::ics24_host::identifier::ConnectionId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						ibc_proto::ibc::core::connection::v1::QueryConnectionResponse,
						Self::Error,
					>,
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

	fn query_channel_end<'life0, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						ibc_proto::ibc::core::channel::v1::QueryChannelResponse,
						Self::Error,
					>,
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

	fn query_proof<'life0, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		keys: Vec<Vec<u8>>,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<u8>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_packet_commitment<'life0, 'life1, 'life2, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		port_id: &'life1 ibc::core::ics24_host::identifier::PortId,
		channel_id: &'life2 ibc::core::ics24_host::identifier::ChannelId,
		seq: u64,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						ibc_proto::ibc::core::channel::v1::QueryPacketCommitmentResponse,
						Self::Error,
					>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		'life1: 'async_trait,
		'life2: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_packet_acknowledgement<'life0, 'life1, 'life2, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		port_id: &'life1 ibc::core::ics24_host::identifier::PortId,
		channel_id: &'life2 ibc::core::ics24_host::identifier::ChannelId,
		seq: u64,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						ibc_proto::ibc::core::channel::v1::QueryPacketAcknowledgementResponse,
						Self::Error,
					>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		'life1: 'async_trait,
		'life2: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_next_sequence_recv<'life0, 'life1, 'life2, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		port_id: &'life1 ibc::core::ics24_host::identifier::PortId,
		channel_id: &'life2 ibc::core::ics24_host::identifier::ChannelId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						ibc_proto::ibc::core::channel::v1::QueryNextSequenceReceiveResponse,
						Self::Error,
					>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		'life1: 'async_trait,
		'life2: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_packet_receipt<'life0, 'life1, 'life2, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		port_id: &'life1 ibc::core::ics24_host::identifier::PortId,
		channel_id: &'life2 ibc::core::ics24_host::identifier::ChannelId,
		seq: u64,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						ibc_proto::ibc::core::channel::v1::QueryPacketReceiptResponse,
						Self::Error,
					>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		'life1: 'async_trait,
		'life2: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn latest_height_and_timestamp<'life0, 'async_trait>(
		&'life0 self,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<(ibc::Height, ibc::timestamp::Timestamp), Self::Error>,
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

	fn query_packet_commitments<'life0, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<u64>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_packet_acknowledgements<'life0, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<u64>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_unreceived_packets<'life0, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<u64>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_unreceived_acknowledgements<'life0, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<u64>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn channel_whitelist(
		&self,
	) -> Vec<(
		ibc::core::ics24_host::identifier::ChannelId,
		ibc::core::ics24_host::identifier::PortId,
	)> {
		todo!()
	}

	fn query_connection_channels<'life0, 'life1, 'async_trait>(
		&'life0 self,
		at: ibc::Height,
		connection_id: &'life1 ibc::core::ics24_host::identifier::ConnectionId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						ibc_proto::ibc::core::channel::v1::QueryChannelsResponse,
						Self::Error,
					>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		'life1: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_send_packets<'life0, 'async_trait>(
		&'life0 self,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<ibc_rpc::PacketInfo>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_recv_packets<'life0, 'async_trait>(
		&'life0 self,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Vec<ibc_rpc::PacketInfo>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn expected_block_time(&self) -> std::time::Duration {
		todo!()
	}

	fn query_client_update_time_and_height<'life0, 'async_trait>(
		&'life0 self,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		client_height: ibc::Height,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<(ibc::Height, ibc::timestamp::Timestamp), Self::Error>,
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

	fn query_host_consensus_state_proof<'life0, 'async_trait>(
		&'life0 self,
		height: ibc::Height,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Option<Vec<u8>>, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_ibc_balance<'life0, 'async_trait>(
		&'life0 self,
		asset_id: Self::AssetId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<Vec<ibc::applications::transfer::PrefixedCoin>, Self::Error>,
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

	fn connection_prefix(&self) -> ibc::core::ics23_commitment::commitment::CommitmentPrefix {
		todo!()
	}

	fn client_id(&self) -> ibc::core::ics24_host::identifier::ClientId {
		todo!()
	}

	fn set_client_id(&mut self, client_id: ibc::core::ics24_host::identifier::ClientId) {
		todo!()
	}

	fn connection_id(&self) -> Option<ibc::core::ics24_host::identifier::ConnectionId> {
		todo!()
	}

	fn set_channel_whitelist(
		&mut self,
		channel_whitelist: Vec<(
			ibc::core::ics24_host::identifier::ChannelId,
			ibc::core::ics24_host::identifier::PortId,
		)>,
	) {
		todo!()
	}

	fn add_channel_to_whitelist(
		&mut self,
		channel: (
			ibc::core::ics24_host::identifier::ChannelId,
			ibc::core::ics24_host::identifier::PortId,
		),
	) {
		todo!()
	}

	fn set_connection_id(
		&mut self,
		connection_id: ibc::core::ics24_host::identifier::ConnectionId,
	) {
		todo!()
	}

	fn client_type(&self) -> ibc::core::ics02_client::client_state::ClientType {
		todo!()
	}

	fn query_timestamp_at<'life0, 'async_trait>(
		&'life0 self,
		block_number: u64,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<u64, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn query_clients<'life0, 'async_trait>(
		&'life0 self,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<Vec<ibc::core::ics24_host::identifier::ClientId>, Self::Error>,
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

	fn query_channels<'life0, 'async_trait>(
		&'life0 self,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						Vec<(
							ibc::core::ics24_host::identifier::ChannelId,
							ibc::core::ics24_host::identifier::PortId,
						)>,
						Self::Error,
					>,
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

	fn query_connection_using_client<'life0, 'async_trait>(
		&'life0 self,
		height: u32,
		client_id: String,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						Vec<ibc_proto::ibc::core::connection::v1::IdentifiedConnection>,
						Self::Error,
					>,
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

	fn is_update_required<'life0, 'async_trait>(
		&'life0 self,
		latest_height: u64,
		latest_client_height_on_counterparty: u64,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<bool, Self::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn initialize_client_state<'life0, 'async_trait>(
		&'life0 self,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						(
							pallet_ibc::light_clients::AnyClientState,
							pallet_ibc::light_clients::AnyConsensusState,
						),
						Self::Error,
					>,
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

	fn query_client_id_from_tx_hash<'life0, 'async_trait>(
		&'life0 self,
		tx_id: Self::TransactionId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<ibc::core::ics24_host::identifier::ClientId, Self::Error>,
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

	fn query_connection_id_from_tx_hash<'life0, 'async_trait>(
		&'life0 self,
		tx_id: Self::TransactionId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<ibc::core::ics24_host::identifier::ConnectionId, Self::Error>,
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

	fn query_channel_id_from_tx_hash<'life0, 'async_trait>(
		&'life0 self,
		tx_id: Self::TransactionId,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						(
							ibc::core::ics24_host::identifier::ChannelId,
							ibc::core::ics24_host::identifier::PortId,
						),
						Self::Error,
					>,
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
}
