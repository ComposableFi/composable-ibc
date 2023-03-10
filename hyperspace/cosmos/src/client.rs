#![allow(clippy::all)]
use super::{
	key_provider::KeyEntry,
	light_client::LightClient,
	tx::{broadcast_tx, confirm_tx, sign_tx, simulate_tx},
};
use crate::error::Error;
use bip32::{ExtendedPrivateKey, Mnemonic, XPub as ExtendedPublicKey, Prefix};
use secp256k1::{Message as Secp256k1Message, PublicKey, Secp256k1, SecretKey};
use bech32::ToBase32;
use core::convert::{From, Into, TryFrom};
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
use std::{str::FromStr, sync::Arc};

use ics07_tendermint::{
	client_message::Header, client_state::ClientState, consensus_state::ConsensusState,
	merkle::convert_tm_to_ics_merkle_proof,
};
use pallet_ibc::light_clients::{AnyClientState, AnyConsensusState, HostFunctionsManager};
use primitives::{IbcProvider, KeyProvider, UpdateType};
use prost::Message;
use serde::{Deserialize, Serialize};
use tendermint::{block::Height as TmHeight, Hash};
use tendermint_light_client::components::io::{AtHeight, Io};
use tendermint_rpc::{endpoint::abci_query::AbciQuery, Client, HttpClient, Url};
use ed25519_zebra::{SigningKey, VerificationKey, VerificationKeyBytes};
use hdpath::StandardHDPath;
use bitcoin::{
    network::constants::Network,
    util::bip32::{ChildNumber, DerivationPath, ExtendedPrivKey, ExtendedPubKey},
};
use ibc_relayer::{
    chain::ChainType,
    config::{ChainConfig, Config, AddressType},
    keyring::{
        AnySigningKeyPair, KeyRing, Secp256k1KeyPair, SigningKeyPair, SigningKeyPairSized, Store
    },
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

impl TryFrom<String> for KeyEntry {
	type Error = bip32::Error;

	fn try_from(mnemonic: String) -> Result<Self, Self::Error> {
		let hdpath = StandardHDPath::from_str("m/44'/118'/0'/0/0").unwrap();
		let key_pair = Secp256k1KeyPair::from_mnemonic(
			&mnemonic,
			&hdpath,
			&AddressType::Cosmos,
			"cosmos",
		).unwrap();
		// TODO: get priv and pub keys
		Ok(KeyEntry {
			public_key: ExtendedPublicKey::from_str(&"todo")?,
			private_key: ExtendedPrivateKey::from_str(&"todo")?,
			account: key_pair.account(),
			address: key_pair.account().as_bytes().to_vec(),
		})
	}
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
	pub rpc_client: HttpClient,
	/// Chain grpc address
	pub grpc_url: Url,
	/// Websocket chain ws client
	pub websocket_url: Url,
	/// Chain Id
	pub chain_id: ChainId,
	/// Light client id on counterparty chain
	pub client_id: Option<ClientId>,
	/// Connection Id
	pub connection_id: Option<ConnectionId>,
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
	/// Channels cleared for packet relay
	pub channel_whitelist: Vec<(ChannelId, PortId)>,
	/// Finality protocol to use, eg Tenderminet
	pub _phantom: std::marker::PhantomData<H>,
	/// Mutex used to sequentially send transactions. This is necessary because
	/// account sequence numbers are not updated until the transaction is processed.
	pub tx_mutex: Arc<tokio::sync::Mutex<()>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KeyBaseConfig {
	ConfigKeyEntry(ConfigKeyEntry),
	Mnemonic(String),
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
	pub client_id: Option<String>,
	/// Connection Id
	pub connection_id: Option<String>,
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
	/// The key that signs transactions
	pub keybase: KeyBaseConfig,
	/// Whitelisted channels
	pub channel_whitelist: Vec<(ChannelId, PortId)>,
}

impl<H> CosmosClient<H>
where
	Self: KeyProvider,
	H: Clone + Send + Sync + 'static,
{
	/// Initializes a [`CosmosClient`] given a [`CosmosClientConfig`]
	pub async fn new(config: CosmosClientConfig) -> Result<Self, Error> {
		let rpc_client = HttpClient::new(config.rpc_url.clone())
			.map_err(|e| Error::RpcError(format!("{:?}", e)))?;
		let chain_id = ChainId::from(config.chain_id);
		let client_id = config
			.client_id
			.map(|client_id| {
				ClientId::from_str(&client_id)
					.map_err(|e| Error::from(format!("Invalid client id {:?}", e)))
			})
			.transpose()?;
		let light_client = LightClient::init_light_client(config.rpc_url.clone()).await?;
		let commitment_prefix = CommitmentPrefix::try_from(config.store_prefix.as_bytes().to_vec())
			.map_err(|e| Error::from(format!("Invalid store prefix {:?}", e)))?;

		let keybase: KeyEntry;
		match config.keybase {
			KeyBaseConfig::ConfigKeyEntry(config_key) => {
				keybase = KeyEntry::try_from(config_key).map_err(|e| e.to_string())?
			},
			KeyBaseConfig::Mnemonic(mnemonic) => {
				keybase = KeyEntry::try_from(mnemonic).map_err(|e| e.to_string())?
			},
		}
		Ok(Self {
			name: config.name,
			chain_id,
			rpc_client,
			grpc_url: config.grpc_url,
			websocket_url: config.websocket_url,
			client_id,
			connection_id: config
				.connection_id
				.map(|connection_id| {
					ConnectionId::from_str(&connection_id)
						.map_err(|e| Error::from(format!("Invalid connection id {:?}", e)))
				})
				.transpose()?,
			light_client,
			account_prefix: config.account_prefix,
			commitment_prefix,
			fee_denom: config.fee_denom,
			fee_amount: config.fee_amount,
			gas_limit: config.gas_limit,
			max_tx_size: config.max_tx_size,
			keybase,
			channel_whitelist: config.channel_whitelist,
			_phantom: std::marker::PhantomData,
			tx_mutex: Default::default(),
		})
	}

	pub fn client_id(&self) -> ClientId {
		self.client_id.clone().expect(
			"Client id should be set after the client state is initialized and the client is created",
		)
	}

	pub fn set_client_id(&mut self, client_id: ClientId) {
		self.client_id = Some(client_id)
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

	pub async fn msg_update_client_header(
		&self,
		from: TmHeight,
		to: TmHeight,
		trusted_height: Height,
	) -> Result<Vec<(Header, UpdateType)>, Error> {
		let mut xs = Vec::new();
		for height in from.value()..=to.value() {
			let latest_light_block = self
				.light_client
				.io
				.fetch_light_block(AtHeight::At(height.try_into()?))
				.map_err(|e| {
					Error::from(format!(
						"Failed to fetch light block for chain {:?} with error {:?}",
						self.name, e
					))
				})?;
			let height = TmHeight::try_from(trusted_height.revision_height).map_err(|e| {
				Error::from(format!(
					"Failed to convert height for chain {:?} with error {:?}",
					self.name, e
				))
			})?;
			let trusted_light_block = self
				.light_client
				.io
				.fetch_light_block(AtHeight::At(height.increment()))
				.map_err(|e| {
					Error::from(format!(
						"Failed to fetch light block for chain {:?} with error {:?}",
						self.name, e
					))
				})?;

			let update_type =
				match latest_light_block.validators == latest_light_block.next_validators {
					true => UpdateType::Mandatory,
					false => UpdateType::Mandatory,
				};
			xs.push((
				Header {
					signed_header: latest_light_block.signed_header,
					validator_set: latest_light_block.validators,
					trusted_height,
					trusted_validator_set: trusted_light_block.validators,
				},
				update_type,
			));
		}
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
			.rpc_client
			.abci_query(Some(path.to_owned()), data, height, prove)
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



#[cfg(test)]
pub mod tests {

	use crate::key_provider::KeyEntry;
    use bip32::{Prefix, XPrv};

	#[test]
	fn test_from_mnemonic() {
		let mnemonic: String =
			"idea gap afford glow ugly suspect exile wedding fiber turn opinion weekend moon project egg certain play obvious slice delay present weekend toe ask".to_string();
		let keyEntry = KeyEntry::try_from(mnemonic);
		let a;
		match keyEntry {
			Ok(gud) => {
				a = gud;
                println!("{:?}", a.account);
                // assert_eq!(a.private_key.private_key())
				assert_eq!(a.account, "cosmos15hf3dgggyt4azpd693ax7fdfve8d5m6ct72z9p".to_string())
			},
			Err(err) => eprintln!("Error: {}", err),
		}
	}
}
