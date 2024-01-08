use crate::{
	chain::consensus_state_abi_token,
	config::{ContractName, EthereumClientConfig},
	contract::UnwrapContractError,
	ibc_provider::{u256_to_bytes, ERC20TOKENABI_ABI},
	jwt::{JwtAuth, JwtKey},
	prove::EventResponse,
	utils::{handle_gas_usage, send_retrying, DeployYuiIbc, ProviderImpl},
};
use anyhow::Error;
use async_trait::async_trait;
use derivative::Derivative;
use ethers::{
	abi::{encode, AbiEncode, Address, Bytes, Token},
	core::k256,
	prelude::{
		signer::SignerMiddlewareError, Authorization, BlockId, BlockNumber, ContractInstance,
		EIP1186ProofResponse, NameOrAddress, SignerMiddleware, Wallet, H256,
	},
	providers::{Http, Middleware, Provider, ProviderError, ProviderExt, Ws},
	signers::Signer,
	types::U256,
	utils::keccak256,
};
use futures::{future::ready, FutureExt, Stream, TryFutureExt, TryStreamExt};
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, PrefixedCoin},
	core::ics24_host::{
		error::ValidationError,
		identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	Height,
};
use ibc_primitives::Timeout;
use log::info;
use once_cell::sync::Lazy;
use pallet_ibc::light_clients::{AnyClientState, HostFunctionsManager};
use primitives::{utils::RecentStream, CommonClientState, IbcProvider};
use reqwest_eventsource::EventSource;
use sqlx::{
	postgres::{PgConnectOptions, PgPoolOptions},
	ConnectOptions,
};
use std::{
	collections::HashSet,
	convert::Infallible,
	future::Future,
	ops::{Add, Deref},
	pin::Pin,
	str::FromStr,
	sync::{Arc, Mutex},
};
use sync_committee_primitives::consensus_types::Checkpoint;
use sync_committee_prover::SyncCommitteeProver;
use thiserror::Error;
use tokio::sync::Mutex as AsyncMutex;

pub type EthRpcClient = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;
pub(crate) type WsEth = Provider<Ws>;

pub static IBC_STORAGE_SLOT: Lazy<U256> =
	Lazy::new(|| U256::from_big_endian(&keccak256(b"ibc.core")[..]));

// TODO: generate this from the contract automatically
pub const COMMITMENTS_STORAGE_INDEX: u32 = 0;
pub const CLIENT_IMPLS_STORAGE_INDEX: u32 = 3;
pub const CONNECTIONS_STORAGE_INDEX: u32 = 4;
pub const CHANNELS_STORAGE_INDEX: u32 = 5;

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct EthereumClient {
	http_rpc: Arc<EthRpcClient>,
	pub(crate) ws_uri: http::Uri,
	pub config: EthereumClientConfig,
	/// Common relayer data
	pub common_state: CommonClientState,
	pub yui: DeployYuiIbc<Arc<ProviderImpl>, ProviderImpl>,
	/// Light client id on counterparty chain
	pub client_id: Arc<Mutex<Option<ClientId>>>,
	/// Light client id on the current chain
	pub counterparty_client_id: Arc<Mutex<Option<ClientId>>>,
	/// Connection Id
	pub connection_id: Arc<Mutex<Option<ConnectionId>>>,
	/// Channels cleared for packet relay
	pub channel_whitelist: Arc<Mutex<HashSet<(ChannelId, PortId)>>>,
	/// Indexer Redis database
	pub redis: redis::Client,
	/// Indexer Postgres database
	pub db_conn: sqlx::Pool<sqlx::Postgres>,
	/// Checkpoint stream
	#[derivative(Debug = "ignore")]
	pub checkpoint_stream:
		Arc<AsyncMutex<Pin<Box<dyn Stream<Item = Checkpoint> + Send + 'static>>>>,
}

pub type MiddlewareErrorType =
	SignerMiddlewareError<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[derive(Debug, Error)]
