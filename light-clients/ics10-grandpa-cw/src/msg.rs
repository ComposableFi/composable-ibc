use crate::{Bytes, ContractError};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_binary, Binary, StdResult};
use ibc::{
	core::{
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
		ics24_host::{identifier::ClientId, Path},
	},
	protobuf::Protobuf,
};
use ibc_proto::{
	ibc::core::{client::v1::Height, commitment::v1::MerkleRoot},
	ics23::ProofSpec,
};
use ics10_grandpa::{
	client_message::{ClientMessage, Header},
	client_state::ClientState,
	consensus_state::ConsensusState,
	proto::{
		client_message::Message as MessageProto, ClientMessage as ClientMessageProto,
		Header as HeaderProto,
	},
};
use prost::Message;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

struct Base64;

impl Base64 {
	pub fn serialize<S: Serializer>(v: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
		ibc_proto::base64::serialize(v, serializer)
	}

	pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
		ibc_proto::base64::deserialize(deserializer)
	}
}

#[cw_serde]
pub struct WasmClientState {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub data: Bytes,
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub code_id: Bytes,
	pub latest_height: Height,
	pub proof_specs: Vec<ProofSpec>,
	pub repository: String,
}

#[cw_serde]
pub struct WasmHeader {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub data: Bytes,
	pub height: Height,
}

#[cw_serde]
pub struct WasmConsensusState {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub data: Bytes,
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub code_id: Bytes,
	pub timestamp: u64,
	pub root: MerkleRoot,
}

#[cw_serde]
pub struct ContractResult {
	pub is_valid: bool,
	pub error_msg: String,
}

impl ContractResult {
	pub fn success() -> Self {
		Self { is_valid: true, error_msg: "".to_string() }
	}

	pub fn error(msg: String) -> Self {
		Self { is_valid: false, error_msg: msg }
	}

	// pub fn into_binary(self) -> StdResult<Binary> {
	// 	Ok(to_binary(&self)?)
	// }
}

#[cw_serde]
pub struct ClientStateCallResponse {
	pub me: WasmClientState,
	pub new_consensus_state: WasmConsensusState,
	pub new_client_state: WasmClientState,
	pub result: ContractResult,
}

#[cw_serde]
pub struct InitializeState {
	pub me: WasmClientState,
	pub consensus_state: WasmConsensusState,
}

#[cw_serde]
pub struct InstantiateMsg {
	initialize_state: InitializeState,
}

#[cw_serde]
pub struct ClientCreateRequest {
	client_create_request: WasmConsensusState,
}

#[cw_serde]
pub enum ExecuteMsg {
	InitializeState(InitializeState),
	ClientCreateRequest(WasmClientState),
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
		let _prefix = todo!("how to get prefix?");
		let _root = todo!("how to get root?");
		Ok(Self { prefix: _prefix, proof: _proof, root: _root, path: _path, value: _value })
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
		let _prefix = todo!("how to get prefix?");
		let _root = todo!("how to get root?");
		Ok(Self { prefix: _prefix, proof: _proof, root: _root, path: _path })
	}
}

/*
{"verify_client_message":{
	"client_message":{"data":"AA==","height":{"revision_number":1,"revision_height":2}},
	"client_state":{"data":"AA==","code_id":"EyhxYATHme1WYM5IQjGjfTb88rxt1t0UejFAHfAc0C0=","latest_height":{"revision_number":1,"revision_height":2},"proof_specs":[{"leaf_spec":{"hash":1,"prehash_value":1,"length":3,"prefix":"AA=="},"inner_spec":{"child_order":[0,1],"child_size":33,"min_prefix_length":4,"max_prefix_length":12,"hash":1}}],"repository":"test"}}}
 */
#[cw_serde]
pub struct VerifyClientMessageRaw {
	pub client_state: WasmClientState,
	pub client_message: WasmHeader,
}

pub struct VerifyClientMessage<H> {
	pub client_state: ClientState<H>,
	pub client_message: ClientMessage,
}

impl<H: Clone> TryFrom<VerifyClientMessageRaw> for VerifyClientMessage<H> {
	type Error = ContractError;

	fn try_from(raw: VerifyClientMessageRaw) -> Result<Self, Self::Error> {
		let client_state = ClientState::decode_vec(&raw.client_state.data).unwrap();
		let client_message =
			ClientMessage::Header(Header::decode_vec(&raw.client_message.data).unwrap());
		Ok(Self { client_state, client_message })
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
		let client_message = ClientMessage::try_from(ClientMessageProto {
			message: Some(MessageProto::Header(HeaderProto::decode(&*raw.client_message)?)),
		})?;
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
