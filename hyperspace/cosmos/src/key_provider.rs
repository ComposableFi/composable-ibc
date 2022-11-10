use super::CosmosClient;
use crate::error::Error;
use bech32::{ToBase32, Variant};
use bitcoin::{
	hashes::hex::ToHex,
	util::bip32::{ExtendedPrivKey, ExtendedPubKey},
};
use ibc::core::ics24_host::identifier::ChainId;
use primitives::KeyProvider;
use serde::{Deserialize, Serialize};
use std::{path::Path, str::FromStr};
use tendermint::account::Id as AccountId;

pub const KEYSTORE_DEFAULT_FOLDER: &str = ".hermes/keys/";  //TODO: Should be adjusted to the correct path later
pub const KEYSTORE_DISK_BACKEND: &str = "keyring-test";
pub const KEYSTORE_FILE_EXTENSION: &str = "json";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyEntry {
	/// Public key
	pub public_key: ExtendedPubKey,

	/// Private key
	pub private_key: ExtendedPrivKey,

	/// Account Bech32 format - TODO allow hrp
	pub account: String,

	/// Address
	pub address: Vec<u8>,
}

impl KeyEntry {
	pub fn new(key_name: &str, chain_id: &ChainId) -> Result<KeyEntry, Error> {
		let home = dirs_next::home_dir().unwrap();
		let keys_folder = Path::new(home.as_path())
			.join(KEYSTORE_DEFAULT_FOLDER)
			.join(chain_id.as_str())
			.join(KEYSTORE_DISK_BACKEND);
		let mut key_file = keys_folder.join(key_name);
		key_file.set_extension(KEYSTORE_FILE_EXTENSION);

		if !key_file.as_path().exists() {
			return Err(Error::from(format!("Key file {} does not exist", key_file.display())));
		}

		let file = std::fs::File::open(&key_file)
			.map_err(|e| Error::from(format!("Could not open key file {}", e)))?;

		let key_entry = serde_json::from_reader(file)
			.map_err(|e| Error::from(format!("Could not deserialize key file {}", e)))?;

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