pub enum ClientError {
	#[error("uri-parse-error: {0} {0}")]
	UriParseError(http::Uri),
	#[error("provider-error: {0}: {0}")]
	ProviderError(http::Uri, ProviderError),
	#[error("Ethereum error: {0}")]
	Ethers(#[from] ProviderError),
	#[error("middleware-error: {0}")]
	MiddlewareError(MiddlewareErrorType),
	#[error("reqwest error: {0}")]
	ReqwestError(#[from] reqwest::Error),
	#[error("Merkleization error: {0}")]
	MerkleizationError(#[from] ssz_rs::MerkleizationError),
	#[error("ABI error: {0}")]
	AbiError(#[from] ethers::abi::Error),
	#[error("Contract ABI error: {0}")]
	ContractAbiError(#[from] ethers::contract::AbiError),
	#[error("Contract error: {0}")]
	ContractError(
		#[from]
		ethers::contract::ContractError<
			SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey<k256::Secp256k1>>>,
		>,
	),
	#[error("IBC Transfer: {0}")]
	IbcTransfer(#[from] ibc::applications::transfer::error::Error),
	#[error("no-storage-proof: there was no storage proof for the given storage index")]
	NoStorageProof,
	#[error("Tendermint error: {0}")]
	Tendermint(#[from] tendermint::Error),
	/// Errors associated with ics-02 client
	#[error("Ibc client error: {0}")]
	IbcClient(#[from] ibc::core::ics02_client::error::Error),
	#[error("Ibc channel error")]
	IbcChannel(#[from] ibc::core::ics04_channel::error::Error),
	#[error("Serde JSON error")]
	JSONError(#[from] serde_json::Error),
	#[error("SQL error")]
	SQLError(#[from] sqlx::Error),
	#[error("{0}")]
	Other(String),
}

impl From<ValidationError> for ClientError {
	fn from(value: ValidationError) -> Self {
		Self::Other(value.to_string())
	}
}

impl From<String> for ClientError {
	fn from(value: String) -> Self {
		Self::Other(value)
	}
}

impl From<anyhow::Error> for ClientError {
	fn from(value: Error) -> Self {
		Self::Other(value.to_string())
	}
}

impl From<Infallible> for ClientError {
	fn from(value: Infallible) -> Self {
		match value {}
	}
}

pub struct AckPacket {
	pub sequence: u64,
	pub source_port: String,
	pub source_channel: String,
	pub dest_port: String,
	pub dest_channel: String,
	pub data: Vec<u8>,
	pub timeout_height: (u64, u64),
	pub timeout_timestamp: u64,
	pub acknowledgement: Vec<u8>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DatabaseLog {
	pub address: String,
	pub chain: String,
	pub data: String,
	pub erc20_transfers_parsed: bool,
	pub hash: String,
	pub log_index: i64,
	pub removed: bool,
	pub topics: Vec<Option<String>>,
}

pub fn subscribe_to_checkpoint_stream(
	node_url: &str,
) -> Pin<Box<dyn Stream<Item = Checkpoint> + Send>> {
	use futures::{Stream, StreamExt};
	let es = EventSource::get(format!("{}/eth/v1/events?topics=finalized_checkpoint", node_url))
		.filter_map(|ev| match ev {
			Ok(reqwest_eventsource::Event::Message(msg)) => {
				info!("Event stream: {:?}", msg);
				let message: EventResponse = serde_json::from_str(&msg.data).unwrap();
				info!("Event stream': {:?}", message);
				ready(Some(Checkpoint {
					epoch: message.epoch.parse().unwrap(),
					root: message.block,
				}))
			},
			Ok(v) => {
				log::error!("Unexpected event: {:?}", v);
				ready(None)
			},
			Err(e) => {
				log::error!("Error in event stream: {:?}", e);
				ready(None)
			},
		});
	RecentStream::new(es.boxed()).boxed()
}

impl EthereumClient {
	pub async fn new(mut config: EthereumClientConfig) -> Result<Self, ClientError> {
		let client = config.client().await?;

		let yui = match config.yui.take() {
			None =>
				DeployYuiIbc::<_, _>::from_addresses(
					client.clone(),
					config.ibc_core_diamond_address.clone().ok_or_else(|| {
						ClientError::Other("diamond address must be provided".to_string())
					})?,
					config.ibc_core_facets.clone(),
					Some(config.tendermint_diamond_address.clone().ok_or_else(|| {
						ClientError::Other("tendermint address must be provided".to_string())
					})?),
					config.tendermint_facets.clone(),
					Some(config.ibc_transfer_diamond_address.clone().ok_or_else(|| {
						ClientError::Other("bank address must be provided".to_string())
					})?),
					config.ibc_transfer_facets.clone(),
					Some(config.bank_diamond_address.clone().ok_or_else(|| {
						ClientError::Other("bank address must be provided".to_string())
					})?),
					config.bank_facets.clone(),
					Some(config.gov_proxy_address.clone().ok_or_else(|| {
						ClientError::Other("government proxy address must be provided".to_string())
					})?),
				)
				.await?,
			Some(yui) => yui,
		};

		let mut connect_options: PgConnectOptions = config.indexer_pg_url.parse().unwrap();
		connect_options.disable_statement_logging();

		let db_conn = PgPoolOptions::new()
			.max_connections(5000)
			.connect_with(connect_options)
			.await
			.expect("Unable to connect to the database");

		let redis = redis::Client::open(config.indexer_redis_url.clone())
			.expect("Unable to connect with Redis server");
		let common_state = CommonClientState::from_config(&config.common);
		Ok(Self {
			http_rpc: client,
			ws_uri: config.ws_rpc_url.clone(),
			common_state: CommonClientState { ..common_state },
			yui,
			client_id: Arc::new(Mutex::new(config.client_id.clone())),
			counterparty_client_id: Arc::new(Mutex::new(None)),
			connection_id: Arc::new(Mutex::new(config.connection_id.clone())),
			channel_whitelist: Arc::new(Mutex::new(
				config.channel_whitelist.clone().into_iter().collect(),
			)),
			checkpoint_stream: Arc::new(AsyncMutex::new(subscribe_to_checkpoint_stream(
				&config.beacon_rpc_url.to_string(),
			))),
			config,
			db_conn,
			redis,
		})
	}

	pub fn contract_address_by_name(&self, contract_name: ContractName) -> Option<Address> {
		self.yui.contract_address_by_name(contract_name).map(|a| a.clone())
	}

	pub fn client(&self) -> Arc<EthRpcClient> {
		self.http_rpc.clone()
	}

	pub fn prover(&self) -> SyncCommitteeProver {
		let mut string = self.config.beacon_rpc_url.to_string();
		string.pop();
		SyncCommitteeProver::new(string)
	}

	pub fn contract_creation_block(&self) -> BlockNumber {
		self.yui.contract_creation_block()
	}

	pub async fn websocket_provider(&self) -> Result<Provider<Ws>, ClientError> {
		if let Some(secret_path) = &self.config.jwt_secret_path {
			let secret = std::fs::read_to_string(secret_path).map_err(|e| {
				ClientError::Other(format!("jwtsecret not found. Search for 'execution/jwtsecret' in the code and replace it with your local path. {e}"))
			})?;
			let secret =
				JwtKey::from_slice(&hex::decode(&secret[2..].trim()).unwrap()).expect("oops");
			let jwt_auth = JwtAuth::new(secret, None, None);
			let token = jwt_auth.generate_token().unwrap();

			let auth = Authorization::bearer(token);
			Provider::<Ws>::connect_with_auth(self.ws_uri.to_string(), auth)
				.await
				.map_err(|e| {
					ClientError::ProviderError(self.ws_uri.clone(), ProviderError::from(e))
				})
		} else {
			Provider::<Ws>::connect(self.ws_uri.to_string()).await.map_err(|e| {
				ClientError::ProviderError(self.ws_uri.clone(), ProviderError::from(e))
			})
		}
	}

	pub async fn get_latest_client_state(
		&self,
		client_id: ClientId,
	) -> Result<ics07_tendermint::client_state::ClientState<HostFunctionsManager>, ClientError> {
		let latest_height = self.latest_height_and_timestamp().await?.0;
		let latest_client_state = AnyClientState::try_from(
			self.query_client_state(latest_height, client_id.clone())
				.await?
				.client_state
				.ok_or_else(|| {
					ClientError::Other("update_client: can't get latest client state".to_string())
				})?,
		)?;
		let AnyClientState::Tendermint(client_state) = latest_client_state.unpack_recursive()
		else {
			return Err(ClientError::Other("create_client: unsupported client state".into()))
		};
		Ok(client_state.clone())
	}

	pub async fn get_latest_client_state_exact_token(
		&self,
		client_id: ClientId,
	) -> Result<(Token, Height), ClientError> {
		let latest_height = self.latest_height_and_timestamp().await?.0;
		let latest_client_state =
			self.query_client_state_exact_token(latest_height, client_id.clone()).await?;

		Ok((latest_client_state, latest_height))
	}

	pub fn eth_query_proof(
		&self,
		key: &str,
		block_height: Option<u64>,
		storage_index: u32,
	) -> impl Future<Output = Result<EIP1186ProofResponse, ClientError>> {
		let key = keccak256(key.as_bytes());
		let var_name = format!("0x{}", hex::encode(key));

		let index = cast::SimpleCast::index(
			"bytes32",
			&var_name,
			&format!("0x{}", hex::encode(IBC_STORAGE_SLOT.add(U256::from(storage_index)).encode())),
		)
		.unwrap();

		let client = self.client().clone();
		let address = self.yui.ibc_core_diamond.address().clone();

		async move {
			Ok(client
				.get_proof(
					NameOrAddress::Address(address),
					vec![H256::from_str(&index).unwrap()],
					block_height.map(BlockId::from),
				)
				.await
				.unwrap())
		}
	}

	pub fn eth_query_proof_tokens(
		&self,
		tokens: &[Token],
		block_height: Option<u64>,
		storage_index: u32,
	) -> impl Future<Output = Result<EIP1186ProofResponse, ClientError>> {
		let vec1 = ethers::abi::encode_packed(tokens).unwrap();
		let key = ethers::utils::keccak256(&vec1);
		let key = hex::encode(key);

		let var_name = format!("0x{key}");
		let storage_index = format!("{storage_index}");
		let index =
			cast::SimpleCast::index("bytes32", dbg!(&var_name), dbg!(&storage_index)).unwrap();

		let client = self.client().clone();
		let address = self.yui.ibc_core_diamond.address().clone();

		async move {
			Ok(client
				.get_proof(
					NameOrAddress::Address(address),
					vec![H256::from_str(&index).unwrap()],
					block_height.map(|i| BlockId::from(i)),
				)
				.await
				.unwrap())
		}
	}

	pub fn eth_query_proof_2d(
		&self,
		key1: &str,
		key2: &str,
		block_height: Option<u64>,
		storage_index: u32,
	) -> impl Future<Output = Result<EIP1186ProofResponse, ClientError>> {
		let key1 = ethers::utils::keccak256(key1.as_bytes());

		let combined_key1 = [key1.as_slice(), storage_index.to_be_bytes().as_ref()].concat();
		let key1_hashed = ethers::utils::keccak256(&combined_key1);
		let key1_hashed_hex = hex::encode(&key1_hashed);

		let key2 = ethers::utils::keccak256(key2.as_bytes());

		let combined_key2 = [key2.as_slice(), key1_hashed_hex.as_bytes()].concat();
		let key2_hashed = ethers::utils::keccak256(&combined_key2);
		let key2_hashed_hex = hex::encode(&key2_hashed);

		let index = cast::SimpleCast::index("bytes32", &key2_hashed_hex, &key2_hashed_hex).unwrap();

		let client = self.client().clone();
		let address = self.yui.ibc_core_diamond.address().clone();

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
		at: Height,
		port_id: String,
		channel_id: String,
		sequence: u64,
	) -> impl Future<Output = Result<bool, ClientError>> + '_ {
		async move {
			let binding = self
				.yui
				.method("hasPacketReceipt", (port_id, channel_id, sequence))
				.expect("contract is missing hasPacketReceipt");

			let receipt: bool = binding
				.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
				.call()
				.await
				.map_err(|err| ClientError::Other(format!("hasPacketReceipt: {}", err.to_string())))
				.unwrap();

			Ok(receipt)
		}
	}

	#[track_caller]
	pub fn has_commitment(
		&self,
		at: Height,
		port_id: String,
		channel_id: String,
		sequence: u64,
	) -> impl Future<Output = Result<bool, ClientError>> + '_ {
		async move {
			let binding = self
				.yui
				.method("hasCommitment", (port_id, channel_id, sequence))
				.expect("contract is missing hasCommitment");

			let receipt: bool = binding
				.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
				.call()
				.await
				.map_err(|err| ClientError::Other(format!("hasCommitment: {}", err.to_string())))
				.unwrap();

			Ok(receipt)
		}
	}

	#[track_caller]
	pub fn has_acknowledgement(
		&self,
		at: Height,
		port_id: String,
		channel_id: String,
		sequence: u64,
	) -> impl Future<Output = Result<bool, ClientError>> + '_ {
		async move {
			let binding = self
				.yui
				.method("hasAcknowledgement", (port_id, channel_id, sequence))
				.expect("contract is missing hasAcknowledgement");

			let receipt: bool = binding
				.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
				.call()
				.await
				.map_err(|err| {
					ClientError::Other(format!("hasAcknowledgement: {}", err.to_string()))
				})
				.unwrap();

			Ok(receipt)
		}
	}

	pub(crate) async fn query_proof_with_value(
		&self,
		path: &str,
		at: Height,
	) -> Result<(Bytes, Vec<u8>), ClientError> {
		let proof = self
			.eth_query_proof(&path, Some(at.revision_height), COMMITMENTS_STORAGE_INDEX)
			.await?;
		// FIXME: verify account proof

		let storage = proof
			.storage_proof
			.first()
			.ok_or(ClientError::Other("storage proof not found".to_string()))?;
		let bytes = u256_to_bytes(&storage.value);

		let proof = encode(&[Token::Array(
			storage.proof.clone().into_iter().map(|p| Token::Bytes(p.to_vec())).collect(),
		)]);

		Ok((proof, bytes))
	}
}

// #[cfg(any(test, feature = "testing"))]
#[async_trait]
impl primitives::TestProvider for EthereumClient {
	async fn send_transfer(&self, params: MsgTransfer<PrefixedCoin>) -> Result<(), Self::Error> {
		if params.token.denom.to_string() == "ETH".to_string() {
			return send_native_eth(&self, params).await
		}
		let method = self
			.yui
			.bank_diamond
			.as_ref()
			.expect("expected bank module")
			.method::<_, Address>("queryTokenContractFromDenom", params.token.denom.to_string())?;
		let erc20_address = method.call().await.unwrap_contract_error();
		assert_ne!(erc20_address, Address::zero(), "erc20 address is zero");

		let client = self.config.client().await.unwrap().deref().clone();
		let contract =
			ContractInstance::<_, _>::new(erc20_address, ERC20TOKENABI_ABI.clone(), client);
		let method = contract
			.method::<_, ()>(
				"approve",
				(
					self.yui.bank_diamond.as_ref().unwrap().clone().address(),
					params.token.amount.as_u256(),
				),
			)
			.unwrap();

		let _ = method.call().await.unwrap_contract_error();
		send_retrying(&method).await.unwrap();

		// let method = contract
		// 	.method::<_, ()>(
		// 		"approve",
		// 		(
		// 			self.yui.ibc_transfer_diamond.as_ref().unwrap().clone().address(),
		// 			params.token.amount.as_u256(),
		// 		),
		// 	)
		// 	.unwrap();

		let params = (
			params.token.denom.to_string(),
			params.token.amount.as_u256(),
			params.receiver.to_string(),
			params.source_port.to_string(),
			params.source_channel.to_string(),
			params.timeout_height.revision_height,
			params.timeout_timestamp.nanoseconds(),
			params.memo,
		);
		let method = self.yui.method_diamond::<_, ()>(
			"sendTransfer",
			params,
			ContractName::IbcTransferDiamond,
		)?;
		let _ = method.call().await.unwrap_contract_error();
		let receipt = send_retrying(&method).await.unwrap();
		handle_gas_usage(&receipt);
		assert_eq!(receipt.status, Some(1.into()));
		log::info!("Sent transfer. Tx hash: {:?}", receipt.transaction_hash);
		Ok(())
	}

	async fn send_ordered_packet(
		&self,
		_channel_id: ChannelId,
		_timeout: Timeout,
	) -> Result<(), Self::Error> {
		unimplemented!("send_ordered_packet")
	}

	async fn subscribe_blocks(&self) -> Pin<Box<dyn Stream<Item = u64> + Send>> {
		use ethers_providers::StreamExt;
		let ws = self.websocket_provider().await.unwrap();
		let stream = async_stream::stream! {
			// TODO: is it really finalized blocks stream?
			let mut stream = ws.subscribe_blocks().await.expect("failed to subscribe to blocks");

			while let Some(block) = stream.next().await {
				yield block.number.unwrap().as_u64()
			}
		};

		Box::pin(stream)
	}

	async fn increase_counters(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}
}

async fn send_native_eth(
	eth_client: &EthereumClient,
	params: MsgTransfer<PrefixedCoin>,
) -> Result<(), ClientError> {
	let amount = params.token.amount.as_u256();

	let params = (
		params.receiver.to_string(),
		params.source_port.to_string(),
		params.source_channel.to_string(),
		params.timeout_height.revision_height,
		params.timeout_timestamp.nanoseconds(),
		params.memo,
	);

	let method = eth_client.yui.method_diamond::<_, ()>(
		"sendTransferNativeToken",
		params,
		ContractName::IbcTransferDiamond,
	)?;

	let method = method.value(amount);
	let _ = method.call().await.unwrap_contract_error();
	let receipt = send_retrying(&method).await.unwrap();
	assert_eq!(receipt.status, Some(1.into()));
	log::info!("Sent ETH transfer. Tx hash: {:?}", receipt.transaction_hash);
	Ok(())
}
