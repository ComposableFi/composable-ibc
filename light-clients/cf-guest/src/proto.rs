macro_rules! import_proto {
	($Msg:ident) => {
		$crate::wrap!(cf_guest_upstream::proto::$Msg as $Msg);
		$crate::wrap!(impl Default for $Msg);
		$crate::wrap!(impl display Debug for $Msg);

		impl prost::Message for $Msg {
			fn encode_raw<B: prost::bytes::BufMut>(&self, buf: &mut B) {
				prost_12::Message::encode_raw(self, buf)
			}

			fn merge_field<B: prost::bytes::Buf>(
				&mut self,
				tag: u32,
				wire_type: prost::encoding::WireType,
				buf: &mut B,
				ctx: prost::encoding::DecodeContext,
			) -> Result<(), prost::DecodeError> {
				// SAFETY: The types are identical in prost 0.11 and prost.12.
				let wire_type = unsafe {
					core::mem::transmute(wire_type as u8)
				};
				prost_12::Message::merge_field(self, tag, wire_type, buf, ctx)
			}

			fn encoded_len(&self) -> usize {
				prost_12::Message::encoded_len(&self)
			}

			fn clear(&mut self) {
				prost_12::Message::clear(&mut self.0)
			}
		}

		impl From<$Msg> for ibc_proto::google::protobuf::Any {
			fn from(msg: $Msg) -> Self {
				Self::from(&msg)
			}
		}

		impl From<&$Msg> for ibc_proto::google::protobuf::Any {
			fn from(msg: &$Msg) -> Self {
				let (url, value) = cf_guest_upstream::proto::AnyConvert::to_any(msg);
				Self { type_url: url.into(), value }
			}
		}

		impl TryFrom<ibc_proto::google::protobuf::Any> for $Msg {
			type Error = DecodeError;
			fn try_from(any: ibc_proto::google::protobuf::Any) -> Result<Self, Self::Error> {
				Self::try_from(&any)
			}
		}

		impl TryFrom<&ibc_proto::google::protobuf::Any> for $Msg {
			type Error = DecodeError;
			fn try_from(any: &ibc_proto::google::protobuf::Any) -> Result<Self, Self::Error> {
				<Self as cf_guest_upstream::proto::AnyConvert>::try_from_any(&any.type_url, &any.value)
			}
		}
	}
}

import_proto!(ClientMessage);
import_proto!(ClientState);
import_proto!(ConsensusState);
import_proto!(Header);
import_proto!(Misbehaviour);
import_proto!(Signature);

/// Error during decoding of a protocol message.
#[derive(Clone, PartialEq, Eq, derive_more::From)]
pub enum DecodeError {
	/// Failed decoding the wire encoded protocol message.
	///
	/// This means that the supplied bytes weren’t a valid protocol buffer or
	/// they didn’t correspond to the expected message.
	BadProto(alloc::string::String),

	/// Protocol message represents invalid state; see [`BadMessage`].
	#[from(ignore)]
	BadMessage,

	/// When decoding an `Any` message, the type URL doesn’t equal the expected
	/// one.
	#[from(ignore)]
	BadType,
}

impl From<cf_guest_upstream::DecodeError> for DecodeError {
	fn from(err: cf_guest_upstream::DecodeError) -> Self {
		match err {
			cf_guest_upstream::DecodeError::BadProto(err) => Self::BadProto(err.to_string()),
			cf_guest_upstream::DecodeError::BadMessage => Self::BadMessage,
			cf_guest_upstream::DecodeError::BadType => Self::BadType,
		}
	}
}

/// Error during validation of a protocol message.
///
/// Typing in protocol messages is less descriptive than in Rust.  It’s possible
/// to represent state in the protocol message which doesn’t correspond to
/// a valid state.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BadMessage;

impl From<cf_guest_upstream::BadMessage> for BadMessage {
	fn from(_: cf_guest_upstream::BadMessage) -> Self { Self }
}

impl From<BadMessage> for DecodeError {
	fn from(_: BadMessage) -> Self { Self::BadMessage }
}

impl core::fmt::Debug for DecodeError {
	fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> core::fmt::Result {
		match self {
			Self::BadProto(err) => err.fmt(fmtr),
			Self::BadMessage => fmtr.write_str("BadMessage"),
			Self::BadType => fmtr.write_str("BadType"),
		}
	}
}

impl core::fmt::Display for DecodeError {
	#[inline]
	fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> core::fmt::Result {
		core::fmt::Debug::fmt(self, fmtr)
	}
}

impl core::fmt::Display for BadMessage {
	#[inline]
	fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> core::fmt::Result {
		core::fmt::Debug::fmt(self, fmtr)
	}
}


impl From<Header> for ClientMessage {
	#[inline]
	fn from(msg: Header) -> Self {
		let msg = cf_guest_upstream::proto::ClientMessage::from(msg.0);
		Self { message: Some(msg) }
	}
}

impl From<Misbehaviour> for ClientMessage {
	#[inline]
	fn from(msg: Misbehaviour) -> Self {
		let msg = cf_guest_upstream::proto::ClientMessage::from(msg.0);
		Self { message: Some(msg) }
	}
}
