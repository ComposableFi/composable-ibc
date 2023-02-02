// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::{convert::TryInto, fmt::Debug, marker::PhantomData, str::FromStr};

use ibc::core::{
	ics02_client::{
		client_consensus::ConsensusState as _,
		client_def::{ClientDef, ConsensusUpdateResult},
		client_state::ClientState as _,
		error::Error as Ics02Error,
	},
	ics03_connection::connection::ConnectionEnd,
	ics04_channel::{
		channel::ChannelEnd,
		commitment::{AcknowledgementCommitment, PacketCommitment},
		packet::Sequence,
	},
	ics23_commitment::{
		commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
		merkle::{apply_prefix, MerkleProof},
	},
	ics24_host::{
		identifier::{ChannelId, ClientId, ConnectionId, PortId},
		path::{
			AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath, CommitmentsPath,
			ConnectionsPath, ReceiptsPath, SeqRecvsPath,
		},
		Path,
	},
	ics26_routing::context::ReaderContext,
};
use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
use prost::Message;
use tendermint_light_client_verifier::{
	types::{TrustedBlockState, UntrustedBlockState},
	PredicateVerifier, Verdict, Verifier,
};
use tendermint_proto::Protobuf;

use crate::{
	client_message::{ClientMessage, Header},
	client_state::ClientState,
	consensus_state::ConsensusState,
	error::Error,
	HostFunctionsProvider,
};
use ibc::{prelude::*, timestamp::Timestamp, Height};
use ibc_proto::ics23::ProofSpec;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TendermintClient<H>(PhantomData<H>);

impl<H> ClientDef for TendermintClient<H>
where
	H: HostFunctionsProvider,
{
	type ClientMessage = ClientMessage;
	type ClientState = ClientState<H>;
	type ConsensusState = ConsensusState;

	fn verify_client_message<Ctx>(
		&self,
		ctx: &Ctx,
		client_id: ClientId,
		client_state: Self::ClientState,
		message: Self::ClientMessage,
	) -> Result<(), Ics02Error>
	where
		Ctx: ReaderContext,
	{
		match message {
			ClientMessage::Header(header) => {
				log::info!("Got header.height = {:?}", header.height());
				log::info!(
					"Got state header.signed_header.header = {:?}",
					header.signed_header.header
				);
				log::info!(
					"Got state header.signed_header.commit.height = {:?}",
					header.signed_header.commit.height
				);

				if header.height().revision_number != client_state.chain_id.version() {
					return Err(Ics02Error::client_error(
						client_state.client_type().to_owned(),
						Error::mismatched_revisions(
							client_state.chain_id.version(),
							header.height().revision_number,
						)
						.to_string(),
					))
				}

				// Check if a consensus state is already installed; if so skip
				let header_consensus_state = <ConsensusState as From<Header>>::from(header.clone());

				let _ = match ctx.maybe_consensus_state(&client_id, header.height())? {
					Some(cs) => {
						let cs: ConsensusState =
							cs.downcast().ok_or(Ics02Error::client_args_type_mismatch(
								client_state.client_type().to_owned(),
							))?;
						// If this consensus state matches, skip verification
						// (optimization)
						if cs == header_consensus_state {
							// Header is already installed and matches the incoming
							// header (already verified)
							return Ok(())
						}
						Some(cs)
					},
					None => None,
				};

				let trusted_consensus_state: Self::ConsensusState = ctx
					.consensus_state(&client_id, header.trusted_height)?
					.downcast()
					.ok_or(Ics02Error::client_args_type_mismatch(
						ClientState::<H>::client_type().to_owned(),
					))?;

				let trusted_state = TrustedBlockState {
					// TODO: make sure it's correct
					chain_id: &tendermint::chain::Id::from_str(client_state.chain_id.as_str())
						.unwrap(),
					header_time: trusted_consensus_state.timestamp().into_tm_time().unwrap(),
					height: header.trusted_height.revision_height.try_into().map_err(|_| {
						Ics02Error::client_error(
							client_state.client_type().to_owned(),
							Error::invalid_header_height(header.trusted_height).to_string(),
						)
					})?,
					next_validators: &header.trusted_validator_set,
					next_validators_hash: trusted_consensus_state.next_validators_hash,
				};

				let untrusted_state = UntrustedBlockState {
					signed_header: &header.signed_header,
					validators: &header.validator_set,
					// NB: This will skip the
					// VerificationPredicates::next_validators_match check for the
					// untrusted state.
					next_validators: None,
				};

				let options = client_state.as_light_client_options()?;

				let verifier = PredicateVerifier::<H, H, H>::default();
				let verdict = verifier.verify(
					untrusted_state,
					trusted_state,
					&options,
					ctx.host_timestamp().into_tm_time().unwrap(),
				);

				match verdict {
					Verdict::Success => {},
					Verdict::NotEnoughTrust(voting_power_tally) =>
						return Err(Error::not_enough_trusted_vals_signed(format!(
							"voting power tally: {}",
							voting_power_tally
						))
						.into()),
					Verdict::Invalid(detail) =>
						return Err(Error::verification_error(detail).into()),
				}
			},
			ClientMessage::Misbehaviour(misbehaviour) => {
				self.verify_client_message(
					ctx,
					client_id.clone(),
					client_state.clone(),
					ClientMessage::Header(misbehaviour.header1),
				)?;
				self.verify_client_message(
					ctx,
					client_id,
					client_state,
					ClientMessage::Header(misbehaviour.header2),
				)?;
			},
		};

		Ok(())
	}

	fn update_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		client_state: Self::ClientState,
		client_message: Self::ClientMessage,
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Ics02Error> {
		let header = match client_message {
			ClientMessage::Header(header) => header,
			_ => unreachable!("02-client will check for Header before calling update_state; qed"),
		};
		let header_consensus_state = <ConsensusState as From<Header>>::from(header.clone());
		let cs = Ctx::AnyConsensusState::wrap(&header_consensus_state).ok_or_else(|| {
			Ics02Error::unknown_consensus_state_type("Ctx::AnyConsensusState".to_string())
		})?;
		let state = client_state.with_header(header.clone());
		log::info!("Updating state header.height = {:?}", header.height());
		log::info!(
			"Updating state header.signed_header.header = {:?}",
			header.signed_header.header
		);
		log::info!(
			"Updating state header.signed_header.commit.height = {:?}",
			header.signed_header.commit.height
		);
		log::info!("state = {state:?}\ncs = {cs:?}");
		Ok((state, ConsensusUpdateResult::Single(cs)))
	}

	fn update_state_on_misbehaviour(
		&self,
		client_state: Self::ClientState,
		client_message: Self::ClientMessage,
	) -> Result<Self::ClientState, Ics02Error> {
		let misbehaviour = match client_message {
			ClientMessage::Misbehaviour(misbehaviour) => misbehaviour,
			_ => unreachable!(
				"02-client will check for misbehaviour before calling update_state_on_misbehaviour; qed"
			),
		};
		client_state
			.with_frozen_height(misbehaviour.header1.height())
			.map_err(|e| e.into())
	}

	fn check_for_misbehaviour<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ClientId,
		client_state: Self::ClientState,
		message: Self::ClientMessage,
	) -> Result<bool, Ics02Error> {
		match message {
			ClientMessage::Header(header) => {
				// Check if a consensus state is already installed; if so it should
				// match the untrusted header.
				let header_consensus_state = <ConsensusState as From<Header>>::from(header.clone());

				let existing_consensus_state =
					match ctx.maybe_consensus_state(&client_id, header.height())? {
						Some(cs) => {
							let cs = cs.downcast::<ConsensusState>().ok_or(
								Ics02Error::client_args_type_mismatch(
									ClientState::<()>::client_type().to_owned(),
								),
							)?;
							// If this consensus state matches, skip verification
							// (optimization)
							if header_consensus_state == cs {
								// Header is already installed and matches the incoming
								// header (already verified)
								return Ok(false)
							}
							Some(cs)
						},
						None => None,
					};

				// If the header has verified, but its corresponding consensus state
				// differs from the existing consensus state for that height, freeze the
				// client and return the installed consensus state.
				if let Some(cs) = existing_consensus_state {
					if cs != header_consensus_state {
						return Ok(true)
					}
				}

				// Monotonicity checks for timestamps for in-the-middle updates
				// (cs-new, cs-next, cs-latest)
				if header.height() < client_state.latest_height() {
					let maybe_next_cs = ctx
						.next_consensus_state(&client_id, header.height())?
						.map(|cs| {
							cs.downcast::<ConsensusState>().ok_or(
								Ics02Error::client_args_type_mismatch(
									ClientState::<H>::client_type().to_owned(),
								),
							)
						})
						.transpose()?;

					if let Some(next_cs) = maybe_next_cs {
						// New (untrusted) header timestamp cannot occur after next
						// consensus state's height
						if Timestamp::from(header.signed_header.header().time).nanoseconds() >
							next_cs.timestamp().nanoseconds()
						{
							return Err(Error::header_timestamp_too_high(
								header.signed_header.header().time.to_string(),
								next_cs.timestamp().to_string(),
							)
							.into())
						}
					}
				}
				// (cs-trusted, cs-prev, cs-new)
				if header.trusted_height < header.height() {
					let maybe_prev_cs = ctx
						.prev_consensus_state(&client_id, header.height())?
						.map(|cs| {
							cs.downcast::<ConsensusState>().ok_or(
								Ics02Error::client_args_type_mismatch(
									ClientState::<()>::client_type().to_owned(),
								),
							)
						})
						.transpose()?;

					if let Some(prev_cs) = maybe_prev_cs {
						// New (untrusted) header timestamp cannot occur before the
						// previous consensus state's height
						if header.signed_header.header().time < prev_cs.timestamp {
							return Err(Error::header_timestamp_too_low(
								header.signed_header.header().time.to_string(),
								prev_cs.timestamp.to_string(),
							)
							.into())
						}
					}
				}
			},
			ClientMessage::Misbehaviour(_misbehaviour) => return Ok(true),
		};

		Ok(false)
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
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Ics02Error> {
		// TODO:
		Err(Ics02Error::implementation_specific("Not implemented".to_string()))
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
		let value = expected_consensus_state.encode_to_vec();
		verify_membership::<H, _>(client_state, prefix, proof, root, path, value)
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
		client_state.verify_height(height)?;

		let path = ConnectionsPath(connection_id.clone());
		let value = expected_connection_end.encode_vec().expect("encode connection end");
		verify_membership::<H, _>(client_state, prefix, proof, root, path, value)
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
		client_state.verify_height(height)?;

		let path = ChannelEndsPath(port_id.clone(), *channel_id);
		let value = expected_channel_end.encode_vec().expect("encode channel end");
		verify_membership::<H, _>(client_state, prefix, proof, root, path, value)
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
		client_state.verify_height(height)?;

		let path = ClientStatePath(client_id.clone());
		let value = expected_client_state.encode_to_vec();
		verify_membership::<H, _>(client_state, prefix, proof, root, path, value)
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
		client_state.verify_height(height)?;
		verify_delay_passed(ctx, height, connection_end)?;

		let commitment_path =
			CommitmentsPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };

		verify_membership::<H, _>(
			client_state,
			connection_end.counterparty().prefix(),
			proof,
			root,
			commitment_path,
			commitment.into_vec(),
		)
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
		ack_commitment: AcknowledgementCommitment,
	) -> Result<(), Ics02Error> {
		// client state height = consensus state height
		client_state.verify_height(height)?;
		verify_delay_passed(ctx, height, connection_end)?;

		let ack_path = AcksPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };
		verify_membership::<H, _>(
			client_state,
			connection_end.counterparty().prefix(),
			proof,
			root,
			ack_path,
			ack_commitment.into_vec(),
		)
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
		client_state.verify_height(height)?;
		verify_delay_passed(ctx, height, connection_end)?;

		let mut seq_bytes = Vec::new();
		u64::from(sequence).encode(&mut seq_bytes).expect("buffer size too small");

		let seq_path = SeqRecvsPath(port_id.clone(), *channel_id);
		verify_membership::<H, _>(
			client_state,
			connection_end.counterparty().prefix(),
			proof,
			root,
			seq_path,
			seq_bytes,
		)
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
		client_state.verify_height(height)?;
		verify_delay_passed(ctx, height, connection_end)?;

		let receipt_path =
			ReceiptsPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };
		verify_non_membership::<H, _>(
			client_state,
			connection_end.counterparty().prefix(),
			proof,
			root,
			receipt_path,
		)
	}
}

