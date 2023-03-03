use super::key_provider::KeyEntry;
use crate::error::Error;
use ibc::core::ics24_host::identifier::ChainId;
use ibc_proto::{
	cosmos::tx::v1beta1::{
		mode_info::{Single, Sum},
		AuthInfo, Fee, ModeInfo, SignDoc, SignerInfo, TxBody, TxRaw,
	},
	google::protobuf::Any,
};
use k256::ecdsa::{signature::Signer as _, Signature, SigningKey};
use prost::Message;

pub fn encode_key_bytes(key: &KeyEntry) -> Result<Vec<u8>, Error> {
	let mut pk_buf = Vec::new();
	Message::encode(&key.public_key.public_key().to_bytes().as_slice().to_vec(), &mut pk_buf)
		.map_err(|e| Error::from(e.to_string()))?;
	Ok(pk_buf)
}

pub fn encode_signer_info(sequence: u64, key_bytes: Vec<u8>) -> Result<SignerInfo, Error> {
	let pk_any = Any { type_url: "/cosmos.crypto.secp256k1.PubKey".to_string(), value: key_bytes };
	let single = Single { mode: 1 };
	let sum_single = Some(Sum::Single(single));
	let mode = Some(ModeInfo { sum: sum_single });
	let signer_info = SignerInfo { public_key: Some(pk_any), mode_info: mode, sequence };
	Ok(signer_info)
}

pub fn encode_auth_info(signer_info: SignerInfo, fee: Fee) -> Result<(AuthInfo, Vec<u8>), Error> {
	let auth_info = AuthInfo { signer_infos: vec![signer_info], fee: Some(fee), tip: None };
	let mut auth_info_bytes = Vec::new();
	Message::encode(&auth_info, &mut auth_info_bytes).map_err(|e| Error::from(e.to_string()))?;

	Ok((auth_info, auth_info_bytes))
}

pub fn encode_sign_doc(
	key: KeyEntry,
	body_bytes: Vec<u8>,
	auth_info_bytes: Vec<u8>,
	chain_id: ChainId,
	account_number: u64,
) -> Result<Vec<u8>, Error> {
	let sign_doc =
		SignDoc { body_bytes, auth_info_bytes, chain_id: chain_id.to_string(), account_number };

	// A protobuf serialization of a SignDoc
	let mut signdoc_buf = Vec::new();
	Message::encode(&sign_doc, &mut signdoc_buf)?;

	// Create signature
	let private_key_bytes = key.private_key.private_key().to_bytes();
	let signing_key = SigningKey::from_bytes(private_key_bytes.as_slice())
		.map_err(|e| Error::from(e.to_string()))?;
	let signature: Signature = signing_key.sign(&signdoc_buf);
	let signature_bytes = signature.as_ref().to_vec();

	Ok(signature_bytes)
}

pub fn encode_tx_body(messages: Vec<Any>) -> Result<(TxBody, Vec<u8>), Error> {
	let body = TxBody {
		messages,
		memo: "ibc".to_string(),
		timeout_height: 0_u64,
		extension_options: Vec::<Any>::default(), // TODO: Check if this is correct
		non_critical_extension_options: Vec::<Any>::default(),
	};
	let mut body_bytes = Vec::new();
	Message::encode(&body, &mut body_bytes).map_err(|e| Error::from(e.to_string()))?;

	Ok((body, body_bytes))
}

pub fn encode_tx(
	body_bytes: Vec<u8>,
	auth_info_bytes: Vec<u8>,
	signature_bytes: Vec<u8>,
) -> Result<(TxRaw, Vec<u8>), Error> {
	// Create and Encode TxRaw
	let tx_raw = TxRaw { body_bytes, auth_info_bytes, signatures: vec![signature_bytes] };
	let mut tx_bytes = Vec::new();
	Message::encode(&tx_raw, &mut tx_bytes).map_err(|e| Error::from(e.to_string()))?;

	Ok((tx_raw, tx_bytes))
}
