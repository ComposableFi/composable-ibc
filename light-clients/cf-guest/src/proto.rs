use ibc_proto::google::protobuf::Any;
use prost::Message as _;

mod pb {
	include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

pub use pb::lightclients::guest::v1::{
	ClientMessage, ClientState, ConsensusState, Header, Misbehaviour, Signature,
};

/// Error during decoding of a protocol message.
#[derive(Clone, PartialEq, Eq, derive_more::From)]
pub enum DecodeError {
	/// Failed decoding the wire encoded protocol message.
	///
	/// This means that the supplied bytes weren’t a valid protocol buffer or
	/// they didn’t correspond to the expected message.
	BadProto(prost::DecodeError),

	/// Protocol message represents invalid state; see [`BadMessage`].
	#[from(ignore)]
	BadMessage,

	/// When decoding an `Any` message, the type URL doesn’t equal the expected
	/// one.
	#[from(ignore)]
	BadType,
}

/// Error during validation of a protocol message.
///
/// Typing in protocol messages is less descriptive than in Rust.  It’s possible
/// to represent state in the protocol message which doesn’t correspond to
/// a valid state.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BadMessage;

impl From<BadMessage> for DecodeError {
	fn from(_: BadMessage) -> Self {
		Self::BadMessage
	}
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

macro_rules! impl_proto {
	($Msg:ident; $test:ident; $test_object:expr) => {
		impl pb::lightclients::guest::v1::$Msg {
			/// Type URL of the type as used in Any protocol message.
			///
			/// This is the same value as returned by [`prost::Name::type_url`]
			/// however it’s a `const` and is set at compile time.  (In current
			/// Prost implementation, `type_url` method computes the URL at
			/// run-time).

			// "/ibc.lightclients.wasm.v1.ClientState"

			pub const TYPE_URL: &'static str =
				concat!("/lightclients.guest.v1.", stringify!($Msg));

			/// An example test message.
			#[cfg(test)]
			pub fn test() -> Self {
				$test_object
			}
		}

		impl From<$Msg> for Any {
			fn from(msg: $Msg) -> Self {
				Self::from(&msg)
			}
		}

		impl From<&$Msg> for Any {
			fn from(msg: &$Msg) -> Self {
				Self { type_url: $Msg::TYPE_URL.into(), value: msg.encode_to_vec() }
			}
		}

		impl TryFrom<Any> for $Msg {
			type Error = DecodeError;
			fn try_from(any: Any) -> Result<Self, Self::Error> {
				Self::try_from(&any)
			}
		}

		impl TryFrom<&Any> for $Msg {
			type Error = DecodeError;
			fn try_from(any: &Any) -> Result<Self, Self::Error> {
				if Self::TYPE_URL == any.type_url {
					Ok($Msg::decode(any.value.as_slice())?)
				} else {
					Err(DecodeError::BadType)
				}
			}
		}

		#[test]
		fn $test() {
			use alloc::format;

			// use prost::Name;

			// // Make sure TYPE_URL we set by hand matches type_url which is
			// // derived.
			// assert_eq!($Msg::type_url(), $Msg::TYPE_URL);

			// Check round-trip conversion through Any.
			let state = $Msg::test();
			let mut any = Any::try_from(&state).unwrap();
			assert_eq!(Ok(state), $Msg::try_from(&any));

			// Check type verifyication
			any.type_url = "bogus".into();
			assert_eq!(Err(DecodeError::BadType), $Msg::try_from(&any));

			// Check ProtoBuf encoding.
			if !cfg!(miri) {
				insta::assert_debug_snapshot!(any.value);
			}
		}
	};
}

impl_proto!(ClientState; test_client_state; Self {
	genesis_hash: lib::hash::CryptoHash::test(24).to_vec(),
	latest_height: 8,
	epoch_commitment: lib::hash::CryptoHash::test(11).to_vec(),
	is_frozen: false,
	trusting_period_ns: 30 * 24 * 3600 * 1_000_000_000,
});

impl_proto!(ConsensusState; test_consensus_state; {
	let block_hash = lib::hash::CryptoHash::test(42).to_vec();
	Self { block_hash, timestamp_ns: 1 }
});

impl_proto!(Header; test_header; {
	// TODO(mina86): Construct a proper signed header.
	Self {
		genesis_hash: alloc::vec![0; 32],
		block_header: alloc::vec![1; 10],
		epoch: alloc::vec![2; 10],
		signatures: alloc::vec![],
	}
});

impl_proto!(Signature; test_signature; Self {
	index: 1,
	signature: alloc::vec![0; 64],
});

impl_proto!(Misbehaviour; test_misbehaviour; Self {
	header1: Some(Header::test()),
	header2: Some(Header::test()),
});

impl_proto!(ClientMessage; test_client_message; Self{
	message: Some(pb::lightclients::guest::v1::client_message::Message::Header(Header {
		genesis_hash: alloc::vec![0; 32],
		block_header: alloc::vec![1; 10],
		epoch: alloc::vec![2; 10],
		signatures: alloc::vec![],
	}))
});
