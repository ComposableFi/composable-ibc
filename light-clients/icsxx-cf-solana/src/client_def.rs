use core::str::FromStr;

use guestchain::Signature;

use crate::alloc::string::ToString;
use alloc::vec::Vec;
use guestchain::{PubKey, Verifier};
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState,
			client_def::{ClientDef, ConsensusUpdateResult},
			client_state::ClientState as OtherClientState,
			error::Error as Ics02ClientError,
		},
		ics26_routing::context::ReaderContext,
	},
	protobuf::Protobuf,
};
use prost::Message;
use std::num::NonZeroU64;

use crate::{error::Error, ClientMessage, ClientState, ConsensusState as ClientConsensusState};

type Result<T = (), E = ibc::core::ics02_client::error::Error> = ::core::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CfSolanaClient<PK>(core::marker::PhantomData<PK>);

impl<PK: PubKey> Default for CfSolanaClient<PK> {
	fn default() -> Self {
		Self(core::marker::PhantomData)
	}
}

impl<PK> ClientDef for CfSolanaClient<PK>
where
	PK: PubKey + Send + Sync,
	PK::Signature: Send + Sync,
{
	type ClientMessage = ClientMessage<PK>;
	type ClientState = ClientState<PK>;
	type ConsensusState = ClientConsensusState;

	fn verify_client_message<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<(), Ics02ClientError> {
		match client_msg {
			ClientMessage::Header(header) => {
				// The client can't be updated if no shreds were received
				let shreds = header.shreds;
				if shreds.is_empty() {
					return Err(Ics02ClientError::implementation_specific(
						"no shreds received".to_string(),
					));
				}

				let slot = shreds.first().unwrap().slot();

				// All shreds' slots should be the same
				shreds.iter().skip(1).all(|s| s.slot() == slot).then(|| ()).ok_or_else(|| {
					Ics02ClientError::implementation_specific(
						"shreds have different slots".to_string(),
					)
				})?;

				if slot <= client_state.latest_height().revision_height {
					return Err(Ics02ClientError::implementation_specific(
						"slot is not greater than latest height".to_string(),
					));
				}

				let leader = client_state.leader_for_slot(slot);

				// Verify all shreds
				shreds
					.iter()
					.map(|shred| shred.sanitize().and_then(|_| shred.verify_with_root(&leader)))
					.collect::<Result<Vec<_>, _>>()
					.map_err(|e| {
						Ics02ClientError::implementation_specific(alloc::format!(
							"shred verification failed: {}",
							e
						))
					})?;
				Ok(())
			},
			ClientMessage::Misbehaviour(_) =>
				return Err(Ics02ClientError::implementation_specific(
					"misbehaviour not supported".to_string(),
				)),
		}
	}

	fn update_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Ics02ClientError> {
		let header = match client_msg {
			ClientMessage::Header(header) => header,
			_ => unreachable!("02-client will check for Header before calling update_state; qed"),
		};
		let hash = header.hash();
		let slot = header.slot();
		let timestamp = client_state.timestamp_for_slot(slot);
		let nanos = NonZeroU64::try_from(timestamp.nanoseconds()).map_err(|e| {
			Ics02ClientError::implementation_specific(alloc::format!("invalid timestamp: {}", e))
		})?;
		let header_consensus_state = ClientConsensusState::new(&hash, nanos);
		let cs = Ctx::AnyConsensusState::wrap(&header_consensus_state).ok_or_else(|| {
			Error::UnknownConsensusStateType { description: "Ctx::AnyConsensusState".to_string() }
		})?;
		Ok((client_state.update_unchecked(header), ConsensusUpdateResult::Single(cs)))
	}

	fn update_state_on_misbehaviour(
		&self,
		client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<Self::ClientState, Ics02ClientError> {
		Ok(client_state.into_frozen())
	}

	fn check_for_misbehaviour<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ibc::core::ics24_host::identifier::ClientId,
		_client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<bool, Ics02ClientError> {
		todo!("check_for_misbehaviour")
	}

	fn verify_upgrade_and_update_state<Ctx: ReaderContext>(
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

	fn check_substitute_and_update_state<Ctx: ReaderContext>(
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

	fn verify_client_consensus_state<Ctx: ReaderContext>(
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
			client_id: convert(client_id),
			revision_number: consensus_height.revision_number,
			revision_height: consensus_height.revision_height,
		};
		let value = expected_consensus_state.encode_to_vec().map_err(Ics02ClientError::encode)?;
		verify(proof, root, path.into(), Some(value))
	}

	fn verify_connection_state<Ctx: ReaderContext>(
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

		let path = ibc_core_host_types::path::ConnectionPath(convert(connection_id));
		let value = expected_connection_end.encode_vec().map_err(Ics02ClientError::encode)?;
		verify(proof, root, path.into(), Some(value))
	}

	fn verify_channel_state<Ctx: ReaderContext>(
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

		let path = ibc_core_host_types::path::ChannelEndPath(convert(port_id), convert(channel_id));
		let value = expected_channel_end.encode_vec().map_err(Ics02ClientError::encode)?;
		verify(proof, root, path.into(), Some(value)).map_err(|e| e.into())
	}

	fn verify_client_full_state<Ctx: ReaderContext>(
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

		let path = ibc_core_host_types::path::ClientStatePath(convert(client_id));
		let value = expected_client_state.encode_to_vec().map_err(Ics02ClientError::encode)?;
		verify(proof, root, path.into(), Some(value)).map_err(|e| e.into())
	}

	fn verify_packet_data<Ctx: ReaderContext>(
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
			port_id: convert(port_id),
			channel_id: convert(channel_id),
			sequence: sequence.0.into(),
		};
		verify(proof, root, path.into(), Some(commitment.into_vec()))
	}

	fn verify_packet_acknowledgement<Ctx: ReaderContext>(
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
			port_id: convert(port_id),
			channel_id: convert(channel_id),
			sequence: sequence.0.into(),
		};
		verify(proof, root, path.into(), Some(ack.into_vec()))
	}

	fn verify_next_sequence_recv<Ctx: ReaderContext>(
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

		let path = ibc_core_host_types::path::SeqRecvPath(convert(port_id), convert(channel_id));
		let mut seq_bytes = Vec::new();
		u64::from(sequence).encode(&mut seq_bytes).expect("buffer size too small");
		verify(proof, root, path.into(), Some(seq_bytes))
	}

	fn verify_packet_receipt_absence<Ctx: ReaderContext>(
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
			port_id: convert(port_id),
			channel_id: convert(channel_id),
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

impl<PK: PubKey> Verifier<PK> for CfSolanaClient<PK> {
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
		&[],
		proof.as_bytes(),
		root.bytes.as_slice(),
		path,
		value.as_deref(),
	)
	.map_err(|err| Ics02ClientError::implementation_specific(err.to_string()))
}

// Conversion between old and new.

fn convert<F, T: ConvertFrom<F>>(value: F) -> T {
	T::convert(value)
}

trait ConvertFrom<From>: Sized {
	fn convert(value: From) -> Self;
}

macro_rules! conv {
	($( $value:ident: $From:ty => $To:ty { $($body:tt)*} )*) => {
		$(
			impl ConvertFrom<$From> for $To {
				fn convert($value: $From) -> Self {
					Self::convert(&$value)
				}
			}

			impl ConvertFrom<&$From> for $To {
				fn convert($value: &$From) -> Self {
					$($body)*
				}
			}
		)*
	}
}

conv! {
	client_id: ibc::core::ics24_host::identifier::ClientId =>
		ibc_core_host_types::identifiers::ClientId {
			FromStr::from_str(client_id.as_str()).unwrap()
		}
	client_id: ibc_core_host_types::identifiers::ClientId => ibc::core::ics24_host::identifier::ClientId {
			FromStr::from_str(client_id.as_str()).unwrap()
		}

	connection_id: ibc::core::ics24_host::identifier::ConnectionId =>
		ibc_core_host_types::identifiers::ConnectionId {
			FromStr::from_str(connection_id.as_str()).unwrap()
		}

	port_id: ibc::core::ics24_host::identifier::PortId =>
		ibc_core_host_types::identifiers::PortId {
			FromStr::from_str(port_id.as_str()).unwrap()
		}

	channel_id: ibc::core::ics24_host::identifier::ChannelId =>
		ibc_core_host_types::identifiers::ChannelId {
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence())
		}


	timestamp: ibc::timestamp::Timestamp => ibc_primitives::Timestamp {
		Self::from_nanoseconds(timestamp.nanoseconds()).unwrap()
	}

	height: ibc::core::ics02_client::height::Height => ibc_core_client_types::Height {
		Self::new(height.revision_number, height.revision_height).unwrap()
	}

	height: ibc_core_client_types::Height => ibc::core::ics02_client::height::Height {
		Self::new(height.revision_number(), height.revision_height())
	}


	err: ibc_core_client_context::types::error::ClientError => Ics02ClientError {
		// TODO(mina86): Create better mapping.
		Self::implementation_specific(err.to_string())
	}
	err: Ics02ClientError => ibc_core_client_context::types::error::ClientError {
		// TODO(mina86): Create better mapping.
		Self::ClientSpecific {
			description: err.to_string()
		}
	}
}

impl<FT, TT: ConvertFrom<FT>> ConvertFrom<Option<FT>> for Option<TT> {
	fn convert(value: Option<FT>) -> Self {
		value.map(convert)
	}
}

impl<FT, FE, TT: ConvertFrom<FT>, TE: ConvertFrom<FE>> ConvertFrom<Result<FT, FE>>
	for Result<TT, TE>
{
	fn convert(value: Result<FT, FE>) -> Self {
		value.map(convert).map_err(convert)
	}
}
