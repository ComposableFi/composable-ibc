use core::{marker::PhantomData, str::FromStr};

use guestchain::Signature;

use crate::alloc::string::ToString;
use alloc::vec::Vec;
use guestchain::{PubKey, Verifier};
use ibc::core::{
	ics02_client::{
		client_consensus::ConsensusState,
		client_def::{ClientDef, ConsensusUpdateResult},
		client_state::ClientState as OtherClientState,
		error::Error as Ics02ClientError,
	},
	ics26_routing::context::ReaderContext,
};
use prost::Message;
use tendermint_proto::Protobuf;

use crate::{error::Error, ClientMessage, ClientState, ConsensusState as ClientConsensusState};

type Result<T = (), E = ibc::core::ics02_client::error::Error> = ::core::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GuestClient<PK>(PhantomData<PK>);

impl<PK: PubKey> Default for GuestClient<PK> {
	fn default() -> Self {
		Self(core::marker::PhantomData)
	}
}

impl<PK> ClientDef for GuestClient<PK>
where
	PK: PubKey + Send + Sync,
	PK::Signature: Send + Sync,
{
	type ClientMessage = ClientMessage<PK>;
	type ClientState = ClientState<PK>;
	type ConsensusState = ClientConsensusState;

	fn verify_client_message<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		_ctx: &Ctx,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<(), Ics02ClientError> {
		client_state
			.0
			.do_verify_client_message(self, client_msg.0)
			.map_err(old_from_new_error)
	}

	fn update_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<
		(Self::ClientState, ibc::core::ics02_client::client_def::ConsensusUpdateResult<Ctx>),
		Ics02ClientError,
	> {
		let header = match client_msg.0 {
			cf_guest_upstream::ClientMessage::Header(header) => header,
			_ => unreachable!("02-client will check for Header before calling update_state; qed"),
		};
		let header_consensus_state = ClientConsensusState::from(&header);
		let cs = Ctx::AnyConsensusState::wrap(&header_consensus_state).ok_or_else(|| {
			Error::UnknownConsensusStateType { description: "Ctx::AnyConsensusState".to_string() }
		})?;
		Ok((client_state.with_header(&header), ConsensusUpdateResult::Single(cs)))
	}

	fn update_state_on_misbehaviour(
		&self,
		_client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<Self::ClientState, Ics02ClientError> {
		todo!()
	}

	fn check_for_misbehaviour<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ibc::core::ics24_host::identifier::ClientId,
		_client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<bool, Ics02ClientError> {
		todo!()
	}

	fn verify_upgrade_and_update_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ibc::core::ics24_host::identifier::ClientId,
		_old_client_state: &Self::ClientState,
		_upgrade_client_state: &Self::ClientState,
		_upgrade_consensus_state: &Self::ConsensusState,
		_proof_upgrade_client: ibc::prelude::Vec<u8>,
		_proof_upgrade_consensus_state: ibc::prelude::Vec<u8>,
	) -> Result<
		(Self::ClientState, ibc::core::ics02_client::client_def::ConsensusUpdateResult<Ctx>),
		Ics02ClientError,
	> {
		// TODO: tendermint verify_upgrade_and_update_state
		Err(Ics02ClientError::implementation_specific("Not implemented".to_string()))
	}

	fn check_substitute_and_update_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		_ctx: &Ctx,
		_subject_client_id: ibc::core::ics24_host::identifier::ClientId,
		_substitute_client_id: ibc::core::ics24_host::identifier::ClientId,
		_old_client_state: Self::ClientState,
		_substitute_client_state: Self::ClientState,
	) -> Result<
		(Self::ClientState, ibc::core::ics02_client::client_def::ConsensusUpdateResult<Ctx>),
		Ics02ClientError,
	> {
		// TODO: tendermint check_substitute_and_update_state
		Err(Ics02ClientError::implementation_specific("Not implemented".to_string()))
	}

	fn verify_client_consensus_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		_ctx: &Ctx,
		client_state: &Self::ClientState,
		height: ibc::Height,
		_prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		consensus_height: ibc::Height,
		expected_consensus_state: &Ctx::AnyConsensusState,
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;

		let path = ibc_core_host_types::path::ClientConsensusStatePath {
			client_id: new_from_old_client(client_id),
			revision_number: consensus_height.revision_number,
			revision_height: consensus_height.revision_height,
		};
		let value = expected_consensus_state.encode_to_vec().map_err(Ics02ClientError::encode)?;
		verify(proof, root, path.into(), Some(value))
	}

	fn verify_connection_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		_ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		_prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		connection_id: &ibc::core::ics24_host::identifier::ConnectionId,
		expected_connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;

		let path =
			ibc_core_host_types::path::ConnectionPath(new_from_old_connection(connection_id));
		let value = expected_connection_end.encode_vec().map_err(Ics02ClientError::encode)?;
		verify(proof, root, path.into(), Some(value))
	}

	fn verify_channel_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		_ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		_prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		expected_channel_end: &ibc::core::ics04_channel::channel::ChannelEnd,
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;

		let path = ibc_core_host_types::path::ChannelEndPath(
			new_from_old_port(port_id),
			new_from_old_channel(channel_id),
		);
		let value = expected_channel_end.encode_vec().map_err(Ics02ClientError::encode)?;
		verify(proof, root, path.into(), Some(value)).map_err(|e| e.into())
	}

	fn verify_client_full_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		_ctx: &Ctx,
		client_state: &Self::ClientState,
		height: ibc::Height,
		_prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		expected_client_state: &Ctx::AnyClientState,
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;

		let path = ibc_core_host_types::path::ClientStatePath(new_from_old_client(client_id));
		let value = expected_client_state.encode_to_vec().map_err(Ics02ClientError::encode)?;
		verify(proof, root, path.into(), Some(value)).map_err(|e| e.into())
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
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;
		verify_delay_passed::<Ctx, PK>(ctx, height, connection_end)?;

		let path = ibc_core_host_types::path::CommitmentPath {
			port_id: new_from_old_port(port_id),
			channel_id: new_from_old_channel(channel_id),
			sequence: sequence.0.into(),
		};
		verify(proof, root, path.into(), Some(commitment.into_vec()))
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
	) -> Result<(), Ics02ClientError> {
		// client state height = consensus state height
		client_state.verify_height(client_id, height)?;
		verify_delay_passed::<Ctx, PK>(ctx, height, connection_end)?;

		let path = ibc_core_host_types::path::AckPath {
			port_id: new_from_old_port(port_id),
			channel_id: new_from_old_channel(channel_id),
			sequence: sequence.0.into(),
		};
		verify(proof, root, path.into(), Some(ack.into_vec()))
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
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;
		verify_delay_passed::<Ctx, PK>(ctx, height, connection_end)?;

		let path = ibc_core_host_types::path::SeqRecvPath(
			new_from_old_port(port_id),
			new_from_old_channel(channel_id),
		);
		let mut seq_bytes = Vec::new();
		u64::from(sequence).encode(&mut seq_bytes).expect("buffer size too small");
		verify(proof, root, path.into(), Some(seq_bytes))
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
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;
		verify_delay_passed::<Ctx, PK>(ctx, height, connection_end)?;

		let path = ibc_core_host_types::path::ReceiptPath {
			port_id: new_from_old_port(port_id),
			channel_id: new_from_old_channel(channel_id),
			sequence: sequence.0.into(),
		};
		verify(proof, root, path.into(), None)
	}
}