fn verify_membership<H, P>(
	client_state: &ClientState<H>,
	prefix: &CommitmentPrefix,
	proof: &CommitmentProofBytes,
	root: &CommitmentRoot,
	path: P,
	value: Vec<u8>,
) -> Result<(), Ics02Error>
where
	P: Into<Path>,
	H: ics23::HostFunctionsProvider,
{
	let merkle_path = apply_prefix(prefix, vec![path.into().to_string()]);
	let merkle_proof: MerkleProof<H> = RawMerkleProof::try_from(proof.clone())
		.map_err(Ics02Error::invalid_commitment_proof)?
		.into();

	let val = hex::encode(&value);
	let rt = hex::encode(root.as_bytes());
	// use ibc_proto::ics23::{InnerSpec as IbcInnerSpec, LeafOp as IbcLeafOp, ProofSpec as
	// IbcProofSpec};

	let ps: Vec<ProofSpec> = client_state.proof_specs.clone().into();
	let ps_e = ps
		.into_iter()
		.map(|ps| hex::encode(ps.encode_to_vec()))
		.collect::<Vec<_>>()
		.join(",");
	let mp: ibc_proto::ibc::core::commitment::v1::MerkleProof = merkle_proof.clone().into();
	let mp_e = hex::encode(mp.encode_to_vec());
	log::trace!(
		target: "pallet_ibc",
		"verify_membership:\npath: {merkle_path:?},\nvalue: {val},\nmerkle_proof: {mp_e}\nroot: {rt}, client_state.proof_specs: {ps_e}",
	);

	merkle_proof
		.verify_membership(&client_state.proof_specs, root.clone().into(), merkle_path, value, 0)
		.map_err(|e| Error::ics23_error(e).into())
}

