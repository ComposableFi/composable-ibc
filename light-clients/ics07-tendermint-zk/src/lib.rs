use ics07_tendermint::client_message::{Header, Misbehaviour};
use ics07_tendermint::error::Error;
use serde::{Deserialize, Serialize};
use ibc_proto::{
	google::protobuf::Any,
	ibc::lightclients::tendermint::v1::{ZkHeader as RawZkHeader, Misbehaviour as RawMisbehaviour},
};
use prost::Message;
use tendermint::{
	block::{signed_header::SignedHeader, Commit, CommitSig},
	validator::Set as ValidatorSet,
	vote::{SignedVote, ValidatorIndex},
	Vote,
};
use ibc::Height;
use bytes::Buf;
use tendermint_proto::Protobuf;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientMessage {
	Header(ZkHeader),
	Misbehaviour(Misbehaviour),
}

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ZkHeader {
    pub signed_header: SignedHeader, // contains the commitment root
	pub validator_set: ValidatorSet, // the validator set that signed Header
	pub trusted_height: Height,      /* the height of a trusted header seen by client less than
	                                  * or equal to Header */
	// TODO(thane): Rename this to trusted_next_validator_set?
	pub trusted_validator_set: ValidatorSet, // the last trusted validator set at trusted height
}

impl core::fmt::Debug for ZkHeader {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, " Header {{...}}")
	}
}

impl Protobuf<RawZkHeader> for ZkHeader {}

impl TryFrom<RawZkHeader> for ZkHeader {
	type Error = Error;

	fn try_from(raw: RawZkHeader) -> Result<Self, Self::Error> {
		let header = Self {
			signed_header: raw
				.signed_header
				.ok_or_else(Error::missing_signed_header)?
				.try_into()
				.map_err(|e| {
				Error::invalid_header("signed header conversion".to_string(), e)
			})?,
			validator_set: raw
				.validator_set
				.ok_or_else(Error::missing_validator_set)?
				.try_into()
				.map_err(Error::invalid_raw_header)?,
			trusted_height: raw.trusted_height.ok_or_else(Error::missing_trusted_height)?.into(),
			trusted_validator_set: raw
				.trusted_validators
				.ok_or_else(Error::missing_trusted_validator_set)?
				.try_into()
				.map_err(Error::invalid_raw_header)?,
		};

		Ok(header)
	}
}

pub fn decode_header<B: Buf>(buf: B) -> Result<ZkHeader, Error> {
	RawZkHeader::decode(buf).map_err(Error::decode)?.try_into()
}

impl From<ZkHeader> for RawZkHeader {
	fn from(value: ZkHeader) -> Self {
		RawZkHeader {
			signed_header: Some(value.signed_header.into()),
			validator_set: Some(value.validator_set.into()),
			trusted_height: Some(value.trusted_height.into()),
			trusted_validators: Some(value.trusted_validator_set.into()),
		}
	}
}