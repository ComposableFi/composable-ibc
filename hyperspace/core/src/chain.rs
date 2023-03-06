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

#![allow(unreachable_patterns)]

use async_trait::async_trait;
#[cfg(feature = "cosmos")]
use cosmos::client::{CosmosClient, CosmosClientConfig};
use derive_more::From;
use futures::Stream;
#[cfg(any(test, feature = "testing"))]
use ibc::applications::transfer::msgs::transfer::MsgTransfer;
use ibc::{
	applications::transfer::PrefixedCoin,
	core::{
		ics02_client::client_state::ClientType,
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	downcast,
	events::IbcEvent,
	signer::Signer,
	timestamp::Timestamp,
	Height,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{
			QueryChannelResponse, QueryChannelsResponse, QueryNextSequenceReceiveResponse,
			QueryPacketAcknowledgementResponse, QueryPacketCommitmentResponse,
			QueryPacketReceiptResponse,
		},
		client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
		connection::v1::{IdentifiedConnection, QueryConnectionResponse},
	},
};
use pallet_ibc::light_clients::AnyClientMessage;
#[cfg(any(test, feature = "testing"))]
use pallet_ibc::Timeout;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use ibc::core::ics02_client::events::UpdateClient;
use pallet_ibc::light_clients::{AnyClientState, AnyConsensusState};
use parachain::{config, ParachainClient};
use primitives::{
	Chain, IbcProvider, KeyProvider, LightClientSync, MisbehaviourHandler, UpdateType,
};
use std::{pin::Pin, time::Duration};
#[cfg(feature = "dali")]
use subxt::config::substrate::{
	SubstrateExtrinsicParams as ParachainExtrinsicParams,
	SubstrateExtrinsicParamsBuilder as ParachainExtrinsicsParamsBuilder,
};
use subxt::{config::ExtrinsicParams, Error, OnlineClient};

use subxt::config::extrinsic_params::Era;
#[cfg(not(feature = "dali"))]
use subxt::config::polkadot::{
	PolkadotExtrinsicParams as ParachainExtrinsicParams,
	PolkadotExtrinsicParamsBuilder as ParachainExtrinsicsParamsBuilder,
};

// TODO: expose extrinsic param builder
#[derive(Debug, Clone)]
pub enum DefaultConfig {}

#[async_trait]
impl config::Config for DefaultConfig {
	type AssetId = u128;
	type Signature = <Self as subxt::Config>::Signature;
	type Address = <Self as subxt::Config>::Address;

	async fn custom_extrinsic_params(
		client: &OnlineClient<Self>,
	) -> Result<
		<Self::ExtrinsicParams as ExtrinsicParams<Self::Index, Self::Hash>>::OtherParams,
		Error,
	> {
		let params =
			ParachainExtrinsicsParamsBuilder::new().era(Era::Immortal, client.genesis_hash());
		Ok(params.into())
	}
}

impl subxt::Config for DefaultConfig {
	type Index = u32;
	type BlockNumber = u32;
	type Hash = sp_core::H256;
	type Hasher = subxt::config::substrate::BlakeTwo256;
	type AccountId = sp_runtime::AccountId32;
	type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
	type Header = subxt::config::substrate::SubstrateHeader<
		Self::BlockNumber,
		subxt::config::substrate::BlakeTwo256,
	>;
	type Signature = sp_runtime::MultiSignature;
	type ExtrinsicParams = ParachainExtrinsicParams<Self>;
}

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub chain_a: AnyConfig,
	pub chain_b: AnyConfig,
	pub core: CoreConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AnyConfig {
	Parachain(parachain::ParachainClientConfig),
	#[cfg(feature = "cosmos")]
	Cosmos(CosmosClientConfig),
}

#[derive(Serialize, Deserialize)]
pub struct CoreConfig {
	pub prometheus_endpoint: Option<String>,
}

#[derive(Clone)]
pub enum AnyChain {
	Parachain(ParachainClient<DefaultConfig>),
	#[cfg(feature = "cosmos")]
	Cosmos(CosmosClient<DefaultConfig>),
}

