use crate::{
	client_def::GrandpaClient,
	error::Error,
	proto::{Authority as RawAuthority, ClientState as RawClientState},
};
use alloc::{string::ToString, vec::Vec};
use anyhow::anyhow;
use core::{marker::PhantomData, time::Duration};
use ibc::{
	core::{ics02_client::client_state::ClientType, ics24_host::identifier::ChainId},
	Height,
};
use light_client_common::RelayChain;
use primitive_types::H256;
use serde::{Deserialize, Serialize};
use sp_core::ed25519::Public;
use tendermint_proto::Protobuf;

/// Protobuf type url for GRANDPA ClientState
pub const GRANDPA_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.ClientState";

#[derive(Clone, PartialEq, Debug)]
pub struct Authority {
	/// ed25519 public key of the authority
	pub public_key: Public,
	/// authority weight
	pub weight: u64,
}

#[derive(PartialEq, Clone, Debug, Default)]
pub struct ClientState<H> {
	/// Relay chain
	pub relay_chain: RelayChain,
	/// Latest mmr root hash
	pub latest_relay_hash: H256,
	/// Block height when the client was frozen due to a misbehaviour
	pub frozen_height: Option<Height>,
	/// latest parachain height
	pub latest_para_height: u32,
	/// ParaId of associated parachain
	pub para_id: u32,
	pub current_set_id: u32,
	/// authorities for the current round
	pub current_authorities: Vec<Authority>,
	/// phantom type.
	_phantom: PhantomData<H>,
}

impl<H: Clone> Protobuf<RawClientState> for ClientState<H> {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeOptions {
	latest_relay_hash: H256,
}

impl<H> ClientState<H> {
	pub fn latest_height(&self) -> Height {
		Height::new(self.para_id.into(), self.latest_para_height.into())
	}

	pub fn chain_id(&self) -> ChainId {
		ChainId::new(self.relay_chain.to_string(), self.para_id as u64)
	}

	pub fn client_type() -> ClientType {
		"10-grandpa"
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
}

impl<H> ibc::core::ics02_client::client_state::ClientState for ClientState<H>
where
	H: light_client_common::HostFunctions + grandpa_client_primitives::HostFunctions,
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

	fn encode_to_vec(&self) -> Vec<u8> {
		self.encode_vec()
	}
}

impl<H> TryFrom<RawClientState> for ClientState<H> {
	type Error = Error;

	fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
		let frozen_height = {
			let height = Height::new(raw.para_id as u64, raw.frozen_height.into());
			if height == Height::zero() {
				None
			} else {
				Some(height)
			}
		};

		let current_authorities = raw
			.current_authorities
			.into_iter()
			.map(|set| {
				Ok(Authority {
					public_key: Public::try_from(&*set.public_key)
						.map_err(|_| anyhow!("Invalid ed25519 public key"))?,
					weight: set.weight,
				})
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
			frozen_height,
			relay_chain,
			latest_para_height: raw.latest_para_height,
			para_id: raw.para_id,
			current_set_id: raw.current_set_id,
			current_authorities,
			latest_relay_hash,
			_phantom: Default::default(),
		})
	}
}

impl<H> From<ClientState<H>> for RawClientState {
	fn from(client_state: ClientState<H>) -> Self {
		RawClientState {
			latest_relay_hash: client_state.latest_relay_hash.as_bytes().to_vec(),
			current_set_id: client_state.current_set_id,
			frozen_height: client_state.frozen_height.unwrap_or_default().revision_height,
			relay_chain: client_state.relay_chain as i32,
			para_id: client_state.para_id,
			latest_para_height: client_state.latest_para_height,
			current_authorities: client_state
				.current_authorities
				.into_iter()
				.map(|authority| RawAuthority {
					public_key: <Public as AsRef<[u8]>>::as_ref(&authority.public_key).to_vec(),
					weight: authority.weight,
				})
				.collect(),
		}
	}
}
