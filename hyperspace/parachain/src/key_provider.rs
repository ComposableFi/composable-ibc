use std::str::FromStr;

use sp_core::crypto::Ss58Codec;
use sp_runtime::traits::IdentifyAccount;

use crate::config;
use primitives::KeyProvider;

use super::ParachainClient;

impl<T: config::Config> KeyProvider for ParachainClient<T> {
	fn account_id(&self) -> ibc::signer::Signer {
		let hex_string = self
			.public_key
			.clone()
			.into_account()
			.to_ss58check_with_version(self.ss58_version);

		ibc::signer::Signer::from_str(&hex_string).expect("Account Id should be valid")
	}
}
