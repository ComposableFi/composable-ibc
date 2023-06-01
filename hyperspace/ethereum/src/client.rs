use std::{future::Future, str::FromStr, sync::Arc};

use ethers::{
	abi::{ParseError, Token},
	providers::{Http, Middleware, Provider, ProviderError, ProviderExt, Ws},
	signers::{coins_bip39::English, MnemonicBuilder, Signer},
	types::{Log, Block, BlockId, BlockNumber, EIP1186ProofResponse, Filter, NameOrAddress, H160, H256},
};

use futures::{Stream, TryFutureExt};
use ibc::core::ics24_host::identifier::ClientId;
use thiserror::Error;

use crate::config::Config;

pub(crate) type EthRpcClient = ethers::prelude::SignerMiddleware<
	ethers::providers::Provider<Http>,
	ethers::signers::Wallet<ethers::prelude::k256::ecdsa::SigningKey>,
>;
pub(crate) type WsEth = Provider<Ws>;

// TODO: generate this from the contract automatically
pub const COMMITMENTS_STORAGE_INDEX: u32 = 0;
pub const CLIENT_IMPLS_STORAGE_INDEX: u32 = 3;
pub const CONNECTIONS_STORAGE_INDEX: u32 = 4;

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
	#[error("{0}")]
	Boxed(Box<dyn std::error::Error + Send + Sync>),
}

impl From<String> for ClientError {
	fn from(value: String) -> Self {
		todo!()
	}
}

impl Client {
	pub async fn new(config: Config) -> Result<Self, ClientError> {
		let client = Provider::<Http>::try_from(config.http_rpc_url.to_string())
			.map_err(|_| ClientError::UriParseError(config.http_rpc_url.clone()))?;

		let chain_id = client.get_chainid().await.unwrap();

		let wallet = MnemonicBuilder::<English>::default()
			.phrase(&*config.mnemonic)
			.build()
			.unwrap()
			.with_chain_id(chain_id.as_u64());

		let client = ethers::middleware::SignerMiddleware::new(client, wallet);

		Ok(Self { http_rpc: Arc::new(client), ws_uri: config.ws_rpc_url.clone(), config })
	}

	/// produce a stream of events emitted from the contract address for the given block range
	pub fn query_events(
		&self,
		event_name: &str,
		from: BlockNumber,
		to: BlockNumber,
	) -> impl Stream<Item = Log> {
		let filter = Filter::new().from_block(from).to_block(to).address(self.config.ibc_handler_address).event(event_name);
		let client = self.http_rpc.clone();

		async_stream::stream! {
			let logs = client.get_logs(&filter).await.unwrap();
			for log in logs {
				yield log;
			}
		}
	}

	pub fn eth_query_proof(
		&self,
		key: &str,
		block_height: Option<u64>,
		storage_index: u32,
	) -> impl Future<Output = Result<EIP1186ProofResponse, ClientError>> {
		let key = ethers::utils::keccak256(
			&ethers::abi::encode_packed(&[Token::String(key.into())]).unwrap(),
		);

		let key = hex::encode(key);

		let var_name = format!("0x{key}", key = &key);
		let var_name = format!("{storage_index}");
		let index = cast::SimpleCast::index("bytes32", &var_name, &var_name).unwrap();

		let client = self.http_rpc.clone();
		let address = self.config.ibc_handler_address.clone();

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

	pub fn eth_query_proof_2d(
		&self,
		key1: &str,
		key2: &str,
		block_height: Option<u64>,
		storage_index: u32,
	) -> impl Future<Output = Result<EIP1186ProofResponse, ClientError>> {
		let key1 = ethers::utils::keccak256(
			&ethers::abi::encode_packed(&[Token::String(key1.into())]).unwrap(),
		);

		let combined_key1 = [key1.as_slice(), storage_index.to_be_bytes().as_ref()].concat();
		let key1_hashed = ethers::utils::keccak256(&combined_key1);
		let key1_hashed_hex = hex::encode(&key1_hashed);

		let key2 = ethers::utils::keccak256(
			&ethers::abi::encode_packed(&[Token::String(key2.into())]).unwrap(),
		);

		let combined_key2 = [key2.as_slice(), key1_hashed_hex.as_bytes()].concat();
		let key2_hashed = ethers::utils::keccak256(&combined_key2);
		let key2_hashed_hex = hex::encode(&key2_hashed);

		let index = cast::SimpleCast::index("bytes32", &key2_hashed_hex, &key2_hashed_hex).unwrap();

		let client = self.http_rpc.clone();
		let address = self.config.ibc_handler_address.clone();

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

	pub fn query_client_impl_address(
		&self,
		client_id: ClientId,
		at: ibc::Height,
	) -> impl Future<Output = Result<(Vec<u8>, bool), ClientError>> {
		let fut = self.eth_query_proof(
			client_id.as_str(),
			Some(at.revision_height),
			CLIENT_IMPLS_STORAGE_INDEX,
		);

		let contract = crate::contract::light_client_contract(
			self.config.ibc_handler_address.clone(),
			Arc::clone(&self.http_rpc),
		);

		async move {
			let proof = fut.await?;

			if let Some(storage_proof) = proof.storage_proof.first() {
				if !storage_proof.value.is_zero() {
					let binding = contract
						.method("getClientState", (client_id.as_str().to_owned(),))
						.expect("contract is missing getClientState");

					let get_client_state_fut = binding.call();
					let client_state: (Vec<u8>, bool) =
						get_client_state_fut.await.map_err(|err| todo!()).unwrap();

					Ok(client_state)
				} else {
					todo!("error: client address is zero")
				}
			} else {
				todo!("error: no storage proof")
			}
		}
	}

	#[track_caller]
	pub fn has_packet_receipt(
		&self,
		port_id: String,
		channel_id: String,
		sequence: u64,
	) -> impl Future<Output = Result<bool, ClientError>> {
		let client = self.http_rpc.clone();
		let address = self.config.ibc_handler_address.clone();

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
