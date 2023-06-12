use alloc::{borrow::ToOwned, boxed::Box, format, string::ToString, vec::Vec};
use core::convert::identity;
use frame_support::{
	pallet_prelude::{Blake2_256, StorageMap, StorageValue, ValueQuery},
	traits::StorageInstance,
};
use ibc::{
	core::{
		ics02_client,
		ics02_client::{
			client_consensus::ConsensusState, client_message::ClientMessage,
			client_state::ClientState,
		},
	},
	Height,
};
use ibc_derive::{ClientDef, ClientMessage, ClientState, ConsensusState, Protobuf};
use ibc_primitives::runtime_interface;
use ibc_proto::google::protobuf::Any;
use ics07_tendermint::{
	client_message::{
		TENDERMINT_CLIENT_MESSAGE_TYPE_URL, TENDERMINT_HEADER_TYPE_URL,
		TENDERMINT_MISBEHAVIOUR_TYPE_URL,
	},
	client_state::TENDERMINT_CLIENT_STATE_TYPE_URL,
	consensus_state::TENDERMINT_CONSENSUS_STATE_TYPE_URL,
};
use ics08_wasm::{
	client_message::{
		WASM_CLIENT_MESSAGE_TYPE_URL, WASM_HEADER_TYPE_URL, WASM_MISBEHAVIOUR_TYPE_URL,
	},
	client_state::WASM_CLIENT_STATE_TYPE_URL,
	consensus_state::WASM_CONSENSUS_STATE_TYPE_URL,
	Bytes,
};
use ics10_grandpa::{
	client_message::{
		RelayChainHeader, GRANDPA_CLIENT_MESSAGE_TYPE_URL, GRANDPA_HEADER_TYPE_URL,
		GRANDPA_MISBEHAVIOUR_TYPE_URL,
	},
	client_state::{GRANDPA_CLIENT_STATE_TYPE_URL, HEADER_ITEM_LIFETIME, HEADER_ITEM_MIN_COUNT},
	consensus_state::GRANDPA_CONSENSUS_STATE_TYPE_URL,
};
use ics11_beefy::{
	client_message::BEEFY_CLIENT_MESSAGE_TYPE_URL, client_state::BEEFY_CLIENT_STATE_TYPE_URL,
	consensus_state::BEEFY_CONSENSUS_STATE_TYPE_URL,
};
use prost::Message;
use sp_core::{crypto::ByteArray, ed25519, H256};
use sp_runtime::{
	app_crypto::RuntimePublic,
	traits::{BlakeTwo256, Header},
};
use tendermint::{
	crypto::{
		signature::{Error as TendermintCryptoError, Verifier},
		Sha256 as TendermintSha256,
	},
	merkle::{Hash, MerkleHash, NonIncremental, HASH_SIZE},
	PublicKey, Signature,
};
use tendermint_proto::Protobuf;

#[derive(Clone, Default, PartialEq, Debug, Eq)]
pub struct HostFunctionsManager;

impl ics23::HostFunctionsProvider for HostFunctionsManager {
	fn sha2_256(message: &[u8]) -> [u8; 32] {
		sp_io::hashing::sha2_256(message)
	}

	fn sha2_512(message: &[u8]) -> [u8; 64] {
		runtime_interface::sha2_512(message)
	}

	fn sha2_512_truncated(message: &[u8]) -> [u8; 32] {
		runtime_interface::sha2_512_truncated(message)
	}

	fn sha3_512(message: &[u8]) -> [u8; 64] {
		runtime_interface::sha3_512(message)
	}

	fn ripemd160(message: &[u8]) -> [u8; 20] {
		runtime_interface::ripemd160(message)
	}
}

impl TendermintSha256 for HostFunctionsManager {
	fn digest(data: impl AsRef<[u8]>) -> [u8; HASH_SIZE] {
		sp_io::hashing::sha2_256(data.as_ref())
	}
}

impl MerkleHash for HostFunctionsManager {
	fn empty_hash(&mut self) -> Hash {
		NonIncremental::<Self>::default().empty_hash()
	}

	fn leaf_hash(&mut self, bytes: &[u8]) -> Hash {
		NonIncremental::<Self>::default().leaf_hash(bytes)
	}