#[test]
fn test_vrfy() {

	// path: MerklePath { key_path: ["ibc/", "connections/connection-0"] },
	// value: 0a0930382d7761736d2d3012230a0131120d4f524445525f4f524445524544120f4f524445525f554e4f524445524544180222270a0f30372d74656e6465726d696e742d30120c636f6e6e656374696f6e2d301a060a046962632f,
	// merkle_proof:
	// 0ab7020ab4020a18636f6e6e656374696f6e732f636f6e6e656374696f6e2d30125b0a0930382d7761736d2d3012230a0131120d4f524445525f4f524445524544120f4f524445525f554e4f524445524544180222270a0f30372d74656e6465726d696e742d30120c636f6e6e656374696f6e2d301a060a046962632f1a0b0801180120012a0300024a22290801122502044a20011bc1c28ab4b61b62656895198442ba54218b9fa2334e1446a01f05d1914b4c20222b0801120404064a201a2120a0bd1f7230386e13a60fcdca6b7cc8c9652448d6b7d69e61eb8cc1f94a738cc5222b08011204060a4a201a2120f6c45e0f0e54c5e90e8ada14e4c42458d96daa16e2f2541c329909140a62656922290801122508164a20d15ced6901f7a9803c9d794b8dc5c12f3f07f460277f291acc0b9e5825874c97200afe010afb010a036962631220743afc84a18ab8a8006223e93b487852defe3dd008de50d0f18cede4661ca51c1a090801180120012a01002225080112210146b20f5b2a0f937792824ba03e427a7a78854e70e47b35d2a98acbe53896f913222708011201011a20aa650406ea0d76e39dd43d2ea6a91e3fdaa1c908fc21a7ca68e5e62cc8115639222708011201011a2044d1394394062f36216bd51d54bd7b2fb5536236113be47f70afcb2861bdd0c222250801122101b081fead92211001a4898079d2abc1d6b0e42f0814fd6a3082233b39d08c049d222708011201011a20e9d4b3c6043cee22db019076776b9feab6a4ff2902ececca9cc823591ddeb19e
	// root: cc97df4265ce9c14c954b6a329bd122bf757651e9610da5dfa733635468e367a
	// client_state.proof_specs:
	// 0a090801180120012a0100120c0a02000110211804200c3001,
	// 0a090801180120012a0100120c0a0200011020180120013001

	/*
	path: MerklePath { key_path: ["ibc", "clients/08-wasm-0/consensusStates/2000-5"] },
	value: 0a2b2f6962632e6c69676874636c69656e74732e6772616e6470612e76312e436f6e73656e7375735374617465122f0a0b0886f5ce9e061080feac0d12204c6578ca737c10594b6ae41ce2effd80004af724cb6878e188550062aca3f2bd,
	merkle_proof: 0aa3030aa0030a28636c69656e74732f30382d7761736d2d302f636f6e73656e7375735374617465732f323030302d3512b8010a282f6962632e6c69676874636c69656e74732e7761736d2e76312e436f6e73656e7375735374617465128b010a5e0a2b2f6962632e6c69676874636c69656e74732e6772616e6470612e76312e436f6e73656e7375735374617465122f0a0b0886f5ce9e061080feac0d12204c6578ca737c10594b6ae41ce2effd80004af724cb6878e188550062aca3f2bd1220d457206d53c0b3ed58a9a08f9666fa85e2be377ac279e4b83c324f794ddbf302180122050a030102031a0b0801180120012a0300025e22290801122502045e2073c10a9dd2ebf7d9934e8bd46738201f10d123032e88a65eace5808be42de9902022290801122504065e20c12d6124eb0e64e5af7cd5297690feed35afb77069f798ff05e8e9d7cded6bc520222908011225060c5e20d33d51f822ae771d9e18e087f4d20dd1750d3ed2ab4315902dd5d659a095f33e20222b0801120408165e201a21205c61b8bc43bd38f658a2d5b42baef7e5c43eaa725c1ef468b915cf6ec33967350afe010afb010a03696263122069974259262f532df7012fb8cbb20083ac6ce65527c8eef4eff177221df2928a1a090801180120012a01002225080112210146b20f5b2a0f937792824ba03e427a7a78854e70e47b35d2a98acbe53896f913222708011201011a20aa650406ea0d76e39dd43d2ea6a91e3fdaa1c908fc21a7ca68e5e62cc8115639222708011201011a2037556dff071439ccb57270d48acc651cac48d56d8522ad09ff709c22777064b322250801122101705b4d753d3aa7ee1d8ca4a30b0e59c194a286d8ba7790db489e4d335b4d509c222708011201011a20e42516ea78865e7c530a55a1506d0d2a1053eec0ee1e8b43f0e17d73479da108
	root: 657d8b3689689ae7247aa868ef82d6e4b7c7b8234c78a5e027b210fbe5733154,
	client_state.proof_specs: 0a090801180120012a0100120c0a02000110211804200c3001,0a090801180120012a0100120c0a0200011020180120013001
	 */

	// use ibc_proto::ibc::core::commitment::v1::MerkleProof as IbcMerkleProof;
	//
	// let path = MerklePath { key_path: vec!["ibc".to_string(),
	// "clients/08-wasm-0/consensusStates/2000-5".to_string()] }; let value =
	// hex::decode("
	// 0a2b2f6962632e6c69676874636c69656e74732e6772616e6470612e76312e436f6e73656e7375735374617465122f0a0b0886f5ce9e061080feac0d12204c6578ca737c10594b6ae41ce2effd80004af724cb6878e188550062aca3f2bd"
	// ).unwrap(); let merkle_proof =
	// hex::decode("
	// 0aa3030aa0030a28636c69656e74732f30382d7761736d2d302f636f6e73656e7375735374617465732f323030302d3512b8010a282f6962632e6c69676874636c69656e74732e7761736d2e76312e436f6e73656e7375735374617465128b010a5e0a2b2f6962632e6c69676874636c69656e74732e6772616e6470612e76312e436f6e73656e7375735374617465122f0a0b0886f5ce9e061080feac0d12204c6578ca737c10594b6ae41ce2effd80004af724cb6878e188550062aca3f2bd1220d457206d53c0b3ed58a9a08f9666fa85e2be377ac279e4b83c324f794ddbf302180122050a030102031a0b0801180120012a0300025e22290801122502045e2073c10a9dd2ebf7d9934e8bd46738201f10d123032e88a65eace5808be42de9902022290801122504065e20c12d6124eb0e64e5af7cd5297690feed35afb77069f798ff05e8e9d7cded6bc520222908011225060c5e20d33d51f822ae771d9e18e087f4d20dd1750d3ed2ab4315902dd5d659a095f33e20222b0801120408165e201a21205c61b8bc43bd38f658a2d5b42baef7e5c43eaa725c1ef468b915cf6ec33967350afe010afb010a03696263122069974259262f532df7012fb8cbb20083ac6ce65527c8eef4eff177221df2928a1a090801180120012a01002225080112210146b20f5b2a0f937792824ba03e427a7a78854e70e47b35d2a98acbe53896f913222708011201011a20aa650406ea0d76e39dd43d2ea6a91e3fdaa1c908fc21a7ca68e5e62cc8115639222708011201011a2037556dff071439ccb57270d48acc651cac48d56d8522ad09ff709c22777064b322250801122101705b4d753d3aa7ee1d8ca4a30b0e59c194a286d8ba7790db489e4d335b4d509c222708011201011a20e42516ea78865e7c530a55a1506d0d2a1053eec0ee1e8b43f0e17d73479da108"
	// ).unwrap(); let merkle_proof: MerkleProof<Crypto> =
	// IbcMerkleProof::decode(merkle_proof.as_slice()).unwrap().into(); // let root =
	// CommitmentRoot::from(hex::decode("
	// 657d8b3689689ae7247aa868ef82d6e4b7c7b8234c78a5e027b210fbe5733154").unwrap()); let root =
	// CommitmentRoot::from(hex::decode("
	// 42C9678300A86BF3E3913DE1DC8DADA181DF40A21B0551E69608B8002749F09B").unwrap()); let proof_spec
	// = ProofSpecs::default(); merkle_proof
	// 	.verify_membership(&proof_spec.into(), root.into(), path, value, 0).unwrap();

	// 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d

	// let data =
	// hex::decode("
	// 0aae02080112087472616e736665721a096368616e6e656c2d3022087472616e736665722a096368616e6e656c2d3032ee017b22746f6b656e223a7b2264656e6f6d223a7b2274726163655f70617468223a22222c22626173655f64656e6f6d223a22554e4954227d2c22616d6f756e74223a223130303030303030303030227d2c2273656e646572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764222c227265636569766572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764227d3a05080110f55040c8a9b09f9efd9e803312487b226572726f72223a224142434920636f64653a2032393a206572726f722068616e646c696e67207061636b65743a20736565206576656e747320666f722064657461696c73227d1aff040afb020af8020a3261636b732f706f7274732f7472616e736665722f6368616e6e656c732f6368616e6e656c2d302f73657175656e6365732f311220f9629c2ac8a2a9c9d9fa4c58aa7cc7bf3376a8163d112ab0bbd51edc53b314c81a0c0801180120012a0400028007222c0801120502048007201a212080c1403f48c22f6486c850284f301b1228a45a7901881d6044aed603c97cbf70222c0801120504088007201a2120c0e3d7caaea2029995bf26ba2bb183755e69c529eea063e49277c230e553e747222c08011205060c8007201a2120bce6d533427364c81c70c03ab4e1263e1dfc7d033a8e0449554414fa5aac7d1b222c08011205081c8007201a2120a07a7ad5c34210d768548cc62f5ed1a454efd2887dc64bc41c2cfdb3d6641ed7222c080112050a388007201a212057031fc7df3c37c76aefaeb2c72ecee4d64f97a57da9f5b3a07321dbe97d24f9222c080112050c4a8007201a21208eef2330b019b045f5aa1e5c2d19082f061ecfa06850c0ea4b3047cd639ce5d70afe010afb010a03696263122005a548db72ea898f909f1cb733b92e60b30884eba8aff7a8d97479c59aef2a291a090801180120012a01002225080112210146b20f5b2a0f937792824ba03e427a7a78854e70e47b35d2a98acbe53896f913222708011201011a20aa650406ea0d76e39dd43d2ea6a91e3fdaa1c908fc21a7ca68e5e62cc8115639222708011201011a20a744b9f883cb82286d1e7b28fe1f0343f43b3ac1c30eaee5c1c72eec5640490422250801122101150e42c80336e401b1256b51bb318803ee4f7fc4d2cc74baeea371690cd14e74222708011201011a2083375dd835e7ee7b9b9456ee963b1895ebc4cf5995e0a54ac23ab3eba9d7ebc22205080110c1032a3035794e5a6a5832346e3265673757364556616d6154584e51625743776368685468456153574237563347526a7448654c"
	// ).unwrap(); let msg = MsgAcknowledgement::decode(&data[..]).unwrap();
	// println!("{:?}", msg);
	// let ack = String::from_utf8(msg.acknowledgement).unwrap();
	// println!("{}", ack);
}
/*
{
	"token": {
		"denom": {"trace_path":"","base_denom":"UNIT"},
		"amount":"10000000000"
	},
	"sender":"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
	"receiver":"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
}

Simulated transaction: events: [
	Event { r#type: "coin_spent", attributes: [
		EventAttribute { key: b"spender", value: b"cosmos1nnypkcfrvu3e9dhzeggpn4kh622l4cq7wwwrn0", index: false },
		EventAttribute { key: b"amount", value: b"1000000stake", index: false }
	] },
	Event { r#type: "coin_received", attributes: [
		EventAttribute { key: b"receiver", value: b"cosmos17xpfvakm2amg962yls6f84z3kell8c5lserqta", index: false },
		EventAttribute { key: b"amount", value: b"1000000stake", index: false }
	] },
	Event { r#type: "transfer", attributes: [
		EventAttribute { key: b"recipient", value: b"cosmos17xpfvakm2amg962yls6f84z3kell8c5lserqta", index: false },
		EventAttribute { key: b"sender", value: b"cosmos1nnypkcfrvu3e9dhzeggpn4kh622l4cq7wwwrn0", index: false },
		EventAttribute { key: b"amount", value: b"1000000stake", index: false }
	] },
	Event { r#type: "message", attributes: [
		EventAttribute { key: b"sender", value: b"cosmos1nnypkcfrvu3e9dhzeggpn4kh622l4cq7wwwrn0", index: false }
	] },
	Event { r#type: "tx", attributes: [
		EventAttribute { key: b"fee", value: b"1000000stake", index: false },
		EventAttribute { key: b"fee_payer", value: b"cosmos1nnypkcfrvu3e9dhzeggpn4kh622l4cq7wwwrn0", index: false }
	] },
	Event { r#type: "tx", attributes: [
		EventAttribute { key: b"acc_seq", value: b"cosmos1nnypkcfrvu3e9dhzeggpn4kh622l4cq7wwwrn0/27", index: false }
	] },
	Event { r#type: "tx", attributes: [
		EventAttribute { key: b"signature", value: b"rEZ5Olfzjjvp2Y87aaBN+9a6u9yvO8NYf3TZXZQWyBh9Jsau++ToqDUZCNbk5wNKrqeSxtl+46/HNUxFuTie6Q==", index: false }
	] },
	Event { r#type: "message", attributes: [
		EventAttribute { key: b"action", value: b"/ibc.core.client.v1.MsgUpdateClient", index: false }
	] },
	Event { r#type: "update_client", attributes: [
		EventAttribute { key: b"client_id", value: b"08-wasm-0", index: false },
		EventAttribute { key: b"client_type", value: b"08-wasm", index: false },
		EventAttribute { key: b"consensus_height", value: b"2000-75", index: false },
		EventAttribute { key: b"consensus_heights", value: b"2000-75", index: false },
		EventAttribute { key: b"header", value: b"0a202f6962632e6c69676874636c69656e74732e7761736d2e76312e48656164657212dd310ad2310a232f6962632e6c69676874636c69656e74732e6772616e6470612e76312e48656164657212aa310aab260a2083e407a6043aeecba2bfebd79b448429a1d0c520355b4ef2c663e1ce89f8528e12ba03080000000000000083e407a6043aeecba2bfebd79b448429a1d0c520355b4ef2c663e1ce89f8528e370100000c83e407a6043aeecba2bfebd79b448429a1d0c520355b4ef2c663e1ce89f8528e37010000a65c4ccf8c007c1a7a3a363710e3516a9150f2ab817f6b4f268ce7ba4eb9fae5909a76e39db713412d6632128106c48224a19c66540c8b46efc131b83debe60b439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f83e407a6043aeecba2bfebd79b448429a1d0c520355b4ef2c663e1ce89f8528e3701000010547fe378e02446b407904a5d857d4ec644743fa5ec48f1698e9f60a74c3295e24785a856b9ab0843fe248f5a84b40876376f2f916afcc20522154b45ad0c0a88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee83e407a6043aeecba2bfebd79b448429a1d0c520355b4ef2c663e1ce89f8528e37010000f834ce6766c59f389885a80070b29457533ce9a6c0b91495186992a07e0c3bde4846d2735c8cd81470ded53f191cfbbfc91563df6efef0ca1ce9ac640db9480cd17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69001adf055c70ea93c31d4af86fb56242c2b07c9533494857111f2e1cd2eb7417748143deb50407c52fe7f97b2ae49104f28ae491afc12a20a62b3aec06d8abc730f50b28a76f5be601e808ebd44db63c4d3e31fc3380f3cf52ae3bddd8f012bbb72833213eb3180642414245b5010102000000aeeea310000000007838977654cc9a2e5dc6c26f4c87c37fb2a7259cfbf4992dd644b2400abfda49ca2996ceba1e2607c0c737e3c699d6fd11f0b771561458c117f1423f544c350c534d85d53e1ad78a8c50de77a69a77f15ff5ac4e6ae5bbe35355e5664923f30c04424142456902010cd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01000000000000008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48010000000000000090b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe220100000000000000592c52b0c22fb94af01c4a5927bcc41ce7eb2c804339dad155184cba6c44d4240442454546b501010c020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a10390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f270389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb1e0000000000000004424545468403db36960adac141751c5a87a6a9fc4f129e3a0ac5131cac5177bf4f06d96970700446524e4bf901010c88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee0100000000000000d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae690100000000000000439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f01000000000000000000000005424142450101aa92c63a5143b4e0883324ceb7657421c3a1dfa2d9ce58701f45ebf90f0b1c1c0747ad509ac81f41444cd56d3fca7c3a545f84e50debf65fba7f68d7bf7bf0821ac5022203486d5e0c329318ca4100e83c643a5b85d72632cfac33fd9b2e68fd2982f0b90488ea4bb14a9a6a678390cee5ea44118df8a5719c8024f3f5f1d5c4a73b87ebf718547dd1a69de0417200a5edb71de87f19ffa56989bf8f3ff9a3227c9557ddb20c0642414245b5010101000000afeea310000000005623f2fbff32268694e8ef2f2fec71a0f3186ff099e7d75a731377fee9d86a76685c303536eea176b8a02cd6161743fda2cc7932af39d47ef2edc208183da40bdaf9395691fb0a58df07888cadbf74cc8b9ccabb43f3fed4ad19450b40ab0c0704424545468403d7dbd6f71111add2182ae3a58261288d7247970ce6cf73c9aff284e5d47bdd9805424142450101600ce4a4f883d852eb60daaafc2ecb84cef21a5aa1cedbf79b3c533bb13bbf7f6f545b157f870c840d0acd4b3b5bc91572e4856ef55c2d15a6d2f7bff9c8b5891ac502e9be7e4e5ef5b4957d3e5cceb1f94022e57dd3136dff9533d4169b27631a9983bd04b80207f32bc0e528355966e3cc6c09b9cdbec93c788efefddbca1f2e5c2e021f569031912023d719e9dbcbc425df882e9e4fb8c30529025d5a317199a5c480630c0642414245b5010301000000b0eea3100000000044b4d12a9bf0fb72f3f55235725644ccfd84caf239436a1ffe10f0173114721556f86256c56cc9668f668fb1b0a6ffddb04755268716c8a10bc324c6bd8e4b05633bd745cf756c936d6359f6fd1f23419b5bc1053ed79f9e952f2eea94dce70b044245454684030288efafc19b3eaa33865943f542d0840af3b7d37e1cb808be24c37eb05064ab0542414245010162c14b6fb4f83b3c6a60193c42203eb2bce9bc5157f46bcaf79187ea28a355524583dd7dad78403f00f273d8b2cec4e380694b2e8e60b27a7f7ee8bfd84647841ac502e6e714dd4b0e825ef5f244416d69b6fe7ec29067a30aad389cf7c87de020e966c1040afb2b698cb374ebdb9ae0b0e7cf4ba130908831bef99e0479eac29f64954019640c969d1453b51884595e6703a032c31d15b8660a2873a291c27f321883d9c30c0642414245b5010101000000b1eea31000000000527c65eb029907438fdbb44972565d2c9ad2cb6a4c7f68f9d88ee03d46e4672c7abeab56b0e90da05750c0eb56cf7d2339dd8d0882c009c066916f5669fdbe08752004e1e8acb76613a1b01e7d78fee7a1a750be358476cce60d3014f6b2eb0404424545468403b35343d11ee652fdcd106f898ae50d98d35adf5d552c9a80208f74a09f5b392305424142450101baaf6a220fb8ee14221e9462271e31ab1837f932b539338b7ffa4ccc9a81775750467822db460b46cdf63a9f6b43b6e6887f719474c6166e3c70fb3dd777f2891ac502a79769b1ee5aa1ca668cf77890be544ca976e06aef88d985c923c0e1d3da98bac5049f4bc3253b19f361d437c70624b72eb63d6b9110ddc0404e695582c52e1fde5ad1738782fc88496bd1bb71b5445287158bd810679d4710c48d246c7f7e2997e00c0642414245b5010300000000b2eea31000000000aaf2a33e874aa2070446add547c5e6d235c6c5404f4d0173c54315165c9f655886fc340a874973f8b64bbf74ae3fc472c892e62a5500b35e4488e541170afb0d8f8797697124f90efbdeacfac7a5dd5515e64941943bebb2a8e76b5ad2d2c30f044245454684037de86a1e162ff7c627511af0e158f5cd017344dde4798e464d35b5dc7fd48846054241424501012eaf0aacac8fc9fb034dff963f2cdbca2bf6dd6ae939851cee74f258805ef86edb5193c88ff9bb93ee59472fd780cf852a98ae37335420ca8a25b3e0218ffe811ac5021825286bf8ddf3c753db05c926f05e7ce2ab16d159b0456e51aad83dccad426dc904f06cab05a7197c3cc29461b3d1a0dd7274c11e77c93851bd82e4ef6f85f59800d8b24465e5810fd4bc1c06c6e15ec9327e12c97cff8e9caee3eab269667023b90c0642414245b5010301000000b3eea31000000000846306802145a91f799a5b344edac4487a77df5e50da3bbdf4b73341e1127e37f1cb07c418b9d2ddcb16e323e9544f4046e3f19afe266322a0bb3a7acce7590424d1ca9bb66e238873a0859e4c0cec2dea6ec30c5eae16ddfd8b66a0977f160204424545468403de7951947f4425e3dd3788bafe14ffd468ac6b2977adf66abb9b3f24b8fcc1b705424142450101c0a0703699bfe58f7b234f5f24fe6a6434f9a28323fe38bf51cc211b0be242272a1620e11dbc48b84142f7ce06ae5fb1c9cf4251c74647e286b66ed96d6dea841ac502dcaf5a5b04a1902beed3c675e662e4c75f742d2e5bef8c75141853b0eb3c5358cd0459263ccde0ce9997ebc5c07a4cc9d49089815542cd6b9edf5400085c05eeae4062f57be06e18b4dc214202180c2e6cd751390d10f9a4c10b48dd1e0c46c620e40c0642414245b5010302000000b4eea310000000002ab8549ce1b7c00458d462bba20637921164ed8030bf1511216423acaaeaf26518a9b3ef6e04638201bca921ecedc9247a3d2ff5d771b594db9b1ee53967e1061beea0ca899172b3d7da1adacec62d71fb85a1e20c9d3ec7f37c3765404f6e0404424545468403f9ae3a6eca953fa14235f0123d0f5bdc9124f71ddbc4e3b6d2557dd2593c40a80542414245010114395926cd58827cb9a0be04ace1a37011f353d6c5d501cdea195c27d08fcc43ca255a99dde87b912c710c1d82bfc53d3b8193dcebffcacb0b0046bf512316851ac50263f0c8e27724b59981ccd9f0839bc62f40f9b24ea1eb0d5054c06a0b550aa067d104d9c3c5fd6f10a56866ed327851fd7eb2f745e8eaac4eb940ef8777c7e2bebb3449192edd32acc587dc446545a7611aa5ee731140f1ddaefeb4e8a618ee1594320c0642414245b5010302000000b5eea310000000003cb121439947e1e40906287d01c944431e3a42661c75f1a221ab9a3a1ec25a0a525e9015a945787fcf05a42ed611bb3aca8026ca55057969954b85a06fa81601522f247e731a3f275a6ca014166c258a316c7feffd78e7faf8edc37356c636090442454546840393301120241648563c3d5a92c10d6009bb8f884fea502cd0d09a09ca5a02c07d05424142450101ae8e82ca9f78c131bbed2ab5e09fc65ad2dcadfcaba8ee3fa1861717da969105f37883161ff71cd99e3e8bb15177587509187a68d2958c6220515433aed34d831ac502bb3c15f4c60be7749f841e20e9967a56202e5eb20999824ccdb823b11a4b49dcd50499a98d99dca26026914421824b7995f1d35f4eede1e52250f07fb97ef52f16e5657200bf492e423caf7d03724b2fca77925a21a94ed4b4f005d3e45c1c529c9d0c0642414245b5010301000000b6eea310000000000e9e0226f7b97d7c75b5a454a50961754702429f2f178b5bd6f8551a722de77d2f252205912a76bb310efe93baaa0b15cb979e1e99cb6ee30c993656c1652301f96dcabffae14568c2315bf4241aa3b7f354c05d0cae3a82a6e07232355c690304424545468403b266651b3028472e70a703f486fce9bfb8ab7fc8b7248b0dc1c5ae6889498bb7054241424501013ca1473f80758d04ab36e073fa4533db3cc13329d81c7e8cb75e7bc7e26ee0225ee938fcb85484d1125eea6abc469f9021dc8b715867604084670433eaef55821ac50271f2d3b86da6d439b6f2e7de996c17017f2cb72a69cb6baa79c6b3da7346ea04d9047aa0329933918d4edf542dae93337ecfd77d234fbcc04a754b9fa5c49fe5613862438be04eaaf818bf5e062fd6cc94af101760f3c0947c5bb0da06f5f8fe34940c0642414245b5010302000000b7eea310000000005688337264ddbdc22217164e07746e1737fc6f91c4d255bf809b5af7be99325a6797483eaec9041ef0085faff0dac67be3605ab19aca26e9debde4c5dac66f0ab9743940fbbb8c8025e903afc956d58e41458d59744b4aa7de076e26547237030442454546840307c2bc2ace7e385fb660a9c0ff01bfd6ccdfe79f886ae80137d416bdd37cf930054241424501017ccaa259dc060134bf9ddc0c1e4502aa6287baae28f311d8bb6e6cc7f58c9f33632e15215fc3c376f14e149895b31bd72b07e811874652b8b523c5684bb7dc8a1adf0580fa5502d1a579be5362dc56223ace11c9ae1538cf21de7696fb259b316023a9dd04a4be3e32ddbcb2877361ca83e0b7a3c589f6140cf180cba9ed3b0c0604f00664cccadc6293cf4f56d93c4baf712a1bfc453a940ebd22b5830084e9c209d8cc93180642414245b5010300000000b8eea31000000000b2dc416d4ce0526560ea0d298a87eda2fbc62050c3bf85f4b7a12f02d2b96133e4ac5a6d28f33f2bb77851c15824444579dc433eae5f85cb928d629a1e97310daee1a77aee2da7fd515fd6aaf7b0c70ee1596e5f1554d55dd57fb88166cb540704424142456902010cd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01000000000000008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48010000000000000090b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe220100000000000000a81d95ba9bcc2b1effb14669e8858d26806e9d573b4a4ce7e5bd5a53135da5bc0442454546b501010c020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a10390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f270389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb1f00000000000000044245454684030c8d17cde64f5f4e078df4b9b99f227d85ccd047b60ae449cdfa48df786899d60446524e4bf901010c88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee0100000000000000d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae690100000000000000439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f01000000000000000000000005424142450101a24df9ce7c157f6ef7308f404e4953a323594b269aa40b253ccc90565a6fb643afef3e23b512472acc13d66512f3a78023d9409e4c620893762a31c77ed2c48b12f90a0a2071f2d3b86da6d439b6f2e7de996c17017f2cb72a69cb6baa79c6b3da7346ea0412d40a0ac9015703f5a4efb16ffa83d0070000e902e1024b218505e59e197d29113e64617ba46054045f57b33d4d62b32f8473ee44009d2d01029bd71ff5abd57aacc79fa393c7e4c11638ba0d15a4c0a794fab4bb9ab33dbc3f6ce44a39d4cd116cdc5686b3885888f6859c5b23ff099a18173a6813e8ec45080661757261205af751080000000005617572610101183ca4dc69eb55f4b1b373c0fe053589fed68b78859f64864e76aab1e509730a7ca99c46d40e22d4593d03e63dff5c3dbfc201247b91ba55db521707918235830a870180046480f6cbcf8517e448ae891923f1d9aebdd7a1e435a18d593074ff6cf873ccb4ac4580f6f6801e4b41e2e6d8ec194dba122bfb9eb33feb2545ef5144cea79551f7cc52800dd377883e4b9771ec344b2b783a04fa0eb1fa90350402333fd53ff27556d87680197cd25f1d206ba50b4cf17119e8385edb017ca4839eb45fd42963f843fe46650a930480ffff80b4a069a7a4bccddb0b95636faac2f398d3522741136a7590775b924dbb13fd4a80a4be8e6809c5b7ec8d2e07c18c921128f13164f9620840cadfabfa02df12d6d1800155cd4c836fa988f1e30eafeba5e40fb408638e5cf093a967c1706c19843bcb80c009721cc23b4e8328b83eb6eb81a09dee1661ffef1eaa9a5ee53bef53bb16ae80b01b3f94b732416acef5b96454111342ab728887032b3375d13f2d71b97c4a218072650239a18bae55b8387ead5b7ccf61606cc146d3e8cad5a4e5c9fcbaa6f00f80507fa8accbc4d71d39f99d732cce9ca29113ff480d5192ada20035c8592d0f1080b1f4de7c3ac5665016d561a60659cd2d8f2d3e0a97e2ea9749279bd8e35eb1f180c2d44d371e5fc1f50227d7491ad65ad049630361cefb4ab1844831237609f08380fd1951387bae8ebd97daec3a75a49803f30b31bfefdbad99d2ae88042ba00483806f3280720de901d869f39d47fb95c18aa3d295fab016b5834cab42a452ead5d5801665cc9b477ebbbbfb14fd962a12bc39dc097b279a0d613261c2a6c2f92f1cd6807fdfcff23c36c548b15a00a329cf15d9f558032ef172ec76983405b7e93f2ccb8038800b033fa00cf8055fa68d10c47d4db1f76bbb76521aac2f2377a8431708eb806bfdbbf0e0bedcb993b65c9cea1e929a56d78a3b7bc53d1b7ca6fc488e2295ee80b384dc7b1e29a1a0aa20dbf449574603baa9379a40d70eb0310954f1e36bfc310a95029e710b30bd2eab0352ddcc26417aa1945f43803b3441f15daa8a53147d69c48eac75356fab581febbb8030520b248c5942a14880569b7d21b190fdf20a75a25abdca3d724be2fe4450c57d4b8e4d60ad57508e0e802e2e0716043a02f2f29fdd52922704af194b98545ce5ca832255e8ec41bcdb64806d8521aada95799d403b3e401f4f73c404b332bf36af46b0350ab6a0b9145f6e505f0e7b9012096b41c4eb3aaf947f6ea4290800004c5f03c716fb8fff3de61a883bb76adb34a204008056775d1f97f0dde31ff4ebe582a2d744f7dc84b295497fac971cc40b006134054c5f0f4993f016e2d2f8e5f43be7bb259486040080d802231e59501ebf38055e31bd19f9e68dcbd4c486ddab4c3834a6ea32d9de6b0a559f0b3c252fcb29d88eff4f3de5de4476c3500080657d40ffa9e7f1e10219de6d7da1d36d9b7515180ad98cc5cb3f8ecfeaa8657c807bff6e046aa19b2b4b3f88deda4b184ed3f2432f365a00329d45ba84e92e3bc7120b280401000bf19a2a0286011a6a81001111084000809b691e38492e3c0965c1f422f2ead2c6c96ff1a5b4d0d41ed5a6d67ccb5ecc5b802ef0dc9fdc126cf51b0fdca72f3f16d805d8701bea2cce1cda5f6b311628df6680137ad176283937d5a87015ef12c91871df0538fa4453bb86b4a2558b7f5fd660120608d00f10b702", index: false }
	] },
	Event { r#type: "message", attributes: [
		EventAttribute { key: b"module", value: b"ibc_client", index: false }
	] },
	Event { r#type: "message", attributes: [
		EventAttribute { key: b"action", value: b"/ibc.core.channel.v1.MsgRecvPacket", index: false }
	] },
	Event { r#type: "recv_packet", attributes: [
		EventAttribute { key: b"packet_data", value: b"{\"token\":{\"denom\":{\"trace_path\":\"\",\"base_denom\":\"UNIT\"},\"amount\":\"10000000000\"},\"sender\":\"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\",\"receiver\":\"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\"}", index: false },
		EventAttribute { key: b"packet_data_hex", value: b"7b22746f6b656e223a7b2264656e6f6d223a7b2274726163655f70617468223a22222c22626173655f64656e6f6d223a22554e4954227d2c22616d6f756e74223a223130303030303030303030227d2c2273656e646572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764222c227265636569766572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764227d", index: false },
		EventAttribute { key: b"packet_timeout_height", value: b"1-10357", index: false },
		EventAttribute { key: b"packet_timeout_timestamp", value: b"3675073540415821000", index: false },
		EventAttribute { key: b"packet_sequence", value: b"1", index: false },
		EventAttribute { key: b"packet_src_port", value: b"transfer", index: false },
		EventAttribute { key: b"packet_src_channel", value: b"channel-0", index: false },
		EventAttribute { key: b"packet_dst_port", value: b"transfer", index: false },
		EventAttribute { key: b"packet_dst_channel", value: b"channel-0", index: false },
		EventAttribute { key: b"packet_channel_ordering", value: b"ORDER_UNORDERED", index: false },
		EventAttribute { key: b"packet_connection", value: b"connection-0", index: false }
	] },
	Event { r#type: "message", attributes: [
		EventAttribute { key: b"module", value: b"ibc_channel", index: false }
	] },
	Event { r#type: "fungible_token_packet", attributes: [
		EventAttribute { key: b"module", value: b"transfer", index: false },
		EventAttribute { key: b"sender", value: b"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d", index: false },
		EventAttribute { key: b"receiver", value: b"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d", index: false },
		EventAttribute { key: b"denom", value: b"", index: false }, EventAttribute { key: b"amount", value: b"", index: false },
		EventAttribute { key: b"success", value: b"false", index: false },
		EventAttribute { key: b"error", value: b"cannot unmarshal ICS-20 transfer packet data: invalid type", index: false }
	] },
	Event { r#type: "write_acknowledgement", attributes: [
		EventAttribute { key: b"packet_data", value: b"{\"token\":{\"denom\":{\"trace_path\":\"\",\"base_denom\":\"UNIT\"},\"amount\":\"10000000000\"},\"sender\":\"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\",\"receiver\":\"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\"}", index: false },
		EventAttribute { key: b"packet_data_hex", value: b"7b22746f6b656e223a7b2264656e6f6d223a7b2274726163655f70617468223a22222c22626173655f64656e6f6d223a22554e4954227d2c22616d6f756e74223a223130303030303030303030227d2c2273656e646572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764222c227265636569766572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764227d", index: false },
		EventAttribute { key: b"packet_timeout_height", value: b"1-10357", index: false },
		EventAttribute { key: b"packet_timeout_timestamp", value: b"3675073540415821000", index: false },
		EventAttribute { key: b"packet_sequence", value: b"1", index: false },
		EventAttribute { key: b"packet_src_port", value: b"transfer", index: false },
		EventAttribute { key: b"packet_src_channel", value: b"channel-0", index: false },
		EventAttribute { key: b"packet_dst_port", value: b"transfer", index: false },
		EventAttribute { key: b"packet_dst_channel", value: b"channel-0", index: false },
		EventAttribute { key: b"packet_ack", value: b"{\"error\":\"ABCI code: 29: error handling packet: see events for details\"}", index: false },
		EventAttribute { key: b"packet_ack_hex", value: b"7b226572726f72223a224142434920636f64653a2032393a206572726f722068616e646c696e67207061636b65743a20736565206576656e747320666f722064657461696c73227d", index: false },
		EventAttribute { key: b"packet_connection", value: b"connection-0", index: false }
	] },
	Event { r#type: "message", attributes: [EventAttribute { key: b"module", value: b"ibc_channel", index: false }] }]





logs: [{"msg_index":0,"events":[
	{"type":"message","attributes":[{"key":"action","value":"/ibc.core.client.v1.MsgUpdateClient"},{"key":"module","value":"ibc_client"}]},
	{"type":"update_client","attributes":[{"key":"client_id","value":"08-wasm-0"},{"key":"client_type","value":"08-wasm"},{"key":"consensus_height","value":"2000-75"},{"key":"consensus_heights","value":"2000-75"},{"key":"header","value":"0a202f6962632e6c69676874636c69656e74732e7761736d2e76312e48656164657212dd310ad2310a232f6962632e6c69676874636c69656e74732e6772616e6470612e76312e48656164657212aa310aab260a2083e407a6043aeecba2bfebd79b448429a1d0c520355b4ef2c663e1ce89f8528e12ba03080000000000000083e407a6043aeecba2bfebd79b448429a1d0c520355b4ef2c663e1ce89f8528e370100000c83e407a6043aeecba2bfebd79b448429a1d0c520355b4ef2c663e1ce89f8528e37010000a65c4ccf8c007c1a7a3a363710e3516a9150f2ab817f6b4f268ce7ba4eb9fae5909a76e39db713412d6632128106c48224a19c66540c8b46efc131b83debe60b439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f83e407a6043aeecba2bfebd79b448429a1d0c520355b4ef2c663e1ce89f8528e3701000010547fe378e02446b407904a5d857d4ec644743fa5ec48f1698e9f60a74c3295e24785a856b9ab0843fe248f5a84b40876376f2f916afcc20522154b45ad0c0a88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee83e407a6043aeecba2bfebd79b448429a1d0c520355b4ef2c663e1ce89f8528e37010000f834ce6766c59f389885a80070b29457533ce9a6c0b91495186992a07e0c3bde4846d2735c8cd81470ded53f191cfbbfc91563df6efef0ca1ce9ac640db9480cd17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69001adf055c70ea93c31d4af86fb56242c2b07c9533494857111f2e1cd2eb7417748143deb50407c52fe7f97b2ae49104f28ae491afc12a20a62b3aec06d8abc730f50b28a76f5be601e808ebd44db63c4d3e31fc3380f3cf52ae3bddd8f012bbb72833213eb3180642414245b5010102000000aeeea310000000007838977654cc9a2e5dc6c26f4c87c37fb2a7259cfbf4992dd644b2400abfda49ca2996ceba1e2607c0c737e3c699d6fd11f0b771561458c117f1423f544c350c534d85d53e1ad78a8c50de77a69a77f15ff5ac4e6ae5bbe35355e5664923f30c04424142456902010cd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01000000000000008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48010000000000000090b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe220100000000000000592c52b0c22fb94af01c4a5927bcc41ce7eb2c804339dad155184cba6c44d4240442454546b501010c020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a10390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f270389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb1e0000000000000004424545468403db36960adac141751c5a87a6a9fc4f129e3a0ac5131cac5177bf4f06d96970700446524e4bf901010c88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee0100000000000000d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae690100000000000000439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f01000000000000000000000005424142450101aa92c63a5143b4e0883324ceb7657421c3a1dfa2d9ce58701f45ebf90f0b1c1c0747ad509ac81f41444cd56d3fca7c3a545f84e50debf65fba7f68d7bf7bf0821ac5022203486d5e0c329318ca4100e83c643a5b85d72632cfac33fd9b2e68fd2982f0b90488ea4bb14a9a6a678390cee5ea44118df8a5719c8024f3f5f1d5c4a73b87ebf718547dd1a69de0417200a5edb71de87f19ffa56989bf8f3ff9a3227c9557ddb20c0642414245b5010101000000afeea310000000005623f2fbff32268694e8ef2f2fec71a0f3186ff099e7d75a731377fee9d86a76685c303536eea176b8a02cd6161743fda2cc7932af39d47ef2edc208183da40bdaf9395691fb0a58df07888cadbf74cc8b9ccabb43f3fed4ad19450b40ab0c0704424545468403d7dbd6f71111add2182ae3a58261288d7247970ce6cf73c9aff284e5d47bdd9805424142450101600ce4a4f883d852eb60daaafc2ecb84cef21a5aa1cedbf79b3c533bb13bbf7f6f545b157f870c840d0acd4b3b5bc91572e4856ef55c2d15a6d2f7bff9c8b5891ac502e9be7e4e5ef5b4957d3e5cceb1f94022e57dd3136dff9533d4169b27631a9983bd04b80207f32bc0e528355966e3cc6c09b9cdbec93c788efefddbca1f2e5c2e021f569031912023d719e9dbcbc425df882e9e4fb8c30529025d5a317199a5c480630c0642414245b5010301000000b0eea3100000000044b4d12a9bf0fb72f3f55235725644ccfd84caf239436a1ffe10f0173114721556f86256c56cc9668f668fb1b0a6ffddb04755268716c8a10bc324c6bd8e4b05633bd745cf756c936d6359f6fd1f23419b5bc1053ed79f9e952f2eea94dce70b044245454684030288efafc19b3eaa33865943f542d0840af3b7d37e1cb808be24c37eb05064ab0542414245010162c14b6fb4f83b3c6a60193c42203eb2bce9bc5157f46bcaf79187ea28a355524583dd7dad78403f00f273d8b2cec4e380694b2e8e60b27a7f7ee8bfd84647841ac502e6e714dd4b0e825ef5f244416d69b6fe7ec29067a30aad389cf7c87de020e966c1040afb2b698cb374ebdb9ae0b0e7cf4ba130908831bef99e0479eac29f64954019640c969d1453b51884595e6703a032c31d15b8660a2873a291c27f321883d9c30c0642414245b5010101000000b1eea31000000000527c65eb029907438fdbb44972565d2c9ad2cb6a4c7f68f9d88ee03d46e4672c7abeab56b0e90da05750c0eb56cf7d2339dd8d0882c009c066916f5669fdbe08752004e1e8acb76613a1b01e7d78fee7a1a750be358476cce60d3014f6b2eb0404424545468403b35343d11ee652fdcd106f898ae50d98d35adf5d552c9a80208f74a09f5b392305424142450101baaf6a220fb8ee14221e9462271e31ab1837f932b539338b7ffa4ccc9a81775750467822db460b46cdf63a9f6b43b6e6887f719474c6166e3c70fb3dd777f2891ac502a79769b1ee5aa1ca668cf77890be544ca976e06aef88d985c923c0e1d3da98bac5049f4bc3253b19f361d437c70624b72eb63d6b9110ddc0404e695582c52e1fde5ad1738782fc88496bd1bb71b5445287158bd810679d4710c48d246c7f7e2997e00c0642414245b5010300000000b2eea31000000000aaf2a33e874aa2070446add547c5e6d235c6c5404f4d0173c54315165c9f655886fc340a874973f8b64bbf74ae3fc472c892e62a5500b35e4488e541170afb0d8f8797697124f90efbdeacfac7a5dd5515e64941943bebb2a8e76b5ad2d2c30f044245454684037de86a1e162ff7c627511af0e158f5cd017344dde4798e464d35b5dc7fd48846054241424501012eaf0aacac8fc9fb034dff963f2cdbca2bf6dd6ae939851cee74f258805ef86edb5193c88ff9bb93ee59472fd780cf852a98ae37335420ca8a25b3e0218ffe811ac5021825286bf8ddf3c753db05c926f05e7ce2ab16d159b0456e51aad83dccad426dc904f06cab05a7197c3cc29461b3d1a0dd7274c11e77c93851bd82e4ef6f85f59800d8b24465e5810fd4bc1c06c6e15ec9327e12c97cff8e9caee3eab269667023b90c0642414245b5010301000000b3eea31000000000846306802145a91f799a5b344edac4487a77df5e50da3bbdf4b73341e1127e37f1cb07c418b9d2ddcb16e323e9544f4046e3f19afe266322a0bb3a7acce7590424d1ca9bb66e238873a0859e4c0cec2dea6ec30c5eae16ddfd8b66a0977f160204424545468403de7951947f4425e3dd3788bafe14ffd468ac6b2977adf66abb9b3f24b8fcc1b705424142450101c0a0703699bfe58f7b234f5f24fe6a6434f9a28323fe38bf51cc211b0be242272a1620e11dbc48b84142f7ce06ae5fb1c9cf4251c74647e286b66ed96d6dea841ac502dcaf5a5b04a1902beed3c675e662e4c75f742d2e5bef8c75141853b0eb3c5358cd0459263ccde0ce9997ebc5c07a4cc9d49089815542cd6b9edf5400085c05eeae4062f57be06e18b4dc214202180c2e6cd751390d10f9a4c10b48dd1e0c46c620e40c0642414245b5010302000000b4eea310000000002ab8549ce1b7c00458d462bba20637921164ed8030bf1511216423acaaeaf26518a9b3ef6e04638201bca921ecedc9247a3d2ff5d771b594db9b1ee53967e1061beea0ca899172b3d7da1adacec62d71fb85a1e20c9d3ec7f37c3765404f6e0404424545468403f9ae3a6eca953fa14235f0123d0f5bdc9124f71ddbc4e3b6d2557dd2593c40a80542414245010114395926cd58827cb9a0be04ace1a37011f353d6c5d501cdea195c27d08fcc43ca255a99dde87b912c710c1d82bfc53d3b8193dcebffcacb0b0046bf512316851ac50263f0c8e27724b59981ccd9f0839bc62f40f9b24ea1eb0d5054c06a0b550aa067d104d9c3c5fd6f10a56866ed327851fd7eb2f745e8eaac4eb940ef8777c7e2bebb3449192edd32acc587dc446545a7611aa5ee731140f1ddaefeb4e8a618ee1594320c0642414245b5010302000000b5eea310000000003cb121439947e1e40906287d01c944431e3a42661c75f1a221ab9a3a1ec25a0a525e9015a945787fcf05a42ed611bb3aca8026ca55057969954b85a06fa81601522f247e731a3f275a6ca014166c258a316c7feffd78e7faf8edc37356c636090442454546840393301120241648563c3d5a92c10d6009bb8f884fea502cd0d09a09ca5a02c07d05424142450101ae8e82ca9f78c131bbed2ab5e09fc65ad2dcadfcaba8ee3fa1861717da969105f37883161ff71cd99e3e8bb15177587509187a68d2958c6220515433aed34d831ac502bb3c15f4c60be7749f841e20e9967a56202e5eb20999824ccdb823b11a4b49dcd50499a98d99dca26026914421824b7995f1d35f4eede1e52250f07fb97ef52f16e5657200bf492e423caf7d03724b2fca77925a21a94ed4b4f005d3e45c1c529c9d0c0642414245b5010301000000b6eea310000000000e9e0226f7b97d7c75b5a454a50961754702429f2f178b5bd6f8551a722de77d2f252205912a76bb310efe93baaa0b15cb979e1e99cb6ee30c993656c1652301f96dcabffae14568c2315bf4241aa3b7f354c05d0cae3a82a6e07232355c690304424545468403b266651b3028472e70a703f486fce9bfb8ab7fc8b7248b0dc1c5ae6889498bb7054241424501013ca1473f80758d04ab36e073fa4533db3cc13329d81c7e8cb75e7bc7e26ee0225ee938fcb85484d1125eea6abc469f9021dc8b715867604084670433eaef55821ac50271f2d3b86da6d439b6f2e7de996c17017f2cb72a69cb6baa79c6b3da7346ea04d9047aa0329933918d4edf542dae93337ecfd77d234fbcc04a754b9fa5c49fe5613862438be04eaaf818bf5e062fd6cc94af101760f3c0947c5bb0da06f5f8fe34940c0642414245b5010302000000b7eea310000000005688337264ddbdc22217164e07746e1737fc6f91c4d255bf809b5af7be99325a6797483eaec9041ef0085faff0dac67be3605ab19aca26e9debde4c5dac66f0ab9743940fbbb8c8025e903afc956d58e41458d59744b4aa7de076e26547237030442454546840307c2bc2ace7e385fb660a9c0ff01bfd6ccdfe79f886ae80137d416bdd37cf930054241424501017ccaa259dc060134bf9ddc0c1e4502aa6287baae28f311d8bb6e6cc7f58c9f33632e15215fc3c376f14e149895b31bd72b07e811874652b8b523c5684bb7dc8a1adf0580fa5502d1a579be5362dc56223ace11c9ae1538cf21de7696fb259b316023a9dd04a4be3e32ddbcb2877361ca83e0b7a3c589f6140cf180cba9ed3b0c0604f00664cccadc6293cf4f56d93c4baf712a1bfc453a940ebd22b5830084e9c209d8cc93180642414245b5010300000000b8eea31000000000b2dc416d4ce0526560ea0d298a87eda2fbc62050c3bf85f4b7a12f02d2b96133e4ac5a6d28f33f2bb77851c15824444579dc433eae5f85cb928d629a1e97310daee1a77aee2da7fd515fd6aaf7b0c70ee1596e5f1554d55dd57fb88166cb540704424142456902010cd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01000000000000008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48010000000000000090b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe220100000000000000a81d95ba9bcc2b1effb14669e8858d26806e9d573b4a4ce7e5bd5a53135da5bc0442454546b501010c020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a10390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f270389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb1f00000000000000044245454684030c8d17cde64f5f4e078df4b9b99f227d85ccd047b60ae449cdfa48df786899d60446524e4bf901010c88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee0100000000000000d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae690100000000000000439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f01000000000000000000000005424142450101a24df9ce7c157f6ef7308f404e4953a323594b269aa40b253ccc90565a6fb643afef3e23b512472acc13d66512f3a78023d9409e4c620893762a31c77ed2c48b12f90a0a2071f2d3b86da6d439b6f2e7de996c17017f2cb72a69cb6baa79c6b3da7346ea0412d40a0ac9015703f5a4efb16ffa83d0070000e902e1024b218505e59e197d29113e64617ba46054045f57b33d4d62b32f8473ee44009d2d01029bd71ff5abd57aacc79fa393c7e4c11638ba0d15a4c0a794fab4bb9ab33dbc3f6ce44a39d4cd116cdc5686b3885888f6859c5b23ff099a18173a6813e8ec45080661757261205af751080000000005617572610101183ca4dc69eb55f4b1b373c0fe053589fed68b78859f64864e76aab1e509730a7ca99c46d40e22d4593d03e63dff5c3dbfc201247b91ba55db521707918235830a870180046480f6cbcf8517e448ae891923f1d9aebdd7a1e435a18d593074ff6cf873ccb4ac4580f6f6801e4b41e2e6d8ec194dba122bfb9eb33feb2545ef5144cea79551f7cc52800dd377883e4b9771ec344b2b783a04fa0eb1fa90350402333fd53ff27556d87680197cd25f1d206ba50b4cf17119e8385edb017ca4839eb45fd42963f843fe46650a930480ffff80b4a069a7a4bccddb0b95636faac2f398d3522741136a7590775b924dbb13fd4a80a4be8e6809c5b7ec8d2e07c18c921128f13164f9620840cadfabfa02df12d6d1800155cd4c836fa988f1e30eafeba5e40fb408638e5cf093a967c1706c19843bcb80c009721cc23b4e8328b83eb6eb81a09dee1661ffef1eaa9a5ee53bef53bb16ae80b01b3f94b732416acef5b96454111342ab728887032b3375d13f2d71b97c4a218072650239a18bae55b8387ead5b7ccf61606cc146d3e8cad5a4e5c9fcbaa6f00f80507fa8accbc4d71d39f99d732cce9ca29113ff480d5192ada20035c8592d0f1080b1f4de7c3ac5665016d561a60659cd2d8f2d3e0a97e2ea9749279bd8e35eb1f180c2d44d371e5fc1f50227d7491ad65ad049630361cefb4ab1844831237609f08380fd1951387bae8ebd97daec3a75a49803f30b31bfefdbad99d2ae88042ba00483806f3280720de901d869f39d47fb95c18aa3d295fab016b5834cab42a452ead5d5801665cc9b477ebbbbfb14fd962a12bc39dc097b279a0d613261c2a6c2f92f1cd6807fdfcff23c36c548b15a00a329cf15d9f558032ef172ec76983405b7e93f2ccb8038800b033fa00cf8055fa68d10c47d4db1f76bbb76521aac2f2377a8431708eb806bfdbbf0e0bedcb993b65c9cea1e929a56d78a3b7bc53d1b7ca6fc488e2295ee80b384dc7b1e29a1a0aa20dbf449574603baa9379a40d70eb0310954f1e36bfc310a95029e710b30bd2eab0352ddcc26417aa1945f43803b3441f15daa8a53147d69c48eac75356fab581febbb8030520b248c5942a14880569b7d21b190fdf20a75a25abdca3d724be2fe4450c57d4b8e4d60ad57508e0e802e2e0716043a02f2f29fdd52922704af194b98545ce5ca832255e8ec41bcdb64806d8521aada95799d403b3e401f4f73c404b332bf36af46b0350ab6a0b9145f6e505f0e7b9012096b41c4eb3aaf947f6ea4290800004c5f03c716fb8fff3de61a883bb76adb34a204008056775d1f97f0dde31ff4ebe582a2d744f7dc84b295497fac971cc40b006134054c5f0f4993f016e2d2f8e5f43be7bb259486040080d802231e59501ebf38055e31bd19f9e68dcbd4c486ddab4c3834a6ea32d9de6b0a559f0b3c252fcb29d88eff4f3de5de4476c3500080657d40ffa9e7f1e10219de6d7da1d36d9b7515180ad98cc5cb3f8ecfeaa8657c807bff6e046aa19b2b4b3f88deda4b184ed3f2432f365a00329d45ba84e92e3bc7120b280401000bf19a2a0286011a6a81001111084000809b691e38492e3c0965c1f422f2ead2c6c96ff1a5b4d0d41ed5a6d67ccb5ecc5b802ef0dc9fdc126cf51b0fdca72f3f16d805d8701bea2cce1cda5f6b311628df6680137ad176283937d5a87015ef12c91871df0538fa4453bb86b4a2558b7f5fd660120608d00f10b702"}]}
]},
	{"msg_index":1,"events":[
	{"type":"fungible_token_packet","attributes":[
		{"key":"module","value":"transfer"},{"key":"sender","value":"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"},
		{"key":"receiver","value":"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"},
		{"key":"denom"},
		{"key":"amount"},
		{"key":"success","value":"false"},
		{"key":"error","value":"cannot unmarshal ICS-20 transfer packet data: invalid type"}
	]},
	{"type":"message","attributes":[
		{"key":"action","value":"/ibc.core.channel.v1.MsgRecvPacket"},
		{"key":"module","value":"ibc_channel"},
		{"key":"module","value":"ibc_channel"}
	]},
	{"type":"recv_packet","attributes":[
		{"key":"packet_data","value":"{\"token\":{\"denom\":{\"trace_path\":\"\",\"base_denom\":\"UNIT\"},\"amount\":\"10000000000\"},\"sender\":\"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\",\"receiver\":\"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\"}"},
		{"key":"packet_data_hex","value":"7b22746f6b656e223a7b2264656e6f6d223a7b2274726163655f70617468223a22222c22626173655f64656e6f6d223a22554e4954227d2c22616d6f756e74223a223130303030303030303030227d2c2273656e646572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764222c227265636569766572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764227d"},
		{"key":"packet_timeout_height","value":"1-10357"},
		{"key":"packet_timeout_timestamp","value":"3675073540415821000"},
		{"key":"packet_sequence","value":"1"},
		{"key":"packet_src_port","value":"transfer"},
		{"key":"packet_src_channel","value":"channel-0"},
		{"key":"packet_dst_port","value":"transfer"},
		{"key":"packet_dst_channel","value":"channel-0"},
		{"key":"packet_channel_ordering","value":"ORDER_UNORDERED"},
		{"key":"packet_connection","value":"connection-0"}
	]},
	{"type":"write_acknowledgement","attributes":[
	{"key":"packet_data","value":"{\"token\":{\"denom\":{\"trace_path\":\"\",\"base_denom\":\"UNIT\"},\"amount\":\"10000000000\"},\"sender\":\"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\",\"receiver\":\"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\"}"},{"key":"packet_data_hex","value":"7b22746f6b656e223a7b2264656e6f6d223a7b2274726163655f70617468223a22222c22626173655f64656e6f6d223a22554e4954227d2c22616d6f756e74223a223130303030303030303030227d2c2273656e646572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764222c227265636569766572223a22307864343335393363373135666464333163363131343161626430346139396664363832326338353538383534636364653339613536383465376135366461323764227d"},{"key":"packet_timeout_height","value":"1-10357"},{"key":"packet_timeout_timestamp","value":"3675073540415821000"},{"key":"packet_sequence","value":"1"},{"key":"packet_src_port","value":"transfer"},{"key":"packet_src_channel","value":"channel-0"},{"key":"packet_dst_port","value":"transfer"},{"key":"packet_dst_channel","value":"channel-0"},{"key":"packet_ack","value":"{\"error\":\"ABCI code: 29: error handling packet: see events for details\"}"},{"key":"packet_ack_hex","value":"7b226572726f72223a224142434920636f64653a2032393a206572726f722068616e646c696e67207061636b65743a20736565206576656e747320666f722064657461696c73227d"},{"key":"packet_connection","value":"connection-0"}]}]}]

 */