#[derive(From)]
pub enum AnyFinalityEvent {
	Parachain(parachain::finality_protocol::FinalityEvent),
	#[cfg(feature = "cosmos")]
	Cosmos(cosmos::provider::FinalityEvent),
}

#[derive(Clone)]
pub enum AnyAssetId {
	Parachain(<ParachainClient<DefaultConfig> as IbcProvider>::AssetId),
	#[cfg(feature = "cosmos")]
	Cosmos(<CosmosClient<DefaultConfig> as IbcProvider>::AssetId),
}

#[derive(From, Debug)]
pub enum AnyTransactionId {
	Parachain(parachain::provider::TransactionId<sp_core::H256>),
	#[cfg(feature = "cosmos")]
	Cosmos(cosmos::provider::TransactionId<cosmos::provider::Hash>),
}

#[derive(Error, Debug)]
pub enum AnyError {
	#[error("{0}")]
	Parachain(#[from] parachain::error::Error),
	#[cfg(feature = "cosmos")]
	#[error("{0}")]
	Cosmos(#[from] cosmos::error::Error),
	#[error("{0}")]
	Other(String),
}

impl From<String> for AnyError {
	fn from(s: String) -> Self {
		Self::Other(s)
	}
}

#[async_trait]
impl IbcProvider for AnyChain {
	type FinalityEvent = AnyFinalityEvent;
	type TransactionId = AnyTransactionId;
	type Error = AnyError;
	type AssetId = AnyAssetId;

	async fn query_latest_ibc_events<T>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &T,
	) -> Result<Vec<(Any, Vec<IbcEvent>, UpdateType)>, anyhow::Error>
	where
		T: Chain,
	{
		match self {
			AnyChain::Parachain(chain) => {
				let finality_event = downcast!(finality_event => AnyFinalityEvent::Parachain)
					.ok_or_else(|| AnyError::Other("Invalid finality event type".to_owned()))?;
				chain.query_latest_ibc_events(finality_event, counterparty).await
			},
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => {
				let finality_event = downcast!(finality_event => AnyFinalityEvent::Cosmos)
					.ok_or_else(|| AnyError::Other("Invalid finality event type".to_owned()))?;
				chain.query_latest_ibc_events(finality_event, counterparty).await
			},
		}
	}

	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>> {
		match self {
			Self::Parachain(chain) => chain.ibc_events().await,
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.ibc_events().await,
		}
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		match self {
			AnyChain::Parachain(chain) => chain
				.query_client_consensus(at, client_id, consensus_height)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain
				.query_client_consensus(at, client_id, consensus_height)
				.await
				.map_err(Into::into),
		}
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		match self {
			AnyChain::Parachain(chain) =>
				chain.query_client_state(at, client_id).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.query_client_state(at, client_id).await.map_err(Into::into),
		}
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		match self {
			AnyChain::Parachain(chain) =>
				chain.query_connection_end(at, connection_id).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) =>
				chain.query_connection_end(at, connection_id).await.map_err(Into::into),
		}
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		match self {
			AnyChain::Parachain(chain) =>
				chain.query_channel_end(at, channel_id, port_id).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) =>
				chain.query_channel_end(at, channel_id, port_id).await.map_err(Into::into),
		}
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		match self {
			AnyChain::Parachain(chain) => chain.query_proof(at, keys).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.query_proof(at, keys).await.map_err(Into::into),
		}
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error> {
		match self {
			AnyChain::Parachain(chain) => chain
				.query_packet_commitment(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain
				.query_packet_commitment(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
		}
	}

	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Self::Error> {
		match self {
			AnyChain::Parachain(chain) => chain
				.query_packet_acknowledgement(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain
				.query_packet_acknowledgement(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
		}
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		match self {
			AnyChain::Parachain(chain) => chain
				.query_next_sequence_recv(at, port_id, channel_id)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain
				.query_next_sequence_recv(at, port_id, channel_id)
				.await
				.map_err(Into::into),
		}
	}

	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error> {
		match self {
			AnyChain::Parachain(chain) => chain
				.query_packet_receipt(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain
				.query_packet_receipt(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
		}
	}

	async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error> {
		match self {
			AnyChain::Parachain(chain) =>
				chain.latest_height_and_timestamp().await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.latest_height_and_timestamp().await.map_err(Into::into),
		}
	}

	async fn query_packet_commitments(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		match self {
			Self::Parachain(chain) => chain
				.query_packet_commitments(at, channel_id, port_id)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.query_packet_commitments(at, channel_id, port_id)
				.await
				.map_err(Into::into),
		}
	}

	async fn query_packet_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		match self {
			Self::Parachain(chain) => chain
				.query_packet_acknowledgements(at, channel_id, port_id)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.query_packet_acknowledgements(at, channel_id, port_id)
				.await
				.map_err(Into::into),
		}
	}

	async fn query_unreceived_packets(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		match self {
			Self::Parachain(chain) => chain
				.query_unreceived_packets(at, channel_id, port_id, seqs)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.query_unreceived_packets(at, channel_id, port_id, seqs)
				.await
				.map_err(Into::into),
		}
	}

	async fn query_unreceived_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		match self {
			Self::Parachain(chain) => chain
				.query_unreceived_acknowledgements(at, channel_id, port_id, seqs)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.query_unreceived_acknowledgements(at, channel_id, port_id, seqs)
				.await
				.map_err(Into::into),
		}
	}

	fn channel_whitelist(&self) -> Vec<(ChannelId, PortId)> {
		match self {
			Self::Parachain(chain) => chain.channel_whitelist(),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.channel_whitelist(),
		}
	}

	async fn query_connection_channels(
		&self,
		at: Height,
		connection_id: &ConnectionId,
	) -> Result<QueryChannelsResponse, Self::Error> {
		match self {
			Self::Parachain(chain) =>
				chain.query_connection_channels(at, connection_id).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) =>
				chain.query_connection_channels(at, connection_id).await.map_err(Into::into),
		}
	}

	async fn query_send_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<ibc_rpc::PacketInfo>, Self::Error> {
		match self {
			Self::Parachain(chain) =>
				chain.query_send_packets(channel_id, port_id, seqs).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) =>
				chain.query_send_packets(channel_id, port_id, seqs).await.map_err(Into::into),
		}
	}

	async fn query_recv_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<ibc_rpc::PacketInfo>, Self::Error> {
		match self {
			Self::Parachain(chain) =>
				chain.query_recv_packets(channel_id, port_id, seqs).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) =>
				chain.query_recv_packets(channel_id, port_id, seqs).await.map_err(Into::into),
		}
	}

	fn expected_block_time(&self) -> Duration {
		match self {
			Self::Parachain(chain) => chain.expected_block_time(),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.expected_block_time(),
		}
	}

	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, Timestamp), Self::Error> {
		match self {
			Self::Parachain(chain) => chain
				.query_client_update_time_and_height(client_id, client_height)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.query_client_update_time_and_height(client_id, client_height)
				.await
				.map_err(Into::into),
		}
	}

