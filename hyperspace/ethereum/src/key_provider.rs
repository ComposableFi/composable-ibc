use ethers::prelude::Signer;
use primitives::KeyProvider;
use std::str::FromStr;

use crate::client::EthereumClient;

impl KeyProvider for EthereumClient {
	fn account_id(&self) -> pallet_ibc::Signer {
		pallet_ibc::Signer::from_str(&format!("{:?}", self.client().signer().address())).unwrap()
	}
}
