// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// self, client_message, ClientMessage as RawClientMessage, Header as RawHeader,
// Misbehaviour as RawMisbehaviour,
use crate::{
	error::Error,
	proto::{
		client_message, ClientMessage as RawClientMessage, LightClientUpdate as RawHeader,
		Misbehaviour as RawMisbehaviour,
	},
};
use alloc::{string::ToString, vec::Vec};
use anyhow::anyhow;
use core::convert::Infallible;
use sync_committee_primitives::types::VerifierStateUpdate as LightClientUpdate;
use tendermint_proto::Protobuf;

/// Protobuf type url for GRANDPA header
pub const ETHEREUM_CLIENT_MESSAGE_TYPE_URL: &str = "/ibc.lightclients.ethereum.v1.ClientMessage";
pub const ETHEREUM_HEADER_TYPE_URL: &str = "/ibc.lightclients.ethereum.v1.Header";
pub const ETHEREUM_MISBEHAVIOUR_TYPE_URL: &str = "/ibc.lightclients.ethereum.v1.Misbehaviour";

#[derive(Clone, Debug)]
pub struct Header {
	pub inner: LightClientUpdate,
}

/// Misbehaviour type for GRANDPA. If both first and second proofs are valid
/// (that is, form a valid canonical chain of blocks where on of the chain is a fork of
/// the main one)
#[derive(Clone, Debug)]
pub struct Misbehaviour {
	pub never: Infallible,
}

/// [`ClientMessage`] for Ics10-GRANDPA
#[derive(Clone, Debug)]
pub enum ClientMessage {
	/// This is the variant for header updates
	Header(Header),
	/// This is for submitting misbehaviors.
	Misbehaviour(Misbehaviour),
}

impl ibc::core::ics02_client::client_message::ClientMessage for ClientMessage {
	fn encode_to_vec(&self) -> Result<Vec<u8>, tendermint_proto::Error> {
		self.encode_vec()
	}
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
	type Error = Error;

	fn try_from(raw_header: RawHeader) -> Result<Self, Self::Error> {
		let attested_header = raw_header
			.attested_header
			.ok_or_else(|| Error::Custom("'attested_header' is none".to_string()))?
			.try_into()?;
		let sync_committee_update =
			raw_header.sync_committee_update.map(TryInto::try_into).transpose()?;
		let finalized_header = raw_header
			.finalized_header
			.ok_or_else(|| Error::Custom("'finalized_header' is none".to_string()))?
			.try_into()?;
		let execution_payload = raw_header
			.execution_payload
			.ok_or_else(|| Error::Custom("'execution_payload' is none".to_string()))?
			.try_into()?;
		let finality_proof = raw_header
			.finality_proof
			.ok_or_else(|| Error::Custom("'finality_proof' is none".to_string()))?
			.try_into()?;
		let sync_aggregate = raw_header
			.sync_aggregate
			.ok_or_else(|| Error::Custom("'sync_aggregate' is none".to_string()))?
			.try_into()?;
		let signature_slot = raw_header.signature_slot;

		let header = Header {
			inner: LightClientUpdate {
				attested_header,
				sync_committee_update,
				finalized_header,
				execution_payload,
				finality_proof,
				sync_aggregate,
				signature_slot,
			},
		};
		Ok(header)
	}
}

impl From<Header> for RawHeader {
	fn from(header: Header) -> Self {
		let inner = header.inner;
		let attested_header = Some(inner.attested_header.into());
		let sync_committee_update = inner.sync_committee_update.map(Into::into);
		let finalized_header = Some(inner.finalized_header.into());
		let execution_payload = Some(inner.execution_payload.into());
		let finality_proof = Some(inner.finality_proof.into());
		let sync_aggregate = Some(inner.sync_aggregate.into());
		let signature_slot = inner.signature_slot;

		RawHeader {
			attested_header,
			sync_committee_update,
			finalized_header,
			execution_payload,
			finality_proof,
			sync_aggregate,
			signature_slot,
		}
	}
}

impl Protobuf<RawMisbehaviour> for Misbehaviour {}

impl TryFrom<RawMisbehaviour> for Misbehaviour {
	type Error = Error;

	fn try_from(_value: RawMisbehaviour) -> Result<Self, Self::Error> {
		unimplemented!("Misbehaviour is not supported yet")
	}
}

impl From<Misbehaviour> for RawMisbehaviour {
	fn from(value: Misbehaviour) -> Self {
		match value.never {}
	}
}

impl Protobuf<RawClientMessage> for ClientMessage {}

impl TryFrom<RawClientMessage> for ClientMessage {
	type Error = Error;

	fn try_from(raw_client_message: RawClientMessage) -> Result<Self, Self::Error> {
		let message = match raw_client_message
			.message
			.ok_or_else(|| anyhow!("Must supply either Header or Misbehaviour type!"))?
		{
			client_message::Message::Header(raw_header) =>
				ClientMessage::Header(Header::try_from(raw_header)?),
			client_message::Message::Misbehaviour(raw_misbehaviour) =>
				ClientMessage::Misbehaviour(Misbehaviour::try_from(raw_misbehaviour)?),
		};

		Ok(message)
	}
}

impl From<ClientMessage> for RawClientMessage {
	fn from(client_message: ClientMessage) -> Self {
		match client_message {
			ClientMessage::Header(header) =>
				RawClientMessage { message: Some(client_message::Message::Header(header.into())) },
			ClientMessage::Misbehaviour(misbehaviior) => RawClientMessage {
				message: Some(client_message::Message::Misbehaviour(misbehaviior.into())),
			},
		}
	}
}
