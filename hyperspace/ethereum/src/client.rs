use ethers::{
	abi::ParseError,
	providers::{Http, Provider, ProviderError, ProviderExt, Ws},
};

use thiserror::Error;

use crate::config::Config;

pub(crate) type HttpEth = Provider<Http>;
pub(crate) type WsEth = Provider<Ws>;

#[derive(Debug, Clone)]
pub struct Client {
	pub(crate) http_rpc: HttpEth,
	pub(crate) ws_uri: http::Uri,
	pub(crate) config: Config,
}

#[derive(Debug, Error)]
pub enum ClientError {
	#[error("uri-parse-error: {0} {0}")]
	UriParseError(http::Uri),
	#[error("provider-error: {0}: {0}")]
	ProviderError(http::Uri, ProviderError),
	#[error("Ethereum error: {0}")]
	Ethers(#[from] ethers::providers::ProviderError),
}

impl From<String> for ClientError {
	fn from(value: String) -> Self {
		todo!()
	}
}

impl Client {
	pub async fn new(config: Config) -> Result<Self, ClientError> {
		let http_rpc = HttpEth::try_from(config.rpc_url.to_string())
			.map_err(|_| ClientError::UriParseError(config.rpc_url.clone()))?;

		Ok(Self { http_rpc, ws_uri: config.ws_url.clone(), config })
	}
}

// #[cfg(any(test, feature = "testing"))]
impl primitives::TestProvider for Client {
	fn send_transfer<'life0, 'async_trait>(
		&'life0 self,
		params: ibc::applications::transfer::msgs::transfer::MsgTransfer<
			ibc::applications::transfer::PrefixedCoin,
		>,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<(), Self::Error>>
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

	fn send_ordered_packet<'life0, 'async_trait>(
		&'life0 self,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		timeout: pallet_ibc::Timeout,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<(), Self::Error>>
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

	fn subscribe_blocks<'life0, 'async_trait>(
		&'life0 self,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<
					Output = std::pin::Pin<Box<dyn futures::Stream<Item = u64> + Send + Sync>>,
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

	fn increase_counters<'life0, 'async_trait>(
		&'life0 mut self,
	) -> core::pin::Pin<
		Box<
			dyn core::future::Future<Output = Result<(), Self::Error>>
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
}
