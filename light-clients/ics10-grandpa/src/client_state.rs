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

#![allow(deprecated)]

use crate::{
	client_def::GrandpaClient,
	client_message::RelayChainHeader,
	error::Error,
	proto::{
		Authority as RawAuthority, AuthorityChange as RawAuthorityChange,
		ClientState as RawClientState,
	},
};
use alloc::{format, string::ToString, vec::Vec};
use anyhow::anyhow;
use core::{convert::identity, fmt::Debug, marker::PhantomData, time::Duration};
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState,
			client_state::{ClientType, Status},
		},
		ics24_host::identifier::{ChainId, ClientId},
		ics26_routing::context::ReaderContext,
	},
	timestamp::Timestamp,
	Height,
};
use ibc_proto::google::protobuf::Any;
use light_client_common::RelayChain;
use serde::{Deserialize, Serialize};
use sp_core::{ed25519::Public, H256};
use sp_finality_grandpa::{AuthorityList, SetId};
use tendermint::Time;
use tendermint_proto::Protobuf;
use vec1::Vec1;

/// Protobuf type url for GRANDPA ClientState
pub const GRANDPA_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.ClientState";

/// Maximum time to keep a block number to block hash mapping (if number of items is greater than
/// HEADER_ITEM_MIN_COUNT).
pub const HEADER_ITEM_LIFETIME: Duration = Duration::from_secs(1 * 60 * 60);
/// Minimum (if possible) number of block number to block hash mappings to keep (oldest pruned
/// first).
pub const HEADER_ITEM_MIN_COUNT: usize = 500;
pub const SESSION_LENGTH: Duration = Duration::from_secs(6 * 60 * 60);
/// Maximum time to keep a an authority set change (if number of items is greater than
/// AUTHORITIES_CHANGE_ITEM_MIN_COUNT).
pub const AUTHORITIES_CHANGE_ITEM_LIFETIME: Duration =
	Duration::from_secs(3 * SESSION_LENGTH.as_secs());
/// Minimum (if possible) number of authority set changes to keep (oldest pruned first).
pub const AUTHORITIES_CHANGE_ITEM_MIN_COUNT: usize = 10;

#[derive(PartialEq, Clone, Debug, Default, Eq)]
pub struct AuthoritiesChange {
	/// Block height at which the authorities set starts to be active.
	pub height: u32,
	/// When the change was scheduled.
	pub timestamp: Timestamp,
	/// Id of the new authority set.
	pub set_id: SetId,
	/// New authorities.
	pub authorities: AuthorityList,
}

impl AuthoritiesChange {
	pub fn new(
		height: u32,
		timestamp: Timestamp,
		set_id: SetId,
		authorities: AuthorityList,
	) -> Self {
		Self { height, timestamp, set_id, authorities }
	}
}

#[derive(PartialEq, Clone, Debug, Default, Eq)]
pub struct ClientStateV1<H> {
	/// Relay chain
	pub relay_chain: RelayChain,
	// Latest relay chain height
	pub latest_relay_height: u32,
	/// Latest relay chain block hash
	pub latest_relay_hash: H256,
	/// Block height when the client was frozen due to a misbehaviour
	pub frozen_height: Option<Height>,
	/// latest parachain height
	pub latest_para_height: u32,
	/// ParaId of associated parachain
	pub para_id: u32,
	/// Id of the current authority set.
	pub current_set_id: u64,
	/// Authorities for the current round.
	pub current_authorities: AuthorityList,
	pub _phantom: PhantomData<H>,
}

