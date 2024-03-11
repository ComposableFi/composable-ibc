use core::marker::PhantomData;

use guestchain::{PubKey, Verifier};
use ibc::core::{
	ics02_client::{client_def::ClientDef, error::Error},
	ics23_commitment::commitment::CommitmentPrefix,
	ics24_host::path::{self, AcksPath},
};

use crate::{proof::verify, ClientMessage, ClientState, CommonContext, ConsensusState};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GuestClient<PK>(PhantomData<PK>);

impl<PK: PubKey> Default for GuestClient<PK> {
	fn default() -> Self {
		Self(core::marker::PhantomData)
	}
}

impl<PK> ClientDef for GuestClient<PK>
where
	// H: HostFunctionsProvider,
	PK: PubKey,
{
	type ClientMessage = ClientMessage<PK>;
	type ClientState = ClientState<PK>;
	type ConsensusState = ConsensusState;

	fn verify_client_message<
		Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<(), Error> {
    todo!()
	}

	fn update_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<
		(Self::ClientState, ibc::core::ics02_client::client_def::ConsensusUpdateResult<Ctx>),
		Error,
	> {
    todo!()
	}

	fn update_state_on_misbehaviour(
		&self,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<Self::ClientState, Error> {
		todo!()
	}

	fn check_for_misbehaviour<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<bool, Error> {
		todo!()
	}

	fn verify_upgrade_and_update_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		old_client_state: &Self::ClientState,
		upgrade_client_state: &Self::ClientState,
		upgrade_consensus_state: &Self::ConsensusState,
		proof_upgrade_client: ibc::prelude::Vec<u8>,
		proof_upgrade_consensus_state: ibc::prelude::Vec<u8>,
	) -> Result<
		(Self::ClientState, ibc::core::ics02_client::client_def::ConsensusUpdateResult<Ctx>),
		Error,
	> {
		todo!()
	}

	fn check_substitute_and_update_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		subject_client_id: ibc::core::ics24_host::identifier::ClientId,
		substitute_client_id: ibc::core::ics24_host::identifier::ClientId,
		old_client_state: Self::ClientState,
		substitute_client_state: Self::ClientState,
	) -> Result<
		(Self::ClientState, ibc::core::ics02_client::client_def::ConsensusUpdateResult<Ctx>),
		Error,
	> {
		todo!()
	}

	fn verify_client_consensus_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_state: &Self::ClientState,
		height: ibc::Height,
		prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		consensus_height: ibc::Height,
		expected_consensus_state: &Ctx::AnyConsensusState,
	) -> Result<(), Error> {
		todo!()
	}

	fn verify_connection_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		connection_id: &ibc::core::ics24_host::identifier::ConnectionId,
		expected_connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
	) -> Result<(), Error> {
		todo!()
	}

	fn verify_channel_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		expected_channel_end: &ibc::core::ics04_channel::channel::ChannelEnd,
	) -> Result<(), Error> {
		todo!()
	}

	fn verify_client_full_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_state: &Self::ClientState,
		height: ibc::Height,
		prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		expected_client_state: &Ctx::AnyClientState,
	) -> Result<(), Error> {
		todo!()
	}

	fn verify_packet_data<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		sequence: ibc::core::ics04_channel::packet::Sequence,
		commitment: ibc::core::ics04_channel::commitment::PacketCommitment,
	) -> Result<(), Error> {
		todo!()
	}

	fn verify_packet_acknowledgement<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		sequence: ibc::core::ics04_channel::packet::Sequence,
		ack: ibc::core::ics04_channel::commitment::AcknowledgementCommitment,
	) -> Result<(), Error> {
		// client state height = consensus state height
		client_state.verify_height(client_id, height)?;
		// verify_delay_passed(ctx, height, connection_end)?;

		let ack_path = AcksPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };
		let path = path::Path::Acks(ack_path);
		verify(&CommitmentPrefix::default(), proof, root, path, Some(&ack.into_vec()))
			.map_err(|e| e.into())
	}

	fn verify_next_sequence_recv<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		sequence: ibc::core::ics04_channel::packet::Sequence,
	) -> Result<(), Error> {
		todo!()
	}

	fn verify_packet_receipt_absence<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		sequence: ibc::core::ics04_channel::packet::Sequence,
	) -> Result<(), Error> {
		todo!()
	}
}