	fn inner_hash(&mut self, left: Hash, right: Hash) -> Hash {
		NonIncremental::<Self>::default().inner_hash(left, right)
	}
}

impl Verifier for HostFunctionsManager {
	fn verify(
		pubkey: PublicKey,
		msg: &[u8],
		signature: &Signature,
	) -> Result<(), TendermintCryptoError> {
		let signature = sp_core::ed25519::Signature::from_slice(signature.as_bytes())
			.ok_or_else(|| TendermintCryptoError::MalformedSignature)?;
		let public_key = sp_core::ed25519::Public::from_slice(pubkey.to_bytes().as_slice())
			.map_err(|_| TendermintCryptoError::MalformedPublicKey)?;
		sp_io::crypto::ed25519_verify(&signature, msg, &public_key)
			.then(|| ())
			.ok_or_else(|| TendermintCryptoError::VerificationFailed)
	}
}

impl ics07_tendermint::HostFunctionsProvider for HostFunctionsManager {}

pub struct GrandpaHeaderHashesStorageInstance;
impl StorageInstance for GrandpaHeaderHashesStorageInstance {
	fn pallet_prefix() -> &'static str {
		"ibc.lightclients.grandpa"
	}

	const STORAGE_PREFIX: &'static str = "HeaderHashesV2";
}
pub type GrandpaHeaderHashesStorage =
	StorageValue<GrandpaHeaderHashesStorageInstance, Vec<(u64, H256)>, ValueQuery>;

pub struct GrandpaHeaderHashesSetStorageInstance;
impl StorageInstance for GrandpaHeaderHashesSetStorageInstance {
	fn pallet_prefix() -> &'static str {
		"ibc.lightclients.grandpa"
	}

	const STORAGE_PREFIX: &'static str = "HeaderHashesSetV2";
}
pub type GrandpaHeaderHashesSetStorage =
	StorageMap<GrandpaHeaderHashesSetStorageInstance, Blake2_256, H256, (), ValueQuery>;

impl grandpa_client_primitives::HostFunctions for HostFunctionsManager {
	type Header = RelayChainHeader;

	fn ed25519_verify(sig: &ed25519::Signature, msg: &[u8], pub_key: &ed25519::Public) -> bool {
		pub_key.verify(&msg, sig)
	}

	fn insert_relay_header_hashes(now_ms: u64, new_hashes: &[<Self::Header as Header>::Hash]) {
		if new_hashes.is_empty() {
			return
		}

		GrandpaHeaderHashesStorage::mutate(|hashes| {
			hashes.reserve(new_hashes.len());
			for hash in new_hashes {
				hashes.push((now_ms, *hash));
				GrandpaHeaderHashesSetStorage::insert(*hash, ());
			}
			let expired = now_ms.saturating_sub(HEADER_ITEM_LIFETIME.as_millis() as u64);
			let key = hashes.binary_search_by_key(&expired, |(t, _)| *t).unwrap_or_else(identity);
			let len = hashes.len();
			// keep at least HEADER_ITEM_MIN_COUNT items if possible
			let remove_up_to_index = key.min(len.saturating_sub(HEADER_ITEM_MIN_COUNT));
			if remove_up_to_index < len {
				for hash in hashes.drain(..remove_up_to_index).map(|(_, hash)| hash) {
					GrandpaHeaderHashesSetStorage::remove(&hash);
				}
			}
		});
	}

	fn contains_relay_header_hash(hash: <Self::Header as Header>::Hash) -> bool {
		GrandpaHeaderHashesSetStorage::contains_key(&hash)
	}
}

impl light_client_common::HostFunctions for HostFunctionsManager {
	type BlakeTwo256 = BlakeTwo256;
}

impl beefy_client_primitives::HostFunctions for HostFunctionsManager {
	fn keccak_256(input: &[u8]) -> [u8; 32] {
		sp_io::hashing::keccak_256(input)
	}

	fn secp256k1_ecdsa_recover_compressed(signature: &[u8; 65], msg: &[u8; 32]) -> Option<Vec<u8>> {
		sp_io::crypto::secp256k1_ecdsa_recover_compressed(signature, msg)
			.ok()
			.map(|pub_key| pub_key.to_vec())
	}
}

