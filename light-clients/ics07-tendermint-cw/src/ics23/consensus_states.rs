use cosmwasm_schema::cw_serde;
use cosmwasm_std::Storage;
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState,
			client_def::{ClientDef, ConsensusUpdateResult},
			client_message::ClientMessage,
			client_state::{ClientState, ClientType},
			error::Error,
		},
		ics03_connection::connection::ConnectionEnd,
		ics04_channel::{
			channel::ChannelEnd,
			commitment::{AcknowledgementCommitment, PacketCommitment},
			packet::Sequence,
		},
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
		ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
		ics26_routing::context::ReaderContext,
	},
	timestamp::Timestamp,
	Height,
};
use ibc_proto::google::protobuf::Any;
use std::{convert::Infallible, time::Duration};

/// client_id, height => consensus_state
/// trie key path: "clients/{client_id}/consensusStates/{height}"
pub struct ConsensusStates<'a>(&'a mut dyn Storage);

impl<'a> ConsensusStates<'a> {
	pub fn new(storage: &'a mut dyn Storage) -> Self {
		ConsensusStates(storage)
	}

	pub fn consensus_state_client_key() -> Vec<u8> {
		format!("consensusStates/").into_bytes()
	}

	pub fn consensus_state_height_key(height: Height) -> Vec<u8> {
		format!("{}", height).into_bytes()
	}

	pub fn consensus_state_key(height: Height, prefix: &mut Vec<u8>) -> Vec<u8> {
		prefix.append(&mut Self::consensus_state_client_key());
		prefix.append(&mut Self::consensus_state_height_key(height));
		prefix.clone()
	}

	pub fn get(&self, height: Height, prefix: &mut Vec<u8>) -> Option<Vec<u8>> {
		let full_key = Self::consensus_state_key(height, prefix);	
		self.0.get(&full_key)
	}

	pub fn insert(&mut self, height: Height, consensus_state: Vec<u8>, prefix: &mut Vec<u8>) {
		let full_key = Self::consensus_state_key(height, prefix);	
		self.0.set(&full_key, &consensus_state);
	}

	pub fn remove(&mut self, height: Height) {
		let full_key = Self::consensus_state_key(height, &mut Vec::new());	
		self.0.remove(&full_key);
	}
}

/// client_id, height => consensus_state
/// trie key path: "clients/{client_id}/consensusStates/{height}"
pub struct ReadonlyConsensusStates<'a>(&'a dyn Storage);

impl<'a> ReadonlyConsensusStates<'a> {
	pub fn new(storage: &'a dyn Storage) -> Self {
		ReadonlyConsensusStates(storage)
	}

	pub fn get(&self, height: Height, prefix: &mut Vec<u8>) -> Option<Vec<u8>> {
		let full_key =
			ConsensusStates::consensus_state_key(height, prefix);
		self.0.get(&full_key)
	}
}

#[derive(Eq, Default)]
#[cw_serde]
pub struct FakeInner;

impl<'a> TryFrom<Any> for FakeInner {
	type Error = Infallible;

	fn try_from(_: Any) -> Result<Self, Self::Error> {
		Ok(FakeInner)
	}
}

impl ConsensusState for FakeInner {
	type Error = Infallible;

	fn root(&self) -> &CommitmentRoot {
		unimplemented!()
	}

	fn timestamp(&self) -> Timestamp {
		unimplemented!()
	}

	fn encode_to_vec(&self) -> Result<Vec<u8>, tendermint_proto::Error> {
		unimplemented!()
	}
}

impl ClientState for FakeInner {
	type UpgradeOptions = ();
	type ClientDef = FakeInner;

	fn chain_id(&self) -> ChainId {
		unimplemented!()
	}

	fn client_def(&self) -> Self::ClientDef {
		unimplemented!()
	}

	fn client_type(&self) -> ClientType {
		unimplemented!()
	}

	fn latest_height(&self) -> Height {
		unimplemented!()
	}

	fn frozen_height(&self) -> Option<Height> {
		unimplemented!()
	}

	fn upgrade(
		self,
		_upgrade_height: Height,
		_upgrade_options: Self::UpgradeOptions,
		_chain_id: ChainId,
	) -> Self {
		unimplemented!()
	}

	fn expired(&self, _elapsed: Duration) -> bool {
		unimplemented!()
	}

	fn encode_to_vec(&self) -> Result<Vec<u8>, tendermint_proto::Error> {
		unimplemented!()
	}
}

