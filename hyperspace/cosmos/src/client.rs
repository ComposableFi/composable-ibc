#![allow(clippy::all)]
use super::{
	key_provider::KeyEntry,
	light_client::LightClient,
	tx::{broadcast_tx, confirm_tx, sign_tx, simulate_tx},
};
use crate::error::Error;
use bech32::ToBase32;
use bip32::{DerivationPath, ExtendedPrivateKey, XPrv, XPub as ExtendedPublicKey};
use core::convert::{From, Into, TryFrom};
use digest::Digest;
use ibc::core::{
	ics02_client::height::Height,
	ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes},
	ics24_host::{
		identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
		IBC_QUERY_PATH,
	},
};
use ibc_proto::{
	cosmos::auth::v1beta1::{query_client::QueryClient, BaseAccount, QueryAccountRequest},
	google::protobuf::Any,
};
use ics07_tendermint::{
	client_message::Header, client_state::ClientState, consensus_state::ConsensusState,
	merkle::convert_tm_to_ics_merkle_proof,
};
use pallet_ibc::light_clients::{AnyClientState, AnyConsensusState, HostFunctionsManager};
use primitives::{
	Chain, CommonClientConfig, CommonClientState, IbcProvider, KeyProvider, UpdateType,
};
use prost::Message;
use quick_cache::sync::Cache;
use rand::Rng;
use ripemd::Ripemd160;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashSet,
	str::FromStr,
	sync::{Arc, Mutex},
	time::{Duration, SystemTime},
};
use tendermint::{block::Height as TmHeight, Hash};
use tendermint_light_client::components::io::{AtHeight, Io};
use tendermint_light_client_verifier::types::{LightBlock, ValidatorSet};
use tendermint_rpc::{endpoint::abci_query::AbciQuery, Client, HttpClient, Url, WebSocketClient};
use tokio::{
	sync::{Mutex as TokioMutex, Mutex as AsyncMutex},
	task::{JoinHandle, JoinSet},
	time::{error::Elapsed, sleep, timeout},
};

const DEFAULT_FEE_DENOM: &str = "stake";
const DEFAULT_FEE_AMOUNT: &str = "400000000000000000";
const DEFAULT_GAS_LIMIT: u64 = (i64::MAX - 1) as u64;

fn default_gas_limit() -> u64 {
	DEFAULT_GAS_LIMIT
}

fn default_fee_denom() -> String {
	DEFAULT_FEE_DENOM.to_string()
}

