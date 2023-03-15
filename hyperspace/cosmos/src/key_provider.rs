use super::client::CosmosClient;
use bech32::{ToBase32, Variant};
use bip32::{XPrv as ExtendedPrivateKey, XPub as ExtendedPublicKey};
use primitives::{error::Error, KeyProvider};
use std::str::FromStr;
use tendermint::account::Id as AccountId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyEntry {
	/// Public key
	pub public_key: ExtendedPublicKey,
	/// Private key
	pub private_key: ExtendedPrivateKey,
	/// Account Bech32 format
	pub account: String,
	/// Address
	pub address: Vec<u8>,
}

impl KeyEntry {
	pub fn new(
		public_key: ExtendedPublicKey,
		private_key: ExtendedPrivateKey,
		account: String,
		address: Vec<u8>,
	) -> Self {
		Self { public_key, private_key, account, address }
	}
}

impl<H> KeyProvider for CosmosClient<H> {
	fn account_id(&self) -> ibc::signer::Signer {
		let key_entry = self.keybase.clone();
		let address = hex::encode(&key_entry.address);
		let account = AccountId::from_str(address.as_str())
			.map_err(|e| Error::from(format!("Could not parse account id {}", e)))
			.unwrap();
		let bech32 =
			bech32::encode(self.account_prefix.as_str(), account.to_base32(), Variant::Bech32)
				.map_err(|e| Error::from(format!("Could not encode account id {}", e)))
				.unwrap();
		let signer = bech32
			.parse()
			.map_err(|e| Error::from(format!("Could not parse account id {}", e)))
			.unwrap();
		signer
	}
}
