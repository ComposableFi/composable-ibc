use alloc::{
	string::{String, ToString},
	vec::Vec,
};
use core::time::Duration;

use ibc::{
	core::{
		ics02_client::height::Height, ics23_commitment::specs::ProofSpecs,
		ics24_host::identifier::ClientId,
	},
	timestamp::Timestamp,
};
use lib::hash::CryptoHash;
use serde::{Deserialize, Serialize};

use crate::{client_def::GuestClient, error::Error, CLIENT_TYPE};

super::wrap!(cf_guest_upstream::ClientState<PK> as ClientState);
super::wrap!(impl<PK> proto for ClientState);

// impl<PK: guestchain::PubKey> Protobuf<crate::proto::ClientState> for ClientState<PK> {}

impl<PK: guestchain::PubKey> ClientState<PK> {
	pub fn new(
		genesis_hash: CryptoHash,
		latest_height: guestchain::BlockHeight,
		trusting_period_ns: u64,
		epoch_commitment: CryptoHash,
		prev_epoch_commitment: Option<CryptoHash>,
		is_frozen: bool,
	) -> Self {
		Self(cf_guest_upstream::ClientState::new(
			genesis_hash,
			latest_height,
			trusting_period_ns,
			epoch_commitment,
			prev_epoch_commitment,
			is_frozen,
		))
	}

	pub fn with_header(&self, header: &cf_guest_upstream::Header<PK>) -> Self {
		Self(self.0.with_header(&header))
	}

	pub fn frozen(&self) -> Self {
		Self(self.0.frozen())
	}

	/// Verify the time and height delays
	pub fn verify_delay_passed(
		current_time: Timestamp,
		current_height: Height,
		processed_time: u64,
		processed_height: u64,
		delay_period_time: u64,
		delay_period_blocks: u64,
	) -> Result<(), Error> {
		let earliest_time = processed_time + delay_period_time;
		// NOTE: delay time period is inclusive, so if current_time is earliest_time, then we
		// return no error https://github.com/cosmos/ibc-go/blob/9ebc2f81049869bc40c443ffb72d9f3e47afb4fc/modules/light-clients/07-tendermint/client_state.go#L306
		if current_time.nanoseconds() < earliest_time {
			return Err(Error::NotEnoughTimeElapsed { current_time, earliest_time })
		}

		let earliest_height = processed_height + delay_period_blocks;
		if current_height.revision_height < earliest_height {
			return Err(Error::NotEnoughBlocksElapsed { current_height, earliest_height })
		}

		Ok(())
	}

	pub fn verify_height(&self, client_id: &ClientId, height: ibc::Height) -> Result<(), Error> {
		if self.0.latest_height < height.revision_height.into() {
			return Err(Error::InsufficientHeight {
				latest_height: Height::new(1, self.0.latest_height.into()),
				target_height: height,
			})
		}

		if self.0.is_frozen {
			return Err(Error::ClientFrozen { client_id: client_id.clone() })
		}
		Ok(())
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeOptions {}

impl<PK> ibc::core::ics02_client::client_state::ClientState for ClientState<PK>
where
	PK: guestchain::PubKey + Send + Sync,
	PK::Signature: Send + Sync,
{
	type UpgradeOptions = UpgradeOptions;

	type ClientDef = GuestClient<PK>;

	fn chain_id(&self) -> ibc::core::ics24_host::identifier::ChainId {
		ibc::core::ics24_host::identifier::ChainId::new(String::from("Solana"), 0)
	}

	fn client_def(&self) -> Self::ClientDef {
		GuestClient::default()
	}

	fn client_type(&self) -> ibc::core::ics02_client::client_state::ClientType {
		CLIENT_TYPE.to_string()
	}

	fn latest_height(&self) -> ibc::Height {
		Height::new(1, u64::from(self.0.latest_height))
	}

	fn frozen_height(&self) -> Option<ibc::Height> {
		self.0.is_frozen.then(|| Height::new(1, u64::from(self.0.latest_height)))
	}

	fn upgrade(
		mut self,
		upgrade_height: ibc::Height,
		upgrade_options: Self::UpgradeOptions,
		_chain_id: ibc::core::ics24_host::identifier::ChainId,
	) -> Self {
		self.0.latest_height = upgrade_height.revision_height.into();
		self
	}

	fn expired(&self, elapsed: core::time::Duration) -> bool {
		elapsed.as_nanos() as u64 > self.0.trusting_period_ns
	}

	fn encode_to_vec(&self) -> Result<ibc::prelude::Vec<u8>, tendermint_proto::Error> {
		Ok(self.0.encode())
	}
}
