use alloc::borrow::Cow;
use core::{fmt, marker::PhantomData, str::FromStr};

use cosmwasm_std::Binary;
use prost::Message;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{ibc, state};

/// A Serde serialisation implementation which encodes binary data as
/// base64-string (when serialising to human-readable form).
pub struct Base64;

/// A Serde serialisation implementation handling `Option<T>` values where `T`
/// can be serialised using [`Base64`].
pub struct MaybeBase64;

/// A Serde serialisation implementation which encodes object using
/// `Display` and deserialises using `FromStr`.
pub struct AsStr;

impl Base64 {
	pub fn serialize<T: BytesConv, S: Serializer>(obj: &T, ser: S) -> Result<S::Ok, S::Error> {
		let bytes = obj.to_bytes()?;
		Base64Bytes(bytes.as_ref()).serialize(ser)
	}

	pub fn deserialize<'de, T: BytesConv, D: Deserializer<'de>>(de: D) -> Result<T, D::Error> {
		T::from_bytes(Binary::deserialize(de)?.into())
	}
}

impl MaybeBase64 {
	pub fn serialize<T: BytesConv, S: Serializer>(
		obj: &Option<T>,
		ser: S,
	) -> Result<S::Ok, S::Error> {
		if let Some(ref obj) = obj {
			let bytes = obj.to_bytes()?;
			ser.serialize_some(&Base64Bytes(bytes.as_ref()))
		} else {
			ser.serialize_none()
		}
	}

	pub fn deserialize<'de, T: BytesConv, D: Deserializer<'de>>(
		de: D,
	) -> Result<Option<T>, D::Error> {
		match Option::<Binary>::deserialize(de)? {
			None => Ok(None),
			Some(bin) => T::from_bytes(bin.into()).map(Some),
		}
	}
}

/// Wrapper which serialised bytes slice using base64 encoding.
///
/// Unfortunately because there’s no `<&cosmwasm_std::Binary>::From<&Vec>`
/// implementation we cannot defer the implementation to `Binary` without making
/// a copy of the bytes.  Because of that we’re using our own implementation for
/// serialisation.
// TODO(mina86): Get rid of it once
// https://github.com/CosmWasm/cosmwasm/pull/2036 lands.
struct Base64Bytes<'a>(&'a [u8]);

impl Serialize for Base64Bytes<'_> {
	fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
		use base64::engine::{general_purpose::STANDARD, Engine};
		ser.serialize_str(&STANDARD.encode(self.0))
	}
}

/// Trait implementing conversion to and from bytes used by [`Base64`] and
/// [`MaybeBase64`].
pub trait BytesConv: Sized {
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
conv_via_any!(state::Header);
conv_via_any!(state::Misbehaviour);

impl AsStr {
	pub fn serialize<T: fmt::Display, S: Serializer>(obj: &T, ser: S) -> Result<S::Ok, S::Error> {
		ser.serialize_str(&obj.to_string())
	}

	pub fn deserialize<'de, T, E, D>(de: D) -> Result<T, D::Error>
	where
		T: FromStr<Err = E>,
		E: fmt::Display,
		D: Deserializer<'de>,
	{
		de.deserialize_str(AsStrVisitor::<T>::default())
	}
}

struct AsStrVisitor<T>(PhantomData<T>);

impl<T> Default for AsStrVisitor<T> {
	fn default() -> Self {
		Self(PhantomData)
	}
}

impl<'de, T, Err> serde::de::Visitor<'de> for AsStrVisitor<T>
where
	T: FromStr<Err = Err>,
	Err: fmt::Display,
{
	type Value = T;

	fn expecting(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
		write!(fmtr, "object formatted to string")
	}

	fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<T, E> {
		T::from_str(value).map_err(E::custom)
	}
}
