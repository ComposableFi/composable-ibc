use primitives::KeyProvider;
use std::str::FromStr;

use crate::client::EthereumClient;

impl KeyProvider for EthereumClient {
	fn account_id(&self) -> pallet_ibc::Signer {
		// TODO:
		pallet_ibc::Signer::from_str(&self.config.name).unwrap()
	}
}
