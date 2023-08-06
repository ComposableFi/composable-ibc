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
	time::Duration,
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
const DEFAULT_FEE_AMOUNT: &str = "4000";
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
	pub wasm_code_hash: Option<String>,
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
			.map_err(|e| Error::RpcError(format!("{:?}", e)))?;
		let rpc_http_client = HttpClient::new(config.rpc_url.clone())
			.map_err(|e| Error::RpcError(format!("{:?}", e)))?;
		let ws_driver_jh = tokio::spawn(rpc_driver.run());
		let grpc_client = tonic::transport::Endpoint::new(config.grpc_url.to_string())
			.map_err(|e| Error::RpcError(format!("{:?}", e)))?
			.connect()
			.await
			.map_err(|e| Error::RpcError(format!("{:?}", e)))?;

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

		let rpc_call_delay = Duration::from_millis(1000);
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

		// Simulate transaction
		let res = simulate_tx(self.grpc_url.clone(), tx, tx_bytes.clone()).await?;
		res.result
			.map(|r| log::debug!(target: "hyperspace_cosmos", "Simulated transaction: events: {:?}\nlogs: {}", r.events, r.log));

		// Broadcast transaction
		let hash = broadcast_tx(&self.rpc_client, tx_bytes).await?;
		log::debug!(target: "hyperspace_cosmos", "🤝 Transaction sent with hash: {:?}", hash);

		// wait for confirmation
		confirm_tx(&self.rpc_client, hash).await
	}

	pub async fn fetch_light_block_with_cache(
		&self,
		height: TmHeight,
		sleep_duration: Duration,
	) -> Result<LightBlock, Error> {
		let fut = async move {
			sleep(sleep_duration).await;
			self.light_client.io.fetch_light_block(AtHeight::At(height)).map_err(|e| {
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
		let mut xs = Vec::new();
		let heightss = (from.value()..=to.value()).collect::<Vec<_>>();
		let client = Arc::new(self.clone());
		let to = self.rpc_call_delay().as_millis();
		for heights in heightss.chunks(5) {
			let mut join_set = JoinSet::<Result<Result<_, Error>, Elapsed>>::new();
			for height in heights.to_owned() {
				let client = client.clone();
				let duration = Duration::from_millis(rand::thread_rng().gen_range(0..to) as u64);
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
			)))
		}

		if prove && response.proof.is_none() {
			// Fail due to empty proof
			return Err(Error::from(format!(
				"Query failed due to empty proof for chain {}",
				self.name
			)))
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
