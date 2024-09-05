use alloc::{
	borrow::{Cow, ToOwned},
	boxed::Box,
	format,
	string::ToString,
	vec::Vec,
};
use borsh::maybestd::io;
use frame_support::{
	pallet_prelude::{StorageValue, ValueQuery},
	traits::StorageInstance,
};
use ibc::core::{
	ics02_client,
	ics02_client::{
		client_consensus::ConsensusState, client_message::ClientMessage, client_state::ClientState,
	},
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
	client_message::WASM_CLIENT_MESSAGE_TYPE_URL, client_state::WASM_CLIENT_STATE_TYPE_URL,
	consensus_state::WASM_CONSENSUS_STATE_TYPE_URL, Bytes,
};
use ics10_grandpa::{
	client_message::{
		RelayChainHeader, GRANDPA_CLIENT_MESSAGE_TYPE_URL, GRANDPA_HEADER_TYPE_URL,
		GRANDPA_MISBEHAVIOUR_TYPE_URL,
	},
	client_state::GRANDPA_CLIENT_STATE_TYPE_URL,
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
	traits::{BlakeTwo256, ConstU32, Header},
	BoundedBTreeSet, BoundedVec,
};
use tendermint::{
	crypto::{
		signature::{Error as TendermintCryptoError, Verifier},
		Sha256 as TendermintSha256,
	},
	merkle::{Hash, MerkleHash, NonIncremental, HASH_SIZE},
	PublicKey, Signature as TmSignature,
};
use tendermint_proto::Protobuf;

const GUEST_CLIENT_MESSAGE_TYPE_URL: &'static str = cf_guest::proto::ClientMessage::IBC_TYPE_URL;
const GUEST_CLIENT_STATE_TYPE_URL: &'static str = cf_guest::proto::ClientState::IBC_TYPE_URL;
const GUEST_CONSENSUS_STATE_TYPE_URL: &'static str = cf_guest::proto::ConsensusState::IBC_TYPE_URL;
const GUEST_HEADER_TYPE_URL: &'static str = cf_guest::proto::Header::IBC_TYPE_URL;
const GUEST_MISBEHAVIOUR_TYPE_URL: &'static str = cf_guest::proto::Misbehaviour::IBC_TYPE_URL;

const ROLLUP_CLIENT_MESSAGE_TYPE_URL: &'static str = cf_solana::proto::ClientMessage::IBC_TYPE_URL;
const ROLLUP_CLIENT_STATE_TYPE_URL: &'static str = cf_solana::proto::ClientState::IBC_TYPE_URL;
const ROLLUP_CONSENSUS_STATE_TYPE_URL: &'static str =
	cf_solana::proto::ConsensusState::IBC_TYPE_URL;
const ROLLUP_HEADER_TYPE_URL: &'static str = cf_solana::proto::Header::IBC_TYPE_URL;
const ROLLUP_MISBEHAVIOUR_TYPE_URL: &'static str = cf_solana::proto::Misbehaviour::IBC_TYPE_URL;

#[derive(Clone, Default, PartialEq, Debug, Eq)]
pub struct HostFunctionsManager;

/// Ed25519 public key (a.k.a. verifying key).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct PubKey(ed25519_dalek::VerifyingKey);

impl guestchain::PubKey for PubKey {
	type Signature = Signature;

	fn to_vec(&self) -> Vec<u8> {
		self.0.as_bytes().to_vec()
	}
	fn from_bytes(bytes: &[u8]) -> Result<Self, guestchain::BadFormat> {
		bytes.try_into().map(Self).map_err(|_| guestchain::BadFormat)
	}

	fn as_bytes(&self) -> Cow<'_, [u8]> {
		todo!()
	}
}

impl borsh::BorshSerialize for PubKey {
	fn serialize<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
		wr.write_all(self.0.as_bytes())
	}
}

