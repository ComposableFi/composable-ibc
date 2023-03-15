use super::{
	encode::{
		encode_auth_info, encode_key_bytes, encode_sign_doc, encode_signer_info, encode_tx,
		encode_tx_body,
	},
	key_provider::KeyEntry,
};
use crate::error::Error;
use core::time::Duration;
use futures::TryFutureExt;
use ibc::core::ics24_host::identifier::ChainId;
use ibc_proto::{
	cosmos::{
		auth::v1beta1::BaseAccount,
		tx::v1beta1::{
			service_client::ServiceClient, Fee, SimulateRequest, SimulateResponse, Tx, TxRaw,
		},
	},
	google::protobuf::Any,
};
use prost::Message;
use tendermint::Hash;
use tendermint_rpc::{
	endpoint::tx::Response as TxResponse, query::Query, Client, HttpClient, Order, Url,
};

pub fn sign_tx(
	key: KeyEntry,
	chain_id: ChainId,
	account_info: &BaseAccount,
	messages: Vec<Any>,
	fee: Fee,
) -> Result<(Tx, TxRaw, Vec<u8>), Error> {
	let pk_bytes = encode_key_bytes(&key)?;
	let signer_info = encode_signer_info(account_info.sequence, pk_bytes)?;

	// Create and Encode AuthInfo
	let (auth_info, auth_info_bytes) = encode_auth_info(signer_info, fee)?;

	// Create and Encode TxBody
	let (body, body_bytes) = encode_tx_body(messages)?;

	// Create and Encode TxRaw
	let signature_bytes = encode_sign_doc(
		key.clone(),
		body_bytes.clone(),
		auth_info_bytes.clone(),
		chain_id.clone(),
		account_info.account_number,
	)?;

	// Encode SignDoc and Create Signature
	let (tx_raw, tx_bytes) = encode_tx(body_bytes, auth_info_bytes, signature_bytes.clone())?;

	let tx = Tx { body: Some(body), auth_info: Some(auth_info), signatures: vec![signature_bytes] };

	Ok((tx, tx_raw, tx_bytes))
}

pub async fn simulate_tx(
	grpc_url: Url,
	tx: Tx,
	tx_bytes: Vec<u8>,
) -> Result<SimulateResponse, Error> {
	#[allow(deprecated)]
	let req = SimulateRequest {
		tx: Some(tx), // needed for simulation to go through with Cosmos SDK <  0.43
		tx_bytes,     // needed for simulation to go through with Cosmos SDk >= 0.43
	};
	let mut client = ServiceClient::connect(grpc_url.clone().to_string())
		.await
		.map_err(|e| Error::from(e.to_string()))?;
	let request = tonic::Request::new(req);

	let response = tokio::time::timeout(
		Duration::from_secs(15),
		client.simulate(request).map_err(|e| Error::from(e.to_string())),
	)
	.await
	.map_err(|_| Error::from("simulation timeout".to_string()))??
	.into_inner();
	Ok(response)
}

pub async fn broadcast_tx(rpc_client: &HttpClient, tx_bytes: Vec<u8>) -> Result<Hash, Error> {
	let response = rpc_client
		.broadcast_tx_sync(tx_bytes)
		.await
		.map_err(|e| Error::from(format!("failed to broadcast transaction {:?}", e)))?;
	Ok(response.hash)
}

pub async fn confirm_tx(rpc_client: &HttpClient, tx_hash: Hash) -> Result<Hash, Error> {
	let start_time = tokio::time::Instant::now();
	let timeout = Duration::from_millis(30000);
	const WAIT_BACKOFF: Duration = Duration::from_millis(300);
	let response: TxResponse = loop {
		let response = rpc_client
			.tx_search(
				Query::eq("tx.hash", tx_hash.to_string()),
				false,
				1,
				1, // get only the first Tx matching the query
				Order::Ascending,
			)
			.await
			.map_err(|e| Error::from(format!("failed to search for transaction {:?}", e)))?;
		match response.txs.into_iter().next() {
			None => {
				let elapsed = start_time.elapsed();
				if &elapsed > &timeout {
					return Err(Error::from(format!(
						"transaction {} not found after {} seconds",
						tx_hash,
						elapsed.as_secs()
					)))
				} else {
					tokio::time::sleep(WAIT_BACKOFF).await;
				}
			},
			Some(response) => break response,
		}
	};

	let response_code = response.tx_result.code;
	if response_code.is_err() {
		return Err(Error::from(format!(
			"transaction {} failed with code {:?}",
			tx_hash, response_code
		)))
	}
	Ok(response.hash)
}

pub fn encoded_tx_metrics(
	key: KeyEntry,
	chain_id: ChainId,
	account_info: &BaseAccount,
	fee: Fee,
) -> Result<(usize, usize), Error> {
	let (_, tx_raw, _) = sign_tx(key, chain_id, account_info, vec![], fee)?;

	let total_len = tx_raw.encoded_len();
	let body_bytes_len = tx_raw.body_bytes.len();
	let envelope_len = if body_bytes_len == 0 {
		total_len
	} else {
		total_len - 1 - prost::length_delimiter_len(body_bytes_len) - body_bytes_len
	};

	Ok((total_len, envelope_len))
}
