use primitives::KeyProvider;

use crate::client::EthereumClient;

impl KeyProvider for EthereumClient {
	fn account_id(&self) -> pallet_ibc::Signer {
		todo!("return public key in string")
	}
}