impl borsh::BorshDeserialize for PubKey {
	fn deserialize_reader<R: io::Read>(rd: &mut R) -> io::Result<Self> {
		let mut bytes = ed25519_dalek::pkcs8::PublicKeyBytes([0; 32]);
		rd.read_exact(&mut bytes.0[..])?;
		ed25519_dalek::VerifyingKey::try_from(bytes)
			.map(Self)
			.map_err(|_| io::Error::new(io::ErrorKind::Other, "malformed Ed25519 public key"))
	}
}

impl PartialOrd for PubKey {
	fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(rhs))
	}
}

impl Ord for PubKey {
	fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
		self.0.as_bytes().cmp(rhs.0.as_bytes())
	}
}

/// Ed25519 signature.
#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Signature(ed25519_dalek::Signature);

impl guestchain::Signature for Signature {
	fn to_vec(&self) -> Vec<u8> {
		self.0.to_vec()
	}
	fn from_bytes(bytes: &[u8]) -> Result<Self, guestchain::BadFormat> {
		ed25519_dalek::Signature::from_slice(bytes)
			.map(Self)
			.map_err(|_| guestchain::BadFormat)
	}

	fn as_bytes(&self) -> Cow<'_, [u8]> {
		todo!()
	}
}

impl borsh::BorshSerialize for Signature {
	fn serialize<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
		wr.write_all(self.0.r_bytes())?;
		wr.write_all(self.0.s_bytes())?;
		Ok(())
	}
}

impl borsh::BorshDeserialize for Signature {
	fn deserialize_reader<R: io::Read>(rd: &mut R) -> io::Result<Self> {
		let mut buf = [0; 64];
		rd.read_exact(&mut buf[..])?;
		Ok(Self(ed25519_dalek::Signature::from_bytes(&buf)))
	}
}

impl core::hash::Hash for Signature {
	fn hash<H: core::hash::Hasher>(&self, hasher: &mut H) {
		hasher.write(self.0.r_bytes());
		hasher.write(self.0.s_bytes());
	}
}

impl PartialOrd for Signature {
	fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(rhs))
	}
}

impl Ord for Signature {
	fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
		let lhs = (self.0.r_bytes(), self.0.s_bytes());
		let rhs = (rhs.0.r_bytes(), rhs.0.s_bytes());
		lhs.cmp(&rhs)
	}
}

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
		signature: &TmSignature,
	) -> Result<(), TendermintCryptoError> {
		let signature = sp_core::ed25519::Signature::from_slice(signature.as_bytes())
			.ok_or(TendermintCryptoError::MalformedSignature)?;
		let public_key = sp_core::ed25519::Public::from_slice(pubkey.to_bytes().as_slice())
			.map_err(|_| TendermintCryptoError::MalformedPublicKey)?;
		sp_io::crypto::ed25519_verify(&signature, msg, &public_key)
			.then_some(())
			.ok_or(TendermintCryptoError::VerificationFailed)
	}
}

impl ics07_tendermint::HostFunctionsProvider for HostFunctionsManager {}

pub struct GrandpaHeaderHashesStorageInstance;

impl StorageInstance for GrandpaHeaderHashesStorageInstance {
	fn pallet_prefix() -> &'static str {
		"ibc.lightclients.grandpa"
	}

	const STORAGE_PREFIX: &'static str = "HeaderHashes";
}

pub type GrandpaHeaderHashesStorage = StorageValue<
	GrandpaHeaderHashesStorageInstance,
	BoundedVec<H256, ConstU32<GRANDPA_BLOCK_HASHES_CACHE_SIZE>>,
	ValueQuery,
>;

pub struct GrandpaHeaderHashesSetStorageInstance;

impl StorageInstance for GrandpaHeaderHashesSetStorageInstance {
	fn pallet_prefix() -> &'static str {
		"ibc.lightclients.grandpa"
	}

	const STORAGE_PREFIX: &'static str = "HeaderHashesSet";
}

