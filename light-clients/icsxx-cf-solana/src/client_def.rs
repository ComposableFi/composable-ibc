use crate::{
	alloc::string::ToString,
	error::Error,
	solana::{
		entry::Entry,
		shred::{merkle::SIZE_OF_MERKLE_PROOF_ENTRY, shred_code::ShredCode, Shred, ShredData},
		shredder::Shredder,
	},
	ClientMessage, ClientState, ConsensusState as ClientConsensusState, Header,
};
use alloc::vec::Vec;
use core::str::FromStr;
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState,
			client_def::{ClientDef, ConsensusUpdateResult},
			client_state::ClientState as OtherClientState,
			error::Error as Ics02ClientError,
		},
		ics24_host::identifier::ClientId,
		ics26_routing::context::ReaderContext,
	},
	protobuf::Protobuf,
};
use prost::Message;

type Result<T = (), E = ibc::core::ics02_client::error::Error> = ::core::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CfSolanaClient;

impl Default for CfSolanaClient {
	fn default() -> Self {
		Self
	}
}

impl ClientDef for CfSolanaClient {
	type ClientMessage = ClientMessage;
	type ClientState = ClientState;
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
				let slot = shreds.slot();

				// TODO: verify that the header is within trusting period

				let leader = client_state.leader_for_slot(slot);

