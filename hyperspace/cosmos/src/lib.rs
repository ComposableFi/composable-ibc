#![allow(clippy::all)]

pub mod chain;
pub mod error;
pub mod finality_protocol;
pub mod key_provider;
pub mod provider;
#[cfg(any(test, feature = "testing"))]
pub mod test_provider;
use error::Error;
use ibc::{
	core::{
		ics02_client::{height::Height, trust_threshold::TrustThreshold},
		ics23_commitment::{
			commitment::{CommitmentPrefix, CommitmentRoot},
			specs::ProofSpecs,
		},
		ics24_host::{
			identifier::{ChainId, ClientId, ConnectionId},
			path::ClientConsensusStatePath,
			Path,
		},
	},
	protobuf::Protobuf,
};
use ibc_proto::{
	cosmos::base::query::v1beta1::PageRequest,
	google::protobuf::Any,
	ibc::core::{
		client::v1::{
			IdentifiedClientState, QueryConsensusStateRequest, QueryConsensusStateResponse,
		},
		connection::v1::{IdentifiedConnection, QueryConnectionResponse},
	},
};
use ics07_tendermint::{
	client_state::ClientState as TmClientState, consensus_state::ConsensusState as TmConsensusState,
};
use key_provider::KeyEntry;
use pallet_ibc::{
	light_clients::{AnyClientState, AnyConsensusState, HostFunctionsManager},
	MultiAddress, Timeout, TransferParams,
};
use primitives::{IbcProvider, KeyProvider};
use serde::Deserialize;
use std::{str::FromStr, time::Duration};
use tendermint::block::Height as TmHeight;
use tendermint::time::Time;
use tendermint_rpc::{abci::Path as TendermintABCIPath, Client, HttpClient, Url, WebSocketClient};
use tendermint_verifier::LightClient;
// Implements the [`crate::Chain`] trait for cosmos.
/// This is responsible for:
/// 1. Tracking a cosmos light client on a counter-party chain, advancing this light
/// client state  as new finality proofs are observed.
/// 2. Submiting new IBC messages to this cosmos.
#[derive(Clone)]
pub struct CosmosClient<H> {
	/// Chain name
	pub name: String,
	/// Chain rpc client
	pub rpc_client: HttpClient,
	/// Chain grpc address
	pub grpc_url: Url,
	/// Websocket chain ws client
	pub ws_client: WebSocketClient,
	/// Chain Id
	pub chain_id: ChainId,
	/// Light client id on counterparty chain
	pub client_id: Option<ClientId>,
	/// Connection Id
	pub connection_id: Option<ConnectionId>,
	/// Light client to track the counterparty chain
	pub light_client: LightClient,
	/// Name of the key to use for signing
	pub keybase: KeyEntry,
	/// Account prefix
	pub account_prefix: String,
	/// Reference to commitment
	pub commitment_prefix: CommitmentPrefix,
	/// Reference to proof specs
	pub finality_protocol: finality_protocol::FinalityProtocol,
	pub _phantom: std::marker::PhantomData<H>,
}
/// config options for [`ParachainClient`]
#[derive(Debug, Deserialize)]
pub struct CosmosClientConfig {
	/// Chain name
	pub name: String,
	/// Cosmos chain Id
	pub chain_id: String,
	/// rpc url for cosmos
	pub rpc_url: Url,
	/// grpc url for cosmos
	pub grpc_url: Url,
	/// websocket url for cosmos
	pub websocket_url: Url,
	/// Light client id on counterparty chain
	pub client_id: Option<String>,
	/// Connection Id
	pub connection_id: Option<String>,
	/// Account prefix
	pub account_prefix: String,
	/// Store prefix
	pub store_prefix: String,
	/// Name of the key that signs transactions
	pub key_name: String,
	/*
	Here is a list of dropped configuration parameters from Hermes Config.toml
	that could be set to default values or removed for the MVP phase:

	ub key_store_type: Store,					//TODO: Could be set to any of SyncCryptoStorePtr or KeyStore or KeyEntry types, but not sure yet
	pub rpc_timeout: Duration,				    //TODO: Could be set to '15s' by default
	pub default_gas: Option<u64>,	  			//TODO: Could be set to `0` by default
	pub max_gas: Option<u64>,                   //TODO: DEFAULT_MAX_GAS: u64 = 400_000
	pub gas_multiplier: Option<GasMultiplier>,  //TODO: Could be set to `1.1` by default
	pub fee_granter: Option<String>,            //TODO: DEFAULT_FEE_GRANTER: &str = ""
	pub max_msg_num: MaxMsgNum,                 //TODO: Default is 30, Could be set usize = 1 for test
	pub max_tx_size: MaxTxSize,					//TODO: Default is usize = 180000, pub memo_prefix: Memo
												//TODO: Could be set to const MAX_LEN: usize = 50;
	pub proof_specs: Option<ProofSpecs>,        //TODO: Could be set to None
	pub sequential_batch_tx: bool,			    //TODO: sequential_send_batched_messages_and_wait_commit() or send_batched_messages_and_wait_commit() ?
	pub trust_threshold: TrustThreshold,
	pub gas_price: GasPrice,   				    //TODO: Could be set to `0`
	pub packet_filter: PacketFilter,            //TODO: AllowAll
	pub address_type: AddressType,			    //TODO: Type = cosmos
	pub extension_options: Vec<ExtensionOption>,//TODO: Could be set to None
	*/
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
		let (ws_client, _ws_driver) = WebSocketClient::new(config.websocket_url.clone())
			.await
			.map_err(|e| Error::from(format!("Web Socket Client Error {:?}", e)))?;
		let chain_id = ChainId::from(config.chain_id);
		let client_id = Some(
			ClientId::new(config.client_id.unwrap().as_str(), 0)
				.map_err(|e| Error::from(format!("Invalid client id {}", e)))?,
		);
		let light_client = LightClient::init_light_client(config.rpc_url).await.map_err(|e| {
			Error::from(format!(
				"Failed to initialize light client for chain {} with error {:?}",
				config.name, e
			))
		})?;
		let keybase = KeyEntry::new(&config.key_name, &chain_id)?;
		let commitment_prefix = CommitmentPrefix::try_from(config.store_prefix.as_bytes().to_vec())
			.map_err(|e| Error::from(format!("Invalid store prefix {}", e)))?;

