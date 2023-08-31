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

use crate::client_message::{ClientMessage, Misbehaviour};
use alloc::{format, string::ToString, vec, vec::Vec};
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
use light_client_common::{
	state_machine, verify_delay_passed, verify_membership, verify_non_membership,
};
use sync_committee_verifier::{verify_sync_committee_attestation, LightClientState};
use tendermint_proto::Protobuf;

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
				let _ = verify_sync_committee_attestation(client_state.inner, header)
					.map_err(|e| Ics02Error::implementation_specific(e.to_string()))?;
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
		// header.finality_proof.epoch
		// let bs = header.finalized_header.state_root;
		// let bs = header.execution_payload.state_root;
		// let bs = header.execution_payload.timestamp;
		// let bs = header.execution_payload.block_number;

		let mut css = header
			.ancestor_blocks
			.iter()
			.map(|b| {
				let height = Height::new(
					2, // TODO: check this
					b.execution_payload.block_number as u64,
				);
				let cs = Ctx::AnyConsensusState::wrap(&ConsensusState::new(
					b.execution_payload.state_root.clone(), //  b.header.state_root?
					b.execution_payload.timestamp,
				))
				.unwrap();
				(height, cs)
			})
			.collect::<Vec<_>>();
		let height = Height::new(
			2, // TODO: check this
			header.execution_payload.block_number as u64,
		);
		let cs = Ctx::AnyConsensusState::wrap(&ConsensusState::new(
			header.execution_payload.state_root.clone(),
			header.execution_payload.timestamp,
		))
		.unwrap();
		css.push((height, cs));

		let cs = client_state.inner;
		let new_client_state = verify_sync_committee_attestation(cs, header)
			.map_err(|e| Ics02Error::implementation_specific(e.to_string()))?;
		client_state.inner = new_client_state;
		Ok((client_state, ConsensusUpdateResult::Batch(css)))
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

	fn check_substitute_and_update_state<Ctx: ReaderContext>(&self, ctx: &Ctx, subject_client_id: ClientId, substitute_client_id: ClientId, old_client_state: Self::ClientState, substitute_client_state: Self::ClientState) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Ics02Error> {
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
		// client_state.verify_height(height)?;
		// let path = ClientConsensusStatePath {
		// 	client_id: client_id.clone(),
		// 	epoch: consensus_height.revision_number,
		// 	height: consensus_height.revision_height,
		// };
		// let value = expected_consensus_state.encode_to_vec().map_err(Ics02Error::encode)?;
		// verify_membership::<H::BlakeTwo256, _>(prefix, proof, root, path, value)
		// 	.map_err(Error::Anyhow)?;
		unimplemented!()
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
