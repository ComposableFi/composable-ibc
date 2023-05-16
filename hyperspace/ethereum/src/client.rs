use std::{str::FromStr, sync::Arc};

use ethers::{
	abi::{ParseError, Token},
	providers::{Http, Middleware, Provider, ProviderError, ProviderExt, Ws},
	signers::{coins_bip39::English, MnemonicBuilder, Signer},
	types::{Block, BlockId, EIP1186ProofResponse, NameOrAddress, H256},
};

use futures::TryFutureExt;
use thiserror::Error;

use crate::config::Config;

pub(crate) type EthRpcClient = ethers::prelude::SignerMiddleware<
	ethers::providers::Provider<Http>,
	ethers::signers::Wallet<ethers::prelude::k256::ecdsa::SigningKey>,
>;
pub(crate) type WsEth = Provider<Ws>;

#[derive(Debug, Clone)]
pub struct Client {
	pub http_rpc: Arc<EthRpcClient>,
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
		let client = Provider::<Http>::try_from(config.rpc_url.to_string())
			.map_err(|_| ClientError::UriParseError(config.rpc_url.clone()))?;

		let chain_id = client.get_chainid().await.unwrap();

		let wallet = MnemonicBuilder::<English>::default()
			.phrase(&*config.mnemonic)
			.build()
			.unwrap()
			.with_chain_id(chain_id.as_u64());

		let client = ethers::middleware::SignerMiddleware::new(client, wallet);

		Ok(Self { http_rpc: Arc::new(client), ws_uri: config.ws_url.clone(), config })
	}

	pub fn eth_query_proof(
		&self,
		key: &str,
		block_height: Option<u64>,
	) -> impl std::future::Future<Output = Result<EIP1186ProofResponse, ClientError>> {
		let key = ethers::utils::keccak256(
			&ethers::abi::encode_packed(&[Token::String(key.into())]).unwrap(),
		);

		let key = hex::encode(key);

		let ix = 0;

		let index =
			cast::SimpleCast::index("bytes32", &format!("0x{key}", key = &key), &format!("{ix}"))
				.unwrap();

		let client = self.http_rpc.clone();
		let address = self.config.address.clone().parse().unwrap();

		async move {
			client
				.get_proof(
					NameOrAddress::Address(address),
					vec![H256::from_str(&index).unwrap()],
					block_height.map(|i| BlockId::from(i)),
				)
				.map_err(|err| panic!("{err}"))
				.await
		}
	}

	#[track_caller]
	pub fn has_packet_receipt(
		&self,
		port_id: String,
		channel_id: String,
		sequence: u64,
	) -> impl std::future::Future<Output = Result<bool, ClientError>> {
		let client = self.http_rpc.clone();
		let address = self.config.address.clone().parse().unwrap();

		let contract = crate::contract::contract(address, Arc::clone(&client));

		async move {
			let binding = contract
				.method("hasPacketReceipt", (port_id, channel_id, sequence))
				.expect("contract is missing hasPacketReceipt");

			let receipt_fut = binding.call();

			let receipt: bool = receipt_fut.await.map_err(|err| todo!()).unwrap();

			Ok(receipt)
		}
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
