use crate::{
	error::Error,
	proof::ethereum_trie::{verify_proof, EIP1186Layout, KeccakHasher, VerifyError},
};
use ethers_core::{
	abi,
	types::EIP1186ProofResponse,
	utils::{keccak256, rlp},
};
use ibc::core::{
	ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
	ics24_host::Path,
};
use primitive_types::H256;
use rlp::DecoderError;
use rs_merkle::Hasher;

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

	let eth_proof = serde_cbor::from_slice::<EIP1186ProofResponse>(proof.as_bytes()).unwrap();
	let key = keccak256(path.into().into_bytes());
	let value_hash = value.map(keccak256).map(|x| x.as_slice());
	let root = H256::from_slice(root.as_bytes());
	let res = verify_proof::<EIP1186Layout<KeccakHasher>>(
		&root,
		&eth_proof.account_proof.into_iter().map(|x| x.to_vec()).collect::<Vec<_>>(),
		&key,
		value_hash,
	);

	match res {
		Ok(_) => Ok(true),
		Err(err) => match err {
			NonExistingValue(_) | ExistingValue(_) | ValueMismatch(_) | HashMismatch(_) =>
				Ok(false),
			e @ IncompleteProof | e @ DecodeError(_) | e @ HashDecodeError(_) =>
				return Err(e.into()),
		},
	}
}