#[derive(Clone, Debug, PartialEq, Eq, ClientDef)]
pub enum AnyClient {
	Grandpa(ics10_grandpa::client_def::GrandpaClient<HostFunctionsManager>),
	Beefy(ics11_beefy::client_def::BeefyClient<HostFunctionsManager>),
	Tendermint(ics07_tendermint::client_def::TendermintClient<HostFunctionsManager>),
	Wasm(ics08_wasm::client_def::WasmClient<AnyClient, AnyClientState, AnyConsensusState>),
	#[cfg(test)]
	Mock(ibc::mock::client_def::MockClient),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnyUpgradeOptions {
	Grandpa(ics10_grandpa::client_state::UpgradeOptions),
	Beefy(ics11_beefy::client_state::UpgradeOptions),
	Tendermint(ics07_tendermint::client_state::UpgradeOptions),
	Wasm(Box<Self>),
	#[cfg(test)]
	Mock(()),
}

mod v1 {
	use super::*;

	#[derive(Clone, Debug, PartialEq, Eq)]
	pub enum AnyClientState {
		Grandpa(ics10_grandpa::client_state::ClientStateV1<HostFunctionsManager>),
		Beefy(ics11_beefy::client_state::ClientState<HostFunctionsManager>),
		Tendermint(ics07_tendermint::client_state::ClientState<HostFunctionsManager>),
		Wasm(ics08_wasm::client_state::ClientState<AnyClient, Self, AnyConsensusState>),
		#[cfg(test)]
		Mock(ibc::mock::client_state::MockClientState),
	}

	impl ibc::protobuf::Protobuf<Any> for AnyClientState {}

	impl TryFrom<Any> for AnyClientState {
		type Error = ics02_client::error::Error;
		fn try_from(value: Any) -> Result<Self, Self::Error> {
			match value.type_url.as_str() {
				"" => Err(ics02_client::error::Error::empty_consensus_state_response()),
				GRANDPA_CLIENT_STATE_TYPE_URL => Ok(Self::Grandpa(<ics10_grandpa::client_state::ClientStateV1<HostFunctionsManager>>::decode_vec(&value.value).map_err(::ibc::core::ics02_client::error::Error::decode_raw_client_state)?)),
				BEEFY_CLIENT_STATE_TYPE_URL => Ok(Self::Beefy(<ics11_beefy::client_state::ClientState<HostFunctionsManager>>::decode_vec(&value.value).map_err(::ibc::core::ics02_client::error::Error::decode_raw_client_state)?)),
				TENDERMINT_CLIENT_STATE_TYPE_URL => Ok(Self::Tendermint(<ics07_tendermint::client_state::ClientState<HostFunctionsManager>>::decode_vec(&value.value).map_err(::ibc::core::ics02_client::error::Error::decode_raw_client_state)?)),
				WASM_CLIENT_STATE_TYPE_URL => Ok(Self::Wasm(<ics08_wasm::client_state::ClientState<AnyClient, Self, AnyConsensusState>>::decode_vec(&value.value).map_err(::ibc::core::ics02_client::error::Error::decode_raw_client_state)?)),
				#[cfg(test)] MOCK_CLIENT_STATE_TYPE_URL => Ok(Self::Mock(<ibc::mock::client_state::MockClientState>::decode_vec(&value.value).map_err(::ibc::core::ics02_client::error::Error::decode_raw_client_state)?)),
				_ => Err(ics02_client::error::Error::unknown_consensus_state_type(value.type_url)),
			}
		}
	}

	impl From<AnyClientState> for Any {
		fn from(value: AnyClientState) -> Self {
			match value {
				AnyClientState::Grandpa(value) => Any {
					type_url: GRANDPA_CLIENT_STATE_TYPE_URL.to_string(),
					value: value.encode_vec().unwrap(),
				},
				AnyClientState::Wasm(value) => Any {
					type_url: WASM_CLIENT_STATE_TYPE_URL.to_string(),
					value: value.encode_vec().unwrap(),
				},
				_ => unreachable!(),
			}
		}
	}
}

mod v2 {
	use super::*;

