use ethers::prelude::Signer as _;
use ibc::signer::Signer;
use primitives::KeyProvider;
use std::str::FromStr;

use crate::client::EthereumClient;

impl KeyProvider for EthereumClient {
	fn account_id(&self) -> Signer {
		Signer::from_str(&format!("{:?}", self.client().signer().address())).unwrap()
	}
}
