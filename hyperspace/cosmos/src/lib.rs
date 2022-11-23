// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![allow(clippy::all)]

pub mod chain;
pub mod error;
pub mod key_provider;
pub mod provider;
use core::convert::TryFrom;
use error::Error;
use ibc::core::{
	ics23_commitment::commitment::CommitmentPrefix,
	ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
};
use ibc_proto::cosmos::auth::v1beta1::BaseAccount;
use ics07_tendermint::client_state::ClientState as TmClientState;
use key_provider::KeyEntry;
use primitives::{IbcProvider, KeyProvider};
use serde::Deserialize;
use tendermint::trust_threshold::TrustThresholdFraction;
use tendermint_light_client::{
	components::{self, io::RpcIo},
	light_client::LightClient as TmLightClient,
};
use tendermint_light_client_verifier::{
	host_functions::CryptoProvider,
	operations::{ProdCommitValidator, ProdVotingPowerCalculator},
	options::Options as TmOptions,
	predicates::ProdPredicates,
	types::PeerId,
	PredicateVerifier, ProdVerifier,
};
use tendermint_rpc::{Client, HttpClient, Url};

// Implements the [`crate::Chain`] trait for cosmos.
/// This is responsible for:
/// 1. Tracking a cosmos light client on a counter-party chain, advancing this light
/// client state  as new finality proofs are observed.
/// 2. Submiting new IBC messages to this cosmos.
#[derive(Clone)]
pub struct CosmosClient<H> {
	/// Chain name
	pub name: String,
	/// Chain Light Provider
	pub light_provider: RpcIo,
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
	/// The key that signs transactions
	pub keybase: KeyEntry,
	/// Account prefix
	pub account_prefix: String,
	/// Reference to commitment
	pub commitment_prefix: CommitmentPrefix,
	/// Channels cleared for packet relay
	pub channel_whitelist: Vec<(ChannelId, PortId)>,
	/// Finality protocol to use, eg Tenderminet
	pub _phantom: std::marker::PhantomData<H>,
}
/// config options for [`ParachainClient`]
#[derive(Debug, Deserialize)]
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
	/// Store prefix
	pub store_prefix: String,
	/// The key that signs transactions
	pub keybase: KeyEntry,
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
		let peer_id: PeerId = rpc_client
			.status()
			.await
			.map(|s| s.node_info.id)
			.map_err(|e| Error::from(e.to_string()))?;
		let light_provider = RpcIo::new(peer_id, rpc_client.clone(), None);
		let chain_id = ChainId::from(config.chain_id);
		let client_id = Some(
			ClientId::new(config.client_id.unwrap().as_str(), 0)
				.map_err(|e| Error::from(format!("Invalid client id {:?}", e)))?,
		);
		let commitment_prefix = CommitmentPrefix::try_from(config.store_prefix.as_bytes().to_vec())
			.map_err(|e| Error::from(format!("Invalid store prefix {:?}", e)))?;

		Ok(Self {
			name: config.name,
			light_provider,
			chain_id,
			rpc_client,
			grpc_url: config.grpc_url,
			websocket_url: config.websocket_url,
			client_id,
			connection_id: None,
			account_prefix: config.account_prefix,
			commitment_prefix,
			keybase: config.keybase,
			channel_whitelist: vec![],
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
	pub async fn construct_tendermint_client_state<HostFunctions: CryptoProvider>(
		&self,
		client_state: &TmClientState<HostFunctions>,
	) -> Result<TmLightClient<HostFunctions>, Error>
	where
		Self: KeyProvider + IbcProvider,
		H: Clone + Send + Sync + 'static,
	{
		let params = TmOptions {
			trust_threshold: TrustThresholdFraction::new(
				client_state.trust_level.numerator(),
				client_state.trust_level.denominator(),
			)
			.unwrap(),
			trusting_period: client_state.trusting_period,
			clock_drift: client_state.max_clock_drift,
		};
		let clock = components::clock::SystemClock;
		let scheduler = components::scheduler::basic_bisecting_schedule;
		let verifier: PredicateVerifier<
			ProdPredicates<HostFunctions>,
			ProdVotingPowerCalculator<HostFunctions>,
			ProdCommitValidator<HostFunctions>,
			HostFunctions,
		> = ProdVerifier::default();
		let peer_id: PeerId = self
			.rpc_client
			.status()
			.await
			.map(|s| s.node_info.id)
			.map_err(|e| Error::from(e.to_string()))?;
		Ok(TmLightClient::new(
			peer_id,
			params,
			clock,
			scheduler,
			verifier,
			self.light_provider.clone(),
		))
	}

	pub async fn submit_create_client_msg(&self, _msg: String) -> Result<ClientId, Error> {
		todo!()
	}

	pub async fn transfer_tokens(&self, _asset_id: u128, _amount: u128) -> Result<(), Error> {
		todo!()
	}

	pub async fn submit_call(&self) -> Result<(), Error> {
		todo!()
	}

	/// Uses the GRPC client to retrieve the account sequence
	pub async fn query_account(&self) -> Result<BaseAccount, Error> {
		todo!()
	}
}