fn verify_delay_passed<Ctx: ReaderContext, PK: PubKey>(
	ctx: &Ctx,
	height: ibc::Height,
	connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
) -> Result<(), Ics02ClientError> {
	let current_timestamp = ctx.host_timestamp();
	let current_height = ctx.host_height();

	let client_id = connection_end.client_id();
	let processed_time = ctx
		.client_update_time(client_id, height)
		.map_err(|_| Error::ProcessedTimeNotFound { height })?;
	let processed_height = ctx
		.client_update_height(client_id, height)
		.map_err(|_| Error::ProcessedHeightNotFound { height })?;

	let delay_period_time = connection_end.delay_period();
	let delay_period_height = ctx.block_delay(delay_period_time);
	let delay_period_time_u64 = u64::try_from(delay_period_time.as_nanos()).unwrap();

	ClientState::<PK>::verify_delay_passed(
		current_timestamp,
		current_height,
		processed_time.nanoseconds(),
		processed_height.revision_height,
		delay_period_time_u64,
		delay_period_height,
	)
	.map_err(|e| e.into())
}

impl<PK: PubKey> Verifier<PK> for GuestClient<PK> {
	fn verify(&self, message: &[u8], pubkey: &PK, signature: &PK::Signature) -> bool {
		(|| {
			let pubkey = pubkey.as_bytes();
			let pubkey = ed25519_consensus::VerificationKey::try_from(&pubkey[..]).ok()?;
			let signature = signature.as_bytes();
			let sig = ed25519_consensus::Signature::try_from(&signature[..]).ok()?;
			pubkey.verify(&sig, message).ok()?;
			Some(())
		})()
		.is_some()
	}
}

// Helper wrappers

fn verify(
	proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
	root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
	path: ibc_core_host_types::path::Path,
	value: Option<Vec<u8>>,
) -> Result<(), Ics02ClientError> {
	cf_guest_upstream::proof::verify(
		&Default::default(),
		proof.as_bytes(),
		root.bytes.as_slice(),
		path,
		value.as_deref(),
	)
	.map_err(|err| Ics02ClientError::implementation_specific(err.to_string()))
}

fn new_from_old_client(
	client_id: &ibc::core::ics24_host::identifier::ClientId,
) -> ibc_core_host_types::identifiers::ClientId {
	FromStr::from_str(client_id.as_str()).unwrap()
}

fn new_from_old_connection(
	connection_id: &ibc::core::ics24_host::identifier::ConnectionId,
) -> ibc_core_host_types::identifiers::ConnectionId {
	FromStr::from_str(connection_id.as_str()).unwrap()
}

fn new_from_old_port(
	port_id: &ibc::core::ics24_host::identifier::PortId,
) -> ibc_core_host_types::identifiers::PortId {
	FromStr::from_str(port_id.as_str()).unwrap()
}

fn new_from_old_channel(
	channel_id: &ibc::core::ics24_host::identifier::ChannelId,
) -> ibc_core_host_types::identifiers::ChannelId {
	ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence())
}

fn old_from_new_error(err: ibc_core_client_context::types::error::ClientError) -> Ics02ClientError {
	// TODO(mina86): Create better mapping.
	Ics02ClientError::implementation_specific(err.to_string())
}
