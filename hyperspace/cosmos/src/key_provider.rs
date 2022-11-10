use super::CosmosClient;
use crate::error::Error;
use primitives::KeyProvider;
use bech32::{ToBase32, Variant};
use bitcoin::{
	hashes::hex::ToHex,
	util::bip32::{ExtendedPrivKey, ExtendedPubKey},
};
use ibc::core::ics24_host::identifier::ChainId;
use serde::{Deserialize, Serialize};
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
        todo!()
	}
}
impl<H> KeyProvider for CosmosClient<H> {
	fn account_id(&self) -> pallet_ibc::Signer {
		todo!()
	}
}
