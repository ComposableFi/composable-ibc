use core::{convert::Infallible, marker::PhantomData};
use guestchain::Signature;

use crate::{alloc::string::ToString, proof::VerifyError};
use alloc::vec::Vec;
use guestchain::{PubKey, Verifier};
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState, client_def::{ClientDef, ConsensusUpdateResult},
			client_state::ClientState as OtherClientState, error::Error as Ics02ClientError,
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::{
			identifier::ClientId,
			path::{
				self, AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath,
				CommitmentsPath, ConnectionsPath, ReceiptsPath, SeqRecvsPath,
			},
		},
		ics26_routing::context::ReaderContext,
	},
	timestamp::Timestamp,
};
use prost::Message;
use tendermint_proto::Protobuf;

use crate::{
	error::Error, proof::verify, ClientMessage, ClientState, CommonContext,
	ConsensusState as ClientConsensusState,
};

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
	// H: HostFunctionsProvider,
	PK: PubKey,
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
		client_state.verify_client_message(self, &client_id, client_msg)
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
    let header = match client_msg {
			ClientMessage::Header(header) => header,
			_ => unreachable!("02-client will check for Header before calling update_state; qed"),
		};
		let header_consensus_state = ClientConsensusState::from(header.clone());
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

		let connection_path = ClientConsensusStatePath {
			client_id: client_id.clone(),
			epoch: consensus_height.revision_number,
			height: consensus_height.revision_height,
		};
		let path = path::Path::ClientConsensusState(connection_path);
		let value = expected_consensus_state.encode_to_vec().map_err(Ics02ClientError::encode)?;
		verify(&CommitmentPrefix::default(), proof, root, path, Some(&value)).map_err(|e| e.into())
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

		let connection_path = ConnectionsPath(connection_id.clone());
		let path = path::Path::Connections(connection_path);
		let value = expected_connection_end.encode_vec().map_err(Ics02ClientError::encode)?;
		verify(&CommitmentPrefix::default(), proof, root, path, Some(&value)).map_err(|e| e.into())
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

		let channel_end_path = ChannelEndsPath(port_id.clone(), *channel_id);
		let path = path::Path::ChannelEnds(channel_end_path);
		let value = expected_channel_end.encode_vec().map_err(Ics02ClientError::encode)?;
		verify(&CommitmentPrefix::default(), proof, root, path, Some(&value)).map_err(|e| e.into())
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

		let client_state_path = ClientStatePath(client_id.clone());
		let path = path::Path::ClientState(client_state_path);
		let value = expected_client_state.encode_to_vec().map_err(Ics02ClientError::encode)?;
		verify(&CommitmentPrefix::default(), proof, root, path, Some(&value)).map_err(|e| e.into())
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

		let commitment_path =
			CommitmentsPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };
		let path = path::Path::Commitments(commitment_path);
		verify(&CommitmentPrefix::default(), proof, root, path, Some(&commitment.into_vec()))
			.map_err(|e| e.into())
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
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;
		verify_delay_passed::<Ctx, PK>(ctx, height, connection_end)?;

		let mut seq_bytes = Vec::new();
		u64::from(sequence).encode(&mut seq_bytes).expect("buffer size too small");
		let seq_recv_path = SeqRecvsPath(port_id.clone(), channel_id.clone());
		let path = path::Path::SeqRecvs(seq_recv_path);
		verify(&CommitmentPrefix::default(), proof, root, path, Some(&seq_bytes))
			.map_err(|e| e.into())
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

		let receipt_path =
			ReceiptsPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };
		let path = path::Path::Receipts(receipt_path);
		verify(&CommitmentPrefix::default(), proof, root, path, None).map_err(|e| e.into())
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
		let pubkey_in_bytes = pubkey.to_vec();
		let pubkey = ed25519_consensus::VerificationKey::try_from(&pubkey_in_bytes[..])
			.map_err(|_| VerifyError::MalformedPublicKey)
			.unwrap();
		let sig = ed25519_consensus::Signature::try_from(&signature.to_vec()[..])
			.map_err(|_| VerifyError::MalformedSignature)
			.unwrap();
		pubkey.verify(&sig, message).map_or(false, |_| true)
	}
}