pub type GrandpaHeaderHashesSetStorage = StorageValue<
	GrandpaHeaderHashesSetStorageInstance,
	BoundedBTreeSet<H256, ConstU32<GRANDPA_BLOCK_HASHES_CACHE_SIZE>>,
	ValueQuery,
>;

/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
const GRANDPA_BLOCK_HASHES_CACHE_SIZE: u32 = 500;

impl grandpa_client_primitives::HostFunctions for HostFunctionsManager {
	type Header = RelayChainHeader;

	fn ed25519_verify(sig: &ed25519::Signature, msg: &[u8], pub_key: &ed25519::Public) -> bool {
		pub_key.verify(&msg, sig)
	}

	fn insert_relay_header_hashes(new_hashes: &[<Self::Header as Header>::Hash]) {
		if new_hashes.is_empty() {
			return;
		}

		GrandpaHeaderHashesSetStorage::mutate(|hashes_set| {
			GrandpaHeaderHashesStorage::mutate(|hashes| {
				for hash in new_hashes {
					match hashes.try_push(*hash) {
						Ok(_) => {},
						Err(_) => {
							let old_hash = hashes.remove(0);
							hashes_set.remove(&old_hash);
							hashes.try_push(*hash).expect(
								"we just removed an element, so there is space for this one; qed",
							);
						},
					}
					match hashes_set.try_insert(*hash) {
						Ok(_) => {},
						Err(_) => {
							log::warn!("duplicated value in GrandpaHeaderHashesStorage or the storage is corrupted");
						},
					}
				}
			});
		});
	}

