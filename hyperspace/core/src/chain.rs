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

use crate::{
	chains,
	substrate::{
		default::DefaultConfig, ComposableConfig, PicassoKusamaConfig, PicassoRococoConfig,
	},
};
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
			events::{CodeHash, UpdateClient},
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
use parachain::{ParachainClient, ParachainClientConfig};
use primitives::{
	mock::LocalClientTypes, Chain, CommonClientState, IbcProvider, KeyProvider, LightClientSync,
	MisbehaviourHandler, UpdateType,
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

#[derive(Serialize, Deserialize)]
pub struct CoreConfig {
	pub prometheus_endpoint: Option<String>,
}

impl From<String> for AnyError {
	fn from(s: String) -> Self {
		Self::Other(s)
	}
}

chains! {
	Parachain(ParachainClientConfig, ParachainClient<DefaultConfig>),
	// Dali(ParachainClientConfig, ParachainClient<DaliConfig>),
	Composable(ParachainClientConfig, ParachainClient<ComposableConfig>),
	PicassoRococo(ParachainClientConfig, ParachainClient<PicassoRococoConfig>),
	PicassoKusama(ParachainClientConfig, ParachainClient<PicassoKusamaConfig>),
	#[cfg(feature = "cosmos")]
	Cosmos(CosmosClientConfig, CosmosClient<DefaultConfig>),
}

fn wrap_any_msg_into_wasm(msg: Any, code_hash: Bytes) -> Result<Any, anyhow::Error> {
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
			msg_decoded.consensus_state = AnyConsensusState::wasm(msg_decoded.consensus_state)?;
			msg_decoded.client_state = AnyClientState::wasm(msg_decoded.client_state, code_hash)?;
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

#[derive(Clone)]
pub struct WasmChain {
	pub inner: Box<AnyChain>,
	pub code_hash: Bytes,
}