				// Verify all shreds
				shreds
					.iter()
					.try_for_each(|shred| {
						shred.sanitize()?;
						let _root = shred.verify_with_root(&leader)?;
						Ok(())
					})
					.map_err(|err: Error| {
						Ics02ClientError::implementation_specific(alloc::format!(
							"shred verification failed: {err}",
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
		let header_consensus_state =
			ClientConsensusState::from_header_and_client_state(&header, &client_state)?;
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
		ctx: &Ctx,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<bool, Ics02ClientError> {
		match client_msg {
			ClientMessage::Header(header) =>
				Self::check_header_for_misbehaviour(ctx, &client_id, &client_state, &header)
					.map_err(Into::into),
			ClientMessage::Misbehaviour(_) =>
				return Err(Ics02ClientError::implementation_specific(
					"misbehaviour not supported".to_string(),
				)),
		}
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
		verify_delay_passed::<Ctx>(ctx, height, connection_end)?;

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
		verify_delay_passed::<Ctx>(ctx, height, connection_end)?;

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
		verify_delay_passed::<Ctx>(ctx, height, connection_end)?;

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
		verify_delay_passed::<Ctx>(ctx, height, connection_end)?;

		let path = ibc_core_host_types::path::ReceiptPath {
			port_id: convert(port_id),
			channel_id: convert(channel_id),
			sequence: sequence.0.into(),
		};
		verify(proof, root, path.into(), None)
	}
}

impl CfSolanaClient {
	fn check_header_for_misbehaviour<Ctx: ReaderContext>(
		ctx: &Ctx,
		client_id: &ClientId,
		client_state: &ClientState,
		header: &Header,
	) -> Result<bool, Error> {
		let height = header.height();

		// If we received an update from the past...
		if height <= client_state.latest_height() {
			// ...and we have the consensus state for that height, we need to check if
			// they're the same, otherwise we have a misbehaviour.
			if let Ok(existing_consensus_state) = ctx.consensus_state(&client_id, height) {
				let header_consensus_state =
					ClientConsensusState::from_header_and_client_state(&header, &client_state)?;
				let new_consensus_state = Ctx::AnyConsensusState::wrap(&header_consensus_state)
					.ok_or_else(|| Error::UnknownConsensusStateType {
						description: "Ctx::AnyConsensusState".to_string(),
					})?;

				// The consensus state is different, so we have a misbehaviour.
				if existing_consensus_state != new_consensus_state {
					return Ok(true);
				}
			}
		}

		Ok(false)
	}
}

fn verify_delay_passed<Ctx: ReaderContext>(
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

	ClientState::verify_delay_passed(
		current_timestamp,
		current_height,
		processed_time.nanoseconds(),
		processed_height.revision_height,
		delay_period_time_u64,
		delay_period_height,
	)
	.map_err(|e| e.into())
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

#[test]
fn test_shred_hash() {
	use crate::solana::shred::traits::Shred as _;
	let shreds = [
		"0100000001000000591dce6416118cc1cb16a12c2f1ef122fca1621fba62e831f81d79794c06612878a1359bfd6b3c49bb2f52a4015e1c8cfccc5478770f9ba3f7c6899a31cd1302850a0000000000000000000000ebbd000000000100495303b304000000000000591dce6416118cc1cb16a12c2f1ef122fca1621fba62e831f81d79794c06612878a1359bfd6b3c49bb2f52a4015e1c8cfccc5478770f9ba3f7c6899a31cd1302850a0000000000000000000000ebbd0000000001004953030a00000000000000d43000000000000053f28095a471fb92aa740461432ad41a52a4cec3f507e0d55422e06b7ab712c5000000000000000081020000000000004cc7c0e0d57448b6d2c3924b3dee79284bf88224ebdbf55135f6181751ce60f80100000000000000017bd560611dff3373bd9d12dab527f8a33fd8d50fd483d8924550c0570f062f20287a26ca428389cda82fa9bca6214b3255c62ba81e9f4dbda869f0b4538d6b0d0100010398dedf017a7e44a1250cc88eb3614bfed2aab3acc615bbbc03c84c7a93dcfc5d31fb4a3092290c19e80218c8983a2b084459409c7c6069d15fc661ca9489a4140761481d357474bb7c4d7624ebd3bdb3d8355e73d11043fc0da353800000000069bc30be16be6472bda70c3f0514ae07f25d5b6a3f37e61a3e2eea594d4b6a1d0102020100480c0000000000000000000000090109010801070106010501040103010201012938a24974d4c715ca938d6995d49dd3e5188da8b86f63176a68eadfd2f5dfa401d2bdb46600000000532e00000000000021fbf913c83e1289a84e9cd9a25098dae9e4444b0b7d18585a910613661812530000000000000000d430000000000000106407b8de505aab720b4d181d6d57831a42402a399b1f21b644bc876fb8f0fd0000000000000000d430000000000000308087956d9e3a7de49e1c9d3bd85d802c47945d7670436c67669eebe087cf9f0000000000000000d4300000000000001dece0276e0991c99bbaac5b2cdbbcfe19278032445dc1f0e30d81adfe5000b20000000000000000d43000000000000079453d5ca70f99e609d152609f8bd7066501d8a6293c2319a30beecd1887626a0000000000000000d43000000000000043b760de2a4c5d18c4c7aab49b221c1ebd2a2280b189abca9f3e21f34f0fa6470000000000000000d430000000000000a53f033b9a8874ad71ca05b6e92be1a046aa02462254f82b75b4d9843ea8732c0000000000000000d430000000000000a813a6ec7e673e4808459f48331b47688f5ab68ffa8a32f46758d89d0c3bfdbb0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ae203b0f19448201e1ceb62f8f3257d9bc98b10da2e42c0760a9f44d7ffdfc271217dbd7076646885375b0c5a7a9fdcd8759a23ad2ec1c9751705467420928e6cf861b101aff41e0e4fa1a2247e9698a4c06e1c95362a236c0a8dda0df2aa65c70ba68a7",
		"0100000001000000cc14975afd9aea65d8d1425e32d914096b7f28fa5e9d71c9d157e79f51269d8f27173bde3d3ec67c1dda2d81db2e096334b7e7c8450c53d63b796f817f312703860a0000000000000001000000ebbd010000000100c0b001b304000000000000cc14975afd9aea65d8d1425e32d914096b7f28fa5e9d71c9d157e79f51269d8f27173bde3d3ec67c1dda2d81db2e096334b7e7c8450c53d63b796f817f312703860a0000000000000001000000ebbd010000000100c0b0010700000000000000d430000000000000af542f18cbeb513b15b229d9050c985730d5d0ff1376f374346bc23b6e64c7b20000000000000000d430000000000000852e4ea512344da2fd4c1ffe2696e102ce3017d5023909f52ccf03e9a0e43c480000000000000000d430000000000000d44d0dbd5ab8001a86663bf91f97dc7cd5c2da1287fda355ae6b33ce00d34d4b0000000000000000d4300000000000003b4642acd286836c3c4ffe5cc756e3866905ad15c311eddc0066f03dce03467a0000000000000000d430000000000000ebe7c7c0cc4e8a77838c2f2778b8d993da39ad88834c8bcb2311538f605d10860000000000000000d430000000000000ad9ee5186c324f4e1301c6e683f20194a945269793942d3cab84f9f2363994880000000000000000d4300000000000007615d9294001d2ad98125b843cb5666eed0bc918bb3e3f875e7680557e8eab750000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000da2229a1296b638e4683d0cb39551c26e7e777c6396417fa133c109dc554e689bd99a9aeeca6985ea02626b24c8b123af646d8c3472cf76f8c4749ad01137206242381948049344f393672f24195c7c351c957dc662140d0c3b7481e7f803d5ed57ba872c42e3539b03d519ed054925b551d1d0524c08f2e",
	].into_iter().map(|s| {
		let bytes = hex::decode(s).unwrap();
		bincode::deserialize::<Shred>(&bytes)
	}).collect::<Result<Vec<Shred>, _>>().unwrap();
	for s in &shreds {
		match &s {
			Shred::ShredCode(s) => match &s {
				ShredCode::Legacy(_) => {
					panic!()
				},
				ShredCode::Merkle(s) => {
					std::println!("payload: {}", hex::encode(s.payload()));

					let proof_size = s.proof_size().unwrap();
					let proof_offset = s.proof_offset().unwrap();

					let proof_size = usize::from(proof_size) * SIZE_OF_MERKLE_PROOF_ENTRY;
					let merkle_proof =
						s.payload.get(proof_offset..proof_offset + proof_size).unwrap();
					std::println!("proof: {}", hex::encode(merkle_proof));
					std::println!("node: {}", hex::encode(s.merkle_node().unwrap()));
				},
			},
			Shred::ShredData(s) => match s {
				ShredData::Legacy(_) => {
					panic!()
				},
				ShredData::Merkle(s) => {
					std::println!("payload: {}", hex::encode(s.payload()));

					let proof_size = s.proof_size().unwrap();
					let proof_offset = s.proof_offset().unwrap();

					let proof_size = usize::from(proof_size) * SIZE_OF_MERKLE_PROOF_ENTRY;
					let merkle_proof =
						s.payload.get(proof_offset..proof_offset + proof_size).unwrap();
					std::println!("proof: {}", hex::encode(merkle_proof));

					std::println!("node: {}", hex::encode(s.merkle_node().unwrap()));
				},
			},
		}
	}
	let shreds = shreds.iter().collect::<Vec<_>>();
	let payload = Shredder::deshred(shreds.as_slice()).unwrap();
	let entries = bincode::deserialize::<Vec<Entry>>(&payload).unwrap();
	for entry in &entries {
		std::println!("{}", hex::encode(&entry.hash.to_bytes()));
	}
}