impl ClientMessage for FakeInner {
	fn encode_to_vec(&self) -> Result<Vec<u8>, tendermint_proto::Error> {
		unimplemented!()
	}
}

impl ClientDef for FakeInner {
	type ClientMessage = FakeInner;
	type ClientState = FakeInner;
	type ConsensusState = FakeInner;

	fn verify_misbehaviour_header<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		_client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<(), Error> {
		unimplemented!()
	}
	fn verify_client_message<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		_client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<(), Error> {
		unimplemented!()
	}

	fn update_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		_client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Error> {
		unimplemented!()
	}

	fn update_state_on_misbehaviour(
		&self,
		_client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<Self::ClientState, Error> {
		unimplemented!()
	}

	fn check_for_misbehaviour<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		_client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<bool, Error> {
		unimplemented!()
	}

	fn verify_upgrade_and_update_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		_old_client_state: &Self::ClientState,
		_upgrade_client_state: &Self::ClientState,
		_upgrade_consensus_state: &Self::ConsensusState,
		_proof_upgrade_client: Vec<u8>,
		_proof_upgrade_consensus_state: Vec<u8>,
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Error> {
		unimplemented!()
	}

	fn verify_client_consensus_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_state: &Self::ClientState,
		_height: Height,
		_prefix: &CommitmentPrefix,
		_proof: &CommitmentProofBytes,
		_root: &CommitmentRoot,
		_client_id: &ClientId,
		_consensus_height: Height,
		_expected_consensus_state: &Ctx::AnyConsensusState,
	) -> Result<(), Error> {
		unimplemented!()
	}

	fn verify_connection_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: &ClientId,
		_client_state: &Self::ClientState,
		_height: Height,
		_prefix: &CommitmentPrefix,
		_proof: &CommitmentProofBytes,
		_root: &CommitmentRoot,
		_connection_id: &ConnectionId,
		_expected_connection_end: &ConnectionEnd,
	) -> Result<(), Error> {
		unimplemented!()
	}

	fn verify_channel_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: &ClientId,
		_client_state: &Self::ClientState,
		_height: Height,
		_prefix: &CommitmentPrefix,
		_proof: &CommitmentProofBytes,
		_root: &CommitmentRoot,
		_port_id: &PortId,
		_channel_id: &ChannelId,
		_expected_channel_end: &ChannelEnd,
	) -> Result<(), Error> {
		unimplemented!()
	}

	fn verify_client_full_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_state: &Self::ClientState,
		_height: Height,
		_prefix: &CommitmentPrefix,
		_proof: &CommitmentProofBytes,
		_root: &CommitmentRoot,
		_client_id: &ClientId,
		_expected_client_state: &Ctx::AnyClientState,
	) -> Result<(), Error> {
		unimplemented!()
	}

	fn verify_packet_data<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: &ClientId,
		_client_state: &Self::ClientState,
		_height: Height,
		_connection_end: &ConnectionEnd,
		_proof: &CommitmentProofBytes,
		_root: &CommitmentRoot,
		_port_id: &PortId,
		_channel_id: &ChannelId,
		_sequence: Sequence,
		_commitment: PacketCommitment,
	) -> Result<(), Error> {
		unimplemented!()
	}

	fn verify_packet_acknowledgement<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: &ClientId,
		_client_state: &Self::ClientState,
		_height: Height,
		_connection_end: &ConnectionEnd,
		_proof: &CommitmentProofBytes,
		_root: &CommitmentRoot,
		_port_id: &PortId,
		_channel_id: &ChannelId,
		_sequence: Sequence,
		_ack: AcknowledgementCommitment,
	) -> Result<(), Error> {
		unimplemented!()
	}

	fn verify_next_sequence_recv<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: &ClientId,
		_client_state: &Self::ClientState,
		_height: Height,
		_connection_end: &ConnectionEnd,
		_proof: &CommitmentProofBytes,
		_root: &CommitmentRoot,
		_port_id: &PortId,
		_channel_id: &ChannelId,
		_sequence: Sequence,
	) -> Result<(), Error> {
		unimplemented!()
	}

	fn verify_packet_receipt_absence<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: &ClientId,
		_client_state: &Self::ClientState,
		_height: Height,
		_connection_end: &ConnectionEnd,
		_proof: &CommitmentProofBytes,
		_root: &CommitmentRoot,
		_port_id: &PortId,
		_channel_id: &ChannelId,
		_sequence: Sequence,
	) -> Result<(), Error> {
		unimplemented!()
	}
}
