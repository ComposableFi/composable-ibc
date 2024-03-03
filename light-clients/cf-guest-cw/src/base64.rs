use alloc::borrow::Cow;

use prost::Message;

use crate::{ibc, state};

pub(crate) struct Base64;

impl Base64 {
	pub fn serialize<T: BytesConv, S: serde::ser::Serializer>(
		obj: &T,
		ser: S,
	) -> Result<S::Ok, S::Error> {
		use base64::engine::Engine;

		let bytes = obj.to_bytes()?;
		let bytes = bytes.as_ref();
		// Unfortunately because there’s no `<&Binary>::From<&Vec>` we
		// need to open-code base64 encoding.  TODO(mina86): Change it
		// once https://github.com/CosmWasm/cosmwasm/pull/2036 lands.
		ser.serialize_str(&base64::engine::general_purpose::STANDARD.encode(bytes))
	}

	pub fn deserialize<'de, T: BytesConv, D: serde::Deserializer<'de>>(
		de: D,
	) -> Result<T, D::Error> {
		serde::Deserialize::deserialize(de).and_then(T::from_bytes)
	}
}

pub(crate) trait BytesConv: Sized {
	fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E>;
	fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E>;
}

impl BytesConv for Vec<u8> {
	fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E> {
		Ok(Cow::Borrowed(self.as_slice()))
	}

	fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E> {
		Ok(bytes)
	}
}

impl BytesConv for ibc::CommitmentProofBytes {
	fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E> {
		Ok(Cow::Borrowed(self.as_ref()))
	}

	fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E> {
		Self::try_from(bytes).map_err(E::custom)
	}
}

macro_rules! conv_via_any {
	($msg:ty) => {
		impl BytesConv for $msg {
			fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E> {
				Ok(Cow::Owned(ibc::proto::Any::from(self).encode_to_vec()))
			}

			fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E> {
				let any = ibc::proto::Any::decode(bytes.as_slice()).map_err(E::custom)?;
				<$msg>::try_from(any).map_err(E::custom)
			}
		}
	};
}

conv_via_any!(state::ClientMessage);
conv_via_any!(state::ClientState);
conv_via_any!(state::ConsensusState);
