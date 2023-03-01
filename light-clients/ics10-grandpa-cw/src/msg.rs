use crate::{ics23::FakeInner, Bytes, ContractError};
use core::str::FromStr;
use cosmwasm_schema::{cw_serde, QueryResponses};
use ibc::{
	core::{
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes},
		ics24_host::{identifier::ClientId, Path},
	},
	protobuf::Protobuf,
	Height,
};
use ibc_proto::{google::protobuf::Any, ibc::core::client::v1::Height as HeightRaw};
use ics08_wasm::{
	client_message::Header as WasmHeader, client_state::ClientState as WasmClientState,
	consensus_state::ConsensusState as WasmConsensusState,
};
use ics10_grandpa::{
	client_message::{ClientMessage, Header, Misbehaviour},
	client_state::ClientState,
	consensus_state::ConsensusState,
};
use prost::Message;
use serde::{Deserializer, Serializer};

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
pub struct ContractResult {
	pub is_valid: bool,
	pub error_msg: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Vec<u8>>,
}

impl ContractResult {
	pub fn success() -> Self {
		Self { is_valid: true, error_msg: "".to_string(), data: None }
	}

	pub fn error(msg: String) -> Self {
		Self { is_valid: false, error_msg: msg, data: None }
	}

	pub fn data(mut self, data: Vec<u8>) -> Self {
		self.data = Some(data);
		self
	}
}

#[cw_serde]
pub struct ClientStateCallResponse {
	pub client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
	pub new_consensus_state: WasmConsensusState<FakeInner>,
	pub new_client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
	pub result: ContractResult,
}

#[cw_serde]
pub struct InitializeState {
	pub client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
	pub consensus_state: WasmConsensusState<FakeInner>,
}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct ClientCreateRequest {
	client_create_request: WasmConsensusState<FakeInner>,
}

#[cw_serde]
pub enum ExecuteMsg {
	InitializeState(InitializeState),
	ClientCreateRequest(WasmClientState<FakeInner, FakeInner, FakeInner>),
	Status(StatusMsg),
	VerifyMembership(VerifyMembershipMsgRaw),
	VerifyNonMembership(VerifyNonMembershipMsgRaw),
	VerifyClientMessage(VerifyClientMessageRaw),
	CheckForMisbehaviour(CheckForMisbehaviourMsgRaw),
	UpdateStateOnMisbehaviour(UpdateStateOnMisbehaviourMsgRaw),
	UpdateState(UpdateStateMsgRaw),
	CheckSubstituteAndUpdateState(CheckSubstituteAndUpdateStateMsg),
	VerifyUpgradeAndUpdateState(VerifyUpgradeAndUpdateStateMsgRaw),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
	#[returns(String)]
	ClientTypeMsg(ClientTypeMsg),
	#[returns(HeightRaw)]
	GetLatestHeightsMsg(GetLatestHeightsMsg),
}

#[cw_serde]
pub struct ClientTypeMsg {}

#[cw_serde]
pub struct GetLatestHeightsMsg {}

#[cw_serde]
pub struct StatusMsg {}

#[cw_serde]
pub struct MerklePath {
	pub key_path: Vec<String>,
}

#[cw_serde]
pub struct VerifyMembershipMsgRaw {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub proof: Bytes,
	pub path: MerklePath,
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub value: Bytes,
	pub height: HeightRaw,
	pub delay_block_period: u64,
	pub delay_time_period: u64,
}

pub struct VerifyMembershipMsg {
	pub prefix: CommitmentPrefix,
	pub proof: CommitmentProofBytes,
	pub path: Path,
	pub value: Vec<u8>,
	pub height: Height,
}

impl TryFrom<VerifyMembershipMsgRaw> for VerifyMembershipMsg {
	type Error = ContractError;

	fn try_from(mut raw: VerifyMembershipMsgRaw) -> Result<Self, Self::Error> {
		let proof = CommitmentProofBytes::try_from(raw.proof)?;
		let prefix = raw.path.key_path.remove(0).into_bytes();
		let path_str = raw.path.key_path.join("");
		let path = Path::from_str(&path_str)?;
		let height = Height::from(raw.height);
		Ok(Self {
			proof,
			path,
			value: raw.value,
			height,
			prefix: CommitmentPrefix::try_from(prefix)?,
		})
	}
}

#[cw_serde]
pub struct VerifyNonMembershipMsgRaw {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub proof: Bytes,
	pub path: MerklePath,
	pub height: HeightRaw,
	pub delay_block_period: u64,
	pub delay_time_period: u64,
}

pub struct VerifyNonMembershipMsg {
	pub prefix: CommitmentPrefix,
	pub proof: CommitmentProofBytes,
	pub path: Path,
	pub height: Height,
}

impl TryFrom<VerifyNonMembershipMsgRaw> for VerifyNonMembershipMsg {
	type Error = ContractError;

