use super::{error::Error, CosmosClient};
use crate::provider::TransactionId;
use futures::Stream;
use ibc_proto::{
	cosmos::tx::v1beta1::{
		mode_info::{Single, Sum},
		AuthInfo, ModeInfo, SignDoc, SignerInfo, TxBody, TxRaw,
	},
	google::protobuf::Any,
};
use k256::ecdsa::{signature::Signer as _, Signature, SigningKey};
use primitives::{Chain, IbcProvider};
use prost::Message;
use std::pin::Pin;
use tendermint_rpc::Client;

#[async_trait::async_trait]
impl<H> Chain for CosmosClient<H>
where
	H: Clone + Send + Sync + 'static,
{
	fn name(&self) -> &str {
		&*self.name
	}

	fn block_max_weight(&self) -> u64 {
		todo!()
	}

	async fn estimate_weight(&self, _messages: Vec<Any>) -> Result<u64, Self::Error> {
		todo!()
	}

	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>> {
		todo!()
	}

	async fn submit(&self, messages: Vec<Any>) -> Result<Self::TransactionId, Error> {
		// Create SignerInfo by encoding the keybase
		let mut pk_buf = Vec::new();
		Message::encode(&self.keybase.public_key.to_pub().to_bytes(), &mut pk_buf)
			.map_err(|e| Error::from(e.to_string()))?;

		let pk_type = "/cosmos.crypto.secp256k1.PubKey".to_string();
		let pk_any = Any { type_url: pk_type, value: pk_buf };
		let single = Single { mode: 1 };
		let sum_single = Some(Sum::Single(single));
		let mode = Some(ModeInfo { sum: sum_single });
		let signer_info =
			SignerInfo { public_key: Some(pk_any), mode_info: mode, sequence: 0 as u64 };

		// Create and Encode TxBody
		let body = TxBody {
			messages: messages.to_vec(),
			memo: "".to_string(), //TODO: Check if this is correct
			timeout_height: 0_u64,
			extension_options: Vec::<Any>::new(), //TODO: Check if this is correct
			non_critical_extension_options: Vec::<Any>::new(),
		};
		let mut body_bytes = Vec::new();
		Message::encode(&body, &mut body_bytes).map_err(|e| Error::from(e.to_string()))?;

		// Create and Encode AuthInfo
		let auth_info = AuthInfo { signer_infos: vec![signer_info], fee: None };
		let mut auth_info_bytes = Vec::new();
		Message::encode(&auth_info, &mut auth_info_bytes)
			.map_err(|e| Error::from(e.to_string()))?;

		// Create and Encode SignDoc
		let sign_doc = SignDoc {
			body_bytes: body_bytes.clone(),
			auth_info_bytes: auth_info_bytes.clone(),
			chain_id: self.chain_id.to_string(),
			account_number: self.query_account().await?.account_number,
		};
		let mut signdoc_buf = Vec::new();
		Message::encode(&sign_doc, &mut signdoc_buf).unwrap();

		// Create signature
		let private_key_bytes = self.keybase.private_key.to_priv().to_bytes();
		let signing_key = SigningKey::from_bytes(private_key_bytes.as_slice())
			.map_err(|e| Error::from(e.to_string()))?;
		let signature: Signature = signing_key.sign(&signdoc_buf);
		let signature_bytes = signature.as_ref().to_vec();

		// Create and Encode TxRaw
		let tx_raw = TxRaw { body_bytes, auth_info_bytes, signatures: vec![signature_bytes] };
		let mut tx_bytes = Vec::new();
		Message::encode(&tx_raw, &mut tx_bytes).map_err(|e| Error::from(e.to_string()))?;

		// Submit transaction
		let response = self
			.rpc_client
			.broadcast_tx_sync(tx_bytes.into())
			.await
			.map_err(|e| Error::from(format!("failed to broadcast transaction {:?}", e)))?;

		Ok(TransactionId { hash: response.hash })
	}
}
