use futures::Stream;
use primitives::{Chain, LightClientSync, MisbehaviourHandler};

use crate::client::Client;

impl LightClientSync for Client {
	fn is_synced<'life0, 'life1, 'async_trait, C>(
		&'life0 self,
		counterparty: &'life1 C,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<bool, anyhow::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		C: 'async_trait + Chain,
		'life0: 'async_trait,
		'life1: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}

	fn fetch_mandatory_updates<'life0, 'life1, 'async_trait, C>(
		&'life0 self,
		counterparty: &'life1 C,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						(Vec<ibc_proto::google::protobuf::Any>, Vec<ibc::events::IbcEvent>),
						anyhow::Error,
					>,
				> + core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		C: 'async_trait + Chain,
		'life0: 'async_trait,
		'life1: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}
}

impl MisbehaviourHandler for Client {
	fn check_for_misbehaviour<'life0, 'life1, 'async_trait, C>(
		&'life0 self,
		counterparty: &'life1 C,
		client_message: pallet_ibc::light_clients::AnyClientMessage,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<(), anyhow::Error>>
				+ core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		C: 'async_trait + Chain,
		'life0: 'async_trait,
		'life1: 'async_trait,
		Self: 'async_trait,
	{
		todo!()
	}
}

impl Chain for Client {
	fn name(&self) -> &str {
		todo!()
	}

	fn block_max_weight(&self) -> u64 {
		todo!()
	}

	fn estimate_weight<'life0, 'async_trait>(
		&'life0 self,
		msg: Vec<ibc_proto::google::protobuf::Any>,
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

	fn finality_notifications<'life0, 'async_trait>(
		&'life0 self,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = Result<
						std::pin::Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send + Sync>>,
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

	fn submit<'life0, 'async_trait>(
		&'life0 self,
		messages: Vec<ibc_proto::google::protobuf::Any>,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<Self::TransactionId, Self::Error>>
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

	fn handle_error<'life0, 'life1, 'async_trait>(
		&'life0 mut self,
		error: &'life1 anyhow::Error,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<(), anyhow::Error>>
				+ core::marker::Send
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

	fn rpc_call_delay(&self) -> std::time::Duration {
		todo!()
	}

	fn set_rpc_call_delay(&mut self, delay: std::time::Duration) {
		todo!()
	}
}