	fn try_from(mut raw: VerifyNonMembershipMsgRaw) -> Result<Self, Self::Error> {
		let proof = CommitmentProofBytes::try_from(raw.proof)?;
		let prefix = raw.path.key_path.remove(0).into_bytes();
		let path_str = raw.path.key_path.join("");
		let path = Path::from_str(&path_str)?;
		let height = Height::from(raw.height);
		Ok(Self { proof, path, height, prefix: CommitmentPrefix::try_from(prefix)? })
	}
}

#[cw_serde]
pub struct WasmMisbehaviour {
	pub client_id: String,
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub data: Bytes,
}

#[cw_serde]
pub enum ClientMessageRaw {
	Header(WasmHeader<FakeInner>),
	Misbehaviour(WasmMisbehaviour),
}

#[cw_serde]
pub struct VerifyClientMessageRaw {
	pub client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
	pub client_message: ClientMessageRaw,
}

pub struct VerifyClientMessage<H> {
	pub client_state: ClientState<H>,
	pub client_message: ClientMessage,
}

impl<H: Clone> TryFrom<VerifyClientMessageRaw> for VerifyClientMessage<H> {
	type Error = ContractError;

	fn try_from(raw: VerifyClientMessageRaw) -> Result<Self, Self::Error> {
		let any = Any::decode(&mut raw.client_state.data.as_slice())?;
		let client_state = ClientState::decode_vec(&any.value)?;
		let client_message = Self::decode_client_message(raw.client_message)?;
		Ok(Self { client_state, client_message })
	}
}

impl<H: Clone> VerifyClientMessage<H> {
	fn decode_client_message(raw: ClientMessageRaw) -> Result<ClientMessage, ContractError> {
		let client_message = match raw {
			ClientMessageRaw::Header(header) => {
				let any = Any::decode(&mut header.data.as_slice())?;
				ClientMessage::Header(Header::decode_vec(&any.value)?)
			},
			ClientMessageRaw::Misbehaviour(misbehaviour) => {
				let any = Any::decode(&mut misbehaviour.data.as_slice())?;
				ClientMessage::Misbehaviour(Misbehaviour::decode_vec(&any.value)?)
			},
		};
		Ok(client_message)
	}
}

#[cw_serde]
pub struct CheckForMisbehaviourMsgRaw {
	// pub client_id: String,
	pub client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
	pub misbehaviour: WasmMisbehaviour,
}

pub struct CheckForMisbehaviourMsg<H> {
	// pub client_id: ClientId,
	pub client_state: ClientState<H>,
	pub client_message: ClientMessage,
}

impl<H: Clone> TryFrom<CheckForMisbehaviourMsgRaw> for CheckForMisbehaviourMsg<H> {
	type Error = ContractError;

	fn try_from(raw: CheckForMisbehaviourMsgRaw) -> Result<Self, Self::Error> {
		// let client_id = ClientId::from_str(&raw.client_id)?;
		let any = Any::decode(&*raw.client_state.data)?;
		let client_state = ClientState::<H>::decode_vec(&any.value)?;
		let any = Any::decode(&*raw.misbehaviour.data)?;
		let client_message = ClientMessage::Misbehaviour(Misbehaviour::decode_vec(&any.value)?);
		Ok(Self { client_state, client_message })
	}
}

#[cw_serde]
pub struct UpdateStateOnMisbehaviourMsgRaw {
	pub client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
	pub client_message: WasmMisbehaviour,
}

pub struct UpdateStateOnMisbehaviourMsg<H> {
	pub client_state: ClientState<H>,
	pub client_message: ClientMessage,
}

impl<H: Clone> TryFrom<UpdateStateOnMisbehaviourMsgRaw> for UpdateStateOnMisbehaviourMsg<H> {
	type Error = ContractError;

	fn try_from(raw: UpdateStateOnMisbehaviourMsgRaw) -> Result<Self, Self::Error> {
		let any = Any::decode(&*raw.client_state.data)?;
		let client_state = ClientState::<H>::decode_vec(&any.value)?;
		let any = Any::decode(&*raw.client_message.data)?;
		let client_message = ClientMessage::Misbehaviour(Misbehaviour::decode_vec(&any.value)?);
		Ok(Self { client_state, client_message })
	}
}

#[cw_serde]
pub struct UpdateStateMsgRaw {
	pub client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
	pub client_message: ClientMessageRaw,
}

pub struct UpdateStateMsg<H> {
	pub client_state: ClientState<H>,
	pub client_message: ClientMessage,
}

impl<H: Clone> TryFrom<UpdateStateMsgRaw> for UpdateStateMsg<H> {
	type Error = ContractError;

	fn try_from(raw: UpdateStateMsgRaw) -> Result<Self, Self::Error> {
		let any = Any::decode(&mut raw.client_state.data.as_slice())?;
		let client_state = ClientState::decode_vec(&any.value)?;
		let client_message = VerifyClientMessage::<H>::decode_client_message(raw.client_message)?;
		Ok(Self { client_state, client_message })
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