	async fn query_host_consensus_state_proof(
		&self,
		height: Height,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		match self {
			AnyChain::Parachain(chain) =>
				chain.query_host_consensus_state_proof(height).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) =>
				chain.query_host_consensus_state_proof(height).await.map_err(Into::into),
		}
	}

	async fn query_ibc_balance(
		&self,
		asset_id: AnyAssetId,
	) -> Result<Vec<PrefixedCoin>, Self::Error> {
		match (self, asset_id) {
			(Self::Parachain(chain), AnyAssetId::Parachain(asset_id)) =>
				chain.query_ibc_balance(asset_id).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			(Self::Cosmos(chain), AnyAssetId::Cosmos(asset_id)) =>
				chain.query_ibc_balance(asset_id).await.map_err(Into::into),
			_ => unimplemented!(),
		}
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		match self {
			AnyChain::Parachain(chain) => chain.connection_prefix(),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.connection_prefix(),
		}
	}

	fn client_id(&self) -> ClientId {
		match self {
			AnyChain::Parachain(chain) => chain.client_id(),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.client_id(),
		}
	}

	fn connection_id(&self) -> ConnectionId {
		match self {
			AnyChain::Parachain(chain) => chain.connection_id(),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.connection_id(),
		}
	}

	fn client_type(&self) -> ClientType {
		match self {
			AnyChain::Parachain(chain) => chain.client_type(),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.client_type(),
		}
	}

	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
		match self {
			Self::Parachain(chain) =>
				chain.query_timestamp_at(block_number).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.query_timestamp_at(block_number).await.map_err(Into::into),
		}
	}

	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
		match self {
			Self::Parachain(chain) => chain.query_clients().await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.query_clients().await.map_err(Into::into),
		}
	}

	async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
		match self {
			Self::Parachain(chain) => chain.query_channels().await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.query_channels().await.map_err(Into::into),
		}
	}

	async fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>, Self::Error> {
		match self {
			Self::Parachain(chain) =>
				chain.query_connection_using_client(height, client_id).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) =>
				chain.query_connection_using_client(height, client_id).await.map_err(Into::into),
		}
	}

	async fn is_update_required(
		&self,
		latest_height: u64,
		latest_client_height_on_counterparty: u64,
	) -> Result<bool, Self::Error> {
		match self {
			Self::Parachain(chain) => chain
				.is_update_required(latest_height, latest_client_height_on_counterparty)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.is_update_required(latest_height, latest_client_height_on_counterparty)
				.await
				.map_err(Into::into),
		}
	}
	async fn initialize_client_state(
		&self,
	) -> Result<(AnyClientState, AnyConsensusState), Self::Error> {
		match self {
			Self::Parachain(chain) => chain.initialize_client_state().await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.initialize_client_state().await.map_err(Into::into),
		}
	}

	async fn query_client_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ClientId, Self::Error> {
		match self {
			Self::Parachain(chain) => chain
				.query_client_id_from_tx_hash(
					downcast!(tx_id => AnyTransactionId::Parachain)
						.expect("Should be parachain transaction id"),
				)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.query_client_id_from_tx_hash(
					downcast!(tx_id => AnyTransactionId::Cosmos)
						.expect("Should be cosmos transaction id"),
				)
				.await
				.map_err(Into::into),
		}
	}
}