		Ok(Self {
			name: config.name,
			chain_id,
			rpc_client,
			grpc_url: config.grpc_url,
			ws_client,
			client_id,
			connection_id: None,
			light_client,
			account_prefix: config.account_prefix,
			commitment_prefix,
			keybase,
			finality_protocol: finality_protocol::FinalityProtocol::Tendermint,
			_phantom: std::marker::PhantomData,
		})
	}

	pub fn client_id(&self) -> ClientId {
		self.client_id.as_ref().unwrap().clone()
	}

	pub fn set_client_id(&mut self, client_id: ClientId) {
		self.client_id = Some(client_id)
	}

	/// Construct a tendermint client state to be submitted to the counterparty chain
	pub async fn construct_tendermint_client_state(
		&self,
	) -> Result<(AnyClientState, AnyConsensusState), Error>
	where
		Self: KeyProvider + IbcProvider,
		H: Clone + Send + Sync + 'static,
	{
		{
			let latest_height_timestamp = self.latest_height_and_timestamp().await.unwrap();
			let client_state = TmClientState::<HostFunctionsManager>::new(
				self.chain_id.clone(),
				TrustThreshold::default(),
				Duration::new(64000, 0),
				Duration::new(128000, 0),
				Duration::new(3, 0),
				latest_height_timestamp.0,
				ProofSpecs::default(),
				vec!["".to_string()],
			)
			.map_err(|e| Error::from(format!("Invalid client state {}", e)))?;
			let light_block = self
				.light_client
				.verify::<HostFunctionsManager>(latest_height_timestamp.0, latest_height_timestamp.0, &client_state)
				.await.map_err(|e| Error::from(format!("Invalid light block {}", e)))?;
			let consensus_state = TmConsensusState::from(light_block.signed_header.header);
			Ok((
				AnyClientState::Tendermint(client_state),
				AnyConsensusState::Tendermint(consensus_state),
			))
		}
	}

	pub async fn submit_create_client_msg(&self, msg: String) -> Result<ClientId, Error> {
		todo!()
	}

	pub async fn transfer_tokens(&self, asset_id: u128, amount: u128) -> Result<(), Error> {
		Ok(())
	}

	pub async fn submit_sudo_call(&self) -> Result<(), Error> {
		Ok(())
	}
}
