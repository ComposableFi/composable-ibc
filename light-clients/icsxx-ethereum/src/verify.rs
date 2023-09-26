use crate::{
	alloc::string::ToString,
	error::Error,
	proof::ethereum_trie::{verify_proof, EIP1186Layout, KeccakHasher, VerifyError},
	utils::keccak256,
};
use alloc::{boxed::Box, format, vec::Vec};
use ethabi::{ParamType, Token};
use ibc::core::{
	ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
	ics24_host::{identifier::ConnectionId, path::ConnectionsPath, Path},
};
use primitive_types::H256;

pub fn verify_ibc_proof<P>(
	prefix: &CommitmentPrefix, // TODO: use prefix
	proof: &CommitmentProofBytes,
	root: &CommitmentRoot,
	path: P,
	value: Option<&[u8]>,
) -> Result<bool, Error>
where
	P: Into<Path>,
{
	use VerifyError::*;

	let mut tokens =
		ethabi::decode(&[ParamType::Array(Box::new(ParamType::Bytes))], proof.as_bytes())
			.map_err(|e| Error::Custom(format!("failed to decode proof: {:?}", e)))?;
	let Token::Array(tokens) = tokens.remove(0) else {
		return Err(Error::Custom("invalid proof".to_string()))
	};
	let proof = tokens
		.into_iter()
		.map(|x| x.into_bytes())
		.collect::<Option<Vec<_>>>()
		.ok_or_else(|| Error::Custom("invalid proof (expected bytes)".to_string()))?;
	let key = keccak256(path.into().into_bytes());
	let option =
		value
			.map(|x| if x.len() <= H256::len_bytes() { x.to_vec() } else { keccak256(x).to_vec() });
	let value_hash = option.as_ref().map(|x| x.as_slice());
	let root = H256::from_slice(root.as_bytes());
	let res = verify_proof::<EIP1186Layout<KeccakHasher>>(&root, &proof, &key, value_hash);

	match res {
		Ok(_) => Ok(true),
		Err(err) => match err {
			NonExistingValue(_) | ExistingValue(_) | ValueMismatch(_) | HashMismatch(_) =>
				Ok(false),
			e @ IncompleteProof | e @ DecodeError(_) | e @ HashDecodeError(_) =>
				return Err(Error::Custom(format!("{e}"))),
		},
	}
}