#[async_trait]
impl MisbehaviourHandler for AnyChain {
	async fn check_for_misbehaviour<C: Chain>(
		&self,
		counterparty: &C,
		client_message: AnyClientMessage,
	) -> Result<(), anyhow::Error> {
		match self {
			AnyChain::Parachain(parachain) =>
				parachain.check_for_misbehaviour(counterparty, client_message).await,
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(cosmos) => cosmos.check_for_misbehaviour(counterparty, client_message).await,
		}
	}
}

impl KeyProvider for AnyChain {
	fn account_id(&self) -> Signer {
		match self {
			AnyChain::Parachain(parachain) => parachain.account_id(),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(cosmos) => cosmos.account_id(),
		}
	}
}

#[async_trait]
impl Chain for AnyChain {
	fn name(&self) -> &str {
		match self {
			Self::Parachain(chain) => chain.name(),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.name(),
		}
	}

	fn block_max_weight(&self) -> u64 {
		match self {
			Self::Parachain(chain) => chain.block_max_weight(),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.block_max_weight(),
		}
	}

	async fn estimate_weight(&self, msg: Vec<Any>) -> Result<u64, Self::Error> {
		match self {
			Self::Parachain(chain) => chain.estimate_weight(msg).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.estimate_weight(msg).await.map_err(Into::into),
		}
	}

	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send + Sync>> {
		match self {
			Self::Parachain(chain) => {
				use futures::StreamExt;
				Box::pin(chain.finality_notifications().await.map(Into::into))
			},
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => {
				use futures::StreamExt;
				Box::pin(chain.finality_notifications().await.map(Into::into))
			},
		}
	}

	async fn submit(&self, messages: Vec<Any>) -> Result<Self::TransactionId, Self::Error> {
		match self {
			Self::Parachain(chain) => chain
				.submit(messages)
				.await
				.map_err(Into::into)
				.map(|id| AnyTransactionId::Parachain(id)),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.submit(messages)
				.await
				.map_err(Into::into)
				.map(|id| AnyTransactionId::Cosmos(id)),
		}
	}

