use crate::{Bytes, ContractError};
use cosmwasm_schema::{cw_serde, QueryResponses};
use ibc::{
	core::{
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
		ics24_host::{identifier::ClientId, Path},
	},
	protobuf::Protobuf,
};
use ics10_grandpa::{
	client_message::ClientMessage, client_state::ClientState, consensus_state::ConsensusState,
};

use std::str::FromStr;

use crate::types::Height;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
	ValidateMsg(ValidateMsg),
	StatusMsg(StatusMsg),
	ExportedMetadataMsg(ExportedMetadataMsg),
	ZeroCustomFieldsMsg(ZeroCustomFieldsMsg),
	GetTimestampAtHeightMsg(GetTimestampAtHeightMsg),
	InitializeMsg(InitializeMsg),
	VerifyMembershipMsg(VerifyMembershipMsgRaw),
	VerifyNonMembershipMsg(VerifyNonMembershipMsgRaw),
	VerifyClientMessage(VerifyClientMessageRaw),
	CheckForMisbehaviourMsg(CheckForMisbehaviourMsgRaw),
	UpdateStateOnMisbehaviourMsg(UpdateStateOnMisbehaviourMsgRaw),
	UpdateStateMsg(UpdateStateMsgRaw),
	CheckSubstituteAndUpdateStateMsg(CheckSubstituteAndUpdateStateMsg),
	VerifyUpgradeAndUpdateStateMsg(VerifyUpgradeAndUpdateStateMsgRaw),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
	#[returns(String)]
	ClientTypeMsg(ClientTypeMsg),
	#[returns(Height)]
	GetLatestHeightsMsg(GetLatestHeightsMsg),
}

// ClientState interface related messages
// Reference: https://github.com/cosmos/ibc-go/blob/main/modules/core/exported/client.go#L36
#[cw_serde]
pub struct ClientTypeMsg {}

#[cw_serde]
pub struct GetLatestHeightsMsg {}

#[cw_serde]
pub struct ValidateMsg {}

#[cw_serde]
pub struct StatusMsg {
	// how do we handle ctx sdk.Context, clientStore sdk.KVStore, cdc codec.BinaryCodec
}

#[cw_serde]
pub struct ExportedMetadataMsg {
	// clientStore sdk.KVStore
}

#[cw_serde]
pub struct ZeroCustomFieldsMsg {}

#[cw_serde]
pub struct GetTimestampAtHeightMsg {}

#[cw_serde]
pub struct InitializeMsg {}

#[cw_serde]
pub struct VerifyMembershipMsgRaw {
	pub proof: Vec<u8>,
	pub path: String,
	pub value: Vec<u8>,
}

pub struct VerifyMembershipMsg {
	pub prefix: CommitmentPrefix,
	pub proof: CommitmentProofBytes,
	pub root: CommitmentRoot,
	pub path: Path,
	pub value: Vec<u8>,
}

impl TryFrom<VerifyMembershipMsgRaw> for VerifyMembershipMsg {
	type Error = ContractError;

	fn try_from(raw: VerifyMembershipMsgRaw) -> Result<Self, Self::Error> {
		let _proof = CommitmentProofBytes::try_from(raw.proof)?;
		let _path = Path::from_str(&raw.path)?;
		let _value = raw.value;
		todo!()
	}
}

#[cw_serde]
pub struct VerifyNonMembershipMsgRaw {
	pub proof: Vec<u8>,
	pub path: String,
}

pub struct VerifyNonMembershipMsg {
	pub prefix: CommitmentPrefix,
	pub proof: CommitmentProofBytes,
	pub root: CommitmentRoot,
	pub path: Path,
}

impl TryFrom<VerifyNonMembershipMsgRaw> for VerifyNonMembershipMsg {
	type Error = ContractError;

	fn try_from(raw: VerifyNonMembershipMsgRaw) -> Result<Self, Self::Error> {
		let _proof = CommitmentProofBytes::try_from(raw.proof)?;
		let _path = Path::from_str(&raw.path)?;
		todo!();
	}
}

#[cw_serde]
pub struct VerifyClientMessageRaw {
	pub client_id: String,
	pub client_state: Bytes,
	pub client_message: Bytes,
}

pub struct VerifyClientMessage<H> {
	pub client_id: ClientId,
	pub client_state: ClientState<H>,
	pub client_message: ClientMessage,
}

impl<H: Clone> TryFrom<VerifyClientMessageRaw> for VerifyClientMessage<H> {
	type Error = ContractError;

