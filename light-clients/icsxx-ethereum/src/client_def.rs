// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{client_state::ClientState, consensus_state::ConsensusState, error::Error};
use ibc::core::ics02_client::{
	client_consensus::ConsensusState as _, client_state::ClientState as _,
};

use crate::{
	client_message::{ClientMessage, Misbehaviour},
	verify::verify_ibc_proof,
};
use alloc::{format, string::ToString, vec, vec::Vec};
use anyhow::anyhow;
use core::{fmt::Debug, marker::PhantomData};
use ibc::{
	core::{
		ics02_client::{
			client_def::{ClientDef, ConsensusUpdateResult},
			error::Error as Ics02Error,
		},
		ics03_connection::connection::ConnectionEnd,
		ics04_channel::{
			channel::ChannelEnd,
			commitment::{AcknowledgementCommitment, PacketCommitment},
			packet::Sequence,
		},
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
		ics24_host::{
			identifier::{ChannelId, ClientId, ConnectionId, PortId},
			path::{
				AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath,
				CommitmentsPath, ConnectionsPath, ReceiptsPath, SeqRecvsPath,
			},
		},
		ics26_routing::context::ReaderContext,
	},
	Height,
};
use sync_committee_verifier::{verify_sync_committee_attestation, LightClientState};
use tendermint_proto::Protobuf;

// TODO: move this function in a separate crate and remove the one from `light_client_common` crate
/// This will verify that the connection delay has elapsed for a given [`ibc::Height`]
pub fn verify_delay_passed<H, C>(
	ctx: &C,
	height: Height,
	connection_end: &ConnectionEnd,
) -> Result<(), anyhow::Error>
where
	H: Clone,
	C: ReaderContext,
{
	let current_time = ctx.host_timestamp();
	let current_height = ctx.host_height();

	let client_id = connection_end.client_id();
	let processed_time = ctx.client_update_time(client_id, height).map_err(anyhow::Error::msg)?;
	let processed_height =
		ctx.client_update_height(client_id, height).map_err(anyhow::Error::msg)?;

	let delay_period_time = connection_end.delay_period();
	let delay_period_blocks = ctx.block_delay(delay_period_time);

	let earliest_time =
		(processed_time + delay_period_time).map_err(|_| anyhow!("Timestamp overflowed!"))?;
	if !(current_time == earliest_time || current_time.after(&earliest_time)) {
		return Err(anyhow!(
			"Not enough time elapsed current time: {current_time}, earliest time: {earliest_time}"
		))
	}

	let earliest_height = processed_height.add(delay_period_blocks);
	if current_height < earliest_height {
		return Err(anyhow!("Not enough blocks elapsed, current height: {current_height}, earliest height: {earliest_height}"));
	}

	Ok(())
}

const CLIENT_STATE_UPGRADE_PATH: &[u8] = b"client-state-upgrade-path";
const CONSENSUS_STATE_UPGRADE_PATH: &[u8] = b"consensus-state-upgrade-path";

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EthereumClient<T>(PhantomData<T>);

