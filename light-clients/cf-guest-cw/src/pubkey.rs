#[derive(
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Debug,
	core::hash::Hash,
	derive_more::From,
	derive_more::Into,
	borsh::BorshSerialize,
	borsh::BorshDeserialize,
)]
#[repr(transparent)]
pub struct PubKey([u8; 32]);

#[derive(
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Debug,
	core::hash::Hash,
	derive_more::From,
	derive_more::Into,
	borsh::BorshSerialize,
	borsh::BorshDeserialize,
)]
#[repr(transparent)]
pub struct Signature([u8; 64]);

impl guestchain::PubKey for PubKey {
	type Signature = Signature;

	fn to_vec(&self) -> Vec<u8> {
		self.0.to_vec()
	}
	fn from_bytes(bytes: &[u8]) -> Result<Self, guestchain::BadFormat> {
		bytes.try_into().map(Self).map_err(|_| guestchain::BadFormat)
	}
}

impl guestchain::Signature for Signature {
	fn to_vec(&self) -> Vec<u8> {
		self.0.to_vec()
	}
	fn from_bytes(bytes: &[u8]) -> Result<Self, guestchain::BadFormat> {
		bytes.try_into().map(Self).map_err(|_| guestchain::BadFormat)
	}
}

pub(crate) struct NullVerifier;

impl guestchain::Verifier<PubKey> for NullVerifier {
	fn verify(&self, message: &[u8], pubkey: &PubKey, signature: &Signature) -> bool {
		todo!()
	}
}