#[derive(PartialEq, Clone, Debug, Default, Eq)]
pub struct ClientStateV2<H> {
	/// Relay chain
	pub relay_chain: RelayChain,
	// Latest relay chain height
	pub latest_relay_height: u32,
	/// Latest relay chain block hash
	pub latest_relay_hash: H256,
	/// Block height when the client was frozen due to a misbehaviour
	pub frozen_height: Option<Height>,
	/// latest parachain height
	pub latest_para_height: u32,
	/// ParaId of associated parachain
	pub para_id: u32,
	// Id of the current authority set.
	// pub current_set_id: u64,
	/// Authorities changes history. More than one authorities change is needed to
	/// properly verify misbehaviour.
	pub authorities_changes: Vec1<AuthoritiesChange>,
	/// phantom type.
	pub _phantom: PhantomData<H>,
}

pub type ClientState<H> = ClientStateV2<H>;

impl<H> From<ClientStateV1<H>> for ClientStateV2<H> {
	fn from(value: ClientStateV1<H>) -> Self {
		ClientStateV2 {
			relay_chain: value.relay_chain,
			latest_relay_height: value.latest_relay_height,
			latest_relay_hash: value.latest_relay_hash,
			frozen_height: value.frozen_height,
			latest_para_height: value.latest_para_height,
			para_id: value.para_id,
			// current_set_id: value.current_set_id,
			authorities_changes: Vec1::new(AuthoritiesChange {
				height: 0,
				timestamp: Timestamp::from_nanoseconds(1).unwrap(),
				set_id: value.current_set_id,
				authorities: value.current_authorities,
			}),
			_phantom: Default::default(),
		}
	}
}

impl<H: Clone> From<ClientState<H>> for grandpa_client_primitives::ClientState {
	fn from(client_state: ClientState<H>) -> grandpa_client_primitives::ClientState {
		grandpa_client_primitives::ClientState {
			current_authorities: client_state.current_authorities().clone(),
			current_set_id: *client_state.current_set_id(),
			latest_relay_hash: client_state.latest_relay_hash,
			latest_relay_height: client_state.latest_relay_height,
			latest_para_height: client_state.latest_para_height,
			para_id: client_state.para_id,
		}
	}
}

impl<H: Clone> Protobuf<RawClientState> for ClientState<H> {}

impl<H: Clone> Protobuf<RawClientState> for ClientStateV1<H> {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeOptions {
	latest_relay_hash: H256,
}

impl<H: Clone> ClientState<H> {
	/// Verify that the client is at a sufficient height and unfrozen at the given height
	pub fn verify_height(&self, height: Height) -> Result<(), Error> {
		let latest_para_height = Height::new(self.para_id.into(), self.latest_para_height.into());
		if latest_para_height < height {
			return Err(Error::Custom(format!(
				"Insufficient height, known height: {latest_para_height}, given height: {height}"
			)))
		}

		match self.frozen_height {
			Some(frozen_height) if frozen_height <= height =>
				Err(Error::Custom(format!("Client has been frozen at height {frozen_height}"))),
			_ => Ok(()),
		}
	}

	pub fn to_any(&self) -> Any {
		Any {
			type_url: GRANDPA_CLIENT_STATE_TYPE_URL.to_string(),
			value: self.encode_vec().unwrap(),
		}
	}

	pub fn current_authorities(&self) -> &AuthorityList {
		let key = self
			.authorities_changes
			.binary_search_by_key(&self.latest_relay_height, |x| x.height)
			.unwrap_or_else(identity);
		match self.authorities_changes.get(key) {
			Some(change) => &change.authorities,
			_ => &self.authorities_changes.last().authorities,
		}
	}

	pub fn current_set_id(&self) -> &SetId {
		let key = self
			.authorities_changes
			.binary_search_by_key(&self.latest_relay_height, |x| x.height)
			.unwrap_or_else(identity);
		match self.authorities_changes.get(key) {
			Some(change) => &change.set_id,
			_ => &self.authorities_changes.last().set_id,
		}
	}

	pub fn last_set_id(&self) -> &SetId {
		&self.authorities_changes.last().set_id
	}
}

impl<H> ClientState<H> {
	pub fn latest_height(&self) -> Height {
		Height::new(self.para_id.into(), self.latest_para_height.into())
	}

