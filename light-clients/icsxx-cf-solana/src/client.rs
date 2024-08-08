use crate::{client_def::CfSolanaClient, error::Error, proto, Header, CLIENT_TYPE};
use alloc::string::{String, ToString};
use ibc::{
	core::{ics02_client::height::Height, ics24_host::identifier::ClientId},
	timestamp::Timestamp,
};
use proto_utils::BadMessage;
use serde::{Deserialize, Serialize};
use solana_sdk::{clock::Slot, pubkey::Pubkey};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientState {
	/// Highest available guest block height.
	pub latest_height: Slot,

	pub trusting_period_ns: u64,

	/// Whether client is frozen.
	pub is_frozen: bool,

	/// Current validator's Public Key
	pub current_leader: Pubkey,

	/// Genesis timestamp
	pub genesis_time: Timestamp,

	/// Chain's slot duration.
	pub slot_duration: Duration,
}

// super::wrap!(cf_guest_upstream::ClientState as ClientState);
// super::wrap!(impl proto for ClientState);

impl ClientState {
	pub(crate) fn timestamp_for_slot(&self, slot: Slot) -> Timestamp {
		Timestamp::from_nanoseconds(
			self.genesis_time.nanoseconds() + (slot as u64 * self.slot_duration.as_nanos() as u64),
		)
		.unwrap()
	}
}

impl ClientState {
	pub fn new(
		latest_height: Slot,
		trusting_period_ns: u64,
		is_frozen: bool,
		current_validator: Pubkey,
		genesis_time: Timestamp,
		slot_duration: Duration,
	) -> Self {
		Self {
			latest_height,
			trusting_period_ns,
			is_frozen,
			current_leader: current_validator,
			genesis_time,
			slot_duration,
		}
	}

	pub fn update_unchecked(self, header: Header) -> Self {
		Self { latest_height: header.slot(), ..self }
	}

	pub fn into_frozen(self) -> Self {
		Self { is_frozen: true, ..self }
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
		if self.latest_height < height.revision_height {
			return Err(Error::InsufficientHeight {
				latest_height: Height::new(1, self.latest_height.into()),
				target_height: height,
			})
		}

		if self.is_frozen {
			return Err(Error::ClientFrozen { client_id: client_id.clone() })
		}
		Ok(())
	}

	pub(crate) fn leader_for_slot(&self, _slot: Slot) -> Pubkey {
		// TODO: implement the actual mapping from slot to leader (see
		// `crate::solana::leader_schedule`)
		self.current_leader.clone()
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeOptions {}

impl ibc::core::ics02_client::client_state::ClientState for ClientState {
	type UpgradeOptions = UpgradeOptions;

	type ClientDef = CfSolanaClient;

	fn chain_id(&self) -> ibc::core::ics24_host::identifier::ChainId {
		ibc::core::ics24_host::identifier::ChainId::new(String::from("Solana"), 0)
	}

	fn client_def(&self) -> Self::ClientDef {
		CfSolanaClient::default()
	}

	fn client_type(&self) -> ibc::core::ics02_client::client_state::ClientType {
		CLIENT_TYPE.to_string()
	}

	fn latest_height(&self) -> ibc::Height {
		Height::new(1, u64::from(self.latest_height))
	}

	fn frozen_height(&self) -> Option<ibc::Height> {
		self.is_frozen.then(|| Height::new(1, u64::from(self.latest_height)))
	}

	fn upgrade(
		mut self,
		upgrade_height: ibc::Height,
		_upgrade_options: Self::UpgradeOptions,
		_chain_id: ibc::core::ics24_host::identifier::ChainId,
	) -> Self {
		self.latest_height = upgrade_height.revision_height.into();
		self
	}

	fn expired(&self, elapsed: core::time::Duration) -> bool {
		elapsed.as_nanos() as u64 > self.trusting_period_ns
	}

	fn encode_to_vec(&self) -> Result<ibc::prelude::Vec<u8>, ibc::protobuf::Error> {
		Ok(self.encode())
	}
}

impl From<ClientState> for proto::ClientState {
	fn from(state: ClientState) -> Self {
		Self::from(&state)
	}
}

impl From<&ClientState> for proto::ClientState {
	fn from(state: &ClientState) -> Self {
		Self {
			latest_height: state.latest_height.into(),
			trusting_period_ns: state.trusting_period_ns,
			is_frozen: state.is_frozen,
			current_leader: state.current_leader.to_bytes().to_vec(),
			genesis_time: state.genesis_time.nanoseconds(),
			slot_duration: state.slot_duration.as_nanos() as u64,
		}
	}
}

impl TryFrom<proto::ClientState> for ClientState {
	type Error = BadMessage;
	fn try_from(msg: proto::ClientState) -> Result<Self, Self::Error> {
		Self::try_from(&msg)
	}
}

impl TryFrom<&proto::ClientState> for ClientState {
	type Error = BadMessage;

	fn try_from(msg: &proto::ClientState) -> Result<Self, Self::Error> {
		let current_leader_bytes: &[u8] = msg.current_leader.as_ref();
		let current_leader = Pubkey::try_from(current_leader_bytes).map_err(|_| BadMessage)?;
		let genesis_time = Timestamp::from_nanoseconds(msg.genesis_time).map_err(|_| BadMessage)?;
		let slot_duration = Duration::from_nanos(msg.slot_duration);

		Ok(Self {
			latest_height: msg.latest_height.into(),
			trusting_period_ns: msg.trusting_period_ns,
			is_frozen: msg.is_frozen,
			current_leader,
			genesis_time,
			slot_duration,
		})
	}
}

proto_utils::define_wrapper! {
	proto: crate::proto::ClientState,
	wrapper: ClientState,
}

#[cfg(test)]
mod tests {
	use super::*;

	use guestchain::validators::MockPubKey;

	const ANY_MESSAGE: [u8; 116] = [
		10, 34, 47, 108, 105, 103, 104, 116, 99, 108, 105, 101, 110, 116, 115, 46, 103, 117, 101,
		115, 116, 46, 118, 49, 46, 67, 108, 105, 101, 110, 116, 83, 116, 97, 116, 101, 18, 78, 10,
		32, 243, 148, 241, 41, 122, 49, 51, 253, 97, 145, 113, 22, 234, 164, 193, 183, 185, 48,
		160, 186, 69, 72, 144, 156, 126, 229, 103, 131, 220, 174, 140, 165, 16, 5, 24, 128, 128,
		144, 202, 210, 198, 14, 34, 32, 86, 12, 131, 131, 127, 125, 82, 54, 32, 207, 121, 149, 204,
		11, 121, 102, 180, 211, 111, 54, 0, 207, 247, 125, 195, 57, 10, 10, 80, 84, 86, 152,
	];

	fn message() -> &'static [u8] {
		&ANY_MESSAGE[38..]
	}

	fn check(state: ClientState) {
		let want = ClientState::new(5.into(), 64000000000000, false);
		assert_eq!(want, state);
	}

	#[test]
	fn test_decode_vec() {
		check(ibc::protobuf::Protobuf::decode_vec(message()).unwrap());
	}

	#[test]
	fn test_from_any() {
		use ibc_proto::google::protobuf::Any;

		let any: Any = prost::Message::decode(ANY_MESSAGE.as_ref()).unwrap();
		check(any.try_into().unwrap());
	}
}
