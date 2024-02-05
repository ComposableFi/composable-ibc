use crate::{
	abi::TendermintClientAbi,
	alloc::string::ToString,
	error::Error,
	proof::ethereum_trie::{verify_proof, EIP1186Layout, KeccakHasher, VerifyError},
	utils::keccak256,
};
use alloc::{borrow::Cow, boxed::Box, format, vec::Vec};
use alloy_sol_types::SolValue;
use ethabi::{Address, ParamType, Token};
use ibc::core::{
	ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
	ics24_host::{
		path::{ClientConsensusStatePath, ClientStatePath},
		Path,
	},
};
use ibc_proto::google::protobuf::Any;
use primitive_types::H256;
use prost::Message;
use tendermint_proto::Protobuf;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[must_use]
pub enum Verified {
	Yes,
	No,
}

#[cfg(feature = "no_beacon")]
pub fn verify_ibc_proof<P>(
	_prefix: &CommitmentPrefix,
	_proof: &CommitmentProofBytes,
	_root: &CommitmentRoot,
	_contract_address: Address,
	_path: P,
	_value: Option<Cow<[u8]>>,
) -> Result<Verified, Error>
where
	P: Into<Path>,
{
	return Ok(Verified::Yes);
}

#[cfg(not(feature = "no_beacon"))]
pub fn verify_ibc_proof<P>(
	prefix: &CommitmentPrefix,
	proof: &CommitmentProofBytes,
	root: &CommitmentRoot,
	contract_address: Address,
	path: P,
	mut value: Option<Cow<[u8]>>,
) -> Result<Verified, Error>
where
	P: Into<Path>,
{
	use crate::utils;
	use VerifyError::*;

	let mut tokens = ethabi::decode(
		&[
			ParamType::Array(Box::new(ParamType::Bytes)),
			ParamType::Array(Box::new(ParamType::Bytes)),
		],
		proof.as_bytes(),
	)
	.map_err(|e| Error::Custom(format!("failed to decode proof: {:?}", e)))?;
	let Some(Token::Array(storage_proof_tokens)) = tokens.pop() else {
		return Err(Error::Custom("invalid proof".to_string()))
	};
	let Some(Token::Array(account_proof_toks)) = tokens.pop() else {
		return Err(Error::Custom("invalid proof".to_string()))
	};
	let storage_proof = storage_proof_tokens
		.into_iter()
		.map(|x| x.into_bytes())
		.collect::<Option<Vec<_>>>()
		.ok_or_else(|| Error::Custom("invalid proof (expected bytes)".to_string()))?;
	let account_proof = account_proof_toks
		.into_iter()
		.map(|x| x.into_bytes())
		.collect::<Option<Vec<_>>>()
		.ok_or_else(|| Error::Custom("invalid proof (expected bytes)".to_string()))?;

	let state_root = H256::from_slice(root.as_bytes());
	let path = path.into();
	if value.is_some() {
		match &path {
			Path::ClientState(ClientStatePath(client_id)) => {
				if client_id.as_str().starts_with("07-tendermint-") {
					let any = Any::decode(value.as_ref().unwrap().as_ref())
						.map_err(|e| Error::Custom(format!("failed to decode any: {:?}", e)))?;
					let tendermint_state =
						ics07_tendermint::client_state::ClientState::<()>::decode(&*any.value)
							.map_err(|e| {
								Error::Custom(format!("failed to decode client state: {:?}", e))
							})?;
					let eth_tendermint_state =
						TendermintClientAbi::ClientStateData::from(tendermint_state);
					let encoded = eth_tendermint_state.abi_encode();
					value = Some(Cow::Owned(encoded.to_vec()));
				}
			},
			Path::ClientConsensusState(ClientConsensusStatePath { client_id, .. }) => {
				if client_id.as_str().starts_with("07-tendermint-") {
					let any = Any::decode(value.as_ref().unwrap().as_ref())
						.map_err(|e| Error::Custom(format!("failed to decode any: {:?}", e)))?;
					let tendermint_state =
						ics07_tendermint::consensus_state::ConsensusState::decode(&*any.value)
							.map_err(|e| {
								Error::Custom(format!("failed to decode client state: {:?}", e))
							})?;
					let eth_tendermint_state =
						TendermintClientAbi::ConsensusStateData::from(tendermint_state);
					let encoded = eth_tendermint_state.abi_encode();
					value = Some(Cow::Owned(encoded.to_vec()));
				}
			},
			_ => {},
		}
	}
	let string = path.to_string();

	let storage_key = utils::commitment_storage_raw_key(string.as_str(), prefix.as_bytes());

	let stream = rlp::Rlp::new(&account_proof.last().unwrap());
	let storage_root: Vec<u8> = stream.val_at(1).unwrap();
	let stream = rlp::Rlp::new(&storage_root);
	let storage_hash: Vec<u8> = stream.val_at(2).unwrap();
	let storage_hash = H256::from_slice(&storage_hash);

	let key = keccak256(&contract_address).to_vec();
	let res = verify_proof::<EIP1186Layout<KeccakHasher>>(
		&state_root,
		&account_proof,
		&key,
		Some(&account_proof.last().unwrap()),
	);

	match res {
		Ok(_) => (),
		Err(err) => match err {
			NonExistingValue(_) | ExistingValue(_) | ValueMismatch(_) | HashMismatch(_) =>
				return Ok(Verified::No),
			e @ IncompleteProof | e @ DecodeError(_) | e @ HashDecodeError(_) =>
				return Err(Error::Custom(format!("{e}"))),
		},
	};

	let key = keccak256(storage_key).to_vec();
	let opt_value = value.map(|v| {
		let hash = keccak256(v);
		let mut hash_ptr = hash.as_slice();
		while hash_ptr.len() > 0 {
			if hash_ptr[0] == 0 {
				hash_ptr = &hash_ptr[1..];
			} else {
				break;
			}
		}
		rlp::encode(&hash_ptr).to_vec()
	});
	let res = verify_proof::<EIP1186Layout<KeccakHasher>>(
		&storage_hash,
		&storage_proof,
		&key,
		opt_value.as_deref(),
	);

	match res {
		Ok(_) => Ok(Verified::Yes),
		Err(err) => match err {
			NonExistingValue(_) | ExistingValue(_) | ValueMismatch(_) | HashMismatch(_) =>
				Ok(Verified::No),
			e @ IncompleteProof | e @ DecodeError(_) | e @ HashDecodeError(_) =>
				return Err(Error::Custom(format!("{e}"))),
		},
	}
}
