use crate::{
	chain::consensus_state_abi_token,
	config::EthereumClientConfig,
	contract::UnwrapContractError,
	ibc_provider::{u256_to_bytes, ERC20TOKENABI_ABI},
	jwt::{JwtAuth, JwtKey},
	mock::utils::mock::ClientState,
	utils::{handle_gas_usage, DeployYuiIbc, ProviderImpl},
};
use anyhow::Error;
use async_trait::async_trait;
use ethers::{
	abi::{encode, AbiEncode, Address, Bytes, ParamType, Token},
	core::k256,
	prelude::{
		coins_bip39::English, signer::SignerMiddlewareError, Authorization, BlockId, BlockNumber,
		ContractInstance, EIP1186ProofResponse, Filter, LocalWallet, Log, MnemonicBuilder,
		NameOrAddress, SignerMiddleware, Wallet, H256,
	},
	providers::{Http, Middleware, Provider, ProviderError, ProviderExt, Ws},
	signers::Signer,
	types::U256,
	utils::keccak256,
};
use futures::{FutureExt, Stream, TryFutureExt, TryStreamExt};
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, PrefixedCoin},
	core::ics24_host::{
		error::ValidationError,
		identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	Height,
};
use ibc_primitives::Timeout;
use once_cell::sync::Lazy;
use pallet_ibc::light_clients::{AnyClientState, AnyConsensusState, HostFunctionsManager};
use primitives::{CommonClientState, IbcProvider};
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
use sync_committee_prover::SyncCommitteeProver;
use thiserror::Error;

pub type EthRpcClient = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;
pub(crate) type WsEth = Provider<Ws>;

pub static IBC_STORAGE_SLOT: Lazy<U256> =
	Lazy::new(|| U256::from_big_endian(&keccak256(b"ibc.core")[..]));

// TODO: generate this from the contract automatically
pub const COMMITMENTS_STORAGE_INDEX: u32 = 0;
pub const CLIENT_IMPLS_STORAGE_INDEX: u32 = 3;
pub const CONNECTIONS_STORAGE_INDEX: u32 = 4;
pub const CHANNELS_STORAGE_INDEX: u32 = 5;

#[derive(Debug, Clone)]
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

