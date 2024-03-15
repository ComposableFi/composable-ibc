use borsh::maybestd::io;

/// Ed25519 public key (a.k.a. verifying key).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct PubKey(ed25519_dalek::VerifyingKey);

impl guestchain::PubKey for PubKey {
	type Signature = Signature;

	fn to_vec(&self) -> Vec<u8> {
		self.0.as_bytes().to_vec()
	}
	fn from_bytes(bytes: &[u8]) -> Result<Self, guestchain::BadFormat> {
		bytes.try_into().map(Self).map_err(|_| guestchain::BadFormat)
	}
}

impl borsh::BorshSerialize for PubKey {
	fn serialize<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
		wr.write_all(self.0.as_bytes())
	}
}

impl borsh::BorshDeserialize for PubKey {
	fn deserialize_reader<R: io::Read>(rd: &mut R) -> io::Result<Self> {
		let mut bytes = ed25519_dalek::pkcs8::PublicKeyBytes([0; 32]);
		rd.read_exact(&mut bytes.0[..])?;
		ed25519_dalek::VerifyingKey::try_from(bytes)
			.map(Self)
			.map_err(|_| io::Error::new(io::ErrorKind::Other, "malformed Ed25519 public key"))
	}
}

impl PartialOrd for PubKey {
	fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(rhs))
	}
}

impl Ord for PubKey {
	fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
		self.0.as_bytes().cmp(rhs.0.as_bytes())
	}
}

/// Ed25519 signature.
#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Signature(ed25519_dalek::Signature);

impl guestchain::Signature for Signature {
	fn to_vec(&self) -> Vec<u8> {
		self.0.to_vec()
	}
	fn from_bytes(bytes: &[u8]) -> Result<Self, guestchain::BadFormat> {
		ed25519_dalek::Signature::from_slice(bytes)
			.map(Self)
			.map_err(|_| guestchain::BadFormat)
	}
}

impl borsh::BorshSerialize for Signature {
	fn serialize<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
		wr.write_all(self.0.r_bytes())?;
		wr.write_all(self.0.s_bytes())?;
		Ok(())
	}
}

impl borsh::BorshDeserialize for Signature {
	fn deserialize_reader<R: io::Read>(rd: &mut R) -> io::Result<Self> {
		let mut buf = [0; 64];
		rd.read_exact(&mut buf[..])?;
		Ok(Self(ed25519_dalek::Signature::from_bytes(&buf)))
	}
}

impl core::hash::Hash for Signature {
	fn hash<H: core::hash::Hasher>(&self, hasher: &mut H) {
		hasher.write(self.0.r_bytes());
		hasher.write(self.0.s_bytes());
	}
}

impl PartialOrd for Signature {
	fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(rhs))
	}
}

impl Ord for Signature {
	fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
		let lhs = (self.0.r_bytes(), self.0.s_bytes());
		let rhs = (rhs.0.r_bytes(), rhs.0.s_bytes());
		lhs.cmp(&rhs)
	}
}

/// Verifier for Ed25519 signatures using ed25519-dalek implementation.
pub(crate) struct Verifier;

impl guestchain::Verifier<PubKey> for Verifier {
	fn verify(&self, message: &[u8], pubkey: &PubKey, signature: &Signature) -> bool {
		pubkey.0.verify_strict(message, &signature.0).is_ok()
	}
}
