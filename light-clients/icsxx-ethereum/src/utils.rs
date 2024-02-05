use crate::{IBC_COMMITMENT_STORAGE_INDEX, IBC_CORE_STORAGE_PREFIX};
use ethabi::Token;
use primitive_types::{H256, U256};
use std::ops::Add;
use tiny_keccak::Hasher;

pub fn keccak256<T: AsRef<[u8]>>(x: T) -> [u8; 32] {
	let mut hasher = tiny_keccak::Keccak::v256();
	hasher.update(x.as_ref());
	let mut output = [0u8; 32];
	hasher.finalize(&mut output);
	output
}

pub fn commitment_storage_raw_key(key: &str, prefix: &[u8]) -> H256 {
	let slot_index =
		U256::from_big_endian(&keccak256([prefix, IBC_CORE_STORAGE_PREFIX].concat())[..]);
	let key = keccak256(key.as_bytes());
	let encoded = ethabi::encode(&[
		Token::FixedBytes(key.to_vec()),
		Token::Uint(slot_index.add(IBC_COMMITMENT_STORAGE_INDEX)),
	]);
	let index = keccak256(&encoded).to_vec();
	H256::from_slice(&index)
}