	async fn query_client_message(
		&self,
		update: UpdateClient,
	) -> Result<AnyClientMessage, Self::Error> {
		match self {
			Self::Parachain(chain) => chain.query_client_message(update).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.query_client_message(update).await.map_err(Into::into),
		}
	}

	async fn get_proof_height(&self, block_height: Height) -> Height {
		match self {
			Self::Parachain(chain) => chain.get_proof_height(block_height).await,
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.get_proof_height(block_height).await,
		}
	}
}

#[async_trait]
impl LightClientSync for AnyChain {
	async fn is_synced<C: Chain>(&self, counterparty: &C) -> Result<bool, anyhow::Error> {
		match self {
			Self::Parachain(chain) => chain.is_synced(counterparty).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.is_synced(counterparty).await.map_err(Into::into),
		}
	}

	async fn fetch_mandatory_updates<C: Chain>(
		&self,
		counterparty: &C,
	) -> Result<(Vec<Any>, Vec<IbcEvent>), anyhow::Error> {
		match self {
			Self::Parachain(chain) =>
				chain.fetch_mandatory_updates(counterparty).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.fetch_mandatory_updates(counterparty).await.map_err(Into::into),
		}
	}
}

#[cfg(any(test, feature = "testing"))]
impl AnyChain {
	pub fn set_client_id(&mut self, client_id: ClientId) {
		match self {
			Self::Parachain(chain) => chain.set_client_id(client_id),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.set_client_id(client_id),
		}
	}
}

#[cfg(any(test, feature = "testing"))]
#[async_trait]
impl primitives::TestProvider for AnyChain {
	async fn send_transfer(&self, params: MsgTransfer<PrefixedCoin>) -> Result<(), Self::Error> {
		match self {
			Self::Parachain(chain) => chain.send_transfer(params).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.send_transfer(params).await.map_err(Into::into),
		}
	}

	async fn send_ordered_packet(
		&self,
		channel_id: ChannelId,
		timeout: Timeout,
	) -> Result<(), Self::Error> {
		match self {
			Self::Parachain(chain) =>
				chain.send_ordered_packet(channel_id, timeout).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.send_ordered_packet(channel_id, timeout).await.map_err(Into::into),
		}
	}

	async fn subscribe_blocks(&self) -> Pin<Box<dyn Stream<Item = u64> + Send + Sync>> {
		match self {
			Self::Parachain(chain) => chain.subscribe_blocks().await,
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.subscribe_blocks().await,
		}
	}

	fn set_channel_whitelist(&mut self, channel_whitelist: Vec<(ChannelId, PortId)>) {
		match self {
			Self::Parachain(chain) => chain.set_channel_whitelist(channel_whitelist),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.set_channel_whitelist(channel_whitelist),
		}
	}
}

impl AnyConfig {
	pub async fn into_client(self) -> anyhow::Result<AnyChain> {
		Ok(match self {
			AnyConfig::Parachain(config) =>
				AnyChain::Parachain(ParachainClient::new(config).await?),
			#[cfg(feature = "cosmos")]
			AnyConfig::Cosmos(config) => AnyChain::Cosmos(CosmosClient::new(config).await?),
		})
	}

	pub fn set_client_id(&mut self, client_id: ClientId) {
		match self {
			Self::Parachain(chain) => {
				chain.client_id.replace(client_id);
			},
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => {
				chain.client_id.replace(client_id.to_string());
			},
		}
	}

	pub fn set_connection_id(&mut self, connection_id: ConnectionId) {
		match self {
			Self::Parachain(chain) => {
				chain.connection_id.replace(connection_id);
			},
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => {
				chain.connection_id.replace(connection_id.to_string());
			},
		}
	}

	pub fn set_channel_whitelist(&mut self, channel_id: ChannelId, port_id: PortId) {
		match self {
			Self::Parachain(chain) => {
				chain.channel_whitelist.push((channel_id, port_id));
			},
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => {
				chain.channel_whitelist.push((channel_id, port_id));
			},
		}
	}
}