	#[derive(Clone, Debug, PartialEq, Eq, ClientState, Protobuf)]
	pub enum AnyClientState {
		#[ibc(proto_url = "GRANDPA_CLIENT_STATE_TYPE_URL")]
		Grandpa(ics10_grandpa::client_state::ClientStateV2<HostFunctionsManager>),
		#[ibc(proto_url = "BEEFY_CLIENT_STATE_TYPE_URL")]
		Beefy(ics11_beefy::client_state::ClientState<HostFunctionsManager>),
		#[ibc(proto_url = "TENDERMINT_CLIENT_STATE_TYPE_URL")]
		Tendermint(ics07_tendermint::client_state::ClientState<HostFunctionsManager>),
		#[ibc(proto_url = "WASM_CLIENT_STATE_TYPE_URL")]
		Wasm(ics08_wasm::client_state::ClientState<AnyClient, Self, AnyConsensusState>),
		#[cfg(test)]
		#[ibc(proto_url = "MOCK_CLIENT_STATE_TYPE_URL")]
		Mock(ibc::mock::client_state::MockClientState),
	}
}

pub use v1::AnyClientState as AnyClientStateV1;
pub use v2::AnyClientState as AnyClientStateV2;

pub type AnyClientState = AnyClientStateV2;

impl AnyClientState {
	/// Recursively decode the client state from the given `Any` type, until it
	/// matches the given predicate `f`. Only `Wasm` variant may be unpacked recursively.
	pub fn decode_recursive<F>(mut any: Any, f: F) -> Option<Self>
	where
		F: Fn(&Self) -> bool,
	{
		loop {
			let client_state = AnyClientState::try_from(any).ok()?;

			match client_state {
				AnyClientState::Wasm(wasm_client_state) =>
					any = Any::decode(&*wasm_client_state.data).ok()?,
				c =>
					if f(&c) {
						break Some(c)
					} else {
						return None
					},
			};
		}
	}

	pub fn unpack_recursive(&self) -> &Self {
		match self {
			AnyClientState::Wasm(wasm_state) => wasm_state.inner.unpack_recursive(),
			c => c,
		}
	}
}

impl AnyClientState {
	pub fn wasm(inner: Self, code_id: Bytes) -> Result<Self, tendermint_proto::Error> {
		Ok(Self::Wasm(
			ics08_wasm::client_state::ClientState::<AnyClient, Self, AnyConsensusState> {
				data: inner.encode_to_vec()?,
				latest_height: inner.latest_height(),
				inner: Box::new(inner),
				code_id,
				_phantom: Default::default(),
			},
		))
	}
}

#[derive(Clone, Debug, PartialEq, Eq, ConsensusState, Protobuf)]
pub enum AnyConsensusState {
	#[ibc(proto_url = "GRANDPA_CONSENSUS_STATE_TYPE_URL")]
	Grandpa(ics10_grandpa::consensus_state::ConsensusState),
	#[ibc(proto_url = "BEEFY_CONSENSUS_STATE_TYPE_URL")]
	Beefy(ics11_beefy::consensus_state::ConsensusState),
	#[ibc(proto_url = "TENDERMINT_CONSENSUS_STATE_TYPE_URL")]
	Tendermint(ics07_tendermint::consensus_state::ConsensusState),
	#[ibc(proto_url = "WASM_CONSENSUS_STATE_TYPE_URL")]
	Wasm(ics08_wasm::consensus_state::ConsensusState<Self>),
	#[cfg(test)]
	#[ibc(proto_url = "MOCK_CONSENSUS_STATE_TYPE_URL")]
	Mock(ibc::mock::client_state::MockConsensusState),
}

impl AnyConsensusState {
	pub fn wasm(inner: Self) -> Result<Self, tendermint_proto::Error> {
		Ok(Self::Wasm(ics08_wasm::consensus_state::ConsensusState {
			timestamp: inner.timestamp().nanoseconds(),
			data: inner.encode_to_vec()?,
			inner: Box::new(inner),
		}))
	}
}

#[derive(Clone, Debug, ClientMessage)]
#[allow(clippy::large_enum_variant)]
pub enum AnyClientMessage {
	#[ibc(proto_url = "GRANDPA_CLIENT_MESSAGE_TYPE_URL")]
	Grandpa(ics10_grandpa::client_message::ClientMessage),
	#[ibc(proto_url = "BEEFY_CLIENT_MESSAGE_TYPE_URL")]
	Beefy(ics11_beefy::client_message::ClientMessage),
	#[ibc(proto_url = "TENDERMINT_CLIENT_MESSAGE_TYPE_URL")]
	Tendermint(ics07_tendermint::client_message::ClientMessage),
	#[ibc(proto_url = "WASM_CLIENT_MESSAGE_TYPE_URL")]
	Wasm(ics08_wasm::client_message::ClientMessage<Self>),
	#[cfg(test)]
	#[ibc(proto_url = "MOCK_CLIENT_MESSAGE_TYPE_URL")]
	Mock(ibc::mock::header::MockClientMessage),
}

impl AnyClientMessage {
	pub fn maybe_header_height(&self) -> Option<Height> {
		match self {
			Self::Tendermint(inner) => match inner {
				ics07_tendermint::client_message::ClientMessage::Header(h) => Some(h.height()),
				ics07_tendermint::client_message::ClientMessage::Misbehaviour(_) => None,
			},
			Self::Beefy(inner) => match inner {
				ics11_beefy::client_message::ClientMessage::Header(_) =>
					unimplemented!("beefy header height"),
				ics11_beefy::client_message::ClientMessage::Misbehaviour(_) => None,
			},
			Self::Grandpa(inner) => match inner {
				ics10_grandpa::client_message::ClientMessage::Header(h) => Some(h.height()),
				ics10_grandpa::client_message::ClientMessage::Misbehaviour(_) => None,
			},
			Self::Wasm(inner) => match inner {
				ics08_wasm::client_message::ClientMessage::Header(h) =>
					h.inner.maybe_header_height(),
				ics08_wasm::client_message::ClientMessage::Misbehaviour(_) => None,
			},
			#[cfg(test)]
			Self::Mock(inner) => match inner {
				ibc::mock::header::MockClientMessage::Header(h) => Some(h.height()),
				ibc::mock::header::MockClientMessage::Misbehaviour(_) => None,
			},
		}
	}

