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
use crate::{
	alloc::string::ToString,
	client_message::{ClientMessage, Misbehaviour},
	client_state::ClientState,
	consensus_state::ConsensusState,
	error::Error,
	verify::{verify_ibc_proof, Verified},
	Network,
};
use alloc::{borrow::Cow, format, vec, vec::Vec};
use anyhow::anyhow;
use core::{fmt::Debug, marker::PhantomData, str::FromStr};
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState as _,
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
			path::ClientConsensusStatePath,
			ClientUpgradePath, Path,
		},
		ics26_routing::context::ReaderContext,
	},
	Height,
};
use primitive_types::H256;
#[cfg(feature = "no_beacon")]
use sync_committee_primitives::types::VerifierState as LightClientState;
#[cfg(not(feature = "no_beacon"))]
use sync_committee_verifier::verify_sync_committee_attestation;

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

#[allow(unused)]
const CLIENT_STATE_UPGRADE_PATH: &[u8] = b"client-state-upgrade-path";
#[allow(unused)]
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
		_client_state: Self::ClientState,
		client_message: Self::ClientMessage,
	) -> Result<(), Ics02Error> {
		match client_message {
			ClientMessage::Header(_header) => {
				// TODO: should we verify twice (also in update_state)?
				// let _ = verify_sync_committee_attestation(client_state.inner, header.inner)
				// 	.map_err(|e| Ics02Error::implementation_specific(e.to_string()))?;
			},
			ClientMessage::Misbehaviour(Misbehaviour { never }) => match never {},
		}

		Ok(())
	}

	fn update_state<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		_client_id: ClientId,
		client_state: Self::ClientState,
		client_message: Self::ClientMessage,
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Ics02Error> {
		let header = match client_message {
			ClientMessage::Header(header) => header,
			_ => unreachable!(
				"02-client will check for misbehaviour before calling update_state; qed"
			),
		};

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
		let css = vec![(height, cs)];

		let cs = &client_state.inner;
		let latest_height = header.execution_payload.block_number;
		if header.attested_header.slot <= cs.finalized_header.slot ||
			header.finality_proof.epoch <= cs.latest_finalized_epoch
		{
			// Check if the state root matches the one in the consensus state, otherwise, freeze the
			// client
			let height = Height::new(0, header.execution_payload.block_number);
			if let Ok(cs) = ctx.consensus_state(&ClientId::from_str("00-unknown").unwrap(), height)
			{
				if header.execution_payload.state_root.as_bytes() != cs.root().as_bytes() {
					let mut client_state = client_state;
					client_state.frozen_height = Some(height);
					return Ok((client_state, ConsensusUpdateResult::Batch(vec![])))
				}
			}

			return Ok((client_state, ConsensusUpdateResult::Batch(vec![])))
		}

		#[cfg(not(feature = "no_beacon"))]
		let new_light_client_state = {
			let cs = client_state.inner;
			verify_sync_committee_attestation::<Network>(cs, header)
				.map_err(|e| Ics02Error::implementation_specific(e.to_string()))?
		};
		#[cfg(feature = "no_beacon")]
		let new_light_client_state = {
			let update = header;
			if let Some(sync_committee_update) = update.sync_committee_update {
				LightClientState {
					finalized_header: update.finalized_header,
					latest_finalized_epoch: update.finality_proof.epoch,
					current_sync_committee: client_state.inner.next_sync_committee,
					next_sync_committee: sync_committee_update.next_sync_committee,
					state_period: 0,
				}
			} else {
				LightClientState { finalized_header: update.finalized_header, ..client_state.inner }
			}
		};

		let new_client_state = ClientState {
			inner: new_light_client_state,
			frozen_height: None,
			latest_height: latest_height as _,
			ibc_core_address: client_state.ibc_core_address,
			next_upgrade_id: client_state.next_upgrade_id,
			_phantom: Default::default(),
		};

		Ok((new_client_state, ConsensusUpdateResult::Batch(css)))
	}

	fn update_state_on_misbehaviour(
		&self,
		_client_state: Self::ClientState,
		_client_message: Self::ClientMessage,
	) -> Result<Self::ClientState, Ics02Error> {
		unimplemented!()
	}

	fn check_for_misbehaviour<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		_client_state: Self::ClientState,
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
		let height = old_client_state.latest_height();

		let consenus_state = ctx.consensus_state(&client_id, height)?
			.downcast::<Self::ConsensusState>()
			.ok_or_else(|| Error::Custom(format!("Wrong consensus state type stored for Grandpa client with {client_id} at {height}")))?;

		let root = H256::from_slice(consenus_state.root.as_bytes());

		if upgrade_client_state.next_upgrade_id != old_client_state.next_upgrade_id + 1 {
			return Err(Ics02Error::implementation_specific(
				"client state upgrade id mismatch".to_string(),
			))
		}

		// verify client state upgrade proof
		{
			let encoded = upgrade_client_state.clone().abi_encode();

			let verified = verify_ibc_proof(
				&CommitmentPrefix::try_from(b"ibc".to_vec()).unwrap(),
				&CommitmentProofBytes::try_from(proof_upgrade_client).map_err(|e| {
					Ics02Error::implementation_specific(format!("client state proof: {}", e))
				})?,
				&CommitmentRoot::from_bytes(&root.as_bytes()),
				old_client_state.ibc_core_address,
				Path::Upgrade(ClientUpgradePath::UpgradedClientState(
					old_client_state.next_upgrade_id,
				)),
				Some(Cow::Borrowed(&encoded)),
			)?;
			if verified == Verified::No {
				return Err(Ics02Error::implementation_specific(
					"client state proof verification failed".to_string(),
				))
			}
		}

		// verify consensus state upgrade proof
		{
			let encoded = upgrade_consensus_state.encode_to_vec().map_err(Ics02Error::encode)?;

			let verified = verify_ibc_proof(
				&CommitmentPrefix::try_from(b"ibc".to_vec()).unwrap(),
				&CommitmentProofBytes::try_from(proof_upgrade_consensus_state).map_err(|e| {
					Ics02Error::implementation_specific(format!("consensus state proof: {}", e))
				})?,
				&CommitmentRoot::from_bytes(&root.as_bytes()),
				old_client_state.ibc_core_address,
				Path::Upgrade(ClientUpgradePath::UpgradedClientConsensusState(
					old_client_state.next_upgrade_id,
				)),
				Some(Cow::Borrowed(&encoded)),
			)?;
			if verified == Verified::No {
				return Err(Ics02Error::implementation_specific(
					"consensus state proof verification failed".to_string(),
				))
			}
		}

		Ok((
			upgrade_client_state.clone(),
			ConsensusUpdateResult::Single(
				Ctx::AnyConsensusState::wrap(upgrade_consensus_state)
					.expect("AnyConsensusState is type-checked; qed"),
			),
		))
	}

	fn check_substitute_and_update_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_subject_client_id: ClientId,
		_substitute_client_id: ClientId,
		_old_client_state: Self::ClientState,
		_substitute_client_state: Self::ClientState,
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
		let verified = verify_ibc_proof(
			prefix,
			proof,
			root,
			client_state.ibc_core_address,
			path,
			Some(Cow::Borrowed(&value)),
		)?;
		if verified == Verified::No {
			return Err(Ics02Error::implementation_specific("client consensus state".to_string()))
		}
		Ok(())
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
	) -> Result<(), Ics02Error> {
		// client_state.verify_height(height)?;
		// let path = ConnectionsPath(connection_id.clone());
		// let value = expected_connection_end.encode_vec().map_err(Ics02Error::encode)?;
		// verify_membership::<H::BlakeTwo256, _>(prefix, proof, root, path, value)
		// 	.map_err(Error::Anyhow)?;
		unimplemented!("verify_connection_state")
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
	) -> Result<(), Ics02Error> {
		// client_state.verify_height(height)?;
		// let path = ChannelEndsPath(port_id.clone(), *channel_id);
		// let value = expected_channel_end.encode_vec().map_err(Ics02Error::encode)?;
		// verify_membership::<H::BlakeTwo256, _>(prefix, proof, root, path, value)
		// 	.map_err(Error::Anyhow)?;
		unimplemented!("verify_channel_state")
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
	) -> Result<(), Ics02Error> {
		// client_state.verify_height(height)?;
		// let path = ClientStatePath(client_id.clone());
		// let value = expected_client_state.encode_to_vec().map_err(Ics02Error::encode)?;
		// verify_membership::<H::BlakeTwo256, _>(prefix, proof, root, path, value)
		// 	.map_err(Error::Anyhow)?;
		unimplemented!("verify_client_full_state")
	}

	fn verify_packet_data<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		_client_id: &ClientId,
		_client_state: &Self::ClientState,
		height: Height,
		connection_end: &ConnectionEnd,
		_proof: &CommitmentProofBytes,
		_root: &CommitmentRoot,
		_port_id: &PortId,
		_channel_id: &ChannelId,
		_sequence: Sequence,
		_commitment: PacketCommitment,
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
		unimplemented!("verify_packet_data")
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
		unimplemented!("verify_packet_acknowledgement")
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
		unimplemented!("verify_next_sequence_recv")
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
		unimplemented!("verify_packet_receipt_absence")
	}
}