	fn try_from(raw: VerifyClientMessageRaw) -> Result<Self, Self::Error> {
		let client_id = ClientId::from_str(&raw.client_id)?;
		let client_state = ClientState::decode_vec(&raw.client_state).unwrap();
		let client_message = ClientMessage::decode_vec(&raw.client_message).unwrap();
		Ok(Self { client_id, client_state, client_message })
	}
}

#[cw_serde]
pub struct CheckForMisbehaviourMsgRaw {
	pub client_id: String,
	pub client_state: Bytes,
	pub client_message: Bytes,
}

pub struct CheckForMisbehaviourMsg<H> {
	pub client_id: ClientId,
	pub client_state: ClientState<H>,
	pub client_message: ClientMessage,
}

impl<H: Clone> TryFrom<CheckForMisbehaviourMsgRaw> for CheckForMisbehaviourMsg<H> {
	type Error = ContractError;

	fn try_from(raw: CheckForMisbehaviourMsgRaw) -> Result<Self, Self::Error> {
		let client_id = ClientId::from_str(&raw.client_id)?;
		let client_state = ClientState::decode_vec(&raw.client_state)?;
		let client_message = ClientMessage::decode_vec(&raw.client_message)?;
		Ok(Self { client_id, client_state, client_message })
	}
}

#[cw_serde]
pub struct UpdateStateOnMisbehaviourMsgRaw {
	pub client_state: Bytes,
	pub client_message: Bytes,
}

pub struct UpdateStateOnMisbehaviourMsg<H> {
	pub client_state: ClientState<H>,
	pub client_message: ClientMessage,
}

impl<H: Clone> TryFrom<UpdateStateOnMisbehaviourMsgRaw> for UpdateStateOnMisbehaviourMsg<H> {
	type Error = ContractError;

	fn try_from(msg: UpdateStateOnMisbehaviourMsgRaw) -> Result<Self, Self::Error> {
		let client_state = ClientState::decode_vec(&msg.client_state)?;
		let client_message = ClientMessage::decode_vec(&msg.client_message)?;
		Ok(Self { client_state, client_message })
	}
}

#[cw_serde]
pub struct UpdateStateMsgRaw {
	pub client_id: String,
	pub client_state: Bytes,
	pub client_message: Bytes,
}

pub struct UpdateStateMsg<H> {
	pub client_id: ClientId,
	pub client_state: ClientState<H>,
	pub client_message: ClientMessage,
}

impl<H: Clone> TryFrom<UpdateStateMsgRaw> for UpdateStateMsg<H> {
	type Error = ContractError;

	fn try_from(raw: UpdateStateMsgRaw) -> Result<Self, Self::Error> {
		let client_id = ClientId::from_str(&raw.client_id)?;
		let client_state = ClientState::decode_vec(&raw.client_state)?;
		let client_message = ClientMessage::decode_vec(&raw.client_message)?;
		Ok(Self { client_id, client_state, client_message })
	}
}

#[cw_serde]
pub struct CheckSubstituteAndUpdateStateMsg {
	substitute_client_msg: Vec<u8>,
}

#[cw_serde]
pub struct VerifyUpgradeAndUpdateStateMsgRaw {
	pub client_id: String,
	pub old_client_state: Bytes,
	pub upgrade_client_state: Bytes,
	pub upgrade_consensus_state: Bytes,
	pub proof_upgrade_client: Vec<u8>,
	pub proof_upgrade_consensus_state: Vec<u8>,
}

pub struct VerifyUpgradeAndUpdateStateMsg<H> {
	pub client_id: ClientId,
	pub old_client_state: ClientState<H>,
	pub upgrade_client_state: ClientState<H>,
	pub upgrade_consensus_state: ConsensusState,
	pub proof_upgrade_client: Vec<u8>,
	pub proof_upgrade_consensus_state: Vec<u8>,
}

impl<H: Clone> TryFrom<VerifyUpgradeAndUpdateStateMsgRaw> for VerifyUpgradeAndUpdateStateMsg<H> {
	type Error = ContractError;

	fn try_from(value: VerifyUpgradeAndUpdateStateMsgRaw) -> Result<Self, Self::Error> {
		let client_id = ClientId::from_str(&value.client_id)?;
		let old_client_state = ClientState::decode_vec(&value.old_client_state)?;
		let upgrade_client_state = ClientState::decode_vec(&value.upgrade_client_state)?;
		let upgrade_consensus_state = ConsensusState::decode_vec(&value.upgrade_consensus_state)?;
		Ok(VerifyUpgradeAndUpdateStateMsg {
			client_id,
			old_client_state,
			upgrade_client_state,
			upgrade_consensus_state,
			proof_upgrade_client: value.proof_upgrade_client,
			proof_upgrade_consensus_state: value.proof_upgrade_consensus_state,
		})
	}
}