impl<H> ClientDef for EthereumClient<H>
where
	H: Clone + Eq + Send + Sync + Debug + Default,
{
	type ClientMessage = ClientMessage;
	type ClientState = ClientState<H>;
	type ConsensusState = ConsensusState;

	fn verify_client_message<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		client_state: Self::ClientState,
		client_message: Self::ClientMessage,
	) -> Result<(), Ics02Error> {
		match client_message {
			ClientMessage::Header(header) => {
				// let _ = verify_sync_committee_attestation::<H>(client_state.inner, header.inner)
				// 	.map_err(|e| Ics02Error::implementation_specific(e.to_string()))?;
			},
			ClientMessage::Misbehaviour(Misbehaviour { never }) => match never {},
		}

		Ok(())
	}

	fn update_state<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ClientId,
		mut client_state: Self::ClientState,
		client_message: Self::ClientMessage,
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Ics02Error> {
		let header = match client_message {
			ClientMessage::Header(header) => header,
			_ => unreachable!(
				"02-client will check for misbehaviour before calling update_state; qed"
			),
		};

		let mut css = header
			.ancestor_blocks
			.iter()
			.map(|b| {
				let height = Height::new(
					0, // TODO: check this
					b.execution_payload.block_number as u64,
				);
				let cs = Ctx::AnyConsensusState::wrap(&ConsensusState::new(
					b.execution_payload.state_root.clone(),
					b.execution_payload.timestamp,
				))
				.unwrap();
				(height, cs)
			})
			.collect::<Vec<_>>();
		let header = header.inner;
		let height = Height::new(
			0, // TODO: check this
			header.execution_payload.block_number as u64,
		);
		let cs = Ctx::AnyConsensusState::wrap(&ConsensusState::new(
			header.execution_payload.state_root.clone(),
			header.execution_payload.timestamp,
		))
		.unwrap();
		css.push((height, cs));

		// let cs = client_state.inner;
		// let new_client_state = verify_sync_committee_attestation::<H>(cs, header)
		// 	.map_err(|e| Ics02Error::implementation_specific(e.to_string()))?;
		let update = header;
		let new_light_client_state =
			if let Some(sync_committee_update) = update.sync_committee_update {
				LightClientState {
					finalized_header: update.finalized_header,
					latest_finalized_epoch: update.finality_proof.epoch,
					current_sync_committee: client_state.inner.next_sync_committee,
					next_sync_committee: sync_committee_update.next_sync_committee,
				}
			} else {
				LightClientState { finalized_header: update.finalized_header, ..client_state.inner }
			};
		let new_client_state = ClientState {
			inner: new_light_client_state,
			frozen_height: None,
			latest_height: update.execution_payload.block_number as _,
			// latest_height: update.attested_header.slot.into(),
			_phantom: Default::default(),
		};

		// client_state.inner = new_client_state;
		Ok((new_client_state, ConsensusUpdateResult::Batch(css)))
	}

	fn update_state_on_misbehaviour(
		&self,
		mut client_state: Self::ClientState,
		_client_message: Self::ClientMessage,
	) -> Result<Self::ClientState, Ics02Error> {
		unimplemented!()
	}

	fn check_for_misbehaviour<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ClientId,
		client_state: Self::ClientState,
		client_message: Self::ClientMessage,
	) -> Result<bool, Ics02Error> {
		if matches!(client_message, ClientMessage::Misbehaviour(_)) {
			return Ok(true)
		}

		Ok(false)
	}

	fn verify_upgrade_and_update_state<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ClientId,
		old_client_state: &Self::ClientState,
		upgrade_client_state: &Self::ClientState,
		upgrade_consensus_state: &Self::ConsensusState,
		proof_upgrade_client: Vec<u8>,
		proof_upgrade_consensus_state: Vec<u8>,
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Ics02Error> {
		unimplemented!()
	}

	fn check_substitute_and_update_state<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		subject_client_id: ClientId,
		substitute_client_id: ClientId,
		old_client_state: Self::ClientState,
		substitute_client_state: Self::ClientState,
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Ics02Error> {
		todo!("check_substitute_and_update_state")
	}

	fn verify_client_consensus_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		client_state: &Self::ClientState,
		height: Height,
		prefix: &CommitmentPrefix,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		client_id: &ClientId,
		consensus_height: Height,
		expected_consensus_state: &Ctx::AnyConsensusState,
	) -> Result<(), Ics02Error> {
		client_state.verify_height(height)?;
		let path = ClientConsensusStatePath {
			client_id: client_id.clone(),
			epoch: consensus_height.revision_number,
			height: consensus_height.revision_height,
		};
		let value = expected_consensus_state.encode_to_vec().map_err(Ics02Error::encode)?;
		verify_ibc_proof(prefix, proof, root, path, Some(&value))?;
		Ok(())
	}

	fn verify_connection_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: &ClientId,
		client_state: &Self::ClientState,
		height: Height,
		prefix: &CommitmentPrefix,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		connection_id: &ConnectionId,
		expected_connection_end: &ConnectionEnd,
	) -> Result<(), Ics02Error> {
		// client_state.verify_height(height)?;
		// let path = ConnectionsPath(connection_id.clone());
		// let value = expected_connection_end.encode_vec().map_err(Ics02Error::encode)?;
		// verify_membership::<H::BlakeTwo256, _>(prefix, proof, root, path, value)
		// 	.map_err(Error::Anyhow)?;
		Ok(())
	}

	fn verify_channel_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: &ClientId,
		client_state: &Self::ClientState,
		height: Height,
		prefix: &CommitmentPrefix,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		port_id: &PortId,
		channel_id: &ChannelId,
		expected_channel_end: &ChannelEnd,
	) -> Result<(), Ics02Error> {
		// client_state.verify_height(height)?;
		// let path = ChannelEndsPath(port_id.clone(), *channel_id);
		// let value = expected_channel_end.encode_vec().map_err(Ics02Error::encode)?;
		// verify_membership::<H::BlakeTwo256, _>(prefix, proof, root, path, value)
		// 	.map_err(Error::Anyhow)?;
		Ok(())
	}

	fn verify_client_full_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		client_state: &Self::ClientState,
		height: Height,
		prefix: &CommitmentPrefix,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		client_id: &ClientId,
		expected_client_state: &Ctx::AnyClientState,
	) -> Result<(), Ics02Error> {
		// client_state.verify_height(height)?;
		// let path = ClientStatePath(client_id.clone());
		// let value = expected_client_state.encode_to_vec().map_err(Ics02Error::encode)?;
		// verify_membership::<H::BlakeTwo256, _>(prefix, proof, root, path, value)
		// 	.map_err(Error::Anyhow)?;
		Ok(())
	}

	fn verify_packet_data<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		_client_id: &ClientId,
		client_state: &Self::ClientState,
		height: Height,
		connection_end: &ConnectionEnd,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		port_id: &PortId,
		channel_id: &ChannelId,
		sequence: Sequence,
		commitment: PacketCommitment,
	) -> Result<(), Ics02Error> {
		// client_state.verify_height(height)?;
		verify_delay_passed::<H, _>(ctx, height, connection_end).map_err(Error::Anyhow)?;
		//
		// let commitment_path =
		// 	CommitmentsPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };
		//
		// verify_membership::<H::BlakeTwo256, _>(
		// 	connection_end.counterparty().prefix(),
		// 	proof,
		// 	root,
		// 	commitment_path,
		// 	commitment.into_vec(),
		// )
		// .map_err(Error::Anyhow)?;
		Ok(())
	}

	fn verify_packet_acknowledgement<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		_client_id: &ClientId,
		client_state: &Self::ClientState,
		height: Height,
		connection_end: &ConnectionEnd,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		port_id: &PortId,
		channel_id: &ChannelId,
		sequence: Sequence,
		ack: AcknowledgementCommitment,
	) -> Result<(), Ics02Error> {
		// client_state.verify_height(height)?;
		// verify_delay_passed::<H, _>(ctx, height, connection_end).map_err(Error::Anyhow)?;
		//
		// let ack_path = AcksPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };
		// verify_membership::<H::BlakeTwo256, _>(
		// 	connection_end.counterparty().prefix(),
		// 	proof,
		// 	root,
		// 	ack_path,
		// 	ack.into_vec(),
		// )
		// .map_err(Error::Anyhow)?;
		Ok(())
	}

	fn verify_next_sequence_recv<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		_client_id: &ClientId,
		client_state: &Self::ClientState,
		height: Height,
		connection_end: &ConnectionEnd,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		port_id: &PortId,
		channel_id: &ChannelId,
		sequence: Sequence,
	) -> Result<(), Ics02Error> {
		// client_state.verify_height(height)?;
		// verify_delay_passed::<H, _>(ctx, height, connection_end).map_err(Error::Anyhow)?;
		//
		// let seq_bytes = codec::Encode::encode(&u64::from(sequence));
		//
		// let seq_path = SeqRecvsPath(port_id.clone(), *channel_id);
		// verify_membership::<H::BlakeTwo256, _>(
		// 	connection_end.counterparty().prefix(),
		// 	proof,
		// 	root,
		// 	seq_path,
		// 	seq_bytes,
		// )
		// .map_err(Error::Anyhow)?;
		Ok(())
	}

	fn verify_packet_receipt_absence<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		_client_id: &ClientId,
		client_state: &Self::ClientState,
		height: Height,
		connection_end: &ConnectionEnd,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		port_id: &PortId,
		channel_id: &ChannelId,
		sequence: Sequence,
	) -> Result<(), Ics02Error> {
		// client_state.verify_height(height)?;
		// verify_delay_passed::<H, _>(ctx, height, connection_end).map_err(Error::Anyhow)?;
		//
		// let receipt_path =
		// 	ReceiptsPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };
		// verify_non_membership::<H::BlakeTwo256, _>(
		// 	connection_end.counterparty().prefix(),
		// 	proof,
		// 	root,
		// 	receipt_path,
		// )
		// .map_err(Error::Anyhow)?;
		Ok(())
	}
}
