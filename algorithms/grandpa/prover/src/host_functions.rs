use primitives::HostFunctions;
use sp_core::ed25519::{Public, Signature};
use sp_runtime::{app_crypto::RuntimePublic, traits::BlakeTwo256};

pub struct HostFunctionsProvider;

impl HostFunctions for HostFunctionsProvider {
	type BlakeTwo256 = BlakeTwo256;

	fn ed25519_verify(sig: &Signature, msg: &[u8], pubkey: &Public) -> bool {
		pubkey.verify(&msg, sig)
	}
}