fn default_fee_amount() -> String {
	DEFAULT_FEE_AMOUNT.to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigKeyEntry {
	pub public_key: String,
	pub private_key: String,
	pub account: String,
	pub address: Vec<u8>,
}

impl TryFrom<ConfigKeyEntry> for KeyEntry {
	type Error = bip32::Error;

	fn try_from(value: ConfigKeyEntry) -> Result<Self, Self::Error> {
		Ok(KeyEntry {
			public_key: ExtendedPublicKey::from_str(&value.public_key)?,
			private_key: ExtendedPrivateKey::from_str(&value.private_key)?,
			account: value.account,
			address: value.address,
		})
	}
}

impl TryFrom<MnemonicEntry> for KeyEntry {
	type Error = bip32::Error;

	fn try_from(mnemonic_entry: MnemonicEntry) -> Result<Self, Self::Error> {
		// From mnemonic to pubkey
		let mnemonic =
			bip39::Mnemonic::from_phrase(&mnemonic_entry.mnemonic, bip39::Language::English)
				.unwrap();
		let seed = bip39::Seed::new(&mnemonic, "");
		let key_m = XPrv::derive_from_path(seed, &DerivationPath::from_str("m/44'/118'/0'/0/0")?)?;

		// From pubkey to address
		let sha256 = sha2::Sha256::digest(key_m.public_key().to_bytes());
		let public_key_hash: [u8; 20] = Ripemd160::digest(sha256).into();
		let account = bech32::encode(
			&mnemonic_entry.prefix,
			public_key_hash.to_base32(),
			bech32::Variant::Bech32,
		)
		.unwrap();
		Ok(KeyEntry {
			public_key: key_m.public_key(),
			private_key: key_m,
			account,
			address: public_key_hash.into(),
		})
	}
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MnemonicEntry {
	pub mnemonic: String,
	pub prefix: String,
}
// Implements the [`crate::Chain`] trait for cosmos.
/// This is responsible for:
/// 1. Tracking a cosmos light client on a counter-party chain, advancing this light
/// client state  as new finality proofs are observed.
/// 2. Submitting new IBC messages to this cosmos.
#[derive(Clone)]
pub struct CosmosClient<H> {
	/// Chain name
	pub name: String,
	/// Chain rpc client
	pub rpc_client: WebSocketClient,
	/// Chain http rpc client
	pub rpc_http_client: HttpClient,
	/// Reusable GRPC client
	pub grpc_client: tonic::transport::Channel,
	/// Chain rpc address
	pub rpc_url: Url,
	/// Chain grpc address
	pub grpc_url: Url,
	/// Websocket chain ws client
	pub websocket_url: Url,
	/// Chain Id
	pub chain_id: ChainId,
	/// Light client id on counterparty chain
	pub client_id: Arc<Mutex<Option<ClientId>>>,
	/// Connection Id
	pub connection_id: Arc<Mutex<Option<ConnectionId>>>,
	/// Channels cleared for packet relay
	pub channel_whitelist: Arc<Mutex<HashSet<(ChannelId, PortId)>>>,
	/// Light Client instance
	pub light_client: LightClient,
	/// The key that signs transactions
	pub keybase: KeyEntry,
	/// Account prefix
	pub account_prefix: String,
	/// Reference to commitment
	pub commitment_prefix: CommitmentPrefix,
	/// Fee denom
	pub fee_denom: String,
	/// Fee amount
	pub fee_amount: String,
	/// Fee amount
	pub gas_limit: u64,
	/// Maximun transaction size
	pub max_tx_size: usize,
	/// Finality protocol to use, eg Tenderminet
	pub _phantom: std::marker::PhantomData<H>,
	/// Mutex used to sequentially send transactions. This is necessary because
	/// account sequence numbers are not updated until the transaction is processed.
	pub tx_mutex: Arc<tokio::sync::Mutex<()>>,
	/// Light-client blocks cache
	pub light_block_cache: Arc<Cache<TmHeight, LightBlock>>,
	/// Relayer data
	pub common_state: CommonClientState,
	/// Join handles for spawned tasks
	pub join_handles: Arc<TokioMutex<Vec<JoinHandle<Result<(), tendermint_rpc::Error>>>>>,
}

/// config options for [`ParachainClient`]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CosmosClientConfig {
	/// Chain name
	pub name: String,
	/// rpc url for cosmos
	pub rpc_url: Url,
	/// grpc url for cosmos
	pub grpc_url: Url,
	/// websocket url for cosmos
	pub websocket_url: Url,
	/// Cosmos chain Id
	pub chain_id: String,
	/// Light client id on counterparty chain
	pub client_id: Option<ClientId>,
	/// Connection Id
	pub connection_id: Option<ConnectionId>,
	/// Account prefix
	pub account_prefix: String,
	/// Fee denom
	#[serde(default = "default_fee_denom")]
	pub fee_denom: String,
	/// Fee amount
	#[serde(default = "default_fee_amount")]
	pub fee_amount: String,
	/// Fee amount
	#[serde(default = "default_gas_limit")]
	pub gas_limit: u64,
	/// Store prefix
	pub store_prefix: String,
	/// Maximun transaction size
	pub max_tx_size: usize,
	/// All the client states and headers will be wrapped in WASM ones using the WASM code ID.
	#[serde(default)]
	pub wasm_code_id: Option<String>,
	/*
	Here is a list of dropped configuration parameters from Hermes Config.toml
	that could be set to default values or removed for the MVP phase:

	pub key_store_type: Store,					// TODO: Could be set to any of SyncCryptoStorePtr or KeyStore or KeyEntry types, but not sure yet
	pub rpc_timeout: Duration,				    // TODO: Could be set to '15s' by default
	pub default_gas: Option<u64>,	  			// TODO: Could be set to `0` by default
	pub max_gas: Option<u64>,                   // TODO: DEFAULT_MAX_GAS: u64 = 400_000
	pub gas_multiplier: Option<GasMultiplier>,  // TODO: Could be set to `1.1` by default
	pub fee_granter: Option<String>,            // TODO: DEFAULT_FEE_GRANTER: &str = ""
	pub max_msg_num: MaxMsgNum,                 // TODO: Default is 30, Could be set usize = 1 for test
												// TODO: Could be set to const MAX_LEN: usize = 50;
	pub proof_specs: Option<ProofSpecs>,        // TODO: Could be set to None
	pub sequential_batch_tx: bool,			    // TODO: sequential_send_batched_messages_and_wait_commit() or send_batched_messages_and_wait_commit() ?
	pub trust_threshold: TrustThreshold,
	pub gas_price: GasPrice,   				    // TODO: Could be set to `0`
	pub packet_filter: PacketFilter,            // TODO: AllowAll
	pub address_type: AddressType,			    // TODO: Type = cosmos
	pub extension_options: Vec<ExtensionOption>,// TODO: Could be set to None
	*/
	/// Whitelisted channels
	pub channel_whitelist: Vec<(ChannelId, PortId)>,
	/// The key that signs transactions
	pub mnemonic: String,
	/// Common client config
	#[serde(flatten)]
	pub common: CommonClientConfig,
	/// Skip transfer packets with the following tokens base denoms
	pub skip_tokens_list: Option<Vec<String>>,
}

impl<H> CosmosClient<H>
where
	Self: KeyProvider,
	H: Clone + Send + Sync + 'static,
{
	/// Initializes a [`CosmosClient`] given a [`CosmosClientConfig`]
	pub async fn new(config: CosmosClientConfig) -> Result<Self, Error> {
		let (rpc_client, rpc_driver) = WebSocketClient::new(config.websocket_url.clone())
			.await
			.map_err(|e| Error::RpcError(format!("failed to connect to Websocket {:?}", e)))?;
		let rpc_http_client = HttpClient::new(config.rpc_url.clone())
			.map_err(|e| Error::RpcError(format!("failed to connect to RPC {:?}", e)))?;
		let ws_driver_jh = tokio::spawn(rpc_driver.run());
		let grpc_client = tonic::transport::Endpoint::new(config.grpc_url.to_string())
			.map_err(|e| Error::RpcError(format!("failed to create a GRPC endpoint {:?}", e)))?
			.connect()
			.await
			.map_err(|e| Error::RpcError(format!("failed to connect to GRPC {:?}", e)))?;

		let chain_id = ChainId::from(config.chain_id);
		let light_client =
			LightClient::init_light_client(config.rpc_url.clone(), Duration::from_secs(10)).await?;
		let commitment_prefix = CommitmentPrefix::try_from(config.store_prefix.as_bytes().to_vec())
			.map_err(|e| Error::from(format!("Invalid store prefix {:?}", e)))?;

		let keybase: KeyEntry = KeyEntry::try_from(MnemonicEntry {
			mnemonic: config.mnemonic,
			prefix: config.account_prefix.clone(),
		})
		.map_err(|e| e.to_string())?;

		let rpc_call_delay = Duration::from_millis(50);
		Ok(Self {
			name: config.name,
			chain_id,
			rpc_client,
			rpc_http_client,
			grpc_client,
			rpc_url: config.rpc_url,
			grpc_url: config.grpc_url,
			websocket_url: config.websocket_url,
			client_id: Arc::new(Mutex::new(config.client_id)),
			connection_id: Arc::new(Mutex::new(config.connection_id)),
			channel_whitelist: Arc::new(Mutex::new(config.channel_whitelist.into_iter().collect())),
			light_client,
			account_prefix: config.account_prefix,
			commitment_prefix,
			fee_denom: config.fee_denom,
			fee_amount: config.fee_amount,
			gas_limit: config.gas_limit,
			max_tx_size: config.max_tx_size,
			keybase,
			_phantom: std::marker::PhantomData,
			tx_mutex: Default::default(),
			light_block_cache: Arc::new(Cache::new(100000)),
			common_state: CommonClientState {
				skip_optional_client_updates: config.common.skip_optional_client_updates,
				maybe_has_undelivered_packets: Default::default(),
				rpc_call_delay,
				initial_rpc_call_delay: rpc_call_delay,
				misbehaviour_client_msg_queue: Arc::new(AsyncMutex::new(vec![])),
				max_packets_to_process: config.common.max_packets_to_process as usize,
				// skip_tokens_list: config.skip_tokens_list.unwrap_or_default(),
				ignored_timeouted_sequences: Arc::new(AsyncMutex::new(HashSet::new())),
				client_update_interval: Duration::from_secs(1),
				last_client_update_time: SystemTime::now(),
				handshake_completed: false,
			},
			join_handles: Arc::new(TokioMutex::new(vec![ws_driver_jh])),
		})
	}

	pub fn client_id(&self) -> ClientId {
		self.client_id
			.lock()
			.unwrap()
			.as_ref()
			.expect("Client Id should be defined")
			.clone()
	}

	pub fn set_client_id(&mut self, client_id: ClientId) {
		*self.client_id.lock().unwrap() = Some(client_id);
	}

	/// Construct a tendermint client state to be submitted to the counterparty chain
	pub async fn construct_tendermint_client_state(
		&self,
	) -> Result<(ClientState<HostFunctionsManager>, ConsensusState), Error>
	where
		Self: KeyProvider + IbcProvider,
		H: Clone + Send + Sync + 'static,
	{
		let (client_state, consensus_state) =
			self.initialize_client_state().await.map_err(|e| {
				Error::from(format!(
					"Failed to initialize client state for chain {:?} with error {:?}",
					self.name, e
				))
			})?;
		match (client_state, consensus_state) {
			(
				AnyClientState::Tendermint(client_state),
				AnyConsensusState::Tendermint(consensus_state),
			) => Ok((client_state, consensus_state)),
			_ => Err(Error::from(format!(
				"Failed to initialize client state for chain {:?}",
				self.name
			))),
		}
	}

	pub async fn submit_call(&self, messages: Vec<Any>) -> Result<Hash, Error> {
		let _lock = self.tx_mutex.lock().await;
		let account_info = self.query_account().await?;

		// Sign transaction
		let (tx, _, tx_bytes) = sign_tx(
			self.keybase.clone(),
			self.chain_id.clone(),
			&account_info,
			messages,
			self.get_fee(),
		)?;

		log::info!("i came here 1");

		// Simulate transaction
		let res = simulate_tx(self.grpc_url.clone(), tx, tx_bytes.clone()).await?;
		res.result
			.map(|r| log::info!(target: "hyperspace_cosmos", "Simulated transaction: events: {:?}\nlogs: {}", r.events, r.log));

		log::info!("i came here 2");
		// Broadcast transaction
		let hash = broadcast_tx(&self.rpc_client, tx_bytes).await?;
		log::info!(target: "hyperspace_cosmos", "🤝 Transaction sent with hash: {:?}", hash);

		// wait for confirmation
		confirm_tx(&self.rpc_client, hash).await
		// Ok(hash)
	}

	pub async fn fetch_light_block_with_cache(
		&self,
		height: TmHeight,
		sleep_duration: Duration,
	) -> Result<LightBlock, Error> {
		let fut = async move {
			sleep(sleep_duration).await;
			self.fetch_light_block(AtHeight::At(height), self.light_client.peer_id.clone())
				.await
				.map_err(|e| {
					Error::from(format!(
						"Failed to fetch light block for chain {:?} with error {:?}",
						self.name, e
					))
				})
		};
		self.light_block_cache.get_or_insert_async(&height, fut).await
	}

	pub async fn msg_update_client_header(
		&self,
		from: TmHeight,
		to: TmHeight,
		trusted_height: Height,
	) -> Result<Vec<(Header, UpdateType)>, Error> {
		let from = from.increment();
		let mut xs = Vec::new();
		let heightss = (from.value()..=to.value()).collect::<Vec<_>>();
		let client = Arc::new(self.clone());
		let delay_to = self.rpc_call_delay().as_millis();
		for heights in heightss.chunks(100) {
			let mut join_set = JoinSet::<Result<Result<_, Error>, Elapsed>>::new();
			for height in heights.to_owned() {
				let client = client.clone();
				let duration =
					Duration::from_millis(rand::thread_rng().gen_range(0..delay_to) as u64);
				let fut = async move {
					log::trace!(target: "hyperspace_cosmos", "Fetching header at height {:?}", height);
					let latest_light_block =
						client.fetch_light_block_with_cache(height.try_into()?, duration).await?;

					let height =
						TmHeight::try_from(trusted_height.revision_height).map_err(|e| {
							Error::from(format!(
								"Failed to convert height for chain {:?} with error {:?}",
								client.name, e
							))
						})?;

					let trusted_light_block =
						client.fetch_light_block_with_cache(height.increment(), duration).await?;

					let update_type = match is_validators_equal(
						&latest_light_block.validators,
						&latest_light_block.next_validators,
					) {
						true => UpdateType::Optional,
						false => UpdateType::Mandatory,
					};

					Ok((
						Header {
							signed_header: latest_light_block.signed_header,
							validator_set: latest_light_block.validators,
							trusted_height,
							trusted_validator_set: trusted_light_block.validators,
						},
						update_type,
					))
				};
				join_set.spawn(timeout(Duration::from_secs(30), fut));
			}
			while let Some(res) = join_set.join_next().await {
				xs.push(res.map_err(|e| Error::Custom(e.to_string()))?.map_err(|_| {
					Error::Custom("failed to fetch light block: timeout".to_string())
				})??);
			}
		}
		xs.sort_by_key(|(h, _)| h.signed_header.header.height.value());
		Ok(xs)
	}

	/// Uses the GRPC client to retrieve the account sequence
	pub async fn query_account(&self) -> Result<BaseAccount, Error> {
		let mut client = QueryClient::connect(self.grpc_url.clone().to_string())
			.await
			.map_err(|e| Error::from(format!("GRPC client error: {:?}", e)))?;

		let request =
			tonic::Request::new(QueryAccountRequest { address: self.keybase.account.to_string() });

		let response = client.account(request).await;

		// Querying for an account might fail, i.e. if the account doesn't actually exist
		let resp_account =
			match response.map_err(|e| Error::from(format!("{:?}", e)))?.into_inner().account {
				Some(account) => account,
				None => return Err(Error::from(format!("Account not found"))),
			};

		Ok(BaseAccount::decode(resp_account.value.as_slice())
			.map_err(|e| Error::from(format!("Failed to decode account {}", e)))?)
	}

	pub async fn query_path(
		&self,
		data: Vec<u8>,
		height_query: Height,
		prove: bool,
	) -> Result<(AbciQuery, Vec<u8>), Error> {
		let path = IBC_QUERY_PATH;
		let height = TmHeight::try_from(height_query.revision_height)
			.map_err(|e| Error::from(format!("Invalid height {}", e)))?;

		let height = match height.value() {
			0 => None,
			_ => Some(height),
		};

		// Use the Tendermint-rs RPC client to do the query.
		let response = self
			.rpc_http_client
			.abci_query(Some(path.to_owned()), data.clone(), height, prove)
			.await
			.map_err(|e| {
				Error::from(format!("Failed to query chain {} with error {:?}", self.name, e))
			})?;

		if !response.code.is_ok() {
			// Fail with response log.
			return Err(Error::from(format!(
				"Query failed with code {:?} and log {:?}",
				response.code, response.log
			)));
		}

		if prove && response.proof.is_none() {
			// Fail due to empty proof
			return Err(Error::from(format!(
				"Query failed due to empty proof for chain {}",
				self.name
			)));
		}

		let merkle_proof = response
			.clone()
			.proof
			.map(|p| convert_tm_to_ics_merkle_proof::<H>(&p))
			.transpose()
			.map_err(|_| Error::Custom(format!("bad client state proof")))?
			.ok_or_else(|| Error::Custom(format!("proof not found")))?;
		let proof = CommitmentProofBytes::try_from(merkle_proof)
			.map_err(|err| Error::Custom(format!("bad client state proof: {}", err)))?;
		Ok((response, proof.into()))
	}
}

/// Checks that the two validator sets are equal. The default implementation
/// of `Eq` cannot be used, because the `proposer` should be ignored.
fn is_validators_equal(set_a: &ValidatorSet, set_b: &ValidatorSet) -> bool {
	set_a.hash() == set_b.hash()
}

#[tokio::test]
pub fn testing() {
	let payload = Any {
		type_url: "/ibc.core.client.v1.MsgUpdateClient".to_string(),
		value: vec![
			10, 11, 48, 56, 45, 119, 97, 115, 109, 45, 50, 49, 50, 18, 215, 19, 10, 32, 47, 105,
			98, 99, 46, 108, 105, 103, 104, 116, 99, 108, 105, 101, 110, 116, 115, 46, 119, 97,
			115, 109, 46, 118, 49, 46, 72, 101, 97, 100, 101, 114, 18, 178, 19, 10, 167, 19, 10,
			36, 47, 108, 105, 103, 104, 116, 99, 108, 105, 101, 110, 116, 115, 46, 103, 117, 101,
			115, 116, 46, 118, 49, 46, 67, 108, 105, 101, 110, 116, 77, 101, 115, 115, 97, 103,
			101, 18, 254, 18, 10, 251, 18, 10, 32, 123, 13, 40, 181, 204, 199, 196, 48, 119, 192,
			246, 129, 83, 218, 219, 81, 219, 36, 192, 163, 160, 213, 120, 56, 4, 43, 98, 24, 93,
			171, 141, 175, 18, 122, 0, 143, 242, 38, 100, 127, 242, 99, 237, 211, 102, 176, 60,
			221, 133, 133, 212, 86, 246, 53, 251, 197, 3, 192, 102, 88, 44, 166, 179, 123, 160, 25,
			200, 215, 179, 0, 0, 0, 0, 0, 0, 80, 183, 198, 15, 0, 0, 0, 0, 0, 124, 140, 29, 8, 176,
			205, 23, 23, 207, 52, 225, 200, 44, 0, 197, 45, 139, 142, 92, 20, 27, 90, 132, 70, 148,
			127, 175, 22, 159, 97, 123, 31, 137, 57, 238, 240, 35, 133, 229, 143, 242, 38, 100,
			127, 242, 99, 237, 211, 102, 176, 60, 221, 133, 133, 212, 86, 246, 53, 251, 197, 3,
			192, 102, 88, 44, 166, 179, 123, 160, 25, 200, 0, 26, 252, 8, 0, 23, 0, 0, 0, 0, 14,
			98, 165, 230, 42, 119, 161, 28, 123, 78, 137, 248, 80, 181, 149, 183, 20, 27, 15, 13,
			99, 90, 41, 65, 207, 252, 50, 226, 118, 151, 173, 129, 236, 64, 206, 197, 16, 1, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 115, 173, 50, 243, 91, 161, 194, 83, 216, 242, 66, 41, 33,
			243, 209, 58, 218, 115, 252, 135, 181, 220, 204, 200, 118, 189, 99, 55, 5, 217, 248,
			73, 136, 60, 254, 218, 207, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 185, 255, 169, 227,
			136, 152, 166, 152, 175, 216, 37, 170, 86, 183, 15, 33, 126, 60, 128, 206, 119, 43, 53,
			163, 196, 210, 217, 11, 114, 121, 31, 224, 149, 191, 43, 108, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 142, 232, 23, 145, 191, 183, 8, 213, 52, 206, 132, 209, 242, 203, 202, 67,
			0, 219, 115, 214, 1, 62, 250, 107, 22, 8, 44, 13, 112, 109, 230, 76, 224, 119, 89, 61,
			42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 154, 76, 48, 35, 198, 93, 88, 130, 88, 239,
			184, 111, 16, 91, 246, 60, 17, 213, 1, 189, 48, 53, 49, 33, 142, 158, 150, 113, 26,
			210, 238, 144, 32, 59, 116, 126, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 180, 227, 19,
			155, 201, 182, 229, 180, 220, 130, 231, 132, 158, 34, 100, 98, 1, 23, 204, 199, 78, 23,
			109, 97, 233, 231, 152, 244, 202, 89, 169, 183, 128, 253, 28, 251, 30, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 39, 194, 208, 173, 186, 210, 70, 234, 98, 244, 168, 67, 72, 15,
			245, 31, 47, 222, 38, 75, 114, 124, 7, 192, 75, 193, 79, 66, 196, 25, 82, 251, 0, 48,
			53, 63, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 91, 51, 222, 11, 137, 194, 72, 44, 128,
			241, 60, 206, 236, 36, 132, 38, 5, 146, 232, 206, 157, 138, 134, 233, 23, 218, 222, 60,
			184, 108, 117, 143, 0, 222, 186, 203, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 139,
			140, 216, 11, 143, 48, 125, 165, 70, 234, 97, 41, 193, 125, 72, 131, 93, 180, 31, 220,
			22, 9, 140, 185, 225, 152, 13, 209, 0, 162, 93, 64, 221, 195, 192, 8, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 222, 212, 130, 113, 181, 245, 17, 146, 247, 19, 183, 86, 253, 102,
			87, 9, 184, 8, 253, 150, 110, 72, 208, 219, 17, 151, 114, 46, 84, 155, 214, 7, 32, 153,
			128, 76, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 114, 127, 72, 161, 154, 141, 37,
			66, 18, 179, 185, 164, 188, 11, 7, 224, 144, 38, 93, 49, 242, 220, 239, 211, 164, 47,
			0, 223, 5, 239, 84, 128, 9, 110, 27, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130, 222,
			244, 156, 213, 16, 5, 228, 33, 16, 207, 38, 134, 233, 211, 126, 247, 20, 116, 236, 3,
			179, 249, 162, 231, 6, 24, 109, 14, 137, 141, 251, 0, 219, 41, 24, 6, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 195, 156, 72, 197, 101, 19, 160, 98, 204, 76, 95, 95, 92, 243, 222,
			84, 204, 16, 197, 102, 94, 181, 53, 194, 146, 171, 236, 33, 210, 178, 15, 89, 0, 132,
			184, 13, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 157, 78, 54, 19, 200, 189, 204, 183,
			67, 126, 218, 132, 26, 144, 168, 206, 44, 147, 137, 172, 245, 203, 216, 96, 146, 42,
			237, 202, 223, 20, 34, 211, 0, 161, 42, 212, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55,
			195, 90, 194, 134, 75, 145, 88, 131, 32, 244, 145, 50, 107, 118, 26, 61, 62, 129, 127,
			137, 138, 20, 8, 103, 183, 203, 254, 218, 178, 49, 52, 64, 252, 44, 210, 5, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 7, 140, 204, 32, 170, 188, 122, 71, 237, 4, 13, 68, 161, 102,
			220, 95, 71, 40, 76, 23, 231, 84, 189, 216, 54, 181, 106, 205, 250, 0, 68, 3, 0, 186,
			29, 210, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 46, 14, 108, 113, 154, 250, 108,
			59, 57, 149, 92, 65, 241, 176, 124, 147, 179, 103, 14, 132, 120, 165, 51, 194, 202,
			181, 32, 154, 88, 24, 152, 0, 186, 29, 210, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128,
			236, 16, 194, 7, 176, 190, 98, 206, 219, 114, 42, 164, 92, 131, 86, 33, 14, 115, 56,
			105, 185, 54, 111, 171, 51, 168, 38, 115, 235, 104, 67, 0, 186, 29, 210, 5, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 140, 201, 58, 244, 143, 10, 170, 135, 228, 157, 208, 234, 121,
			35, 224, 139, 239, 235, 152, 213, 172, 40, 1, 227, 5, 209, 60, 14, 85, 88, 147, 65, 0,
			186, 29, 210, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 155, 71, 165, 16, 121, 22, 155,
			83, 222, 160, 107, 254, 214, 91, 29, 68, 90, 77, 131, 118, 76, 232, 134, 114, 147, 128,
			134, 197, 64, 38, 179, 74, 0, 186, 29, 210, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 197,
			241, 178, 229, 147, 67, 0, 170, 74, 238, 148, 173, 22, 33, 191, 141, 105, 83, 18, 150,
			33, 46, 179, 70, 162, 94, 240, 53, 39, 59, 109, 60, 0, 186, 29, 210, 5, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 200, 228, 51, 63, 11, 59, 196, 108, 72, 51, 221, 152, 236, 196,
			137, 138, 27, 249, 219, 240, 201, 202, 236, 175, 35, 234, 159, 147, 98, 9, 56, 223, 0,
			186, 29, 210, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 203, 39, 182, 59, 194, 141, 218,
			197, 115, 252, 154, 43, 116, 28, 189, 182, 62, 83, 62, 167, 146, 229, 85, 88, 97, 243,
			36, 199, 146, 167, 16, 10, 0, 186, 29, 210, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 251,
			142, 32, 10, 160, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 68, 8, 10, 18, 64, 61, 111, 63,
			58, 5, 250, 130, 247, 6, 27, 112, 39, 141, 115, 38, 92, 141, 76, 111, 85, 121, 57, 71,
			240, 167, 221, 2, 115, 74, 90, 156, 218, 252, 155, 90, 25, 52, 163, 101, 241, 201, 208,
			123, 146, 246, 99, 25, 85, 183, 77, 56, 135, 166, 6, 94, 121, 28, 239, 238, 5, 94, 240,
			178, 14, 34, 68, 8, 3, 18, 64, 201, 166, 216, 54, 72, 23, 146, 254, 66, 130, 133, 86,
			182, 109, 210, 41, 172, 100, 10, 142, 223, 167, 129, 131, 134, 104, 218, 113, 194, 62,
			21, 107, 56, 122, 192, 57, 89, 130, 30, 171, 150, 71, 211, 181, 219, 193, 97, 105, 135,
			239, 247, 126, 238, 179, 42, 235, 120, 43, 216, 34, 196, 239, 144, 0, 34, 68, 8, 14,
			18, 64, 214, 111, 155, 98, 244, 254, 93, 15, 145, 54, 16, 223, 194, 193, 56, 91, 158,
			111, 65, 156, 73, 135, 58, 201, 238, 185, 117, 165, 215, 166, 194, 167, 141, 212, 198,
			20, 111, 171, 101, 40, 157, 234, 153, 79, 127, 11, 37, 41, 86, 49, 62, 107, 49, 101,
			76, 137, 238, 47, 190, 237, 149, 241, 120, 0, 34, 68, 8, 20, 18, 64, 100, 166, 154,
			124, 140, 238, 9, 213, 219, 85, 124, 63, 220, 2, 56, 159, 230, 125, 45, 49, 12, 109,
			72, 203, 43, 247, 236, 150, 134, 209, 127, 249, 177, 161, 155, 112, 170, 22, 131, 148,
			90, 125, 197, 175, 49, 12, 147, 0, 102, 233, 11, 15, 182, 51, 179, 217, 219, 233, 33,
			75, 59, 127, 53, 9, 34, 68, 8, 12, 18, 64, 64, 173, 164, 110, 22, 21, 34, 82, 109, 128,
			91, 30, 40, 124, 74, 56, 29, 80, 155, 111, 56, 79, 43, 243, 245, 187, 85, 46, 141, 190,
			242, 19, 103, 209, 157, 79, 139, 158, 199, 132, 55, 169, 75, 242, 121, 225, 193, 202,
			67, 0, 8, 217, 192, 188, 28, 124, 240, 73, 83, 152, 210, 172, 128, 6, 34, 68, 8, 6, 18,
			64, 87, 105, 16, 64, 124, 129, 8, 172, 154, 9, 149, 169, 73, 139, 220, 22, 36, 5, 145,
			41, 145, 253, 166, 193, 6, 1, 52, 101, 81, 126, 75, 211, 175, 48, 82, 0, 231, 4, 122,
			106, 51, 217, 195, 149, 209, 38, 66, 135, 222, 107, 193, 191, 133, 114, 22, 35, 179,
			17, 82, 242, 109, 18, 148, 6, 34, 68, 8, 19, 18, 64, 23, 81, 84, 180, 23, 70, 156, 229,
			120, 34, 57, 217, 34, 240, 115, 19, 156, 24, 251, 152, 148, 168, 24, 188, 112, 143,
			117, 100, 237, 94, 117, 248, 191, 159, 113, 37, 29, 154, 244, 62, 72, 72, 134, 64, 170,
			238, 125, 181, 202, 221, 231, 34, 199, 80, 106, 144, 50, 0, 157, 225, 71, 251, 29, 5,
			34, 68, 8, 16, 18, 64, 244, 253, 140, 88, 132, 81, 40, 187, 38, 108, 122, 225, 170,
			130, 13, 178, 203, 56, 104, 95, 198, 140, 203, 101, 3, 2, 3, 135, 84, 183, 14, 232,
			229, 249, 19, 252, 140, 46, 44, 50, 19, 248, 163, 221, 187, 210, 151, 193, 0, 153, 136,
			8, 113, 169, 61, 26, 165, 201, 200, 187, 198, 99, 29, 1, 34, 68, 8, 9, 18, 64, 248, 84,
			213, 143, 44, 128, 183, 229, 242, 147, 31, 103, 25, 241, 92, 172, 105, 92, 40, 110,
			188, 14, 190, 31, 254, 180, 137, 89, 236, 80, 19, 199, 105, 6, 137, 48, 150, 226, 17,
			104, 175, 176, 101, 152, 201, 98, 223, 137, 92, 24, 247, 16, 56, 198, 175, 224, 189,
			213, 209, 140, 11, 71, 186, 1, 34, 68, 8, 8, 18, 64, 32, 161, 232, 237, 18, 33, 21, 91,
			42, 125, 220, 209, 195, 33, 204, 151, 47, 167, 20, 123, 86, 207, 158, 108, 187, 141,
			111, 166, 58, 212, 222, 218, 66, 237, 233, 110, 191, 45, 231, 88, 137, 12, 227, 67,
			234, 51, 166, 66, 57, 252, 213, 116, 61, 150, 1, 52, 4, 183, 77, 156, 253, 150, 86, 3,
			34, 68, 8, 15, 18, 64, 79, 143, 228, 116, 248, 187, 216, 3, 127, 203, 27, 191, 137, 32,
			92, 182, 107, 125, 149, 177, 255, 14, 61, 153, 78, 67, 211, 163, 83, 132, 151, 106,
			145, 207, 161, 102, 70, 220, 224, 237, 112, 47, 77, 240, 87, 95, 64, 83, 184, 242, 85,
			246, 201, 56, 27, 75, 160, 94, 156, 197, 149, 255, 226, 11, 34, 68, 8, 13, 18, 64, 153,
			104, 114, 9, 113, 53, 157, 33, 101, 62, 169, 170, 139, 144, 183, 208, 247, 108, 42,
			174, 104, 83, 47, 20, 204, 57, 173, 125, 201, 224, 25, 119, 31, 36, 117, 131, 216, 213,
			227, 17, 222, 50, 167, 84, 108, 247, 80, 65, 31, 229, 170, 27, 227, 203, 130, 109, 0,
			7, 217, 161, 126, 12, 48, 13, 34, 66, 18, 64, 91, 0, 63, 120, 150, 247, 113, 107, 46,
			178, 90, 32, 142, 98, 137, 215, 53, 62, 234, 244, 168, 168, 222, 13, 168, 172, 139,
			159, 104, 127, 174, 201, 115, 67, 191, 117, 244, 123, 17, 159, 28, 117, 86, 20, 198,
			222, 53, 46, 102, 169, 240, 111, 122, 73, 219, 19, 47, 68, 66, 103, 66, 203, 242, 15,
			34, 68, 8, 21, 18, 64, 126, 174, 253, 15, 115, 180, 101, 207, 240, 137, 243, 170, 206,
			70, 151, 252, 28, 3, 79, 175, 95, 46, 215, 189, 201, 131, 58, 129, 98, 42, 118, 242,
			184, 119, 152, 190, 238, 31, 79, 202, 62, 20, 218, 234, 40, 34, 30, 122, 141, 159, 111,
			24, 235, 113, 221, 42, 147, 178, 5, 105, 144, 193, 218, 6, 34, 68, 8, 11, 18, 64, 149,
			131, 102, 27, 185, 252, 34, 253, 169, 99, 237, 137, 33, 43, 229, 153, 76, 17, 181, 155,
			188, 51, 31, 192, 23, 79, 101, 189, 200, 118, 155, 141, 155, 184, 138, 225, 125, 107,
			122, 96, 20, 27, 116, 10, 242, 101, 77, 252, 215, 132, 186, 134, 126, 86, 41, 184, 42,
			13, 103, 125, 216, 135, 99, 11, 34, 68, 8, 18, 18, 64, 90, 184, 93, 12, 63, 113, 86,
			94, 47, 112, 164, 165, 67, 212, 42, 229, 14, 108, 189, 145, 79, 194, 232, 52, 93, 239,
			83, 101, 16, 28, 94, 176, 88, 154, 112, 13, 162, 54, 160, 184, 249, 221, 183, 147, 90,
			137, 183, 53, 124, 145, 94, 121, 169, 34, 110, 26, 235, 63, 159, 35, 56, 182, 88, 4,
			18, 6, 8, 1, 16, 215, 231, 2, 26, 43, 112, 105, 99, 97, 49, 122, 53, 52, 55, 121, 110,
			51, 57, 102, 114, 116, 57, 102, 52, 48, 113, 52, 122, 57, 113, 52, 101, 115, 114, 104,
			103, 55, 101, 54, 113, 106, 106, 119, 120, 122, 109, 48, 120,
		],
	};
	let banksy_config = toml::from_str::<CosmosClientConfig>(
		&std::fs::read_to_string("../../config/centauri-mainnet.toml").unwrap(),
	)
	.unwrap();
	let cosmos = CosmosClient::<()>::new(banksy_config).await.unwrap();
	cosmos.submit(vec![payload]);
}

#[cfg(test)]
pub mod tests {
	use super::MnemonicEntry;
	use crate::key_provider::KeyEntry;

	struct TestVector {
		mnemonic: &'static str,
		private_key: [u8; 32],
		public_key: [u8; 33],
		account: &'static str,
	}
	const TEST_VECTORS : &[TestVector] = &[
		TestVector {
			mnemonic: "idea gap afford glow ugly suspect exile wedding fiber turn opinion weekend moon project egg certain play obvious slice delay present weekend toe ask",
			private_key: [
				220, 53, 10, 206, 12, 57, 15, 47, 116, 210, 236, 140, 173, 220, 159, 74,
				105, 112, 131, 55, 152, 173, 197, 173, 254, 22, 161, 53, 60, 30, 97, 181
			],
			public_key: [
				2, 21, 157, 166, 61, 81, 112, 226, 211, 32, 5, 1, 133, 147, 182, 183, 41,
				26, 243, 17, 241, 200, 87, 140, 93, 229, 26, 42, 81, 39, 208, 4, 219
			],
			account: "cosmos15hf3dgggyt4azpd693ax7fdfve8d5m6ct72z9p",
		},
		TestVector {
			mnemonic: "elite program lift later ask fox change process dirt talk type coconut",
			private_key: [97, 173, 171, 67, 228, 198, 20, 233, 30, 232, 208, 250, 151, 66, 76, 129, 83, 100, 17, 219, 74, 20, 43, 202, 110, 166, 72, 184, 100, 180, 135, 132],
			public_key: [2, 167, 203, 215, 223, 101, 49, 90, 51, 44, 171, 156, 157, 167, 99, 213, 97, 84, 38, 210, 64, 168, 133, 38, 159, 49, 4, 24, 159, 137, 83, 92, 160],
			account: "cosmos1dxsre7u4zkg28k4fqtgy2slcrrq46hqafe6547",
		},
		TestVector {
			mnemonic: "habit few zero correct fancy hair common club slow lunch brief spawn away brief loyal flee witness possible faint legend spell arrive gravity hybrid",
			private_key: [91, 189, 78, 43, 217, 14, 16, 247, 18, 196, 173, 149, 131, 156, 254, 191, 156, 154, 60, 255, 196, 2, 97, 219, 92, 160, 15, 224, 177, 216, 27, 44],
			public_key: [3, 45, 249, 112, 87, 75, 114, 244, 199, 129, 6, 142, 9, 221, 205, 100, 226, 233, 131, 167, 146, 187, 181, 7, 176, 80, 107, 61, 151, 44, 185, 116, 116],
			account: "cosmos1xf5280nzgqxyxps526vw0t6vd90gthd76fv6s8",
		},
	];

	#[test]
	fn test_from_mnemonic() {
		for vector in TEST_VECTORS {
			match KeyEntry::try_from(MnemonicEntry {
				mnemonic: vector.mnemonic.to_string(),
				prefix: "cosmos".to_string(),
			}) {
				Ok(key_entry) => {
					assert_eq!(key_entry.private_key.to_bytes(), vector.private_key);
					assert_eq!(key_entry.public_key.to_bytes(), vector.public_key);
					assert_eq!(key_entry.account, vector.account);
				},
				Err(_) => panic!("Try from mnemonic failed"),
			}
		}
	}
}
