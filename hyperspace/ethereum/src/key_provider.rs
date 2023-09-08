use primitives::KeyProvider;
use std::str::FromStr;

use crate::client::EthereumClient;

impl KeyProvider for EthereumClient {
	fn account_id(&self) -> pallet_ibc::Signer {
		// TODO:
		pallet_ibc::Signer::from_str("0x73db010c3275EB7a92E5C38770316248f4C644ee").unwrap()
	}
}
