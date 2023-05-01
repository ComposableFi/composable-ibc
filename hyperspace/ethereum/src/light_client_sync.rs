use primitives::{Chain, LightClientSync};

use crate::client::Client;

#[async_trait::async_trait]
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
