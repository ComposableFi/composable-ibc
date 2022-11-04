use super::CosmosClient;
use crate::error::Error;
use primitives::KeyProvider;
use std::str::FromStr;

impl<H> KeyProvider for CosmosClient<H> {
	fn account_id(&self) -> ibc::signer::Signer {
		ibc::signer::Signer::from_str(&self.key_name.as_str())
			.map_err(|_| Error::Custom("Invalid key name".to_string()))
			.unwrap()
	}
}