	pub fn wasm(inner: Self) -> Result<Self, tendermint_proto::Error> {
		let maybe_height = inner.maybe_header_height();
		Ok(match maybe_height {
			Some(height) => Self::Wasm(ics08_wasm::client_message::ClientMessage::Header(
				ics08_wasm::client_message::Header {
					data: inner.encode_to_vec()?,
					height,
					inner: Box::new(inner),
				},
			)),
			None => Self::Wasm(ics08_wasm::client_message::ClientMessage::Misbehaviour(
				ics08_wasm::client_message::Misbehaviour {
					data: inner.encode_to_vec()?,
					inner: Box::new(inner),
				},
			)),
		})
	}

	pub fn unpack_recursive(&self) -> &Self {
		match self {
			Self::Wasm(ics08_wasm::client_message::ClientMessage::Header(h)) =>
				h.inner.unpack_recursive(),
			Self::Wasm(ics08_wasm::client_message::ClientMessage::Misbehaviour(m)) =>
				m.inner.unpack_recursive(),
			_ => self,
		}
	}

	pub fn unpack_recursive_into(self) -> Self {
		match self {
			Self::Wasm(ics08_wasm::client_message::ClientMessage::Header(h)) =>
				h.inner.unpack_recursive_into(),
			Self::Wasm(ics08_wasm::client_message::ClientMessage::Misbehaviour(m)) =>
				m.inner.unpack_recursive_into(),
			_ => self,
		}
	}
}

impl Protobuf<Any> for AnyClientMessage {}

impl TryFrom<Any> for AnyClientMessage {
	type Error = ics02_client::error::Error;

