// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use codec::Decode;
use primitives::KeyProvider;
use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::{
	app_crypto::CryptoTypePublicPair,
	traits::{IdentifyAccount, Verify},
	KeyTypeId, MultiSignature, MultiSigner,
};
use subxt::tx::Signer;

/// A [`Signer`] implementation.
#[derive(Clone)]
pub struct ExtrinsicSigner<T: light_client_common::config::Config, Provider: KeyProvider> {
	account_id: T::AccountId,
	signer: MultiSigner,
	key_store: SyncCryptoStorePtr,
	key_type_id: KeyTypeId,
	_phantom: std::marker::PhantomData<Provider>,
}

impl<T, P> ExtrinsicSigner<T, P>
where
	T: light_client_common::config::Config + Send + Sync,
	<<T as light_client_common::config::Config>::Signature as Verify>::Signer:
		From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	P: KeyProvider,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::AccountId: Send + Sync,
	<T as subxt::Config>::Address: Send + Sync,
	<T as subxt::Config>::Signature: Send + Sync,
{
	/// Creates a new [`Signer`] from a key store reference and key type
	pub fn new(
		key_store: SyncCryptoStorePtr,
		key_type_id: KeyTypeId,
		public_key: MultiSigner,
	) -> Self {
		let account_id =
			<<T as light_client_common::config::Config>::Signature as Verify>::Signer::from(
				public_key.clone(),
			)
			.into_account();
		Self {
			account_id,
			key_store,
			key_type_id,
			signer: MultiSigner::from(public_key),
			_phantom: Default::default(),
		}
	}
}

impl<T, P> Signer<T> for ExtrinsicSigner<T, P>
where
	T: light_client_common::config::Config + Send + Sync,
	T::AccountId: Into<<T as subxt::Config>::Address> + Clone + 'static,
	P: KeyProvider + 'static,
	<T as subxt::Config>::Signature: From<MultiSignature> + Send + Sync,
	<T as subxt::Config>::AccountId: Send + Sync,
	<T as subxt::Config>::Address: Send + Sync,
{
	fn account_id(&self) -> &T::AccountId {
		&self.account_id
	}

	fn address(&self) -> <T as subxt::Config>::Address {
		self.account_id.clone().into()
	}

	fn sign(&self, signer_payload: &[u8]) -> <T as subxt::Config>::Signature {
		let (crypto_type_id, public_key) = match &self.signer {
			MultiSigner::Ed25519(key) => (sp_core::ed25519::CRYPTO_ID, key.0.to_vec()),
			MultiSigner::Sr25519(key) => (sp_core::sr25519::CRYPTO_ID, key.0.to_vec()),
			MultiSigner::Ecdsa(key) => (sp_core::ecdsa::CRYPTO_ID, key.0.to_vec()),
		};
		let key = CryptoTypePublicPair(crypto_type_id, public_key);
		let encoded_sig =
			SyncCryptoStore::sign_with(&*self.key_store, self.key_type_id, &key, signer_payload)
				.ok()
				.flatten()
				.expect("Signing should not fail");
		let signature: MultiSignature = match self.signer {
			MultiSigner::Ed25519(_) => sp_core::ed25519::Signature::decode(&mut &encoded_sig[..])
				.expect("Should decode same signature type as public key; qed")
				.into(),
			MultiSigner::Sr25519(_) => sp_core::sr25519::Signature::decode(&mut &encoded_sig[..])
				.expect("Should decode same signature type as public key; qed")
				.into(),
			MultiSigner::Ecdsa(_) => sp_core::ecdsa::Signature::decode(&mut &encoded_sig[..])
				.expect("Should decode same signature type as public key; qed")
				.into(),
		};
		signature.into()
	}
}