	fn contains_relay_header_hash(hash: <Self::Header as Header>::Hash) -> bool {
		GrandpaHeaderHashesSetStorage::get().contains(&hash)
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
	Guest(cf_guest::client_def::GuestClient<PubKey>),
	Rollup(cf_solana::client_def::SolanaClient),
	#[cfg(any(test, feature = "testing"))]
	Mock(ibc::mock::client_def::MockClient),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnyUpgradeOptions {
	Grandpa(ics10_grandpa::client_state::UpgradeOptions),
	Beefy(ics11_beefy::client_state::UpgradeOptions),
	Tendermint(ics07_tendermint::client_state::UpgradeOptions),
	Wasm(Box<Self>),
	Guest(cf_guest::client::UpgradeOptions),
	Rollup(cf_solana::client::UpgradeOptions),
	#[cfg(any(test, feature = "testing"))]
	Mock(()),
}

#[derive(Clone, Debug, PartialEq, Eq, ClientState, Protobuf)]
pub enum AnyClientState {
	#[ibc(proto_url = "GRANDPA_CLIENT_STATE_TYPE_URL")]
	Grandpa(ics10_grandpa::client_state::ClientState<HostFunctionsManager>),
	#[ibc(proto_url = "BEEFY_CLIENT_STATE_TYPE_URL")]
	Beefy(ics11_beefy::client_state::ClientState<HostFunctionsManager>),
	#[ibc(proto_url = "TENDERMINT_CLIENT_STATE_TYPE_URL")]
	Tendermint(ics07_tendermint::client_state::ClientState<HostFunctionsManager>),
	#[ibc(proto_url = "WASM_CLIENT_STATE_TYPE_URL")]
	Wasm(ics08_wasm::client_state::ClientState<AnyClient, Self, AnyConsensusState>),
	#[ibc(proto_url = "GUEST_CLIENT_STATE_TYPE_URL")]
	Guest(cf_guest::ClientState<PubKey>),
	#[ibc(proto_url = "ROLLUP_CLIENT_STATE_TYPE_URL")]
	Rollup(cf_solana::ClientState),
	#[cfg(any(test, feature = "testing"))]
	#[ibc(proto_url = "MOCK_CLIENT_STATE_TYPE_URL")]
	Mock(ibc::mock::client_state::MockClientState),
}

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
				AnyClientState::Wasm(wasm_client_state) => {
					any = Any::decode(&*wasm_client_state.data).ok()?
				},
				c => {
					if f(&c) {
						break Some(c);
					} else {
						return None;
					}
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
	pub fn wasm(inner: Self, checksum: Bytes) -> Result<Self, tendermint_proto::Error> {
		log::info!("This is height in any client state {:?}", inner.latest_height());
		Ok(Self::Wasm(
			ics08_wasm::client_state::ClientState::<AnyClient, Self, AnyConsensusState> {
				data: inner.encode_to_vec()?,
				latest_height: inner.latest_height(),
				inner: Box::new(inner),
				checksum,
				_phantom: Default::default(),
			},
		))
	}

	pub fn latest_height(&self) -> ibc::Height {
		match self {
			AnyClientState::Grandpa(client_state) => client_state.latest_height(),
			AnyClientState::Beefy(client_state) => client_state.latest_height(),
			AnyClientState::Tendermint(client_state) => client_state.latest_height(),
			AnyClientState::Wasm(client_state) => client_state.latest_height(),
			AnyClientState::Guest(client_state) => client_state.latest_height(),
			AnyClientState::Rollup(client_state) => client_state.latest_height(),
			#[cfg(any(test, feature = "testing"))]
			AnyClientState::Mock(client_state) => client_state.latest_height(),
		}
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
	#[ibc(proto_url = "GUEST_CONSENSUS_STATE_TYPE_URL")]
	Guest(cf_guest::ConsensusState),
	#[ibc(proto_url = "ROLLUP_CONSENSUS_STATE_TYPE_URL")]
	Rollup(cf_solana::ConsensusState),
	#[cfg(any(test, feature = "testing"))]
	#[ibc(proto_url = "MOCK_CONSENSUS_STATE_TYPE_URL")]
	Mock(ibc::mock::client_state::MockConsensusState),
}

impl AnyConsensusState {
	pub fn wasm(inner: Self) -> Result<Self, tendermint_proto::Error> {
		Ok(Self::Wasm(ics08_wasm::consensus_state::ConsensusState {
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
	#[ibc(proto_url = "GUEST_CLIENT_MESSAGE_TYPE_URL")]
	Guest(cf_guest::ClientMessage<PubKey>),
	#[ibc(proto_url = "ROLLUP_CLIENT_MESSAGE_TYPE_URL")]
	Rollup(cf_solana::ClientMessage),
	#[cfg(any(test, feature = "testing"))]
	#[ibc(proto_url = "MOCK_CLIENT_MESSAGE_TYPE_URL")]
	Mock(ibc::mock::header::MockClientMessage),
}

impl AnyClientMessage {
	pub fn wasm(inner: Self) -> Result<Self, tendermint_proto::Error> {
		Ok(Self::Wasm(ics08_wasm::client_message::ClientMessage {
			data: inner.encode_to_vec()?,
			inner: Box::new(inner),
		}))
	}

	pub fn unpack_recursive_into(self) -> Self {
		match self {
			Self::Wasm(ics08_wasm::client_message::ClientMessage { inner, data }) => *inner,
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
			GRANDPA_HEADER_TYPE_URL => {
				Ok(Self::Grandpa(ics10_grandpa::client_message::ClientMessage::Header(
					ics10_grandpa::client_message::Header::decode_vec(&value.value)
						.map_err(ics02_client::error::Error::decode_raw_header)?,
				)))
			},
			GRANDPA_MISBEHAVIOUR_TYPE_URL => {
				Ok(Self::Grandpa(ics10_grandpa::client_message::ClientMessage::Misbehaviour(
					ics10_grandpa::client_message::Misbehaviour::decode_vec(&value.value)
						.map_err(ics02_client::error::Error::decode_raw_header)?,
				)))
			},
			// TODO: beefy header, misbehaviour impl From<Any>
			BEEFY_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Beefy(
				ics11_beefy::client_message::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			TENDERMINT_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Tendermint(
				ics07_tendermint::client_message::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			TENDERMINT_HEADER_TYPE_URL => {
				Ok(Self::Tendermint(ics07_tendermint::client_message::ClientMessage::Header(
					ics07_tendermint::client_message::Header::decode_vec(&value.value)
						.map_err(ics02_client::error::Error::decode_raw_header)?,
				)))
			},
			TENDERMINT_MISBEHAVIOUR_TYPE_URL => {
				Ok(Self::Tendermint(ics07_tendermint::client_message::ClientMessage::Misbehaviour(
					ics07_tendermint::client_message::Misbehaviour::decode_vec(&value.value)
						.map_err(ics02_client::error::Error::decode_raw_header)?,
				)))
			},
			GUEST_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Guest(
				cf_guest::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			GUEST_HEADER_TYPE_URL => Ok(Self::Guest(cf_guest::ClientMessage::from(
				cf_guest::Header::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			))),
			GUEST_MISBEHAVIOUR_TYPE_URL => Ok(Self::Guest(cf_guest::ClientMessage::from(
				cf_guest::Misbehaviour::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			))),
			ROLLUP_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Rollup(
				cf_solana::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			ROLLUP_HEADER_TYPE_URL => Ok(Self::Rollup(cf_solana::ClientMessage::from(
				cf_solana::Header::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			))),
			ROLLUP_MISBEHAVIOUR_TYPE_URL => Ok(Self::Rollup(cf_solana::ClientMessage::from(
				cf_solana::Misbehaviour::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			))),
			WASM_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Wasm(
				ics08_wasm::client_message::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			_ => Err(ics02_client::error::Error::unknown_consensus_state_type(value.type_url)),
		}
	}
}

impl From<AnyClientMessage> for Any {
	fn from(client_msg: AnyClientMessage) -> Self {
		match client_msg {
			AnyClientMessage::Wasm(msg) => {
				Any { type_url: WASM_CLIENT_MESSAGE_TYPE_URL.to_string(), value: msg.encode_vec() }
			},
			AnyClientMessage::Grandpa(msg) => match msg {
				ics10_grandpa::client_message::ClientMessage::Header(h) => {
					Any { type_url: GRANDPA_HEADER_TYPE_URL.to_string(), value: h.encode_vec() }
				},
				ics10_grandpa::client_message::ClientMessage::Misbehaviour(m) => Any {
					type_url: GRANDPA_MISBEHAVIOUR_TYPE_URL.to_string(),
					value: m.encode_vec(),
				},
			},
			AnyClientMessage::Beefy(msg) => {
				Any { type_url: BEEFY_CLIENT_MESSAGE_TYPE_URL.to_string(), value: msg.encode_vec() }
			},
			AnyClientMessage::Tendermint(msg) => Any {
				type_url: TENDERMINT_CLIENT_MESSAGE_TYPE_URL.to_string(),
				value: msg.encode_vec(),
			},
			AnyClientMessage::Guest(msg) => {
				Any { type_url: GUEST_CLIENT_MESSAGE_TYPE_URL.to_string(), value: msg.encode_vec() }
			},
			AnyClientMessage::Rollup(msg) => Any {
				type_url: ROLLUP_CLIENT_MESSAGE_TYPE_URL.to_string(),
				value: msg.encode_vec(),
			},
			#[cfg(any(test, feature = "testing"))]
			AnyClientMessage::Mock(_msg) => panic!("MockHeader can't be serialized"),
		}
	}
}

#[cfg(any(test, feature = "testing"))]
pub use mocks::*;

#[cfg(any(test, feature = "testing"))]
mod mocks {
	pub const MOCK_CLIENT_STATE_TYPE_URL: &str = "/ibc.mock.ClientState";
	pub const MOCK_CLIENT_MESSAGE_TYPE_URL: &str = "/ibc.mock.ClientMessage";
	pub const MOCK_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.mock.ConsensusState";
}
