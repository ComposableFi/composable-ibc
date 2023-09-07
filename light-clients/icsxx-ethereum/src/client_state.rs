// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
	client_def::EthereumClient,
	error::Error,
	proto::{ClientState as RawClientState, LightClientState as RawLightClientState},
};
use alloc::{format, string::ToString, vec::Vec};
use anyhow::anyhow;
use core::{fmt::Debug, marker::PhantomData, time::Duration};
use ethereum_consensus::primitives::Hash32;
use ibc::{
	core::{ics02_client::client_state::ClientType, ics24_host::identifier::ChainId},
	Height,
};
use ibc_proto::google::protobuf::Any;
use serde::{Deserialize, Serialize};
use sync_committee_verifier::{BlsVerify, LightClientState};
use tendermint_proto::Protobuf;

/// Protobuf type url for GRANDPA ClientState
pub const ETHEREUM_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.ethereum.v1.ClientState";

#[derive(PartialEq, Clone, Debug, Default, Eq)]
pub struct ClientState<H> {
	pub inner: LightClientState,
	pub frozen_height: Option<Height>,
	pub latest_height: u32,
	pub _phantom: PhantomData<H>,
}

impl<H: Clone> Protobuf<RawClientState> for ClientState<H> {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeOptions {
	latest_relay_hash: (),
}

impl<H: Clone> ClientState<H> {
	/// Verify that the client is at a sufficient height and unfrozen at the given height
	pub fn verify_height(&self, height: Height) -> Result<(), Error> {
		// let latest_para_height = Height::new(self.para_id.into(),
		// self.latest_para_height.into()); if latest_para_height < height {
		// 	return Err(Error::Custom(format!(
		// 		"Insufficient height, known height: {latest_para_height}, given height: {height}"
		// 	)))
		// }

		match self.frozen_height {
			Some(frozen_height) if frozen_height <= height =>
				Err(Error::Custom(format!("Client has been frozen at height {frozen_height}"))),
			_ => Ok(()),
		}
	}

	pub fn to_any(&self) -> Any {
		Any {
			type_url: ETHEREUM_CLIENT_STATE_TYPE_URL.to_string(),
			value: self.encode_vec().unwrap(),
		}
	}
}

impl<H> ClientState<H> {
	pub fn latest_height(&self) -> Height {
		Height::new(0, self.latest_height.into())
		// Height::new(self.inner.latest_finalized_epoch.into(), self.latest_height.into())
	}

	pub fn chain_id(&self) -> ChainId {
		ChainId::new("ethereum".to_string(), self.inner.latest_finalized_epoch)
	}

	pub fn client_type() -> ClientType {
		"xx-ethereum".to_string()
	}

	pub fn frozen_height(&self) -> Option<Height> {
		self.frozen_height
	}

	pub fn upgrade(
		mut self,
		_upgrade_height: Height,
		upgrade_options: UpgradeOptions,
		_chain_id: ChainId,
	) -> Self {
		self.frozen_height = None;
		// Upgrade the client state
		// self.latest_relay_hash = upgrade_options.latest_relay_hash;

		self
	}

	/// Check if the state is expired when `elapsed` time has passed since the latest consensus
	/// state timestamp
	pub fn expired(&self, elapsed: Duration) -> bool {
		// elapsed > self.relay_chain.trusting_period()
		// TODO
		false
	}

	pub fn with_frozen_height(self, h: Height) -> Result<Self, Error> {
		if h == Height::zero() {
			return Err(Error::Custom(
				"ClientState frozen height must be greater than zero".to_string(),
			))
		}
		Ok(Self { frozen_height: Some(h), ..self })
	}
}

impl<H> ibc::core::ics02_client::client_state::ClientState for ClientState<H>
where
	H: Send + Sync + Clone + Debug + Default + Eq + BlsVerify,
{
	type UpgradeOptions = UpgradeOptions;
	type ClientDef = EthereumClient<H>;

	fn chain_id(&self) -> ChainId {
		self.chain_id()
	}

	fn client_def(&self) -> Self::ClientDef {
		EthereumClient::default()
	}

	fn client_type(&self) -> ClientType {
		Self::client_type()
	}

	fn latest_height(&self) -> Height {
		self.latest_height()
	}

	fn frozen_height(&self) -> Option<Height> {
		self.frozen_height()
	}

	fn upgrade(
		self,
		upgrade_height: Height,
		upgrade_options: UpgradeOptions,
		chain_id: ChainId,
	) -> Self {
		self.upgrade(upgrade_height, upgrade_options, chain_id)
	}

	fn expired(&self, elapsed: Duration) -> bool {
		self.expired(elapsed)
	}

	fn encode_to_vec(&self) -> Result<Vec<u8>, tendermint_proto::Error> {
		self.encode_vec()
	}
}

impl<H> TryFrom<RawClientState> for ClientState<H> {
	type Error = Error;

	fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
		let inner = raw
			.inner
			.ok_or(Error::Custom("missing inner client state".to_string()))?
			.try_into()?;
		let height = raw.frozen_height_revision_height.zip(raw.frozen_height_revision_number);
		Ok(ClientState {
			inner,
			frozen_height: height.map(|x| Height::new(x.0, x.1)),
			latest_height: raw.latest_height,
			_phantom: Default::default(),
		})
	}
}

impl<H> From<ClientState<H>> for RawClientState {
	fn from(client_state: ClientState<H>) -> Self {
		let (frozen_height_revision_height, frozen_height_revision_number) = client_state
			.frozen_height
			.map(|x| (x.revision_number, x.revision_height))
			.unzip();
		RawClientState {
			inner: Some(client_state.inner.into()),
			frozen_height_revision_height,
			frozen_height_revision_number,
			latest_height: client_state.latest_height,
		}
	}
}
