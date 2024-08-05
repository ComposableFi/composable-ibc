use alloc::string::ToString;

mod pb {
	include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

pub use pb::lightclients::cf_solana::v1::*;

macro_rules! define_proto {
	($Msg:ident; $test:ident; $test_object:expr) => {
		proto_utils::define_message! {
			pub use pb::lightclients::cf_solana::v1::$Msg as $Msg;
			$test $test_object;
		}
	};
}

define_proto!(ClientState; test_client_state; Self {
	genesis_hash: lib::hash::CryptoHash::test(24).to_vec(),
	latest_height: 8,
	epoch_commitment: lib::hash::CryptoHash::test(11).to_vec(),
	prev_epoch_commitment: lib::hash::CryptoHash::test(12).to_vec(),
	is_frozen: false,
	trusting_period_ns: 30 * 24 * 3600 * 1_000_000_000,
});

define_proto!(ConsensusState; test_consensus_state; {
	let block_hash = lib::hash::CryptoHash::test(42).to_vec();
	Self { block_hash, timestamp_ns: 1 }
});

define_proto!(ClientMessage; test_client_message; Header::test().into());

define_proto!(Header; test_header; {
	// TODO(mina86): Construct a proper signed header.
	Self {
		shreds: vec![
			Shred {
				data: vec![1; 10],
				slot: 1,
				index: 1,
				parent: vec![0; 32],
				..Default::default()
			},
		],
	}
});

define_proto!(Misbehaviour; test_misbehaviour; { panic!() });

// import_proto!(ClientMessage);
// import_proto!(ClientState);
// import_proto!(ConsensusState);
// import_proto!(Header);
// import_proto!(Misbehaviour);
// import_proto!(Signature);

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

impl From<Header> for ClientMessage {
	#[inline]
	fn from(msg: Header) -> Self {
		Self { message: Some(client_message::Message::Header(msg)) }
	}
}

impl From<Misbehaviour> for ClientMessage {
	#[inline]
	fn from(msg: Misbehaviour) -> Self {
		Self { message: Some(client_message::Message::Misbehaviour(msg)) }
	}
}
