use primitives::KeyProvider;

use crate::client::Client;

impl KeyProvider for Client {
	fn account_id(&self) -> pallet_ibc::Signer {
		todo!()
	}
}