	pub fn chain_id(&self) -> ChainId {
		ChainId::new(self.relay_chain.to_string(), self.para_id as u64)
	}

	pub fn client_type() -> ClientType {
		"10-grandpa".to_string()
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
		self.latest_relay_hash = upgrade_options.latest_relay_hash;

		self
	}

	/// Check if the state is expired when `elapsed` time has passed since the latest consensus
	/// state timestamp
	pub fn expired(&self, elapsed: Duration) -> bool {
		elapsed > self.relay_chain.trusting_period()
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
	H: grandpa_client_primitives::HostFunctions<Header = RelayChainHeader>,
{
	type UpgradeOptions = UpgradeOptions;
	type ClientDef = GrandpaClient<H>;

	fn chain_id(&self) -> ChainId {
		self.chain_id()
	}

	fn client_def(&self) -> Self::ClientDef {
		GrandpaClient::default()
	}

	fn client_type(&self) -> ClientType {
		Self::client_type()
	}

	fn latest_height(&self) -> Height {
		self.latest_height()
	}

	fn status<Ctx: ReaderContext>(&self, ctx: &Ctx, client_id: &ClientId) -> Status {
		if self.frozen_height.is_some() {
			return Status::Frozen
		}

		// get latest consensus state from clientStore to check for expiry
		let consensus_state = match ctx.consensus_state(client_id, self.latest_height()) {
			Ok(consensus_state) => consensus_state,
			Err(_) => {
				// if the client state does not have an associated consensus state for its latest
				// height then it must be expired
				return Status::Expired
			},
		};

		let elapsed = ctx
			.host_timestamp()
			.duration_since(&consensus_state.timestamp())
			.unwrap_or_else(|| Duration::from_secs(0));

		if self.expired(elapsed) {
			return Status::Expired
		}

		Status::Active
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
		let relay_chain = RelayChain::from_i32(raw.relay_chain)?;
		if raw.latest_relay_hash.len() != 32 {
			Err(anyhow!("Invalid ed25519 public key lenght: {}", raw.latest_relay_hash.len()))?
		}
		let mut fixed_bytes = [0u8; 32];
		fixed_bytes.copy_from_slice(&*raw.latest_relay_hash);
		let latest_relay_hash = H256::from(fixed_bytes);

		Ok(Self {
			frozen_height: raw.frozen_height.map(|height| Height::new(raw.para_id.into(), height)),
			relay_chain,
			latest_para_height: raw.latest_para_height,
			para_id: raw.para_id,
			authorities_changes: Vec1::try_from(
				raw.authorities_changes
					.into_iter()
					.map(TryInto::try_into)
					.collect::<Result<Vec<_>, _>>()?,
			)
			.map_err(|e| Error::Custom(e.to_string()))?,
			latest_relay_hash,
			latest_relay_height: raw.latest_relay_height,
			_phantom: Default::default(),
		})
	}
}

impl<H> TryFrom<RawClientState> for ClientStateV1<H> {
	type Error = Error;

	fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
		let current_authorities = raw
			.current_authorities
			.into_iter()
			.map(|set| {
				let id = Public::try_from(&*set.public_key)
					.map_err(|_| anyhow!("Invalid ed25519 public key"))?;
				Ok((id.into(), set.weight))
			})
			.collect::<Result<_, Error>>()?;

		let relay_chain = RelayChain::from_i32(raw.relay_chain)?;
		if raw.latest_relay_hash.len() != 32 {
			Err(anyhow!("Invalid ed25519 public key lenght: {}", raw.latest_relay_hash.len()))?
		}
		let mut fixed_bytes = [0u8; 32];
		fixed_bytes.copy_from_slice(&*raw.latest_relay_hash);
		let latest_relay_hash = H256::from(fixed_bytes);

		Ok(Self {
			frozen_height: raw.frozen_height.map(|height| Height::new(raw.para_id.into(), height)),
			relay_chain,
			latest_para_height: raw.latest_para_height,
			para_id: raw.para_id,
			current_set_id: raw.current_set_id,
			current_authorities,
			latest_relay_hash,
			latest_relay_height: raw.latest_relay_height,
			_phantom: Default::default(),
		})
	}
}

impl<H> From<ClientState<H>> for RawClientState {
	fn from(client_state: ClientState<H>) -> Self {
		RawClientState {
			latest_relay_height: client_state.latest_relay_height,
			latest_relay_hash: client_state.latest_relay_hash.as_bytes().to_vec(),
			current_set_id: client_state.authorities_changes.last().set_id,
			frozen_height: client_state
				.frozen_height
				.map(|frozen_height| frozen_height.revision_height),
			relay_chain: client_state.relay_chain as i32,
			para_id: client_state.para_id,
			latest_para_height: client_state.latest_para_height,
			current_authorities: Vec::new(),
			authorities_changes: client_state
				.authorities_changes
				.into_iter()
				.map(Into::into)
				.collect(),
		}
	}
}

impl<H> From<ClientStateV1<H>> for RawClientState {
	fn from(client_state: ClientStateV1<H>) -> Self {
		RawClientState {
			latest_relay_height: client_state.latest_relay_height,
			latest_relay_hash: client_state.latest_relay_hash.as_bytes().to_vec(),
			current_set_id: client_state.current_set_id,
			frozen_height: client_state
				.frozen_height
				.map(|frozen_height| frozen_height.revision_height),
			relay_chain: client_state.relay_chain as i32,
			para_id: client_state.para_id,
			latest_para_height: client_state.latest_para_height,
			current_authorities: client_state
				.current_authorities
				.into_iter()
				.map(|(id, weight)| RawAuthority {
					public_key: <sp_finality_grandpa::AuthorityId as AsRef<[u8]>>::as_ref(&id)
						.to_vec(),
					weight,
				})
				.collect(),
			authorities_changes: Vec::new(),
		}
	}
}

impl From<AuthoritiesChange> for RawAuthorityChange {
	fn from(change: AuthoritiesChange) -> Self {
		let authorities = change
			.authorities
			.into_iter()
			.map(|(id, weight)| RawAuthority {
				public_key: <sp_finality_grandpa::AuthorityId as AsRef<[u8]>>::as_ref(&id).to_vec(),
				weight,
			})
			.collect();
		let tendermint_proto::google::protobuf::Timestamp { seconds, nanos } =
			change.timestamp.into_tm_time().expect("shouldn't fail").into();
		let timestamp = prost_types::Timestamp { seconds, nanos };
		Self {
			timestamp: Some(timestamp),
			height: change.height,
			set_id: change.set_id,
			authorities,
		}
	}
}

impl TryFrom<RawAuthorityChange> for AuthoritiesChange {
	type Error = Error;

	fn try_from(raw: RawAuthorityChange) -> Result<Self, Self::Error> {
		let authorities = raw
			.authorities
			.into_iter()
			.map(|authority| {
				let id = Public::try_from(&*authority.public_key)
					.map_err(|_| anyhow!("Invalid ed25519 public key"))?;
				Ok((id.into(), authority.weight))
			})
			.collect::<Result<_, Error>>()?;
		let prost_types::Timestamp { seconds, nanos } =
			raw.timestamp.ok_or_else(|| Error::Custom(format!("missing timestamp")))?;
		let proto_timestamp = tendermint_proto::google::protobuf::Timestamp { seconds, nanos };
		let timestamp: Time = proto_timestamp
			.try_into()
			.map_err(|e| Error::Custom(format!("invalid timestamp {e}")))?;

		Ok(AuthoritiesChange {
			height: raw.height,
			timestamp: Timestamp::from(timestamp),
			set_id: raw.set_id,
			authorities,
		})
	}
}
