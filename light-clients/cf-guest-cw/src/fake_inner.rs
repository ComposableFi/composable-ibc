use cosmwasm_schema::cw_serde;
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


#[derive(Eq, Default)]
#[cw_serde]
pub struct FakeInner;

impl TryFrom<Any> for FakeInner {
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

	fn encode_to_vec(&self) -> Result<Vec<u8>, ibc::protobuf::Error> {
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

	fn encode_to_vec(&self) -> Result<Vec<u8>, ibc::protobuf::Error> {
		unimplemented!()
	}
}

impl ClientMessage for FakeInner {
	fn encode_to_vec(&self) -> Result<Vec<u8>, ibc::protobuf::Error> {
		unimplemented!()
	}
}

impl ClientDef for FakeInner {
	type ClientMessage = FakeInner;
	type ClientState = FakeInner;
	type ConsensusState = FakeInner;

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

	fn check_substitute_and_update_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_subject_client_id: ClientId,
		_substitute_client_id: ClientId,
		_old_client_state: Self::ClientState,
		_substitute_client_state: Self::ClientState,
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
