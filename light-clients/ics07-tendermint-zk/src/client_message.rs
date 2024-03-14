//Coppied from composable package ics07-tendermint/src/client_message.rs

use ibc::core::ics02_client;
use ibc_proto::{
	google::protobuf::Any, ibc::lightclients::tendermint::v1::ZkHeader as RawZkHeader,
};
use ics07_tendermint::{client_message::Misbehaviour, error::Error};
use prost::Message;
use serde::{Deserialize, Serialize};
use tendermint::{block::signed_header::SignedHeader, validator::Set as ValidatorSet};

use bytes::Buf;
use ibc::{core::ics24_host::identifier::ChainId, Height};
use tendermint_proto::Protobuf;

pub const TENDERMINT_HEADER_TYPE_URL: &str = "/ibc.lightclients.tendermintzk.v1.Header";
pub const TENDERMINT_MISBEHAVIOUR_TYPE_URL: &str = "/ibc.lightclients.tendermintzk.v1.Misbehaviour";
pub const TENDERMINT_CLIENT_MESSAGE_TYPE_URL: &str =
	"/ibc.lightclients.tendermintzk.v1.ClientMessage";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZkClientMessage {
	Header(ZkHeader),
	Misbehaviour(Misbehaviour),
}

impl ics02_client::client_message::ClientMessage for ZkClientMessage {
	fn encode_to_vec(&self) -> Result<Vec<u8>, tendermint_proto::Error> {
		self.encode_vec()
	}
}

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ZkHeader {
	pub signed_header: SignedHeader, // contains the commitment root
	pub validator_set: ValidatorSet, // the validator set that signed Header
	pub trusted_height: Height,      /* the height of a trusted header seen by client less than
	                                  * or equal to Header */
	// TODO(thane): Rename this to trusted_next_validator_set?
	pub trusted_validator_set: ValidatorSet, // the last trusted validator set at trusted height
	pub zk_proof: Vec<u8>,
	pub zk_bitmask: u64,
}

impl ZkHeader {
	pub fn height(&self) -> Height {
		Height::new(
			ChainId::chain_version(self.signed_header.header.chain_id.as_str()),
			u64::from(self.signed_header.header.height),
		)
	}
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
			zk_proof: raw.zk_proof.iter().map(|x| x.clone()).collect(),
			zk_bitmask: raw.zk_bitmask,
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
			zk_proof: value.zk_proof.into(),
			zk_bitmask: value.zk_bitmask,
		}
	}
}

impl Protobuf<Any> for ZkClientMessage {}

impl TryFrom<Any> for ZkClientMessage {
	type Error = Error;

	fn try_from(any: Any) -> Result<Self, Self::Error> {
		let msg = match &*any.type_url {
			TENDERMINT_HEADER_TYPE_URL => Self::Header(
				ZkHeader::decode(&*any.value).map_err(|e| Error::validation(format!("{e:?}")))?,
			),
			TENDERMINT_MISBEHAVIOUR_TYPE_URL => Self::Misbehaviour(
				Misbehaviour::decode(&*any.value)
					.map_err(|e| Error::validation(format!("{e:?}")))?,
			),
			_ => Err(Error::validation(format!("Unknown type: {}", any.type_url)))?,
		};

		Ok(msg)
	}
}

impl From<ZkClientMessage> for Any {
	fn from(msg: ZkClientMessage) -> Self {
		match msg {
			ZkClientMessage::Header(header) => Any {
				value: header.encode_vec().expect("failed to encode ClientMessage.header"),
				type_url: TENDERMINT_HEADER_TYPE_URL.to_string(),
			},
			ZkClientMessage::Misbehaviour(misbheaviour) => Any {
				value: misbheaviour
					.encode_vec()
					.expect("failed to encode ClientMessage.misbehaviour"),
				type_url: TENDERMINT_MISBEHAVIOUR_TYPE_URL.to_string(),
			},
		}
	}
}