	fn try_from(value: Any) -> Result<Self, Self::Error> {
		match value.type_url.as_str() {
			GRANDPA_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Grandpa(
				ics10_grandpa::client_message::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			GRANDPA_HEADER_TYPE_URL =>
				Ok(Self::Grandpa(ics10_grandpa::client_message::ClientMessage::Header(
					ics10_grandpa::client_message::Header::decode_vec(&value.value)
						.map_err(ics02_client::error::Error::decode_raw_header)?,
				))),
			GRANDPA_MISBEHAVIOUR_TYPE_URL =>
				Ok(Self::Grandpa(ics10_grandpa::client_message::ClientMessage::Misbehaviour(
					ics10_grandpa::client_message::Misbehaviour::decode_vec(&value.value)
						.map_err(ics02_client::error::Error::decode_raw_header)?,
				))),
			// TODO: beefy header, misbehaviour impl From<Any>
			BEEFY_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Beefy(
				ics11_beefy::client_message::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			TENDERMINT_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Tendermint(
				ics07_tendermint::client_message::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			TENDERMINT_HEADER_TYPE_URL =>
				Ok(Self::Tendermint(ics07_tendermint::client_message::ClientMessage::Header(
					ics07_tendermint::client_message::Header::decode_vec(&value.value)
						.map_err(ics02_client::error::Error::decode_raw_header)?,
				))),
			TENDERMINT_MISBEHAVIOUR_TYPE_URL =>
				Ok(Self::Tendermint(ics07_tendermint::client_message::ClientMessage::Misbehaviour(
					ics07_tendermint::client_message::Misbehaviour::decode_vec(&value.value)
						.map_err(ics02_client::error::Error::decode_raw_header)?,
				))),
			WASM_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Wasm(
				ics08_wasm::client_message::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			WASM_HEADER_TYPE_URL =>
				Ok(Self::Wasm(ics08_wasm::client_message::ClientMessage::Header(
					ics08_wasm::client_message::Header::decode_vec(&value.value)
						.map_err(ics02_client::error::Error::decode_raw_header)?,
				))),
			WASM_MISBEHAVIOUR_TYPE_URL =>
				Ok(Self::Wasm(ics08_wasm::client_message::ClientMessage::Misbehaviour(
					ics08_wasm::client_message::Misbehaviour::decode_vec(&value.value)
						.map_err(ics02_client::error::Error::decode_raw_header)?,
				))),
			_ => Err(ics02_client::error::Error::unknown_consensus_state_type(value.type_url)),
		}
	}
}

impl From<AnyClientMessage> for Any {
	fn from(client_msg: AnyClientMessage) -> Self {
		match client_msg {
			AnyClientMessage::Wasm(msg) => match msg {
				ics08_wasm::client_message::ClientMessage::Header(h) => Any {
					type_url: WASM_HEADER_TYPE_URL.to_string(),
					value: h.encode_vec().expect("encode_vec failed"),
				},
				ics08_wasm::client_message::ClientMessage::Misbehaviour(m) => Any {
					type_url: WASM_MISBEHAVIOUR_TYPE_URL.to_string(),
					value: m.encode_vec().expect("encode_vec failed"),
				},
			},
			AnyClientMessage::Grandpa(msg) => match msg {
				ics10_grandpa::client_message::ClientMessage::Header(h) => Any {
					type_url: GRANDPA_HEADER_TYPE_URL.to_string(),
					value: h.encode_vec().expect("encode_vec failed"),
				},
				ics10_grandpa::client_message::ClientMessage::Misbehaviour(m) => Any {
					type_url: GRANDPA_MISBEHAVIOUR_TYPE_URL.to_string(),
					value: m.encode_vec().expect("encode_vec failed"),
				},
			},
			AnyClientMessage::Beefy(msg) => Any {
				type_url: BEEFY_CLIENT_MESSAGE_TYPE_URL.to_string(),
				value: msg.encode_vec().expect("encode_vec failed"),
			},
			AnyClientMessage::Tendermint(msg) => Any {
				type_url: TENDERMINT_CLIENT_MESSAGE_TYPE_URL.to_string(),
				value: msg.encode_vec().expect("encode_vec failed"),
			},

			#[cfg(test)]
			AnyClientMessage::Mock(_msg) => panic!("MockHeader can't be serialized"),
		}
	}
}

#[cfg(test)]
pub use mocks::*;

#[cfg(test)]
mod mocks {
	pub const MOCK_CLIENT_STATE_TYPE_URL: &str = "/ibc.mock.ClientState";
	pub const MOCK_CLIENT_MESSAGE_TYPE_URL: &str = "/ibc.mock.ClientMessage";
	pub const MOCK_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.mock.ConsensusState";
}
