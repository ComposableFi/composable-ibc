use anyhow::{anyhow, Error};
use std::{
	thread,
	time::{Duration, SystemTime},
};
use ureq;

pub struct ZKProver {
	prover_url: String,
}

#[derive(Debug, serde::Deserialize)]
struct Response {
	proof_id: String,
}

#[derive(Debug, serde::Deserialize)]
struct ResponseProofRequest {
	pub status: String,
	pub proof: Option<Vec<u8>>,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateProofInput {
	signatures: Vec<Vec<u8>>,
	msgs: Vec<Vec<u8>>,
	public_keys: Vec<Vec<u8>>,
}

impl ZKProver {
	pub fn new(prover_url: String, proof_timeout: Duration) -> Self {
		Self { prover_url, /* proof_timeout */}
	}

	pub fn create_proof(&self, proof_input: CreateProofInput) -> Result<Response, Error> {
		let r= ureq::post(self.prover_url.as_str())
			.send_json(ureq::json!(proof_input))?
			.into_string();
        match r {
            Ok(r) => {
                let resp: Response = serde_json::from_str(&r)?;
                Ok(resp)
            },
            Err(e) => {
                Err(anyhow!("Error: {:?}", e))
            }
        }
	}

	fn poll_proof(&self, proof_id: &str) -> Result<Option<Vec<u8>>, Error> {
		let url = self.prover_url.clone() + "/get_proof";
		let resp = ureq::post(self.prover_url.as_str())
			.send_json(ureq::json!({
				"proof_id": proof_id
			}))?
			.into_string()?;
        let resp: ResponseProofRequest = serde_json::from_str(&resp)?;

		// TOOD: handle some edge cases
		match resp.status.as_str() {
			"COMPLETED" => Ok(Some(resp.proof.unwrap())),
			_ => Ok(None),
		}
	}
}