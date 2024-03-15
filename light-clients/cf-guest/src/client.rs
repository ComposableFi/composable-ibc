use core::time::Duration;

use crate::error::Error;
use alloc::{string::{String, ToString}, vec::Vec};
use ibc::{
	core::{
		ics02_client::{height::Height}, ics23_commitment::specs::ProofSpecs, ics24_host::identifier::ClientId
	},
	timestamp::Timestamp,
};
use lib::hash::CryptoHash;
use serde::{Deserialize, Serialize};

use crate::{client_def::GuestClient, proto, CLIENT_TYPE};

/// The client state of the light client for the guest blockchain as a Rust
/// object.
///
/// `From` and `TryFrom` conversions define mapping between this Rust object and
/// corresponding Protocol Message [`proto::ClientState`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientState<PK> {
	/// Hash of the chain’s genesis block
	///
	/// It serves as chain id allowing discarding blocks which are meant for
	/// different blockchains.
	pub genesis_hash: CryptoHash,

	/// Highest available guest block height.
	pub latest_height: guestchain::BlockHeight,

	pub trusting_period_ns: u64,

	/// Commitment of the epoch used to verify future states.
	pub epoch_commitment: CryptoHash,

	/// Whether client is frozen.
	pub is_frozen: bool,

	_ph: core::marker::PhantomData<PK>,
}

// impl<PK: guestchain::PubKey> Protobuf<crate::proto::ClientState> for ClientState<PK> {}

impl<PK: guestchain::PubKey> ClientState<PK> {
	pub fn new(
		genesis_hash: CryptoHash,
		latest_height: guestchain::BlockHeight,
		trusting_period_ns: u64,
		epoch_commitment: CryptoHash,
		is_frozen: bool,
	) -> Self {
		Self {
			genesis_hash,
			latest_height,
			trusting_period_ns,
			epoch_commitment,
			is_frozen,
			_ph: core::marker::PhantomData::<PK>,
		}
	}
	pub fn with_header(&self, header: &super::Header<PK>) -> Self {
		let mut this = self.clone();
		if header.block_header.block_height > this.latest_height {
			this.latest_height = header.block_header.block_height;
			// If the block is the last last block of the epoch its header
			// carries next epoch’s commitment.  If the header doesn’t define
			// next epoch’s commitment than it’s not the last block of the epoch
			// and this.epoch_commitment is still the commitment we need to use.
			//
			// The commitment is the hash of borsh-serialised Epoch so it allows
			// us to verify whether Epoch someone sends us is the current one.
			//
			// Updating epoch_commitment means that we will only accept headers
			// belonging to the new epoch.
			//
			// TODO(mina86): Perhaps we should provide a way to allow headers
			// from past epoch to be accepted as well?  At the moment, if we’re
			// in the middle of an epoch and someone sends header for block
			// N someone else can follow up with header for block N-1.  However,
			// If N is the last block of the epoch, submitting block N-1 will
			// fail.  It would succeed if it was done prior to block N.  This
			// does affect proofs since if someone built a proof against block
			// N-1 then they can no longer use it.  Of course proofs can be
			// recalculated with newer blocks so whether this really is an issue
			// is not clear to me.
			this.epoch_commitment = header
				.block_header
				.next_epoch_commitment
				.as_ref()
				.unwrap_or(&self.epoch_commitment)
				.clone();
		}
		this
	}

	pub fn frozen(&self) -> Self {
		Self { is_frozen: true, ..self.clone() }
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
		if self.latest_height < height.revision_height.into() {
			return Err(Error::InsufficientHeight {
				latest_height: Height::new(1, self.latest_height.into()),
				target_height: height,
			})
		}

		if self.is_frozen {
			return Err(Error::ClientFrozen{ client_id: client_id.clone() })
		}
		Ok(())
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeOptions {
	pub unbonding_period: Duration,
	pub proof_specs: ProofSpecs,
	pub upgrade_path: Vec<String>,
}

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
		Height::new(1, u64::from(self.latest_height))
	}

	fn frozen_height(&self) -> Option<ibc::Height> {
		match self.is_frozen {
			true => Some(Height::new(1, u64::from(self.latest_height))),
			false => None,
		}
	}

	fn upgrade(
		self,
		_upgrade_height: ibc::Height,
		_upgrade_options: Self::UpgradeOptions,
		_chain_id: ibc::core::ics24_host::identifier::ChainId,
	) -> Self {
		todo!()
	}

	fn expired(&self, elapsed: core::time::Duration) -> bool {
		elapsed.as_nanos() as u64 > self.trusting_period_ns
	}

	fn encode_to_vec(&self) -> Result<ibc::prelude::Vec<u8>, tendermint_proto::Error> {
		todo!()
	}
}

impl<PK: guestchain::PubKey> From<ClientState<PK>> for proto::ClientState {
	fn from(state: ClientState<PK>) -> Self {
		Self::from(&state)
	}
}

impl<PK: guestchain::PubKey> From<&ClientState<PK>> for proto::ClientState {
	fn from(state: &ClientState<PK>) -> Self {
		Self {
			genesis_hash: state.genesis_hash.to_vec(),
			latest_height: state.latest_height.into(),
			trusting_period_ns: state.trusting_period_ns,
			epoch_commitment: state.epoch_commitment.to_vec(),
			is_frozen: state.is_frozen,
		}
	}
}

impl<PK: guestchain::PubKey> TryFrom<proto::ClientState> for ClientState<PK> {
	type Error = proto::BadMessage;
	fn try_from(msg: proto::ClientState) -> Result<Self, Self::Error> {
		Self::try_from(&msg)
	}
}

impl<PK: guestchain::PubKey> TryFrom<&proto::ClientState> for ClientState<PK> {
	type Error = proto::BadMessage;
	fn try_from(msg: &proto::ClientState) -> Result<Self, Self::Error> {
		let genesis_hash =
			CryptoHash::try_from(msg.genesis_hash.as_slice()).map_err(|_| proto::BadMessage)?;
		let epoch_commitment =
			CryptoHash::try_from(msg.epoch_commitment.as_slice()).map_err(|_| proto::BadMessage)?;
		Ok(Self {
			genesis_hash,
			latest_height: msg.latest_height.into(),
			trusting_period_ns: msg.trusting_period_ns,
			epoch_commitment,
			is_frozen: msg.is_frozen,
			_ph: core::marker::PhantomData,
		})
	}
}

super::any_convert! {
	proto::ClientState,
	ClientState<PK: guestchain::PubKey = guestchain::validators::MockPubKey>,
	obj: ClientState {
		genesis_hash: CryptoHash::test(24),
		latest_height: 8.into(),
		trusting_period_ns: 30 * 24 * 3600 * 1_000_000_000,
		epoch_commitment: CryptoHash::test(11),
		is_frozen: false,
		_ph: core::marker::PhantomData,
	},
	bad: proto::ClientState {
		genesis_hash: [0; 30].to_vec(),
		latest_height: 8,
		epoch_commitment: [0; 30].to_vec(),
		is_frozen: false,
		trusting_period_ns: 30 * 24 * 3600 * 1_000_000_000,
	},
}
