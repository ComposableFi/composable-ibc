use crate::{
	client_message::ClientMessage, client_state::ClientState, consensus_state::ConsensusState,
};
use ibc::{
	core::{
		ics02_client::{
			client_def::{ClientDef, ConsensusUpdateResult},
			error::Error,
		},
		ics03_connection::connection::ConnectionEnd,
		ics04_channel::{
			channel::ChannelEnd,
			commitment::{AcknowledgementCommitment, PacketCommitment},
			packet::Sequence,
		},
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
		ics26_routing::context::ReaderContext,
	},
	Height,
};

#[derive(Clone, Debug, PartialEq, Eq, Copy, Default)]
pub struct WasmClient {}

impl ClientDef for WasmClient {
	type ClientMessage = ClientMessage;
	type ClientState = ClientState;
	type ConsensusState = ConsensusState;

	fn verify_client_message<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		_client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<(), Error> {
		todo!()
	}

	fn update_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		_client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Error> {
		todo!()
	}

	fn update_state_on_misbehaviour(
		&self,
		_client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<Self::ClientState, Error> {
		todo!()
	}

	fn check_for_misbehaviour<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		_client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<bool, Error> {
		todo!()
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
		todo!()
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
		todo!()
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
		todo!()
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
		todo!()
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
		todo!()
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
		todo!()
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
		todo!()
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
		todo!()
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
		todo!()
	}
}
