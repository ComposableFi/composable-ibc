use alloc::{borrow::ToOwned, boxed::Box, format, string::ToString, vec::Vec};
use frame_support::{
	pallet_prelude::{StorageValue, ValueQuery},
	traits::StorageInstance,
};
use ibc::{
	core::{
		ics02_client,
		ics02_client::{
			client_consensus::ConsensusState, client_message::ClientMessage,
			client_state::ClientState,
		},
		ics04_channel::packet::Packet,
		ics23_commitment::{
			commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
			merkle::{apply_prefix, MerkleProof},
			specs::ProofSpecs,
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
	client_state::GRANDPA_CLIENT_STATE_TYPE_URL,
	consensus_state::GRANDPA_CONSENSUS_STATE_TYPE_URL,
};
use ics11_beefy::{
	client_message::BEEFY_CLIENT_MESSAGE_TYPE_URL, client_state::BEEFY_CLIENT_STATE_TYPE_URL,
	consensus_state::BEEFY_CONSENSUS_STATE_TYPE_URL,
};
use ics23::HostFunctionsProvider;
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
			return
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
	#[cfg(test)]
	Mock(ibc::mock::client_def::MockClient),
}

#[test]
fn tst() {
	/*
		call verify_membership function with the following arguments:
		prefix=696263
		proof=0aef040aec040a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d312f73657175656e6365732f311220439dd0ea54f168850977fce615993ac34f8e2d238c76c0c9328a744e89a2230d1a0c0801180120012a040002c028222d080112060406948901201a212018b8b1cd4e5487dd06c9d630e90e4db05476e5c379cff0c346173b70cfa0983e222b08011227060a94890120505f0e6b3039e050189315bd9a868a00274aacabd9ae85abbd58d857a8f5ec5620222d08011206081a948901201a21208261cb18738d315a27474946d85c0ae81715858dee97e6abda2892ccafa18776222d080112060a2a948901201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222d080112060c56948901201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222d080112060e74948901201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222e0801120712bc02948901201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222e0801120714d604948901201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222e0801120716b609948901201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222e0801120718ee13948901201a2120763fe0dc26765a9069f0198cb4dc24ed387a710ba0bf4f75ae6e6ab3a51d80ec222e080112071aac22968901201a2120000544f6164eb772b5e4d861e753b4ad813d7b2e26489d28e22414d04c24ba670afe010afb010a03696263122002d1c5b1ce8dc9ffe45249153d97fd23379cfdb48cc4863004a9d746062fa3121a090801180120012a0100222708011201011a2052532327b15377f3e4610721f7e69778833f471f6af4d8f3f9f06c81c9ab70ec222508011221013d62060c06c86ae55dc892cbefc1d5a63c65a638f8b613ceecc55250d0af20c5222708011201011a2044ba93e2aa5de099a7e0a4cc79a5c0911956d3557cf22ba26b16117da845bdb8222508011221014ccbbe24a16228b2bc74e21d028b42b03753c679ca0dab083a621c561ba27b11222708011201011a2014a66aa0ada7b629a21909b3f812c0dfc91bb73da10c6bc9ac9eddf3f05efd31
		root=07b49df5ea9c2d9ee2b3a67dac9830e411434e61625016a4512a368c1f39597a
		path=acks/ports/transfer/channels/channel-3/sequences/1 // Actually for 1????
		value=08f7557ed51826fe18d84512bf24ec75001edbaf2123a477df72a0a9f3640a7c

	67,157,208,234,84,241,104,133,9,119,252,230,21,153,58,195,79,142,45,35,140,118,192,201,50,138,116,78,137,162,35,13
	8,247,85,126,213,24,38,254,24,216,69,18,191,36,236,117,0,30,219,175,33,35,164,119,223,114,160,169,243,100,10,124
			 */
	use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;

	let p_val = hex::encode(vec![
		67, 157, 208, 234, 84, 241, 104, 133, 9, 119, 252, 230, 21, 153, 58, 195, 79, 142, 45, 35,
		140, 118, 192, 201, 50, 138, 116, 78, 137, 162, 35, 13,
	]);
	let val = hex::encode(vec![
		8, 247, 85, 126, 213, 24, 38, 254, 24, 216, 69, 18, 191, 36, 236, 117, 0, 30, 219, 175, 33,
		35, 164, 119, 223, 114, 160, 169, 243, 100, 10, 124,
	]);
	let ack = vec![123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 34, 65, 81, 61, 61, 34, 125];
	let ack_comm = hex::encode(HostFunctionsManager::sha2_256(&ack));
	println!("p_val={}", p_val);
	println!("val={}", val);
	println!("ack={}", String::from_utf8(ack).unwrap());
	println!("ack_comm={}", ack_comm);
	// let packet = Packet {
	// 	sequence: Sequence(1),
	// 	source_port: PortId("transfer"),
	// 	source_channel: ChannelId("channel-1"),
	// 	destination_port: PortId("transfer"),
	// 	destination_channel: ChannelId("channel-3"),
	// 	data: [123, 34, 100, 101, 110, 111, 109, 34, 58, 34, 85, 78, 73, 84, 34, 44, 34, 97, 109,
	// 111, 117, 110, 116, 34, 58, 34, 50, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 34, 44,
	// 34, 115, 101, 110, 100, 101, 114, 34, 58, 34, 48, 120, 100, 52, 51, 53, 57, 51, 99, 55, 49,
	// 53, 102, 100, 100, 51, 49, 99, 54, 49, 49, 52, 49, 97, 98, 100, 48, 52, 97, 57, 57, 102, 100,
	// 54, 56, 50, 50, 99, 56, 53, 53, 56, 56, 53, 52, 99, 99, 100, 101, 51, 57, 97, 53, 54, 56, 52,
	// 101, 55, 97, 53, 54, 100, 97, 50, 55, 100, 34, 44, 34, 114, 101, 99, 101, 105, 118, 101, 114,
	// 34, 58, 34, 98, 97, 110, 107, 115, 121, 49, 103, 53, 114, 50, 118, 109, 110, 112, 54, 108,
	// 116, 97, 57, 99, 112, 115, 116, 52, 108, 122, 99, 52, 115, 121, 121, 51, 107, 99, 106, 50,
	// 108, 106, 57, 53, 114, 122, 112, 48, 34, 44, 34, 109, 101, 109, 111, 34, 58, 34, 34, 125],
	// 	timeout_height: Height { revision: 1, height: 1008634 },
	// 	timeout_timestamp: Timestamp { time: Some(Time(2023-05-29 15:36:29.403068)) }
	// };

	// let key = get_key_path(KeyPathType::AcksPath, &packet);
	// log::debug!(target: "hyperspace", "query proof for acks path: {:?}", key);
	// let proof = source.query_proof(proof_height, vec![key.into_bytes()]).await?;

	let prefix = CommitmentPrefix::try_from(hex::decode("696263").unwrap()).unwrap();
	// let path = "acks/ports/transfer/channels/channel-3/sequences/1".to_string(); // INCORRECT
	let path = "acks/ports/transfer/channels/channel-3/sequences/1".to_string();
	let merkle_path = apply_prefix(&prefix, vec![path]);
	/*
		0ae3040ae0040a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d312f73657175656e6365732f311220439dd0ea54f168850977fce615993ac34f8e2d238c76c0c9328a744e89a2230d1a0c0801180120012a040002c028222c080112050204fe3a201a2120480c1b0987c13fa1d32eddf907f2e9fe93ab3f7f404425a73d8eaf0098a1605c222a080112260408fe3a20505f0e6b3039e050189315bd9a868a00274aacabd9ae85abbd58d857a8f5ec5620222c08011205060efe3a201a2120935a87d5e9793226060dba025eb7c115d91a5ac4836e12a5148ba6b8ccaaf28a222c080112050816fe3a201a21206668aceaee9fd2b00b15d2bf648eb87cf2b7096bd68a1d19d64ba406c602ec05222c080112050a26fe3a201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222c080112050c52fe3a201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222c080112050e70fe3a201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222d0801120612b802fe3a201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222d0801120614d204fe3a201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222d0801120616b209fe3a201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222d08011206189417803b201a212000d2e3fa0837a004fcfcf55c8c9d6dffdce917fa38fe5bcf486e2570df809b380afe010afb010a03696263122063980b887cc5fbb1396c57606ab2af5e8dbb16148f52df4b651037d4dba118db1a090801180120012a0100222708011201011a2052532327b15377f3e4610721f7e69778833f471f6af4d8f3f9f06c81c9ab70ec222508011221013d62060c06c86ae55dc892cbefc1d5a63c65a638f8b613ceecc55250d0af20c5222708011201011a202ae89f77341cb41a08d4cd8849497a5bc9109ca5efe66013606e361c55ba23f5222508011221019479c3650b54d547b8bfc0c7478201397d23272c94015924844030d9408d772b222708011201011a20439292d258f8fd20dab1eb562d4440cacdf560295afd7f5a88e84209aaa567ea
		0afb0912f8090a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d312f73657175656e6365732f3212e0040a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d312f73657175656e6365732f311220439dd0ea54f168850977fce615993ac34f8e2d238c76c0c9328a744e89a2230d1a0c0801180120012a040002c028222c080112050204fe3a201a2120480c1b0987c13fa1d32eddf907f2e9fe93ab3f7f404425a73d8eaf0098a1605c222a080112260408fe3a20505f0e6b3039e050189315bd9a868a00274aacabd9ae85abbd58d857a8f5ec5620222c08011205060efe3a201a2120935a87d5e9793226060dba025eb7c115d91a5ac4836e12a5148ba6b8ccaaf28a222c080112050816fe3a201a21206668aceaee9fd2b00b15d2bf648eb87cf2b7096bd68a1d19d64ba406c602ec05222c080112050a26fe3a201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222c080112050c52fe3a201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222c080112050e70fe3a201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222d0801120612b802fe3a201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222d0801120614d204fe3a201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222d0801120616b209fe3a201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222d08011206189417803b201a212000d2e3fa0837a004fcfcf55c8c9d6dffdce917fa38fe5bcf486e2570df809b381ade040a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d322f73657175656e6365732f31122008f7557ed51826fe18d84512bf24ec75001edbaf2123a477df72a0a9f3640a7c1a0c0801180120012a040002fe3a222a080112260204fe3a207808e89d75d39d630357c83855ca535b3b7ca1d6a920308153a9c5267b012bfe20222a080112260408fe3a20505f0e6b3039e050189315bd9a868a00274aacabd9ae85abbd58d857a8f5ec5620222c08011205060efe3a201a2120935a87d5e9793226060dba025eb7c115d91a5ac4836e12a5148ba6b8ccaaf28a222c080112050816fe3a201a21206668aceaee9fd2b00b15d2bf648eb87cf2b7096bd68a1d19d64ba406c602ec05222c080112050a26fe3a201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222c080112050c52fe3a201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222c080112050e70fe3a201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222d0801120612b802fe3a201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222d0801120614d204fe3a201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222d0801120616b209fe3a201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222d08011206189417803b201a212000d2e3fa0837a004fcfcf55c8c9d6dffdce917fa38fe5bcf486e2570df809b380afe010afb010a03696263122063980b887cc5fbb1396c57606ab2af5e8dbb16148f52df4b651037d4dba118db1a090801180120012a0100222708011201011a2052532327b15377f3e4610721f7e69778833f471f6af4d8f3f9f06c81c9ab70ec222508011221013d62060c06c86ae55dc892cbefc1d5a63c65a638f8b613ceecc55250d0af20c5222708011201011a202ae89f77341cb41a08d4cd8849497a5bc9109ca5efe66013606e361c55ba23f5222508011221019479c3650b54d547b8bfc0c7478201397d23272c94015924844030d9408d772b222708011201011a20439292d258f8fd20dab1eb562d4440cacdf560295afd7f5a88e84209aaa567ea
		0ada0912d7090a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d332f73657175656e6365732f3112de040a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d322f73657175656e6365732f31122008f7557ed51826fe18d84512bf24ec75001edbaf2123a477df72a0a9f3640a7c1a0c0801180120012a040002fe3a222a080112260204fe3a207808e89d75d39d630357c83855ca535b3b7ca1d6a920308153a9c5267b012bfe20222a080112260408fe3a20505f0e6b3039e050189315bd9a868a00274aacabd9ae85abbd58d857a8f5ec5620222c08011205060efe3a201a2120935a87d5e9793226060dba025eb7c115d91a5ac4836e12a5148ba6b8ccaaf28a222c080112050816fe3a201a21206668aceaee9fd2b00b15d2bf648eb87cf2b7096bd68a1d19d64ba406c602ec05222c080112050a26fe3a201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222c080112050c52fe3a201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222c080112050e70fe3a201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222d0801120612b802fe3a201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222d0801120614d204fe3a201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222d0801120616b209fe3a201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222d08011206189417803b201a212000d2e3fa0837a004fcfcf55c8c9d6dffdce917fa38fe5bcf486e2570df809b381abf040a2d6368616e6e656c456e64732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d301232080310011a150a087472616e7366657212096368616e6e656c2d30220c636f6e6e656374696f6e2d302a0769637332302d311a0c0801180120012a040002a20e222c080112050406c439201a212044d2cf4203c6fdaf07d659a04fd86155373e044bd94d95f6166aa5f26059cb5b222a08011226060efe3a20e75630ab1971fb37e1b04dac930140f7eca457df219a451a65d3483f587616da20222c080112050816fe3a201a21206668aceaee9fd2b00b15d2bf648eb87cf2b7096bd68a1d19d64ba406c602ec05222c080112050a26fe3a201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222c080112050c52fe3a201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222c080112050e70fe3a201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222d0801120612b802fe3a201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222d0801120614d204fe3a201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222d0801120616b209fe3a201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222d08011206189417803b201a212000d2e3fa0837a004fcfcf55c8c9d6dffdce917fa38fe5bcf486e2570df809b380afe010afb010a03696263122063980b887cc5fbb1396c57606ab2af5e8dbb16148f52df4b651037d4dba118db1a090801180120012a0100222708011201011a2052532327b15377f3e4610721f7e69778833f471f6af4d8f3f9f06c81c9ab70ec222508011221013d62060c06c86ae55dc892cbefc1d5a63c65a638f8b613ceecc55250d0af20c5222708011201011a202ae89f77341cb41a08d4cd8849497a5bc9109ca5efe66013606e361c55ba23f5222508011221019479c3650b54d547b8bfc0c7478201397d23272c94015924844030d9408d772b222708011201011a20439292d258f8fd20dab1eb562d4440cacdf560295afd7f5a88e84209aaa567ea

	0ae3040ae0040a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d312f73657175656e6365732f311220439dd0ea54f168850977fce615993ac34f8e2d238c76c0c9328a744e89a2230d1a0c0801180120012a040002c028222c080112050204fe3a201a2120480c1b0987c13fa1d32eddf907f2e9fe93ab3f7f404425a73d8eaf0098a1605c222a080112260408fe3a20505f0e6b3039e050189315bd9a868a00274aacabd9ae85abbd58d857a8f5ec5620222c08011205060efe3a201a2120935a87d5e9793226060dba025eb7c115d91a5ac4836e12a5148ba6b8ccaaf28a222c080112050816fe3a201a21206668aceaee9fd2b00b15d2bf648eb87cf2b7096bd68a1d19d64ba406c602ec05222c080112050a26fe3a201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222c080112050c52fe3a201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222c080112050e70fe3a201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222d0801120612b802fe3a201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222d0801120614d204fe3a201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222d0801120616b209fe3a201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222d08011206189417823b201a21201512619f2a3f2a1752761e813471ce1c9e8a47d936ae62af0005ff735b8a72db0afe010afb010a036962631220e34d32b9bac9a60fa7a13615005578454cfd7fc01ee6b50564793997ef8bb1f01a090801180120012a0100222708011201011a2052532327b15377f3e4610721f7e69778833f471f6af4d8f3f9f06c81c9ab70ec222508011221013d62060c06c86ae55dc892cbefc1d5a63c65a638f8b613ceecc55250d0af20c5222708011201011a20df1aefa952d4e4e1511ec2a7b025f215cb94b30af69ed4c0c40a4c9c72df7896222508011221014e7799463aff63abfd502b6a371c0060b681eb570abeaa6bb8ec8e95f9992fc5222708011201011a20439292d258f8fd20dab1eb562d4440cacdf560295afd7f5a88e84209aaa567ea
	0afb0912f8090a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d312f73657175656e6365732f3212e0040a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d312f73657175656e6365732f311220439dd0ea54f168850977fce615993ac34f8e2d238c76c0c9328a744e89a2230d1a0c0801180120012a040002c028222c080112050204fe3a201a2120480c1b0987c13fa1d32eddf907f2e9fe93ab3f7f404425a73d8eaf0098a1605c222a080112260408fe3a20505f0e6b3039e050189315bd9a868a00274aacabd9ae85abbd58d857a8f5ec5620222c08011205060efe3a201a2120935a87d5e9793226060dba025eb7c115d91a5ac4836e12a5148ba6b8ccaaf28a222c080112050816fe3a201a21206668aceaee9fd2b00b15d2bf648eb87cf2b7096bd68a1d19d64ba406c602ec05222c080112050a26fe3a201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222c080112050c52fe3a201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222c080112050e70fe3a201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222d0801120612b802fe3a201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222d0801120614d204fe3a201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222d0801120616b209fe3a201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222d08011206189417823b201a21201512619f2a3f2a1752761e813471ce1c9e8a47d936ae62af0005ff735b8a72db1ade040a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d322f73657175656e6365732f31122008f7557ed51826fe18d84512bf24ec75001edbaf2123a477df72a0a9f3640a7c1a0c0801180120012a040002fe3a222a080112260204fe3a207808e89d75d39d630357c83855ca535b3b7ca1d6a920308153a9c5267b012bfe20222a080112260408fe3a20505f0e6b3039e050189315bd9a868a00274aacabd9ae85abbd58d857a8f5ec5620222c08011205060efe3a201a2120935a87d5e9793226060dba025eb7c115d91a5ac4836e12a5148ba6b8ccaaf28a222c080112050816fe3a201a21206668aceaee9fd2b00b15d2bf648eb87cf2b7096bd68a1d19d64ba406c602ec05222c080112050a26fe3a201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222c080112050c52fe3a201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222c080112050e70fe3a201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222d0801120612b802fe3a201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222d0801120614d204fe3a201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222d0801120616b209fe3a201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222d08011206189417823b201a21201512619f2a3f2a1752761e813471ce1c9e8a47d936ae62af0005ff735b8a72db0afe010afb010a036962631220e34d32b9bac9a60fa7a13615005578454cfd7fc01ee6b50564793997ef8bb1f01a090801180120012a0100222708011201011a2052532327b15377f3e4610721f7e69778833f471f6af4d8f3f9f06c81c9ab70ec222508011221013d62060c06c86ae55dc892cbefc1d5a63c65a638f8b613ceecc55250d0af20c5222708011201011a20df1aefa952d4e4e1511ec2a7b025f215cb94b30af69ed4c0c40a4c9c72df7896222508011221014e7799463aff63abfd502b6a371c0060b681eb570abeaa6bb8ec8e95f9992fc5222708011201011a20439292d258f8fd20dab1eb562d4440cacdf560295afd7f5a88e84209aaa567ea
	0ada0912d7090a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d332f73657175656e6365732f3112de040a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d322f73657175656e6365732f31122008f7557ed51826fe18d84512bf24ec75001edbaf2123a477df72a0a9f3640a7c1a0c0801180120012a040002fe3a222a080112260204fe3a207808e89d75d39d630357c83855ca535b3b7ca1d6a920308153a9c5267b012bfe20222a080112260408fe3a20505f0e6b3039e050189315bd9a868a00274aacabd9ae85abbd58d857a8f5ec5620222c08011205060efe3a201a2120935a87d5e9793226060dba025eb7c115d91a5ac4836e12a5148ba6b8ccaaf28a222c080112050816fe3a201a21206668aceaee9fd2b00b15d2bf648eb87cf2b7096bd68a1d19d64ba406c602ec05222c080112050a26fe3a201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222c080112050c52fe3a201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222c080112050e70fe3a201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222d0801120612b802fe3a201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222d0801120614d204fe3a201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222d0801120616b209fe3a201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222d08011206189417823b201a21201512619f2a3f2a1752761e813471ce1c9e8a47d936ae62af0005ff735b8a72db1abf040a2d6368616e6e656c456e64732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d301232080310011a150a087472616e7366657212096368616e6e656c2d30220c636f6e6e656374696f6e2d302a0769637332302d311a0c0801180120012a040002a20e222c080112050406c439201a212044d2cf4203c6fdaf07d659a04fd86155373e044bd94d95f6166aa5f26059cb5b222a08011226060efe3a20e75630ab1971fb37e1b04dac930140f7eca457df219a451a65d3483f587616da20222c080112050816fe3a201a21206668aceaee9fd2b00b15d2bf648eb87cf2b7096bd68a1d19d64ba406c602ec05222c080112050a26fe3a201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222c080112050c52fe3a201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222c080112050e70fe3a201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222d0801120612b802fe3a201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222d0801120614d204fe3a201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222d0801120616b209fe3a201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222d08011206189417823b201a21201512619f2a3f2a1752761e813471ce1c9e8a47d936ae62af0005ff735b8a72db0afe010afb010a036962631220e34d32b9bac9a60fa7a13615005578454cfd7fc01ee6b50564793997ef8bb1f01a090801180120012a0100222708011201011a2052532327b15377f3e4610721f7e69778833f471f6af4d8f3f9f06c81c9ab70ec222508011221013d62060c06c86ae55dc892cbefc1d5a63c65a638f8b613ceecc55250d0af20c5222708011201011a20df1aefa952d4e4e1511ec2a7b025f215cb94b30af69ed4c0c40a4c9c72df7896222508011221014e7799463aff63abfd502b6a371c0060b681eb570abeaa6bb8ec8e95f9992fc5222708011201011a20439292d258f8fd20dab1eb562d4440cacdf560295afd7f5a88e84209aaa567ea
			 */
	let proof = CommitmentProofBytes::try_from(hex::decode("0a9b050a98050a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d332f73657175656e6365732f31122008f7557ed51826fe18d84512bf24ec75001edbaf2123a477df72a0a9f3640a7c1a0d0801180120012a050002948901222b08011227020494890120480c1b0987c13fa1d32eddf907f2e9fe93ab3f7f404425a73d8eaf0098a1605c20222b080112270406948901207808e89d75d39d630357c83855ca535b3b7ca1d6a920308153a9c5267b012bfe20222b08011227060a94890120505f0e6b3039e050189315bd9a868a00274aacabd9ae85abbd58d857a8f5ec5620222d08011206081a948901201a21208261cb18738d315a27474946d85c0ae81715858dee97e6abda2892ccafa18776222d080112060a2a948901201a212052e1cd1918fc98b1e22a4b991ad09fd3ed6c436df7c90870915442a6085ea3b0222d080112060c56948901201a21204d67f598989d37a692a4635dbc89a2649db7aa21b6855a6470fae20bd6e0a5cf222d080112060e74948901201a2120f61c857726f77a622fe143f51ae94c8dd0d79d0a8d653d56657c400dd3887478222e0801120712bc02948901201a21201045d06001543194fc62c8de1d929fd83ade885ef8440b169a5f3a4a81a2e1b5222e0801120714d604948901201a212043178002cfe68dc7580ed9f10dbb61f02e8120f628e0f57c127e9f6a8483891b222e0801120716b609948901201a2120c999862e21661294b2347239fdda9eee5bd1519fb143209cd5e01d997da3f058222e0801120718ee13948901201a2120763fe0dc26765a9069f0198cb4dc24ed387a710ba0bf4f75ae6e6ab3a51d80ec222e080112071aac22968901201a2120000544f6164eb772b5e4d861e753b4ad813d7b2e26489d28e22414d04c24ba670afe010afb010a03696263122002d1c5b1ce8dc9ffe45249153d97fd23379cfdb48cc4863004a9d746062fa3121a090801180120012a0100222708011201011a2052532327b15377f3e4610721f7e69778833f471f6af4d8f3f9f06c81c9ab70ec222508011221013d62060c06c86ae55dc892cbefc1d5a63c65a638f8b613ceecc55250d0af20c5222708011201011a2044ba93e2aa5de099a7e0a4cc79a5c0911956d3557cf22ba26b16117da845bdb8222508011221014ccbbe24a16228b2bc74e21d028b42b03753c679ca0dab083a621c561ba27b11222708011201011a2014a66aa0ada7b629a21909b3f812c0dfc91bb73da10c6bc9ac9eddf3f05efd31").unwrap()).unwrap();
	let merkle_proof: MerkleProof<HostFunctionsManager> =
		RawMerkleProof::try_from(proof.clone()).unwrap().into();
	// dbg!(&merkle_proof);
	let root = CommitmentRoot::from_bytes(
		&hex::decode("07b49df5ea9c2d9ee2b3a67dac9830e411434e61625016a4512a368c1f39597a").unwrap(),
	);
	let value =
		hex::decode("08f7557ed51826fe18d84512bf24ec75001edbaf2123a477df72a0a9f3640a7c").unwrap();
	let ps = ProofSpecs::default();
	merkle_proof
		.verify_membership(&ps, root.clone().into(), merkle_path, value, 0)
		.unwrap();
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
	#[cfg(test)]
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
