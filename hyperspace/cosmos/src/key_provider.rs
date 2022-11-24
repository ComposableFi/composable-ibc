use super::CosmosClient;
use crate::error::Error;
use bech32::{ToBase32, Variant};
use bitcoin::{
	hashes::hex::ToHex,
	util::bip32::{ExtendedPrivKey, ExtendedPubKey},
};
use primitives::KeyProvider;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};
use tendermint::account::Id as AccountId;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyEntry {
	/// Public key
	pub public_key: ExtendedPubKey,

	/// Private key
	pub private_key: ExtendedPrivKey,

	/// Account Bech32 format
	pub account: String,

	/// Address
	pub address: Vec<u8>,
}

impl KeyEntry {
	pub fn new(
		public_key: ExtendedPubKey,
		private_key: ExtendedPrivKey,
		account: String,
		address: Vec<u8>,
	) -> Self {
		Self { public_key, private_key, account, address }
	}

	pub fn from_file(path: PathBuf) -> Result<KeyEntry, Error> {
		if !path.as_path().exists() {
			return Err(Error::from(format!("Key file {} does not exist", path.display())))
		}

		let file = std::fs::File::open(&path)
			.map_err(|e| Error::from(format!("Could not open key file {:?}", e)))?;

		let key_entry = serde_json::from_reader(file)
			.map_err(|e| Error::from(format!("Could not deserialize key file {:?}", e)))?;

		Ok(key_entry)
	}
}

impl<H> KeyProvider for CosmosClient<H> {
	fn account_id(&self) -> ibc::signer::Signer {
		let key_entry = self.keybase.clone();
		let address = key_entry.address.to_hex();
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
