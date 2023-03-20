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

use crate::substrate::{dali::DaliConfig, default::DefaultConfig};
use async_trait::async_trait;
#[cfg(feature = "cosmos")]
use cosmos::client::{CosmosClient, CosmosClientConfig};
use futures::Stream;
#[cfg(any(test, feature = "testing"))]
use ibc::applications::transfer::msgs::transfer::MsgTransfer;
use ibc::{
	applications::transfer::PrefixedCoin,
	core::{
		ics02_client::{
			client_state::ClientType,
			events::{CodeId, UpdateClient},
			msgs::{create_client::MsgCreateAnyClient, update_client::MsgUpdateAnyClient},
		},
		ics03_connection::msgs::{
			conn_open_ack::MsgConnectionOpenAck, conn_open_try::MsgConnectionOpenTry,
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	downcast,
	events::IbcEvent,
	signer::Signer,
	timestamp::Timestamp,
	tx_msg::Msg,
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
use ics08_wasm::Bytes;
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState, AnyConsensusState};
#[cfg(any(test, feature = "testing"))]
use pallet_ibc::Timeout;
use parachain::ParachainClient;
use primitives::{
	mock::LocalClientTypes, Chain, IbcProvider, KeyProvider, LightClientSync, MisbehaviourHandler,
	UpdateType,
};
use serde::{Deserialize, Serialize};
use std::{pin::Pin, time::Duration};
use tendermint_proto::Protobuf;
use thiserror::Error;

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
	Dali(parachain::ParachainClientConfig),
	Composable(parachain::ParachainClientConfig),
	Picasso(parachain::ParachainClientConfig),
	#[cfg(feature = "cosmos")]
	Cosmos(CosmosClientConfig),
}

#[derive(Serialize, Deserialize)]
pub struct CoreConfig {
	pub prometheus_endpoint: Option<String>,
}

#[derive(Clone)]
pub struct WasmChain {
	pub inner: Box<AnyChain>,
	pub code_id: Bytes,
}

#[derive(Clone)]
pub enum AnyChain {
	Parachain(ParachainClient<DefaultConfig>),
	Dali(ParachainClient<DaliConfig>),
	Composable(ParachainClient<DefaultConfig>),
	Picasso(ParachainClient<DaliConfig>),
	#[cfg(feature = "cosmos")]
	Cosmos(CosmosClient<DefaultConfig>),
	Wasm(WasmChain),
}

pub enum AnyFinalityEvent {
	Parachain(parachain::finality_protocol::FinalityEvent),
	Dali(parachain::finality_protocol::FinalityEvent),
	Composable(parachain::finality_protocol::FinalityEvent),
	Picasso(parachain::finality_protocol::FinalityEvent),
	#[cfg(feature = "cosmos")]
	Cosmos(cosmos::provider::FinalityEvent),
}

#[derive(Clone)]
pub enum AnyAssetId {
	Parachain(<ParachainClient<DefaultConfig> as IbcProvider>::AssetId),
	Dali(<ParachainClient<DaliConfig> as IbcProvider>::AssetId),
	Composable(<ParachainClient<DaliConfig> as IbcProvider>::AssetId),
	Picasso(<ParachainClient<DaliConfig> as IbcProvider>::AssetId),
	#[cfg(feature = "cosmos")]
	Cosmos(<CosmosClient<DefaultConfig> as IbcProvider>::AssetId),
}

#[derive(Debug)]
pub enum AnyTransactionId {
	Parachain(parachain::provider::TransactionId<sp_core::H256>),
	Dali(parachain::provider::TransactionId<sp_core::H256>),
	Composable(parachain::provider::TransactionId<sp_core::H256>),
	Picasso(parachain::provider::TransactionId<sp_core::H256>),
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

impl From<anyhow::Error> for AnyError {
	fn from(e: anyhow::Error) -> Self {
		Self::Other(e.to_string())
	}
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
			AnyChain::Dali(chain) => {
				let finality_event = ibc::downcast!(finality_event => AnyFinalityEvent::Dali)
					.ok_or_else(|| AnyError::Other("Invalid finality event type".to_owned()))?;
				chain.query_latest_ibc_events(finality_event, counterparty).await
			},
			AnyChain::Composable(chain) => {
				let finality_event = ibc::downcast!(finality_event => AnyFinalityEvent::Composable)
					.ok_or_else(|| AnyError::Other("Invalid finality event type".to_owned()))?;
				chain.query_latest_ibc_events(finality_event, counterparty).await
			},
			AnyChain::Picasso(chain) => {
				let finality_event = ibc::downcast!(finality_event => AnyFinalityEvent::Picasso)
					.ok_or_else(|| AnyError::Other("Invalid finality event type".to_owned()))?;
				chain.query_latest_ibc_events(finality_event, counterparty).await
			},
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => {
				let finality_event = downcast!(finality_event => AnyFinalityEvent::Cosmos)
					.ok_or_else(|| AnyError::Other("Invalid finality event type".to_owned()))?;
				chain.query_latest_ibc_events(finality_event, counterparty).await
			},
			AnyChain::Wasm(c) =>
				c.inner.query_latest_ibc_events(finality_event, counterparty).await,
		}
	}

	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>> {
		match self {
			Self::Parachain(chain) => chain.ibc_events().await,
			Self::Dali(chain) => chain.ibc_events().await,
			Self::Composable(chain) => chain.ibc_events().await,
			Self::Picasso(chain) => chain.ibc_events().await,
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.ibc_events().await,
			Self::Wasm(c) => c.inner.ibc_events().await,
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
			AnyChain::Dali(chain) => chain
				.query_client_consensus(at, client_id, consensus_height)
				.await
				.map_err(Into::into),
			AnyChain::Composable(chain) => chain
				.query_client_consensus(at, client_id, consensus_height)
				.await
				.map_err(Into::into),
			AnyChain::Picasso(chain) => chain
				.query_client_consensus(at, client_id, consensus_height)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain
				.query_client_consensus(at, client_id, consensus_height)
				.await
				.map_err(Into::into),
			AnyChain::Wasm(c) =>
				c.inner.query_client_consensus(at, client_id, consensus_height).await,
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
			AnyChain::Dali(chain) =>
				chain.query_client_state(at, client_id).await.map_err(Into::into),
			AnyChain::Composable(chain) =>
				chain.query_client_state(at, client_id).await.map_err(Into::into),
			AnyChain::Picasso(chain) =>
				chain.query_client_state(at, client_id).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.query_client_state(at, client_id).await.map_err(Into::into),
			AnyChain::Wasm(c) => c.inner.query_client_state(at, client_id).await,
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
			AnyChain::Dali(chain) =>
				chain.query_connection_end(at, connection_id).await.map_err(Into::into),
			AnyChain::Composable(chain) =>
				chain.query_connection_end(at, connection_id).await.map_err(Into::into),
			AnyChain::Picasso(chain) =>
				chain.query_connection_end(at, connection_id).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) =>
				chain.query_connection_end(at, connection_id).await.map_err(Into::into),
			AnyChain::Wasm(c) => c.inner.query_connection_end(at, connection_id).await,
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
			AnyChain::Dali(chain) =>
				chain.query_channel_end(at, channel_id, port_id).await.map_err(Into::into),
			AnyChain::Composable(chain) =>
				chain.query_channel_end(at, channel_id, port_id).await.map_err(Into::into),
			AnyChain::Picasso(chain) =>
				chain.query_channel_end(at, channel_id, port_id).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) =>
				chain.query_channel_end(at, channel_id, port_id).await.map_err(Into::into),
			AnyChain::Wasm(c) => c.inner.query_channel_end(at, channel_id, port_id).await,
		}
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		match self {
			AnyChain::Parachain(chain) => chain.query_proof(at, keys).await.map_err(Into::into),
			AnyChain::Dali(chain) => chain.query_proof(at, keys).await.map_err(Into::into),
			AnyChain::Composable(chain) => chain.query_proof(at, keys).await.map_err(Into::into),
			AnyChain::Picasso(chain) => chain.query_proof(at, keys).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.query_proof(at, keys).await.map_err(Into::into),
			AnyChain::Wasm(c) => c.inner.query_proof(at, keys).await,
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
			AnyChain::Dali(chain) => chain
				.query_packet_commitment(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			AnyChain::Composable(chain) => chain
				.query_packet_commitment(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			AnyChain::Picasso(chain) => chain
				.query_packet_commitment(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain
				.query_packet_commitment(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			AnyChain::Wasm(c) =>
				c.inner.query_packet_commitment(at, port_id, channel_id, seq).await,
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
			AnyChain::Dali(chain) => chain
				.query_packet_acknowledgement(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			AnyChain::Composable(chain) => chain
				.query_packet_acknowledgement(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			AnyChain::Picasso(chain) => chain
				.query_packet_acknowledgement(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain
				.query_packet_acknowledgement(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			AnyChain::Wasm(c) =>
				c.inner.query_packet_acknowledgement(at, port_id, channel_id, seq).await,
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
			AnyChain::Dali(chain) => chain
				.query_next_sequence_recv(at, port_id, channel_id)
				.await
				.map_err(Into::into),
			AnyChain::Composable(chain) => chain
				.query_next_sequence_recv(at, port_id, channel_id)
				.await
				.map_err(Into::into),
			AnyChain::Picasso(chain) => chain
				.query_next_sequence_recv(at, port_id, channel_id)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain
				.query_next_sequence_recv(at, port_id, channel_id)
				.await
				.map_err(Into::into),
			AnyChain::Wasm(c) => c.inner.query_next_sequence_recv(at, port_id, channel_id).await,
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
			AnyChain::Dali(chain) => chain
				.query_packet_receipt(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			AnyChain::Composable(chain) => chain
				.query_packet_receipt(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			AnyChain::Picasso(chain) => chain
				.query_packet_receipt(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain
				.query_packet_receipt(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
			AnyChain::Wasm(c) => c.inner.query_packet_receipt(at, port_id, channel_id, seq).await,
		}
	}

	async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error> {
		match self {
			AnyChain::Parachain(chain) =>
				chain.latest_height_and_timestamp().await.map_err(Into::into),
			AnyChain::Dali(chain) => chain.latest_height_and_timestamp().await.map_err(Into::into),
			AnyChain::Composable(chain) =>
				chain.latest_height_and_timestamp().await.map_err(Into::into),
			AnyChain::Picasso(chain) =>
				chain.latest_height_and_timestamp().await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.latest_height_and_timestamp().await.map_err(Into::into),
			AnyChain::Wasm(c) => c.inner.latest_height_and_timestamp().await,
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
			Self::Dali(chain) => chain
				.query_packet_commitments(at, channel_id, port_id)
				.await
				.map_err(Into::into),
			Self::Composable(chain) => chain
				.query_packet_commitments(at, channel_id, port_id)
				.await
				.map_err(Into::into),
			Self::Picasso(chain) => chain
				.query_packet_commitments(at, channel_id, port_id)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.query_packet_commitments(at, channel_id, port_id)
				.await
				.map_err(Into::into),
			Self::Wasm(c) => c.inner.query_packet_commitments(at, channel_id, port_id).await,
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
			Self::Dali(chain) => chain
				.query_packet_acknowledgements(at, channel_id, port_id)
				.await
				.map_err(Into::into),
			Self::Composable(chain) => chain
				.query_packet_acknowledgements(at, channel_id, port_id)
				.await
				.map_err(Into::into),
			Self::Picasso(chain) => chain
				.query_packet_acknowledgements(at, channel_id, port_id)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.query_packet_acknowledgements(at, channel_id, port_id)
				.await
				.map_err(Into::into),
			Self::Wasm(c) => c.inner.query_packet_acknowledgements(at, channel_id, port_id).await,
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
			Self::Dali(chain) => chain
				.query_unreceived_packets(at, channel_id, port_id, seqs)
				.await
				.map_err(Into::into),
			Self::Composable(chain) => chain
				.query_unreceived_packets(at, channel_id, port_id, seqs)
				.await
				.map_err(Into::into),
			Self::Picasso(chain) => chain
				.query_unreceived_packets(at, channel_id, port_id, seqs)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.query_unreceived_packets(at, channel_id, port_id, seqs)
				.await
				.map_err(Into::into),
			Self::Wasm(c) => c.inner.query_unreceived_packets(at, channel_id, port_id, seqs).await,
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
			Self::Dali(chain) => chain
				.query_unreceived_acknowledgements(at, channel_id, port_id, seqs)
				.await
				.map_err(Into::into),
			Self::Composable(chain) => chain
				.query_unreceived_acknowledgements(at, channel_id, port_id, seqs)
				.await
				.map_err(Into::into),
			Self::Picasso(chain) => chain
				.query_unreceived_acknowledgements(at, channel_id, port_id, seqs)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.query_unreceived_acknowledgements(at, channel_id, port_id, seqs)
				.await
				.map_err(Into::into),
			Self::Wasm(c) =>
				c.inner.query_unreceived_acknowledgements(at, channel_id, port_id, seqs).await,
		}
	}

	fn channel_whitelist(&self) -> Vec<(ChannelId, PortId)> {
		match self {
			Self::Parachain(chain) => chain.channel_whitelist(),
			Self::Dali(chain) => chain.channel_whitelist(),
			Self::Composable(chain) => chain.channel_whitelist(),
			Self::Picasso(chain) => chain.channel_whitelist(),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.channel_whitelist(),
			Self::Wasm(c) => c.inner.channel_whitelist(),
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
			Self::Dali(chain) =>
				chain.query_connection_channels(at, connection_id).await.map_err(Into::into),
			Self::Composable(chain) =>
				chain.query_connection_channels(at, connection_id).await.map_err(Into::into),
			Self::Picasso(chain) =>
				chain.query_connection_channels(at, connection_id).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) =>
				chain.query_connection_channels(at, connection_id).await.map_err(Into::into),
			Self::Wasm(c) => c.inner.query_connection_channels(at, connection_id).await,
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
			Self::Dali(chain) =>
				chain.query_send_packets(channel_id, port_id, seqs).await.map_err(Into::into),
			Self::Composable(chain) =>
				chain.query_send_packets(channel_id, port_id, seqs).await.map_err(Into::into),
			Self::Picasso(chain) =>
				chain.query_send_packets(channel_id, port_id, seqs).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) =>
				chain.query_send_packets(channel_id, port_id, seqs).await.map_err(Into::into),
			Self::Wasm(c) => c.inner.query_send_packets(channel_id, port_id, seqs).await,
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
			Self::Dali(chain) =>
				chain.query_recv_packets(channel_id, port_id, seqs).await.map_err(Into::into),
			Self::Composable(chain) =>
				chain.query_recv_packets(channel_id, port_id, seqs).await.map_err(Into::into),
			Self::Picasso(chain) =>
				chain.query_recv_packets(channel_id, port_id, seqs).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) =>
				chain.query_recv_packets(channel_id, port_id, seqs).await.map_err(Into::into),
			Self::Wasm(c) => c.inner.query_recv_packets(channel_id, port_id, seqs).await,
		}
	}

	fn expected_block_time(&self) -> Duration {
		match self {
			Self::Parachain(chain) => chain.expected_block_time(),
			Self::Dali(chain) => chain.expected_block_time(),
			Self::Composable(chain) => chain.expected_block_time(),
			Self::Picasso(chain) => chain.expected_block_time(),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.expected_block_time(),
			Self::Wasm(c) => c.inner.expected_block_time(),
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
			Self::Dali(chain) => chain
				.query_client_update_time_and_height(client_id, client_height)
				.await
				.map_err(Into::into),
			Self::Composable(chain) => chain
				.query_client_update_time_and_height(client_id, client_height)
				.await
				.map_err(Into::into),
			Self::Picasso(chain) => chain
				.query_client_update_time_and_height(client_id, client_height)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.query_client_update_time_and_height(client_id, client_height)
				.await
				.map_err(Into::into),
			Self::Wasm(c) =>
				c.inner.query_client_update_time_and_height(client_id, client_height).await,
		}
	}

	async fn query_host_consensus_state_proof(
		&self,
		height: &AnyClientState,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		match self {
			AnyChain::Parachain(chain) =>
				chain.query_host_consensus_state_proof(height).await.map_err(Into::into),
			AnyChain::Dali(chain) =>
				chain.query_host_consensus_state_proof(height).await.map_err(Into::into),
			AnyChain::Composable(chain) =>
				chain.query_host_consensus_state_proof(height).await.map_err(Into::into),
			AnyChain::Picasso(chain) =>
				chain.query_host_consensus_state_proof(height).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) =>
				chain.query_host_consensus_state_proof(height).await.map_err(Into::into),
			AnyChain::Wasm(c) => c.inner.query_host_consensus_state_proof(height).await,
		}
	}

	async fn query_ibc_balance(
		&self,
		asset_id: AnyAssetId,
	) -> Result<Vec<PrefixedCoin>, Self::Error> {
		match (self, asset_id) {
			(Self::Parachain(chain), AnyAssetId::Parachain(asset_id)) =>
				chain.query_ibc_balance(asset_id.into()).await.map_err(Into::into),
			(Self::Dali(chain), AnyAssetId::Dali(asset_id)) =>
				chain.query_ibc_balance(asset_id.into()).await.map_err(Into::into),
			(Self::Composable(chain), AnyAssetId::Composable(asset_id)) =>
				chain.query_ibc_balance(asset_id.into()).await.map_err(Into::into),
			(Self::Picasso(chain), AnyAssetId::Picasso(asset_id)) =>
				chain.query_ibc_balance(asset_id.into()).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			(Self::Cosmos(chain), AnyAssetId::Cosmos(asset_id)) =>
				chain.query_ibc_balance(asset_id).await.map_err(Into::into),
			(Self::Wasm(c), asset_id) => c.inner.query_ibc_balance(asset_id).await,
			(chain, _) => panic!("query_ibc_balance is not implemented for {}", chain.name()),
		}
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		match self {
			AnyChain::Parachain(chain) => chain.connection_prefix(),
			AnyChain::Dali(chain) => chain.connection_prefix(),
			AnyChain::Composable(chain) => chain.connection_prefix(),
			AnyChain::Picasso(chain) => chain.connection_prefix(),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.connection_prefix(),
			AnyChain::Wasm(c) => c.inner.connection_prefix(),
		}
	}

	fn client_id(&self) -> ClientId {
		match self {
			AnyChain::Parachain(chain) => chain.client_id(),
			AnyChain::Dali(chain) => chain.client_id(),
			AnyChain::Composable(chain) => chain.client_id(),
			AnyChain::Picasso(chain) => chain.client_id(),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.client_id(),
			AnyChain::Wasm(c) => c.inner.client_id(),
		}
	}

	fn connection_id(&self) -> ConnectionId {
		match self {
			AnyChain::Parachain(chain) => chain.connection_id(),
			AnyChain::Dali(chain) => chain.connection_id(),
			AnyChain::Composable(chain) => chain.connection_id(),
			AnyChain::Picasso(chain) => chain.connection_id(),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.connection_id(),
			AnyChain::Wasm(c) => c.inner.connection_id(),
		}
	}

	fn client_type(&self) -> ClientType {
		match self {
			AnyChain::Parachain(chain) => chain.client_type(),
			AnyChain::Dali(chain) => chain.client_type(),
			AnyChain::Composable(chain) => chain.client_type(),
			AnyChain::Picasso(chain) => chain.client_type(),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(chain) => chain.client_type(),
			AnyChain::Wasm(c) => c.inner.client_type(),
		}
	}

	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
		match self {
			Self::Parachain(chain) =>
				chain.query_timestamp_at(block_number).await.map_err(Into::into),
			Self::Dali(chain) => chain.query_timestamp_at(block_number).await.map_err(Into::into),
			Self::Composable(chain) =>
				chain.query_timestamp_at(block_number).await.map_err(Into::into),
			Self::Picasso(chain) =>
				chain.query_timestamp_at(block_number).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.query_timestamp_at(block_number).await.map_err(Into::into),
			Self::Wasm(c) => c.inner.query_timestamp_at(block_number).await,
		}
	}

	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
		match self {
			Self::Parachain(chain) => chain.query_clients().await.map_err(Into::into),
			Self::Dali(chain) => chain.query_clients().await.map_err(Into::into),
			Self::Composable(chain) => chain.query_clients().await.map_err(Into::into),
			Self::Picasso(chain) => chain.query_clients().await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.query_clients().await.map_err(Into::into),
			Self::Wasm(c) => c.inner.query_clients().await,
		}
	}

	async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
		match self {
			Self::Parachain(chain) => chain.query_channels().await.map_err(Into::into),
			Self::Dali(chain) => chain.query_channels().await.map_err(Into::into),
			Self::Composable(chain) => chain.query_channels().await.map_err(Into::into),
			Self::Picasso(chain) => chain.query_channels().await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.query_channels().await.map_err(Into::into),
			Self::Wasm(c) => c.inner.query_channels().await,
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
			Self::Dali(chain) =>
				chain.query_connection_using_client(height, client_id).await.map_err(Into::into),
			Self::Composable(chain) =>
				chain.query_connection_using_client(height, client_id).await.map_err(Into::into),
			Self::Picasso(chain) =>
				chain.query_connection_using_client(height, client_id).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) =>
				chain.query_connection_using_client(height, client_id).await.map_err(Into::into),
			Self::Wasm(c) => c.inner.query_connection_using_client(height, client_id).await,
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
			Self::Dali(chain) => chain
				.is_update_required(latest_height, latest_client_height_on_counterparty)
				.await
				.map_err(Into::into),
			Self::Composable(chain) => chain
				.is_update_required(latest_height, latest_client_height_on_counterparty)
				.await
				.map_err(Into::into),
			Self::Picasso(chain) => chain
				.is_update_required(latest_height, latest_client_height_on_counterparty)
				.await
				.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain
				.is_update_required(latest_height, latest_client_height_on_counterparty)
				.await
				.map_err(Into::into),
			Self::Wasm(c) => c
				.inner
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
			Self::Dali(chain) => chain.initialize_client_state().await.map_err(Into::into),
			Self::Composable(chain) => chain.initialize_client_state().await.map_err(Into::into),
			Self::Picasso(chain) => chain.initialize_client_state().await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.initialize_client_state().await.map_err(Into::into),
			Self::Wasm(c) => c.inner.initialize_client_state().await,
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
			Self::Dali(chain) => chain
				.query_client_id_from_tx_hash(
					downcast!(tx_id => AnyTransactionId::Dali)
						.expect("Should be dali transaction id"),
				)
				.await
				.map_err(Into::into),
			Self::Composable(chain) => chain
				.query_client_id_from_tx_hash(
					downcast!(tx_id => AnyTransactionId::Dali)
						.expect("Should be dali transaction id"),
				)
				.await
				.map_err(Into::into),
			Self::Picasso(chain) => chain
				.query_client_id_from_tx_hash(
					downcast!(tx_id => AnyTransactionId::Dali)
						.expect("Should be dali transaction id"),
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
			Self::Wasm(c) => c.inner.query_client_id_from_tx_hash(tx_id).await,
		}
	}

	async fn upload_wasm(&self, wasm: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
		match self {
			Self::Parachain(chain) => chain.upload_wasm(wasm).await.map_err(Into::into),
			Self::Dali(chain) => chain.upload_wasm(wasm).await.map_err(Into::into),
			Self::Composable(chain) => chain.upload_wasm(wasm).await.map_err(Into::into),
			Self::Picasso(chain) => chain.upload_wasm(wasm).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.upload_wasm(wasm).await.map_err(Into::into),
			Self::Wasm(c) => c.inner.upload_wasm(wasm).await,
		}
	}

	async fn on_undelivered_sequences(&self, seqs: &[u64]) -> Result<(), Self::Error> {
		match self {
			Self::Parachain(chain) =>
				chain.on_undelivered_sequences(seqs).await.map_err(Into::into),
			Self::Dali(chain) => chain.on_undelivered_sequences(seqs).await.map_err(Into::into),
			Self::Composable(chain) =>
				chain.on_undelivered_sequences(seqs).await.map_err(Into::into),
			Self::Picasso(chain) => chain.on_undelivered_sequences(seqs).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.on_undelivered_sequences(seqs).await.map_err(Into::into),
			Self::Wasm(c) => c.inner.on_undelivered_sequences(seqs).await,
		}
	}

	fn has_undelivered_sequences(&self) -> bool {
		match self {
			Self::Parachain(chain) => chain.has_undelivered_sequences(),
			Self::Dali(chain) => chain.has_undelivered_sequences(),
			Self::Composable(chain) => chain.has_undelivered_sequences(),
			Self::Picasso(chain) => chain.has_undelivered_sequences(),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.has_undelivered_sequences(),
			Self::Wasm(c) => c.inner.has_undelivered_sequences(),
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
			AnyChain::Dali(dali) => dali.check_for_misbehaviour(counterparty, client_message).await,
			AnyChain::Composable(parachain) =>
				parachain.check_for_misbehaviour(counterparty, client_message).await,
			AnyChain::Picasso(dali) =>
				dali.check_for_misbehaviour(counterparty, client_message).await,
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(cosmos) => cosmos.check_for_misbehaviour(counterparty, client_message).await,
			AnyChain::Wasm(c) => c.inner.check_for_misbehaviour(counterparty, client_message).await,
		}
	}
}

impl KeyProvider for AnyChain {
	fn account_id(&self) -> Signer {
		match self {
			AnyChain::Parachain(parachain) => parachain.account_id(),
			AnyChain::Dali(dali) => dali.account_id(),
			AnyChain::Composable(parachain) => parachain.account_id(),
			AnyChain::Picasso(dali) => dali.account_id(),
			#[cfg(feature = "cosmos")]
			AnyChain::Cosmos(cosmos) => cosmos.account_id(),
			AnyChain::Wasm(c) => c.inner.account_id(),
		}
	}
}

#[async_trait]
impl Chain for AnyChain {
	fn name(&self) -> &str {
		match self {
			Self::Parachain(chain) => chain.name(),
			Self::Dali(chain) => chain.name(),
			Self::Composable(chain) => chain.name(),
			Self::Picasso(chain) => chain.name(),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.name(),
			Self::Wasm(c) => c.inner.name(),
		}
	}

	fn block_max_weight(&self) -> u64 {
		match self {
			Self::Parachain(chain) => chain.block_max_weight(),
			Self::Dali(chain) => chain.block_max_weight(),
			Self::Composable(chain) => chain.block_max_weight(),
			Self::Picasso(chain) => chain.block_max_weight(),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.block_max_weight(),
			Self::Wasm(c) => c.inner.block_max_weight(),
		}
	}

	async fn estimate_weight(&self, msg: Vec<Any>) -> Result<u64, Self::Error> {
		match self {
			Self::Parachain(chain) => chain.estimate_weight(msg).await.map_err(Into::into),
			Self::Dali(chain) => chain.estimate_weight(msg).await.map_err(Into::into),
			Self::Composable(chain) => chain.estimate_weight(msg).await.map_err(Into::into),
			Self::Picasso(chain) => chain.estimate_weight(msg).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.estimate_weight(msg).await.map_err(Into::into),
			Self::Wasm(c) => c.inner.estimate_weight(msg).await,
		}
	}

	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send + Sync>> {
		match self {
			Self::Parachain(chain) => {
				use futures::StreamExt;
				Box::pin(chain.finality_notifications().await.map(AnyFinalityEvent::Parachain))
			},
			Self::Dali(chain) => {
				use futures::StreamExt;
				Box::pin(chain.finality_notifications().await.map(AnyFinalityEvent::Dali))
			},
			Self::Composable(chain) => {
				use futures::StreamExt;
				Box::pin(chain.finality_notifications().await.map(AnyFinalityEvent::Parachain))
			},
			Self::Picasso(chain) => {
				use futures::StreamExt;
				Box::pin(chain.finality_notifications().await.map(AnyFinalityEvent::Dali))
			},
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => {
				use futures::StreamExt;
				Box::pin(chain.finality_notifications().await.map(AnyFinalityEvent::Cosmos))
			},
			Self::Wasm(c) => c.inner.finality_notifications().await,
		}
	}

	async fn submit(&self, messages: Vec<Any>) -> Result<Self::TransactionId, Self::Error> {
		match self {
			Self::Parachain(chain) => chain
				.submit(messages)
				.await
				.map_err(Into::into)
				.map(|id| AnyTransactionId::Parachain(id)),
			Self::Dali(chain) => chain
				.submit(messages)
				.await
				.map_err(Into::into)
				.map(|id| AnyTransactionId::Parachain(id)),
			Self::Composable(chain) => chain
				.submit(messages)
				.await
				.map_err(Into::into)
				.map(|id| AnyTransactionId::Parachain(id)),
			Self::Picasso(chain) => chain
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
			Self::Wasm(chain) => {
				let messages = messages
					.into_iter()
					.map(|msg| wrap_any_msg_into_wasm(msg, chain.code_id.clone()))
					.collect::<Result<Vec<_>, _>>()?;
				chain.inner.submit(messages).await.map_err(Into::into)
			},
		}
	}

	async fn query_client_message(
		&self,
		update: UpdateClient,
	) -> Result<AnyClientMessage, Self::Error> {
		match self {
			Self::Parachain(chain) => chain.query_client_message(update).await.map_err(Into::into),
			Self::Dali(chain) => chain.query_client_message(update).await.map_err(Into::into),
			Self::Composable(chain) => chain.query_client_message(update).await.map_err(Into::into),
			Self::Picasso(chain) => chain.query_client_message(update).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.query_client_message(update).await.map_err(Into::into),
			Self::Wasm(c) => c.inner.query_client_message(update).await,
		}
	}

	async fn get_proof_height(&self, block_height: Height) -> Height {
		match self {
			Self::Parachain(chain) => chain.get_proof_height(block_height).await,
			Self::Dali(chain) => chain.get_proof_height(block_height).await,
			Self::Composable(chain) => chain.get_proof_height(block_height).await,
			Self::Picasso(chain) => chain.get_proof_height(block_height).await,
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.get_proof_height(block_height).await,
			Self::Wasm(c) => c.inner.get_proof_height(block_height).await,
		}
	}
}

#[async_trait]
impl LightClientSync for AnyChain {
	async fn is_synced<C: Chain>(&self, counterparty: &C) -> Result<bool, anyhow::Error> {
		match self {
			Self::Parachain(chain) => chain.is_synced(counterparty).await.map_err(Into::into),
			Self::Dali(chain) => chain.is_synced(counterparty).await.map_err(Into::into),
			Self::Composable(chain) => chain.is_synced(counterparty).await.map_err(Into::into),
			Self::Picasso(chain) => chain.is_synced(counterparty).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.is_synced(counterparty).await.map_err(Into::into),
			Self::Wasm(c) => c.inner.is_synced(counterparty).await,
		}
	}

	async fn fetch_mandatory_updates<C: Chain>(
		&self,
		counterparty: &C,
	) -> Result<(Vec<Any>, Vec<IbcEvent>), anyhow::Error> {
		match self {
			Self::Parachain(chain) =>
				chain.fetch_mandatory_updates(counterparty).await.map_err(Into::into),
			Self::Dali(chain) =>
				chain.fetch_mandatory_updates(counterparty).await.map_err(Into::into),
			Self::Composable(chain) =>
				chain.fetch_mandatory_updates(counterparty).await.map_err(Into::into),
			Self::Picasso(chain) =>
				chain.fetch_mandatory_updates(counterparty).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.fetch_mandatory_updates(counterparty).await.map_err(Into::into),
			Self::Wasm(c) => c.inner.fetch_mandatory_updates(counterparty).await,
		}
	}
}

fn wrap_any_msg_into_wasm(msg: Any, code_id: Bytes) -> Result<Any, anyhow::Error> {
	// TODO: consider rewriting with Ics26Envelope
	use ibc::core::{
		ics02_client::msgs::{
			create_client::TYPE_URL as CREATE_CLIENT_TYPE_URL,
			update_client::TYPE_URL as UPDATE_CLIENT_TYPE_URL,
		},
		ics03_connection::msgs::{
			conn_open_ack::TYPE_URL as CONN_OPEN_ACK_TYPE_URL,
			conn_open_try::TYPE_URL as CONN_OPEN_TRY_TYPE_URL,
		},
	};

	let msg = match msg.type_url.as_str() {
		CREATE_CLIENT_TYPE_URL => {
			let mut msg_decoded =
				MsgCreateAnyClient::<LocalClientTypes>::decode_vec(&msg.value).unwrap();
			msg_decoded.consensus_state =
				AnyConsensusState::wasm(msg_decoded.consensus_state, code_id.clone())?;
			msg_decoded.client_state = AnyClientState::wasm(msg_decoded.client_state, code_id)?;
			msg_decoded.to_any()
		},
		CONN_OPEN_TRY_TYPE_URL => {
			let msg_decoded =
				MsgConnectionOpenTry::<LocalClientTypes>::decode_vec(&msg.value).unwrap();
			msg_decoded.to_any()
		},
		CONN_OPEN_ACK_TYPE_URL => {
			let msg_decoded =
				MsgConnectionOpenAck::<LocalClientTypes>::decode_vec(&msg.value).unwrap();
			msg_decoded.to_any()
		},
		UPDATE_CLIENT_TYPE_URL => {
			let mut msg_decoded =
				MsgUpdateAnyClient::<LocalClientTypes>::decode_vec(&msg.value).unwrap();
			msg_decoded.client_message = AnyClientMessage::wasm(msg_decoded.client_message)?;
			let any = msg_decoded.to_any();
			any
		},
		_ => msg,
	};
	Ok(msg)
}

#[cfg(any(test, feature = "testing"))]
impl AnyChain {
	pub fn set_client_id(&mut self, client_id: ClientId) {
		match self {
			Self::Parachain(chain) => chain.set_client_id(client_id),
			Self::Dali(chain) => chain.set_client_id(client_id),
			Self::Composable(chain) => chain.set_client_id(client_id),
			Self::Picasso(chain) => chain.set_client_id(client_id),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.set_client_id(client_id),
			Self::Wasm(chain) => chain.inner.set_client_id(client_id),
		}
	}
}

#[cfg(any(test, feature = "testing"))]
#[async_trait]
impl primitives::TestProvider for AnyChain {
	async fn send_transfer(&self, params: MsgTransfer<PrefixedCoin>) -> Result<(), Self::Error> {
		match self {
			Self::Parachain(chain) => chain.send_transfer(params).await.map_err(Into::into),
			Self::Dali(chain) => chain.send_transfer(params).await.map_err(Into::into),
			Self::Composable(chain) => chain.send_transfer(params).await.map_err(Into::into),
			Self::Picasso(chain) => chain.send_transfer(params).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.send_transfer(params).await.map_err(Into::into),
			Self::Wasm(c) => c.inner.send_transfer(params).await,
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
			Self::Dali(chain) =>
				chain.send_ordered_packet(channel_id, timeout).await.map_err(Into::into),
			Self::Composable(chain) =>
				chain.send_ordered_packet(channel_id, timeout).await.map_err(Into::into),
			Self::Picasso(chain) =>
				chain.send_ordered_packet(channel_id, timeout).await.map_err(Into::into),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.send_ordered_packet(channel_id, timeout).await.map_err(Into::into),
			Self::Wasm(c) => c.inner.send_ordered_packet(channel_id, timeout).await,
		}
	}

	async fn subscribe_blocks(&self) -> Pin<Box<dyn Stream<Item = u64> + Send + Sync>> {
		match self {
			Self::Parachain(chain) => chain.subscribe_blocks().await,
			Self::Dali(chain) => chain.subscribe_blocks().await,
			Self::Composable(chain) => chain.subscribe_blocks().await,
			Self::Picasso(chain) => chain.subscribe_blocks().await,
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.subscribe_blocks().await,
			Self::Wasm(c) => c.inner.subscribe_blocks().await,
		}
	}

	fn set_channel_whitelist(&mut self, channel_whitelist: Vec<(ChannelId, PortId)>) {
		match self {
			Self::Parachain(chain) => chain.set_channel_whitelist(channel_whitelist),
			Self::Dali(chain) => chain.set_channel_whitelist(channel_whitelist),
			Self::Composable(chain) => chain.set_channel_whitelist(channel_whitelist),
			Self::Picasso(chain) => chain.set_channel_whitelist(channel_whitelist),
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => chain.set_channel_whitelist(channel_whitelist),
			Self::Wasm(c) => c.inner.set_channel_whitelist(channel_whitelist),
		}
	}
}

impl AnyConfig {
	pub fn wasm_code_id(&self) -> Option<CodeId> {
		let maybe_code_id = match self {
			AnyConfig::Parachain(config) => config.wasm_code_id.as_ref(),
			AnyConfig::Dali(config) => config.wasm_code_id.as_ref(),
			AnyConfig::Composable(config) => config.wasm_code_id.as_ref(),
			AnyConfig::Picasso(config) => config.wasm_code_id.as_ref(),
			#[cfg(feature = "cosmos")]
			AnyConfig::Cosmos(config) => config.wasm_code_id.as_ref(),
		};

		let maybe_code_id =
			maybe_code_id.map(|s| hex::decode(s).expect("Wasm code id is hex-encoded"));

		maybe_code_id
	}

	pub fn set_wasm_code_id(&mut self, code_id: String) {
		match self {
			AnyConfig::Parachain(config) => config.wasm_code_id = Some(code_id),
			AnyConfig::Dali(config) => config.wasm_code_id = Some(code_id),
			AnyConfig::Composable(config) => config.wasm_code_id = Some(code_id),
			AnyConfig::Picasso(config) => config.wasm_code_id = Some(code_id),
			#[cfg(feature = "cosmos")]
			AnyConfig::Cosmos(config) => config.wasm_code_id = Some(code_id),
		}
	}

	pub async fn into_client(self) -> anyhow::Result<AnyChain> {
		let maybe_wasm_code_id = self.wasm_code_id();
		let chain = match self {
			AnyConfig::Parachain(config) =>
				AnyChain::Parachain(ParachainClient::new(config).await?),
			AnyConfig::Dali(config) => AnyChain::Dali(ParachainClient::new(config).await?),
			AnyConfig::Composable(config) =>
				AnyChain::Composable(ParachainClient::new(config).await?),
			AnyConfig::Picasso(config) => AnyChain::Picasso(ParachainClient::new(config).await?),
			#[cfg(feature = "cosmos")]
			AnyConfig::Cosmos(config) => AnyChain::Cosmos(CosmosClient::new(config).await?),
		};
		if let Some(code_id) = maybe_wasm_code_id {
			Ok(AnyChain::Wasm(WasmChain { inner: Box::new(chain), code_id }))
		} else {
			Ok(chain)
		}
	}

	pub fn set_client_id(&mut self, client_id: ClientId) {
		match self {
			Self::Parachain(chain) => {
				chain.client_id.replace(client_id);
			},
			Self::Dali(chain) => {
				chain.client_id.replace(client_id);
			},
			Self::Composable(chain) => {
				chain.client_id.replace(client_id);
			},
			Self::Picasso(chain) => {
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
			Self::Dali(chain) => {
				chain.connection_id.replace(connection_id);
			},
			Self::Composable(chain) => {
				chain.connection_id.replace(connection_id);
			},
			Self::Picasso(chain) => {
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
			Self::Dali(chain) => {
				chain.channel_whitelist.push((channel_id, port_id));
			},
			Self::Composable(chain) => {
				chain.channel_whitelist.push((channel_id, port_id));
			},
			Self::Picasso(chain) => {
				chain.channel_whitelist.push((channel_id, port_id));
			},
			#[cfg(feature = "cosmos")]
			Self::Cosmos(chain) => {
				chain.channel_whitelist.push((channel_id, port_id));
			},
		}
	}
}