impl EthereumClient {
	pub async fn new(mut config: EthereumClientConfig) -> Result<Self, ClientError> {
		let client = config.client().await?;

		let yui = match config.yui.take() {
			None =>
				DeployYuiIbc::<_, _>::from_addresses(
					client.clone(),
					config.diamond_address.clone().ok_or_else(|| {
						ClientError::Other("diamond address must be provided".to_string())
					})?,
					Some(config.tendermint_address.clone().ok_or_else(|| {
						ClientError::Other("tendermint address must be provided".to_string())
					})?),
					Some(config.ics20_transfer_bank_address.clone().ok_or_else(|| {
						ClientError::Other("bank address must be provided".to_string())
					})?),
					Some(config.ics20_bank_address.clone().ok_or_else(|| {
						ClientError::Other("bank address must be provided".to_string())
					})?),
					config.diamond_facets.clone(),
				)
				.await?,
			Some(yui) => yui,
		};

		let mut connect_options: PgConnectOptions = config.indexer_pg_url.parse().unwrap();
		connect_options.disable_statement_logging();

		let db_conn = PgPoolOptions::new()
			.max_connections(500)
			.connect_with(connect_options)
			.await
			.expect("Unable to connect to the database");

		let redis = redis::Client::open(config.indexer_redis_url.clone())
			.expect("Unable to connect with Redis server");

		Ok(Self {
			http_rpc: client,
			ws_uri: config.ws_rpc_url.clone(),
			common_state: Default::default(),
			yui,
			client_id: Arc::new(Mutex::new(config.client_id.clone())),
			counterparty_client_id: Arc::new(Mutex::new(None)),
			connection_id: Arc::new(Mutex::new(config.connection_id.clone())),
			channel_whitelist: Arc::new(Mutex::new(
				config.channel_whitelist.clone().into_iter().collect(),
			)),
			config,
			db_conn,
			redis,
		})
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
			dbg!(&secret);
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
			//TODO return error support only tendermint client state
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

	pub async fn get_latest_consensus_state_encoded_abi_token(
		&self,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<Token, ClientError> {
		// return Ok(vec![]);
		let latest_height = self.latest_height_and_timestamp().await?.0;
		//TODO what is the height here?
		let latest_consensus_state = self
			.query_client_consensus_exact_token(latest_height, client_id.clone(), consensus_height)
			.await?;
		Ok(latest_consensus_state)
	}

	pub async fn generated_channel_identifiers(
		&self,
		from_block: BlockNumber,
	) -> Result<Vec<(String, String)>, ClientError> {
		let filter = Filter::new()
			.from_block(self.contract_creation_block())
			//.address(ValueOrArray::Value(self.yui.diamond.address()))
			//.from_block(self.contract_creation_block())
			.to_block(BlockNumber::Latest)
			.address(self.yui.diamond.address())
			.event("OpenInitChannel(string,string)");

		let logs = self.client().get_logs(&filter).await.unwrap();
		unimplemented!();

		let v = logs
			.into_iter()
			.map(|log| {
				let toks =
					ethers::abi::decode(&[ParamType::String, ParamType::String], &log.data.0)
						.unwrap();
				(toks[0].to_string(), toks[1].to_string())
			})
			.collect();

		Ok(v)
	}

	pub async fn generated_client_identifiers(&self, from_block: BlockNumber) -> Vec<String> {
		let filter = Filter::new()
			.from_block(from_block)
			.to_block(BlockNumber::Latest)
			.address(self.yui.diamond.address())
			.event("GeneratedClientIdentifier(string)");

		let logs = self.client().get_logs(&filter).await.unwrap();
		unimplemented!();

		logs.into_iter()
			.map(|log| {
				ethers::abi::decode(&[ParamType::String], &log.data.0)
					.unwrap()
					.into_iter()
					.next()
					.unwrap()
					.to_string()
			})
			.collect()
	}

	pub async fn generated_connection_identifiers(&self, from_block: BlockNumber) -> Vec<String> {
		let filter = Filter::new()
			.from_block(from_block)
			.to_block(BlockNumber::Latest)
			.address(self.yui.diamond.address())
			.event("GeneratedConnectionIdentifier(string)");

		let logs = self.client().get_logs(&filter).await.unwrap();
		unimplemented!();

		// logs.into_iter()
		// 	.map(|log| {
		// 		ethers::abi::decode(&[ParamType::String], &log.data.0)
		// 			.unwrap()
		// 			.into_iter()
		// 			.next()
		// 			.unwrap()
		// 			.to_string()
		// 	})
		// 	.collect()
	}

	pub async fn acknowledge_packets(&self, from_block: BlockNumber) -> Vec<AckPacket> {
		let filter = Filter::new()
			.from_block(from_block)
			.to_block(BlockNumber::Latest)
			.address(self.yui.diamond.address())
			.event("AcknowledgePacket((uint64,string,string,string,string,bytes,(uint64,uint64),uint64),bytes)");

		let logs = self.client().get_logs(&filter).await.unwrap();
		unimplemented!();

		logs.into_iter()
			.map(|log| {
				let decoded = ethers::abi::decode(
					&[
						ParamType::Tuple(vec![
							ParamType::Uint(64),
							ParamType::String,
							ParamType::String,
							ParamType::String,
							ParamType::String,
							ParamType::Bytes,
							ParamType::Tuple(vec![ParamType::Uint(64), ParamType::Uint(64)]),
							ParamType::Uint(64),
						]),
						ParamType::Bytes,
					],
					&log.data.0,
				)
				.unwrap();

				let Token::Tuple(packet) = decoded[0].clone() else {
					panic!("expected tuple, got {:?}", decoded[0])
				};

				// use a match statement to destructure the `packet` into the fields
				// for the `AckPacket` struct
				let (sequence, source_port, source_channel, dest_port, dest_channel, data, timeout_height, timeout_timestamp) = match packet.as_slice() {
					[Token::Uint(sequence),
					Token::String(source_port), Token::String(source_channel), Token::String(dest_port), Token::String(dest_channel), Token::Bytes(data), Token::Tuple(timeout_height), Token::Uint(timeout_timestamp)] => {
						let [Token::Uint(rev), Token::Uint(height)] = timeout_height.as_slice() else {
							panic!("need timeout height to be a tuple of two uints, revision and height");
						};

						(sequence.as_u64(), source_port.clone(), source_channel.clone(), dest_port.clone(), dest_channel.clone(), data.clone(), (
							rev.as_u64(),
							height.as_u64(),
						), timeout_timestamp.as_u64())
					},
					_ => panic!("expected tuple, got {:?}", packet),
				};

				let Token::Bytes(acknowledgement) = decoded[1].clone() else {
					panic!("expected bytes, got {:?}", decoded[1])
				};

				let packet = AckPacket {
					sequence,
					source_port,
					source_channel,
					dest_port,
					dest_channel,
					data,
					timeout_height,
					timeout_timestamp,
					acknowledgement,
				};

				packet
			})
			.collect()
	}

	pub async fn address_of_client_id(&self, client_id: &str) -> Address {
		let proof = self.eth_query_proof(dbg!(client_id), None, 3).await.unwrap();

		match proof.storage_proof.last() {
			Some(proof) => todo!("{:?}", proof.value),
			None => Address::zero(),
		}
	}

	pub fn _query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> impl Future<
		Output = Result<
			ibc_proto::ibc::core::channel::v1::QueryPacketCommitmentResponse,
			ClientError,
		>,
	> {
		async move { todo!() }
	}

	/// produce a stream of events emitted from the contract address for the given block range
	pub fn query_events(
		&self,
		event_name: &str,
		from: BlockNumber,
		to: BlockNumber,
	) -> impl Stream<Item = Log> {
		let filter = Filter::new()
			.from_block(from)
			.to_block(to)
			.address(self.yui.diamond.address())
			.event(event_name);
		let client = self.client().clone();

		unimplemented!();
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
		let key = keccak256(key.as_bytes());
		let var_name = format!("0x{}", hex::encode(key));

		let index = cast::SimpleCast::index(
			"bytes32",
			&var_name,
			&format!("0x{}", hex::encode(IBC_STORAGE_SLOT.add(U256::from(storage_index)).encode())),
		)
		.unwrap();

		let client = self.client().clone();
		let address = self.yui.diamond.address().clone();

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
		let address = self.yui.diamond.address().clone();

		dbg!(&address);
		dbg!(&H256::from_str(&index).unwrap());
		dbg!(&block_height);

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
		let address = self.yui.diamond.address().clone();

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
		at: Height,
	) -> impl Future<Output = Result<(Vec<u8>, bool), ClientError>> + '_ {
		let fut = self.eth_query_proof(
			client_id.as_str(),
			Some(at.revision_height),
			CLIENT_IMPLS_STORAGE_INDEX,
		);

		async move {
			let proof = fut.await?;

			if let Some(storage_proof) = proof.storage_proof.first() {
				if !storage_proof.value.is_zero() {
					let binding = self
						.yui
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
				.map_err(|err| todo!())
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
				.map_err(|err| todo!())
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

			// let receipt_fut = ;
			let receipt: bool = binding
				.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
				.call()
				.await
				.map_err(|err| todo!())
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
			.ics20_bank
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
					self.yui.ics20_bank.as_ref().unwrap().clone().address(),
					params.token.amount.as_u256(),
				),
			)
			.unwrap();

		let _ = method.call().await.unwrap_contract_error();
		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		log::info!("Sent approval transfer. Tx hash: {:?}", receipt.transaction_hash);
		assert_eq!(receipt.status, Some(1.into()));

		// let method = contract
		// 	.method::<_, ()>(
		// 		"approve",
		// 		(
		// 			self.yui.ics20_transfer_bank.as_ref().unwrap().clone().address(),
		// 			params.token.amount.as_u256(),
		// 		),
		// 	)
		// 	.unwrap();

		let _ = method.call().await.unwrap_contract_error();
		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));

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
		let method = self
			.yui
			.ics20_transfer_bank
			.as_ref()
			.expect("expected bank module")
			.method::<_, ()>("sendTransfer", params)?;
		let _ = method.call().await.unwrap_contract_error();
		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		handle_gas_usage(&receipt);
		assert_eq!(receipt.status, Some(1.into()));
		log::info!("Sent transfer. Tx hash: {:?}", receipt.transaction_hash);
		Ok(())
	}

	async fn send_ordered_packet(
		&self,
		channel_id: ChannelId,
		timeout: Timeout,
	) -> Result<(), Self::Error> {
		todo!("send_ordered_packet")
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
	let client = eth_client.config.client().await.unwrap().deref().clone();
	let contract = eth_client.yui.ics20_transfer_bank.as_ref().unwrap().clone();

	let amount = params.token.amount.as_u256();

	let params = (
		params.receiver.to_string(),
		params.source_port.to_string(),
		params.source_channel.to_string(),
		params.timeout_height.revision_height,
		params.timeout_timestamp.nanoseconds(),
		params.memo,
	);

	let method = eth_client
		.yui
		.ics20_transfer_bank
		.as_ref()
		.expect("expected bank module")
		.method::<_, ()>("sendTransferNativeToken", params)?;

	let method = method.value(amount);
	let _ = method.call().await.unwrap_contract_error();
	let receipt = method.send().await.unwrap().await.unwrap().unwrap();
	assert_eq!(receipt.status, Some(1.into()));
	log::info!("Sent ETH transfer. Tx hash: {:?}", receipt.transaction_hash);
	Ok(())
}