fn verify_non_membership<H, P>(
	client_state: &ClientState<H>,
	prefix: &CommitmentPrefix,
	proof: &CommitmentProofBytes,
	root: &CommitmentRoot,
	path: P,
) -> Result<(), Ics02Error>
where
	P: Into<Path>,
	H: ics23::HostFunctionsProvider,
{
	let merkle_path = apply_prefix(prefix, vec![path.into().to_string()]);
	let merkle_proof: MerkleProof<H> = RawMerkleProof::try_from(proof.clone())
		.map_err(Ics02Error::invalid_commitment_proof)?
		.into();

	merkle_proof
		.verify_non_membership(&client_state.proof_specs, root.clone().into(), merkle_path)
		.map_err(|e| Error::ics23_error(e).into())
}

fn verify_delay_passed<Ctx: ReaderContext>(
	ctx: &Ctx,
	height: Height,
	connection_end: &ConnectionEnd,
) -> Result<(), Ics02Error> {
	let current_timestamp = ctx.host_timestamp();
	let current_height = ctx.host_height();

	let client_id = connection_end.client_id();
	let processed_time = ctx
		.client_update_time(client_id, height)
		.map_err(|_| Error::processed_time_not_found(client_id.clone(), height))?;
	let processed_height = ctx
		.client_update_height(client_id, height)
		.map_err(|_| Error::processed_height_not_found(client_id.clone(), height))?;

	let delay_period_time = connection_end.delay_period();
	let delay_period_height = ctx.block_delay(delay_period_time);

	ClientState::<()>::verify_delay_passed(
		current_timestamp,
		current_height,
		processed_time,
		processed_height,
		delay_period_time,
		delay_period_height,
	)
	.map_err(|e| e.into())
}
