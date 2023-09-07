use std::{fmt::Debug, sync::Arc, thread, time::Duration};

use crate::{
	client::{ClientError, EthereumClient},
	contract::IbcHandler,
	ibc_provider::BlockHeight,
	yui_types::{ics03_connection::conn_open_try::YuiMsgConnectionOpenTry, IntoToken},
};
use channel_msgs::{
	acknowledgement::MsgAcknowledgement, chan_close_confirm::MsgChannelCloseConfirm,
	chan_close_init::MsgChannelCloseInit, chan_open_ack::MsgChannelOpenAck,
	chan_open_confirm::MsgChannelOpenConfirm, chan_open_init::MsgChannelOpenInit,
	chan_open_try::MsgChannelOpenTry, recv_packet::MsgRecvPacket, timeout::MsgTimeout,
	timeout_on_close::MsgTimeoutOnClose,
};
use ethers::{
	abi::{ParamType, Token},
	prelude::EthAbiType,
	providers::Middleware,
	types::H256,
};
use futures::{Stream, StreamExt};
use ibc::{
	core::{
		ics02_client::{
			events::UpdateClient,
			msgs::{create_client::MsgCreateAnyClient, update_client::MsgUpdateAnyClient},
			trust_threshold::TrustThreshold,
		},
		ics03_connection::msgs::{
			conn_open_ack::MsgConnectionOpenAck, conn_open_confirm::MsgConnectionOpenConfirm,
			conn_open_init::MsgConnectionOpenInit, conn_open_try::MsgConnectionOpenTry,
		},
		ics04_channel::msgs as channel_msgs,
		ics23_commitment::commitment::CommitmentRoot,
	},
	protobuf::{
		google::protobuf::Timestamp,
		types::{PartSetHeader, ValidatorSet},
		Protobuf,
	},
	Height,
};
use ics07_tendermint::{
	client_message::{ClientMessage, Header},
	client_state::ClientState,
	consensus_state::ConsensusState,
};
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState, AnyConsensusState};
use primitives::{
	mock::LocalClientTypes, Chain, CommonClientState, IbcProvider, LightClientSync,
	MisbehaviourHandler,
};
use serde::__private::de;
use tendermint::{
	account, block,
	block::{header::Version, signed_header::SignedHeader, Height as TmHeight},
	trust_threshold::TrustThresholdFraction,
	AppHash, Hash, Time,
};

#[async_trait::async_trait]
impl MisbehaviourHandler for EthereumClient {
	async fn check_for_misbehaviour<C>(
		&self,
		counterparty: &C,
		client_message: AnyClientMessage,
	) -> Result<(), anyhow::Error> {
		// assume no misbehaviour occurs for now.
		Ok(())
	}
}

fn client_state_abi_token<H: Debug>(client: ClientState<H>) -> Token {
	use ethers::abi::{encode as ethers_encode, Token as EthersToken};

	log::info!("client: {:?}", client);

	let client_state_data = EthersToken::Tuple(
		[
			//chain_id
			EthersToken::String(client.chain_id.as_str().to_string()),
			//trust_level
			EthersToken::Tuple(
				[
					//numerator
					EthersToken::Uint(client.trust_level.numerator().into()),
					//denominator
					EthersToken::Uint(client.trust_level.denominator().into()),
				]
				.to_vec(),
			),
			//trusting_period
			EthersToken::Tuple(
				[
					EthersToken::Int(client.trusting_period.as_secs().into()),
					EthersToken::Int(client.trusting_period.as_millis().into()),
				]
				.to_vec(),
			),
			//unbonding_period
			EthersToken::Tuple(
				[
					EthersToken::Int(client.unbonding_period.as_secs().into()),
					EthersToken::Int(client.unbonding_period.as_nanos().into()),
				]
				.to_vec(),
			),
			//max_clock_drift
			EthersToken::Tuple(
				[
					EthersToken::Int(client.max_clock_drift.as_secs().into()),
					EthersToken::Int(client.max_clock_drift.as_nanos().into()),
				]
				.to_vec(),
			),
			//frozen_height
			EthersToken::Int(
				client.frozen_height.unwrap_or(Height::default()).revision_height.into(),
			),
			//latest_height
			EthersToken::Int(client.latest_height.revision_height.into()),
			//allow_update_after_expiry
			EthersToken::Bool(true), //TODO check if this is correct
			//allow_update_after_misbehaviour
			EthersToken::Bool(true), //TODO check if this is correct
		]
		.to_vec(),
	);
	client_state_data
}

pub(crate) fn client_state_from_abi_token<H>(token: Token) -> Result<ClientState<H>, ClientError> {
	use ethers::abi::Token as EthersToken;

	let Token::Bytes(bytes) = token else {
		return Err(ClientError::Other("invalid client state".to_string()))
	};
	let params = vec![ParamType::Tuple(vec![
		ParamType::String,
		ParamType::Tuple(vec![ParamType::Uint(256), ParamType::Uint(256)]),
		ParamType::Tuple(vec![ParamType::Int(256), ParamType::Int(256)]),
		ParamType::Tuple(vec![ParamType::Int(256), ParamType::Int(256)]),
		ParamType::Tuple(vec![ParamType::Int(256), ParamType::Int(256)]),
		ParamType::Int(256),
		ParamType::Int(256),
		ParamType::Bool,
		ParamType::Bool,
	])];

	let Token::Tuple(toks) = ethers::abi::decode(&params, bytes.as_slice())?.pop().unwrap() else {
		return Err(ClientError::Other("invalid client state'".to_string()))
	};

	log::info!("toks: {:?}", toks);

	let chain_id = match toks.get(0).cloned().unwrap() {
		EthersToken::String(chain_id) => chain_id,
		_ => return Err(ClientError::Other("chain_id not found".to_string())),
	};

	let (trust_level_numerator, trust_level_denominator) = match toks.get(1).cloned().unwrap() {
		EthersToken::Tuple(toks) => (
			match toks.get(0).cloned().unwrap() {
				EthersToken::Uint(numerator) => numerator.as_u64(),
				_ => return Err(ClientError::Other("trust_level_numerator not found".to_string())),
			},
			match toks.get(1).cloned().unwrap() {
				EthersToken::Uint(denominator) => denominator.as_u64(),
				_ =>
					return Err(ClientError::Other("trust_level_denominator not found".to_string())),
			},
		),
		_ => return Err(ClientError::Other("trust_level not found".to_string())),
	};

	let (trusting_period_secs, trusting_period_nanos) = match toks.get(2).cloned().unwrap() {
		EthersToken::Tuple(toks) => (
			match toks.get(0).cloned().unwrap() {
				EthersToken::Int(numerator) => numerator.as_u64(),
				_ => return Err(ClientError::Other("trusting_period_secs not found".to_string())),
			},
			match toks.get(1).cloned().unwrap() {
				EthersToken::Int(denominator) => denominator.as_u64(),
				_ => return Err(ClientError::Other("trusting_period_nanos not found".to_string())),
			},
		),
		_ => return Err(ClientError::Other("trusting_period not found".to_string())),
	};

	let (unbonding_period_secs, unbonding_period_nanos) = match toks.get(3).cloned().unwrap() {
		EthersToken::Tuple(toks) => (
			match toks.get(0).cloned().unwrap() {
				EthersToken::Int(numerator) => numerator.as_u64(),
				_ => return Err(ClientError::Other("unbonding_period_secs not found".to_string())),
			},
			match toks.get(1).cloned().unwrap() {
				EthersToken::Int(denominator) => denominator.as_u64(),
				_ => return Err(ClientError::Other("unbonding_period_nanos not found".to_string())),
			},
		),
		_ => return Err(ClientError::Other("unbonding_period not found".to_string())),
	};

	let (max_clock_drift_secs, max_clock_drift_nanos) = match toks.get(4).cloned().unwrap() {
		EthersToken::Tuple(toks) => (
			match toks.get(0).cloned().unwrap() {
				EthersToken::Int(numerator) => numerator.as_u64(),
				_ => return Err(ClientError::Other("max_clock_drift_secs not found".to_string())),
			},
			match toks.get(1).cloned().unwrap() {
				EthersToken::Int(denominator) => denominator.as_u64(),
				_ => return Err(ClientError::Other("max_clock_drift_nanos not found".to_string())),
			},
		),
		_ => return Err(ClientError::Other("max_clock_drift not found".to_string())),
	};

	let frozen_height = match toks.get(5).cloned().unwrap() {
		EthersToken::Int(frozen_height) => frozen_height.as_u64(),
		_ => return Err(ClientError::Other("frozen_height not found".to_string())),
	};

	let latest_height = match toks.get(6).cloned().unwrap() {
		EthersToken::Int(latest_height) => latest_height.as_u64(),
		_ => return Err(ClientError::Other("latest_height not found".to_string())),
	};

	/*
	proof_specs: ProofSpecs([ProofSpec(ProofSpec { leaf_spec: Some(LeafOp { hash: Sha256, prehash_key: NoHash, prehash_value: Sha256, length: VarProto, prefix: [0] }), inner_spec: Some(InnerSpec { child_order: [0, 1], child_size: 33, min_prefix_length: 4, max_prefix_length: 12, empty_child: [], hash: Sha256 }), max_depth: 0, min_depth: 0, prehash_key_before_comparison: false }), ProofSpec(ProofSpec { leaf_spec: Some(LeafOp { hash: Sha256, prehash_key: NoHash, prehash_value: Sha256, length: VarProto, prefix: [0] }), inner_spec: Some(InnerSpec { child_order: [0, 1], child_size: 32, min_prefix_length: 1, max_prefix_length: 1, empty_child: [], hash: Sha256 }), max_depth: 0, min_depth: 0, prehash_key_before_comparison: false })]),
	upgrade_path: ["upgrade", "upgradedIBCState"]
	 */
	let revision_number = 1; // TODO: revision
	Ok(ClientState {
		chain_id: chain_id.parse()?,
		trust_level: TrustThreshold::new(trust_level_numerator, trust_level_denominator)?,
		trusting_period: Duration::new(trusting_period_secs, trusting_period_nanos as u32),
		unbonding_period: Duration::new(unbonding_period_secs, 0u32),
		// unbonding_period: Duration::new(unbonding_period_secs, unbonding_piod_nanos as u32),
		max_clock_drift: Duration::new(max_clock_drift_secs, max_clock_drift_nanos as u32),
		frozen_height: if frozen_height == 0 {
			None
		} else {
			Some(Height::new(revision_number, frozen_height))
		},
		latest_height: Height::new(revision_number, latest_height),
		proof_specs: Default::default(), // TODO: proof_specs?
		upgrade_path: vec![],            // TODO: upgrade_path?
		_phantom: Default::default(),
	})
}

fn consensus_state_abi_token(consensus_state: ConsensusState) -> Token {
	use ethers::abi::{encode as ethers_encode, Token as EthersToken};
	let time = consensus_state
		.timestamp
		.duration_since(Timestamp { seconds: 0, nanos: 0 }.try_into().unwrap())
		.unwrap();
	let consensus_state_data = EthersToken::Tuple(
		[
			//timestamp
			EthersToken::Tuple(
				[EthersToken::Int(time.as_secs().into()), EthersToken::Int(time.as_nanos().into())]
					.to_vec(),
			),
			//root
			EthersToken::Tuple([EthersToken::Bytes(consensus_state.root.bytes)].to_vec()),
			//next_validators_hash
			EthersToken::Bytes(consensus_state.next_validators_hash.as_ref().into()),
		]
		.to_vec(),
	);
	consensus_state_data
}

fn consensus_state_from_abi_token(token: Token) -> Result<ConsensusState, ClientError> {
	use ethers::abi::Token as EthersToken;

	let Token::Bytes(bytes) = token else {
		return Err(ClientError::Other("invalid consensus state".to_string()))
	};
	let params = vec![ParamType::Tuple(vec![
		ParamType::Tuple(vec![ParamType::Int(256), ParamType::Int(256)]),
		ParamType::Tuple(vec![ParamType::Bytes]),
		ParamType::Bytes,
	])];

	let Token::Tuple(toks) = ethers::abi::decode(&params, bytes.as_slice())?.pop().unwrap() else {
		return Err(ClientError::Other("invalid consensus state'".to_string()))
	};

	let (timestamp_secs, timestamp_nanos) = match toks.get(0).cloned().unwrap() {
		EthersToken::Tuple(toks) => (
			match toks.get(0).cloned().unwrap() {
				EthersToken::Int(numerator) => numerator.as_u64(),
				_ => return Err(ClientError::Other("timestamp_secs not found".to_string())),
			},
			match toks.get(1).cloned().unwrap() {
				EthersToken::Int(denominator) => denominator.as_u64(),
				_ => return Err(ClientError::Other("timestamp_nanos not found".to_string())),
			},
		),
		_ => return Err(ClientError::Other("timestamp not found".to_string())),
	};

	let (root,) = match toks.get(1).cloned().unwrap() {
		EthersToken::Tuple(toks) => (match toks.get(0).cloned().unwrap() {
			EthersToken::Bytes(root) => root,
			_ => return Err(ClientError::Other("root not found'".to_string())),
		},),
		_ => return Err(ClientError::Other("root not found".to_string())),
	};

	let next_validators_hash = match toks.get(2).cloned().unwrap() {
		EthersToken::Bytes(next_validators_hash) => next_validators_hash,
		_ => return Err(ClientError::Other("next_validators_hash not found".to_string())),
	};

	Ok(ConsensusState {
		timestamp: Time::from_unix_timestamp(
			timestamp_secs.try_into().unwrap(),
			timestamp_nanos.try_into().unwrap(),
		)?,
		root: CommitmentRoot::from_bytes(root.as_slice()),
		next_validators_hash: Hash::Sha256(next_validators_hash.as_slice().try_into().unwrap()),
	})
}

fn tm_header_abi_token(header: Header) -> Token {
	use ethers::abi::{encode as ethers_encode, Token as EthersToken};
	let block_header = header.signed_header.header;
	let last_commit = header.signed_header.commit;

	// block_header.time.;
	let time = block_header
		.time
		.duration_since(Timestamp { seconds: 0, nanos: 0 }.try_into().unwrap())
		.unwrap();

	let signed_header_header = EthersToken::Tuple(
		[
			//version
			EthersToken::Tuple(
				[
					EthersToken::Uint(block_header.version.block.into()),
					EthersToken::Uint(block_header.version.app.into()),
				]
				.to_vec(),
			),
			//chain_id
			EthersToken::String(block_header.chain_id.into()),
			//height
			EthersToken::Int((block_header.height.value() as i64).into()),
			//time
			EthersToken::Tuple(
				[EthersToken::Int(time.as_secs().into()), EthersToken::Int(time.as_nanos().into())]
					.to_vec(),
			),
			// //last_block_id
			EthersToken::Tuple(
				[
					EthersToken::Bytes(block_header.last_block_id.unwrap().hash.into()),
					//part_set_header
					EthersToken::Tuple(
						[
							EthersToken::Uint(
								block_header.last_block_id.unwrap().part_set_header.total.into(),
							),
							EthersToken::Bytes(
								block_header.last_block_id.unwrap().part_set_header.hash.into(),
							),
						]
						.to_vec(),
					),
				]
				.to_vec(),
			),
			//last_commit_hash
			EthersToken::Bytes(block_header.last_commit_hash.unwrap().into()),
			//data_hash
			EthersToken::Bytes(block_header.data_hash.unwrap().into()),
			//validators_hash
			EthersToken::Bytes(block_header.validators_hash.into()),
			//next_validators_hash
			EthersToken::Bytes(block_header.next_validators_hash.into()),
			//consensus_hash
			EthersToken::Bytes(block_header.consensus_hash.into()),
			//app_hash
			EthersToken::Bytes(block_header.app_hash.into()),
			//last_results_hash
			EthersToken::Bytes(block_header.last_results_hash.unwrap().into()),
			//evidence_hash
			EthersToken::Bytes(block_header.evidence_hash.unwrap().into()),
			//proposer_address
			EthersToken::Bytes(block_header.proposer_address.into()),
		]
		.to_vec(),
	);

	let signed_header_commit = EthersToken::Tuple(
		[
			//height
			EthersToken::Int((last_commit.height.value()).into()),
			//round
			EthersToken::Int((last_commit.round.value()).into()),
			//block_id
			EthersToken::Tuple(
				[
					EthersToken::Bytes(last_commit.block_id.hash.into()),
					//part_set_header
					EthersToken::Tuple(
						[
							//total
							EthersToken::Uint(last_commit.block_id.part_set_header.total.into()),
							//hash
							EthersToken::Bytes(last_commit.block_id.part_set_header.hash.into()),
						]
						.to_vec(),
					),
				]
				.to_vec(),
			),
		]
		.to_vec(),
	);

	EthersToken::Tuple(
		[
			EthersToken::Tuple([signed_header_header, signed_header_commit].to_vec()),
			EthersToken::Int(header.trusted_height.revision_height.into()),
		]
		.to_vec(),
	)
}

pub(crate) fn tm_header_from_abi_token(token: Token) -> Result<Header, ClientError> {
	use ethers::abi::Token as EthersToken;

	let Token::Bytes(bytes) = token else {
		return Err(ClientError::Other("invalid header".to_string()))
	};
	let params = vec![ParamType::Tuple(vec![
		ParamType::Tuple(vec![
			ParamType::Tuple(vec![
				ParamType::Tuple(vec![ParamType::Uint(256), ParamType::Uint(256)]),
				ParamType::String,
				ParamType::Int(256),
				ParamType::Tuple(vec![ParamType::Int(256), ParamType::Int(256)]),
				ParamType::Tuple(vec![
					ParamType::Bytes,
					ParamType::Tuple(vec![ParamType::Uint(256), ParamType::Bytes]),
				]),
				ParamType::Bytes,
				ParamType::Bytes,
				ParamType::Bytes,
				ParamType::Bytes,
				ParamType::Bytes,
				ParamType::Bytes,
				ParamType::Bytes,
				ParamType::Bytes,
				ParamType::Bytes,
			]),
			ParamType::Tuple(vec![
				ParamType::Int(256),
				ParamType::Int(256),
				ParamType::Tuple(vec![
					ParamType::Bytes,
					ParamType::Tuple(vec![ParamType::Uint(256), ParamType::Bytes]),
				]),
			]),
		]),
		ParamType::Int(256),
	])];

	let Token::Tuple(toks) = ethers::abi::decode(&params, bytes.as_slice())?.pop().unwrap() else {
		return Err(ClientError::Other("invalid header'".to_string()))
	};

	let signed_header_header = match toks.get(0).cloned().unwrap() {
		EthersToken::Tuple(toks) => toks,
		_ => return Err(ClientError::Other("signed_header_header not found".to_string())),
	};

	let signed_header_commit = match toks.get(1).cloned().unwrap() {
		EthersToken::Tuple(signed_header_commit) => signed_header_commit,
		_ => return Err(ClientError::Other("signed_header_commit not found".to_string())),
	};

	let version = match signed_header_header.get(0).cloned().unwrap() {
		EthersToken::Tuple(toks) => toks,
		_ => return Err(ClientError::Other("version not found".to_string())),
	};

	let chain_id = match signed_header_header.get(1).cloned().unwrap() {
		EthersToken::String(chain_id) => chain_id,
		_ => return Err(ClientError::Other("chain_id not found".to_string())),
	};

	let height = match signed_header_header.get(2).cloned().unwrap() {
		EthersToken::Int(height) => height,
		_ => return Err(ClientError::Other("height not found".to_string())),
	};

	let time = match signed_header_header.get(3).cloned().unwrap() {
		EthersToken::Tuple(toks) => toks,
		_ => return Err(ClientError::Other("time not found".to_string())),
	};

	let last_block_id = match signed_header_header.get(4).cloned().unwrap() {
		EthersToken::Tuple(toks) => toks,
		_ => return Err(ClientError::Other("last_block_id not found".to_string())),
	};

	let last_commit_hash = match signed_header_header.get(5).cloned().unwrap() {
		EthersToken::Bytes(last_commit_hash) => last_commit_hash,
		_ => return Err(ClientError::Other("last_commit_hash not found".to_string())),
	};

	let data_hash = match signed_header_header.get(6).cloned().unwrap() {
		EthersToken::Bytes(data_hash) => data_hash,
		_ => return Err(ClientError::Other("data_hash not found".to_string())),
	};

	let validators_hash = match signed_header_header.get(7).cloned().unwrap() {
		EthersToken::Bytes(validators_hash) => validators_hash,
		_ => return Err(ClientError::Other("validators_hash not found".to_string())),
	};

	let next_validators_hash = match signed_header_header.get(8).cloned().unwrap() {
		EthersToken::Bytes(next_validators_hash) => next_validators_hash,
		_ => return Err(ClientError::Other("next_validators_hash not found".to_string())),
	};

	let consensus_hash = match signed_header_header.get(9).cloned().unwrap() {
		EthersToken::Bytes(consensus_hash) => consensus_hash,
		_ => return Err(ClientError::Other("consensus_hash not found".to_string())),
	};

	let app_hash = match signed_header_header.get(10).cloned().unwrap() {
		EthersToken::Bytes(app_hash) => app_hash,
		_ => return Err(ClientError::Other("app_hash not found".to_string())),
	};

	let last_results_hash = match signed_header_header.get(11).cloned().unwrap() {
		EthersToken::Bytes(last_results_hash) => last_results_hash,
		_ => return Err(ClientError::Other("last_results_hash not found".to_string())),
	};

	let evidence_hash = match signed_header_header.get(12).cloned().unwrap() {
		EthersToken::Bytes(evidence_hash) => evidence_hash,
		_ => return Err(ClientError::Other("evidence_hash not found".to_string())),
	};

	let proposer_address = match signed_header_header.get(13).cloned().unwrap() {
		EthersToken::Bytes(proposer_address) => proposer_address,
		_ => return Err(ClientError::Other("proposer_address not found".to_string())),
	};

	let version_block = match version.get(0).cloned().unwrap() {
		EthersToken::Uint(version_block) => version_block,
		_ => return Err(ClientError::Other("version_block not found".to_string())),
	};

	let version_app = match version.get(1).cloned().unwrap() {
		EthersToken::Uint(version_app) => version_app,
		_ => return Err(ClientError::Other("version_app not found".to_string())),
	};

	let time_secs = match time.get(0).cloned().unwrap() {
		EthersToken::Int(time_secs) => time_secs,
		_ => return Err(ClientError::Other("time_secs not found".to_string())),
	};

	let time_nanos = match time.get(1).cloned().unwrap() {
		EthersToken::Int(time_nanos) => time_nanos,
		_ => return Err(ClientError::Other("time_nanos not found".to_string())),
	};

	let last_block_id_hash = match last_block_id.get(0).cloned().unwrap() {
		EthersToken::Bytes(last_block_id_hash) => last_block_id_hash,
		_ => return Err(ClientError::Other("last_block_id_hash not found".to_string())),
	};

	let last_block_id_part_set_header = match last_block_id.get(1).cloned().unwrap() {
		EthersToken::Tuple(toks) => toks,
		_ => return Err(ClientError::Other("last_block_id_part_set_header not found".to_string())),
	};

	let last_block_id_part_set_header_total =
		match last_block_id_part_set_header.get(0).cloned().unwrap() {
			EthersToken::Uint(last_block_id_part_set_header_total) =>
				last_block_id_part_set_header_total,
			_ =>
				return Err(ClientError::Other(
					"last_block_id_part_set_header_total not found".to_string(),
				)),
		};

	let last_block_id_part_set_header_hash =
		match last_block_id_part_set_header.get(1).cloned().unwrap() {
			EthersToken::Bytes(last_block_id_part_set_header_hash) =>
				last_block_id_part_set_header_hash,
			_ =>
				return Err(ClientError::Other(
					"last_block_id_part_set_header_hash not found".to_string(),
				)),
		};

	let last_commit_height = match signed_header_commit.get(0).cloned().unwrap() {
		EthersToken::Int(last_commit_height) => last_commit_height,
		_ => return Err(ClientError::Other("last_commit_height not found".to_string())),
	};

	let last_commit_round = match signed_header_commit.get(1).cloned().unwrap() {
		EthersToken::Int(last_commit_round) => last_commit_round,
		_ => return Err(ClientError::Other("last_commit_round not found".to_string())),
	};

	let last_commit_block_id = match signed_header_commit.get(2).cloned().unwrap() {
		EthersToken::Tuple(toks) => toks,
		_ => return Err(ClientError::Other("last_commit_block_id not found".to_string())),
	};

	let last_commit_block_id_hash = match last_commit_block_id.get(0).cloned().unwrap() {
		EthersToken::Bytes(last_commit_block_id_hash) => last_commit_block_id_hash,
		_ => return Err(ClientError::Other("last_commit_block_id_hash not found".to_string())),
	};

	let last_commit_block_id_part_set_header = match last_commit_block_id.get(1).cloned().unwrap() {
		EthersToken::Tuple(toks) => toks,
		_ =>
			return Err(ClientError::Other(
				"last_commit_block_id_part_set_header not found".to_string(),
			)),
	};

	let last_commit_block_id_part_set_header_total =
		match last_commit_block_id_part_set_header.get(0).cloned().unwrap() {
			EthersToken::Uint(last_commit_block_id_part_set_header_total) =>
				last_commit_block_id_part_set_header_total,
			_ =>
				return Err(ClientError::Other(
					"last_commit_block_id_part_set_header_total not found".to_string(),
				)),
		};

	let last_commit_block_id_part_set_header_hash =
		match last_commit_block_id_part_set_header.get(1).cloned().unwrap() {
			EthersToken::Bytes(last_commit_block_id_part_set_header_hash) =>
				last_commit_block_id_part_set_header_hash,
			_ =>
				return Err(ClientError::Other(
					"last_commit_block_id_part_set_header_hash not found".to_string(),
				)),
		};

	let revision_number = 1; // TODO
	Ok(Header {
		signed_header: SignedHeader::new(
			block::Header {
				version: Version { block: version_block.as_u64(), app: version_app.as_u64() },
				chain_id: chain_id.parse()?,
				height: TmHeight::try_from(height.as_u64()).unwrap(),
				time: Time::from_unix_timestamp(
					time_secs.try_into().unwrap(),
					time_nanos.try_into().unwrap(),
				)?,
				last_block_id: Some(block::Id {
					hash: Hash::Sha256(last_block_id_hash.as_slice().try_into().unwrap()),
					part_set_header: block::parts::Header::new(
						last_block_id_part_set_header_total.as_u32(),
						Hash::Sha256(
							last_block_id_part_set_header_hash.to_vec().try_into().unwrap(),
						),
					)
					.unwrap(),
				}),
				last_commit_hash: Some(Hash::Sha256(last_commit_hash.to_vec().try_into().unwrap())),
				data_hash: Some(Hash::Sha256(data_hash.to_vec().try_into().unwrap())),
				validators_hash: Hash::Sha256(validators_hash.to_vec().try_into().unwrap()),
				next_validators_hash: Hash::Sha256(
					next_validators_hash.to_vec().try_into().unwrap(),
				),
				consensus_hash: Hash::Sha256(consensus_hash.to_vec().try_into().unwrap()),
				app_hash: AppHash::try_from(app_hash.to_vec()).unwrap(),
				last_results_hash: Some(Hash::Sha256(
					last_results_hash.to_vec().try_into().unwrap(),
				)),
				evidence_hash: Some(Hash::Sha256(evidence_hash.to_vec().try_into().unwrap())),
				proposer_address: account::Id::try_from(proposer_address.to_vec()).unwrap(),
			},
			block::Commit {
				height: TmHeight::try_from(last_commit_height.as_u64()).unwrap(),
				round: last_commit_round.as_u32().try_into().unwrap(),
				block_id: block::Id {
					hash: Hash::Sha256(last_commit_block_id_hash.to_vec().try_into().unwrap()),
					part_set_header: block::parts::Header::new(
						last_commit_block_id_part_set_header_total.as_u32(),
						Hash::Sha256(
							last_commit_block_id_part_set_header_hash.to_vec().try_into().unwrap(),
						),
					)
					.unwrap(),
				},
				signatures: vec![],
			},
		)
		.unwrap(),
		validator_set: tendermint::validator::Set::new(vec![], None),
		trusted_height: Height::new(revision_number, last_commit_height.as_u64()),
		trusted_validator_set: tendermint::validator::Set::new(vec![], None),
	})
}

pub(crate) fn msg_connection_open_init_token(x: MsgConnectionOpenInit) -> Token {
	use ethers::abi::{encode as ethers_encode, Token as EthersToken};

	let consensus_state_data = EthersToken::Tuple(
		[
			// client id
			EthersToken::String(x.client_id.as_str().to_owned()),
			// counterparty
			EthersToken::Tuple(
				[
					// client id
					EthersToken::String(x.counterparty.client_id().as_str().to_owned()),
					// connection id
					EthersToken::String(
						x.counterparty
							.connection_id()
							.map(|connection_id| connection_id.as_str().to_owned())
							.unwrap_or(String::new()),
					),
					// prefix
					EthersToken::Tuple(
						[
							// key prefix
							EthersToken::Bytes(x.counterparty.prefix().as_bytes().into()),
						]
						.to_vec(),
					),
				]
				.to_vec(),
			),
			// delay_period
			EthersToken::Uint(x.delay_period.as_secs().into()),
		]
		.to_vec(),
	);
	consensus_state_data
}

pub fn msg_connection_open_ack_token<H: Debug>(
	msg: MsgConnectionOpenAck<LocalClientTypes>,
	client_state: ClientState<H>,
) -> Token {
	use ethers::abi::{encode as ethers_encode, Token as EthersToken};

	let client_state = client_state_abi_token(client_state);
	let client_state_data_vec = ethers_encode(&[client_state]);

	let consensus_state_data = EthersToken::Tuple(
		[
			// connectionId
			EthersToken::String(msg.connection_id.as_str().to_owned()),
			//clientStateBytes
			EthersToken::Bytes(client_state_data_vec),
			// //version
			EthersToken::Tuple(
				[
					//identifier
					EthersToken::String(msg.version.identifier().clone()),
					//features
					EthersToken::Array(
						msg.version
							.features()
							.clone()
							.iter()
							.map(|feature| EthersToken::String(feature.to_string()))
							.collect(),
					),
				]
				.to_vec(),
			),
			//counterpartyConnectionID
			EthersToken::String(msg.counterparty_connection_id.as_str().to_owned()),
			//proofTry
			EthersToken::Bytes(msg.proofs.object_proof().clone().into()),
			//proofClient
			EthersToken::Bytes(
				msg.proofs.client_proof().clone().map_or_else(Vec::new, |v| v.into()),
			),
			//proofConsensus
			EthersToken::Bytes(
				msg.proofs.consensus_proof().map_or_else(Vec::new, |v| v.proof().clone().into()),
			),
			//proofHeight tuple
			EthersToken::Tuple(
				[
					//revisionNumber
					EthersToken::Uint(msg.proofs.height().revision_number.into()),
					//revisionHeight
					EthersToken::Uint(msg.proofs.height().revision_height.into()),
				]
				.to_vec(),
			),
			//consensusHeight
			EthersToken::Tuple(
				[
					//revisionNumber
					EthersToken::Uint(
						msg.proofs.consensus_proof().unwrap().height().revision_number.into(),
					),
					//revisionHeight
					EthersToken::Uint(
						msg.proofs.consensus_proof().unwrap().height().revision_number.into(),
					),
				]
				.to_vec(),
			),
		]
		.to_vec(),
	);
	consensus_state_data
}

fn msg_connection_open_try_token<H: Debug>(
	msg: MsgConnectionOpenTry<LocalClientTypes>,
	client_state: ClientState<H>,
) -> Token {
	use ethers::abi::{encode as ethers_encode, Token as EthersToken};
	let client_state = client_state_abi_token(client_state);
	let client_state_data_vec = ethers_encode(&[client_state]);
	let conn_open_try = YuiMsgConnectionOpenTry {
		counterparty: msg.counterparty.into(),
		delay_period: msg.delay_period.as_secs(),
		client_id: msg.client_id.as_str().to_owned(),
		client_state_bytes: client_state_data_vec,
		counterparty_versions: msg
			.counterparty_versions
			.into_iter()
			.map(|version| version.into())
			.collect(),
		proof_init: msg.proofs.object_proof().clone().into(),
		proof_client: msg.proofs.client_proof().clone().map_or_else(Vec::new, |v| v.into()),
		proof_consensus: msg
			.proofs
			.consensus_proof()
			.map_or_else(Vec::new, |v| v.proof().clone().into()),
		proof_height: msg.proofs.height().into(),
		consensus_height: msg.proofs.consensus_proof().unwrap().height().into(),
	};
	let conn_open_try_token = conn_open_try.into_token();
	conn_open_try_token
}

fn msg_connection_open_confirm_token(msg: MsgConnectionOpenConfirm) -> Token {
	use ethers::abi::{encode as ethers_encode, Token as EthersToken};

	let confirm = EthersToken::Tuple(
		[
			// connectionId
			EthersToken::String(msg.connection_id.as_str().to_owned()),
			// proofAck
			EthersToken::Bytes(msg.proofs.object_proof().clone().into()),
			//proofHeight tuple
			EthersToken::Tuple(
				[
					//revisionNumber
					EthersToken::Uint(msg.proofs.height().revision_number.into()),
					//revisionHeight
					EthersToken::Uint(msg.proofs.height().revision_height.into()),
				]
				.to_vec(),
			),
		]
		.to_vec(),
	);
	confirm
}

#[async_trait::async_trait]
impl Chain for EthereumClient {
	#[inline]
	fn name(&self) -> &str {
		&self.config.name
	}

	#[inline]
	fn block_max_weight(&self) -> u64 {
		self.config.max_block_weight
	}

	async fn estimate_weight(
		&self,
		msg: Vec<ibc_proto::google::protobuf::Any>,
	) -> Result<u64, Self::Error> {
		// TODO: estimate gas for the tx. Convert any to another type (see `wrap_any_msg_into_wasm`
		// for an example)
		Ok(1)
	}

	async fn finality_notifications(
		&self,
	) -> Result<std::pin::Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send>>, Self::Error> {
		// let ws = crate::client::WsEth::connect(self.ws_uri.to_string())
		// 	.await
		// 	.map_err(|err| ClientError::ProviderError(self.ws_uri.clone(), err))?;
		let ws = self.websocket_provider().await.unwrap();

		let stream = async_stream::stream! {
			// TODO: is it really finalized blocks stream?
			let mut stream = ws.subscribe_blocks().await.expect("fuck");

			while let Some(block) = stream.next().await {
				yield block
			}
		};

		Ok(stream.boxed())
	}

	async fn submit(
		&self,
		messages: Vec<ibc_proto::google::protobuf::Any>,
	) -> Result<Self::TransactionId, Self::Error> {
		use ethers::abi::{encode as ethers_encode, Token as EthersToken};

		log::info!(target: "hyperspace_ethereum", "Submitting messages: {:?}", messages.iter().map(|x| x.type_url.clone()).collect::<Vec<_>>().join(", "));
		assert_eq!(messages.len(), 1, "messages.len() > 1");

		for msg in messages {
			if msg.type_url == ibc::core::ics02_client::msgs::create_client::TYPE_URL {
				dbg!(&msg.value.len());
				let msg = MsgCreateAnyClient::<LocalClientTypes>::decode_vec(&msg.value).unwrap();
				let AnyClientState::Tendermint(client_state) = msg.client_state else {
					//TODO return error support only tendermint client state
					panic!("unsupported client state")
				};
				let AnyConsensusState::Tendermint(client_consensus) = msg.consensus_state else {
					//TODO return error support only tendermint consensus state
					panic!("unsupported")
				};
				let client_state_abi_token = client_state_abi_token(client_state);
				let consensus_state_abi_token = consensus_state_abi_token(client_consensus);

				let client_state_data_vec = ethers_encode(&[client_state_abi_token]);
				let consensus_state_data_vec = ethers_encode(&[consensus_state_abi_token]);

				dbg!(&client_state_data_vec.len());
				dbg!(&consensus_state_data_vec.len());

				log::info!(target: "hyperspace_ethereum", "encoding: {}", hex::encode(self.config.client_type.as_bytes()));
				log::info!(target: "hyperspace_ethereum", "encoding: {}", hex::encode(&client_state_data_vec.to_vec()));
				log::info!(target: "hyperspace_ethereum", "encoding: {}", hex::encode(&consensus_state_data_vec.to_vec()));
				/*
				[2023-09-06T23:06:36Z INFO  hyperspace_ethereum] encoding: 30372d74656e6465726d696e74
				[2023-09-06T23:06:36Z INFO  hyperspace_ethereum] encoding: 000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000fa000000000000000000000000000000000000000000000000000000000003d0900000000000000000000000000000000000000000000000000000000000001baf800000000000000000000000000000000000000000000000000006722feb7b0000000000000000000000000000000000000000000000000000000000000000000f000000000000000000000000000000000000000000000000000000037e11d60000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ed500000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000001263656e74617572692d746573746e65742d310000000000000000000000000000
				[2023-09-06T23:06:36Z INFO  hyperspace_ethereum] encoding: 00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000064f905f2000000000000000000000000000000000000000000000000178271d8db825068000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020767f0009ebb497c791107b84432ccaec45e92e4d05e5799ac55b2d48f2d5d4170000000000000000000000000000000000000000000000000000000000000020602fa35acfd377900d7fe3459730d96415eef369bd033c0923b2d2e2796a97d9
				[2023-09-06T23:06:36Z INFO  hyperspace_ethereum] encoding: 0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000002c0000000000000000000000000000000000000000000000000000000000000000d30372d74656e6465726d696e74000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000fa000000000000000000000000000000000000000000000000000000000003d0900000000000000000000000000000000000000000000000000000000000001baf800000000000000000000000000000000000000000000000000006722feb7b0000000000000000000000000000000000000000000000000000000000000000000f000000000000000000000000000000000000000000000000000000037e11d60000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ed500000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000001263656e74617572692d746573746e65742d310000000000000000000000000000000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000064f905f2000000000000000000000000000000000000000000000000178271d8db825068000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020767f0009ebb497c791107b84432ccaec45e92e4d05e5799ac55b2d48f2d5d4170000000000000000000000000000000000000000000000000000000000000020602fa35acfd377900d7fe3459730d96415eef369bd033c0923b2d2e2796a97d9
				//-------------------------------------------------d5a244810000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000002c0000000000000000000000000000000000000000000000000000000000000000d30372d74656e6465726d696e74000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000fa000000000000000000000000000000000000000000000000000000000003d0900000000000000000000000000000000000000000000000000000000000001baf800000000000000000000000000000000000000000000000000006722feb7b0000000000000000000000000000000000000000000000000000000000000000000f000000000000000000000000000000000000000000000000000000037e11d60000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ed500000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000001263656e74617572692d746573746e65742d310000000000000000000000000000000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000064f905f2000000000000000000000000000000000000000000000000178271d8db825068000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020767f0009ebb497c791107b84432ccaec45e92e4d05e5799ac55b2d48f2d5d4170000000000000000000000000000000000000000000000000000000000000020602fa35acfd377900d7fe3459730d96415eef369bd033c0923b2d2e2796a97d9
				// 000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000fa000000000000000000000000000000000000000000000000000000000003d0900000000000000000000000000000000000000000000000000000000000001baf800000000000000000000000000000000000000000000000000006722feb7b0000000000000000000000000000000000000000000000000000000000000000000f000000000000000000000000000000000000000000000000000000037e11d60000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ed500000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000001263656e74617572692d746573746e65742d310000000000000000000000000000

								 */
				// 0       1                                                               2
				// d5a244810000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000002c0000000000000000000000000000000000000000000000000000000000000000d30372d74656e6465726d696e74000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000fa000000000000000000000000000000000000000000000000000000000003d0900000000000000000000000000000000000000000000000000000000000001baf800000000000000000000000000000000000000000000000000006722feb7b0000000000000000000000000000000000000000000000000000000000000000000f000000000000000000000000000000000000000000000000000000037e11d60000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000cdc00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000001263656e74617572692d746573746e65742d310000000000000000000000000000000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000064f8fc0400000000000000000000000000000000000000000000000017826f89135e5218000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020008892c98d506a154ef7e5ad89b345e395cdafc8eb77857276f27ff5cef791da0000000000000000000000000000000000000000000000000000000000000020602fa35acfd377900d7fe3459730d96415eef369bd033c0923b2d2e2796a97d9
				// d5a244810000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000000d30372d74656e6465726d696e74000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000221000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001a02000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000fa000000000000000000000000000000000000000000000000000000000003d0900000000000000000000000000000000000000000000000000000000000001baf800000000000000000000000000000000000000000000000000006722feb7b0000000000000000000000000000000000000000000000000000000000000000000f000000000000000000000000000000000000000000000000000000037e11d60000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000dca00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000001263656e74617572692d746573746e65742d31000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000064f900b20000000000000000000000000000000000000000000000001782709ff2f10eb8000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002038c9b9a98f4630114e46c5c394fca947569e8db5f0e01d2aac84ef96ddff8afa0000000000000000000000000000000000000000000000000000000000000020602fa35acfd377900d7fe3459730d96415eef369bd033c0923b2d2e2796a97d9
				//-------------------------------------------------------------------------000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000002c0000000000000000000000000000000000000000000000000000000000000000d30372d74656e6465726d696e74000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000fa000000000000000000000000000000000000000000000000000000000003d0900000000000000000000000000000000000000000000000000000000000001baf800000000000000000000000000000000000000000000000000006722feb7b0000000000000000000000000000000000000000000000000000000000000000000f000000000000000000000000000000000000000000000000000000037e11d60000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e4300000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000001263656e74617572692d746573746e65742d310000000000000000000000000000000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000064f903130000000000000000000000000000000000000000000000001782712dc658a8f0000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020eb5eefd453cb5fd16cce92d669a546a074751d588d89261178dec9a250da2eae0000000000000000000000000000000000000000000000000000000000000020602fa35acfd377900d7fe3459730d96415eef369bd033c0923b2d2e2796a97d9
				//---------0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000002c0000000000000000000000000000000000000000000000000000000000000000d30372d74656e6465726d696e74000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000fa000000000000000000000000000000000000000000000000000000000003d0900000000000000000000000000000000000000000000000000000000000001baf800000000000000000000000000000000000000000000000000006722feb7b0000000000000000000000000000000000000000000000000000000000000000000f000000000000000000000000000000000000000000000000000000037e11d60000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e4300000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000001263656e74617572692d746573746e65742d310000000000000000000000000000000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000064f903130000000000000000000000000000000000000000000000001782712dc658a8f0000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020eb5eefd453cb5fd16cce92d669a546a074751d588d89261178dec9a250da2eae0000000000000000000000000000000000000000000000000000000000000020602fa35acfd377900d7fe3459730d96415eef369bd033c0923b2d2e2796a97d9
				// 30372d74656e6465726d696e74
				// 000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001a02000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000fa000000000000000000000000000000000000000000000000000000000003d0900000000000000000000000000000000000000000000000000000000000001baf800000000000000000000000000000000000000000000000000006722feb7b0000000000000000000000000000000000000000000000000000000000000000000f000000000000000000000000000000000000000000000000000000037e11d60000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000dca00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000001263656e74617572692d746573746e65742d310000000000000000000000000000
				// 00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000064f900b20000000000000000000000000000000000000000000000001782709ff2f10eb8000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002038c9b9a98f4630114e46c5c394fca947569e8db5f0e01d2aac84ef96ddff8afa0000000000000000000000000000000000000000000000000000000000000020602fa35acfd377900d7fe3459730d96415eef369bd033c0923b2d2e2796a97d9
				let token = EthersToken::Tuple(vec![
					//should be the same that we use to register client
					//client type
					EthersToken::String(self.config.client_type.clone()),
					//clientStateBytes
					EthersToken::Bytes(client_state_data_vec.clone()),
					//consensusStateBytes
					EthersToken::Bytes(consensus_state_data_vec.clone()),
				]);
				// let bts = ethers::abi::encode(&vec![
				// 	//should be the same that we use to register client
				// 	//client type
				// 	EthersToken::String(self.config.client_type.clone()),
				// 	//clientStateBytes
				// 	EthersToken::Bytes(client_state_data_vec.clone()),
				// 	//consensusStateBytes
				// 	EthersToken::Bytes(consensus_state_data_vec.clone()),
				// ]);
				// log::info!(target: "hyperspace_ethereum", "encoding: {}",
				// hex::encode(&bts.to_vec()));
				let bts = ethers::abi::encode(&vec![token.clone()]);
				log::info!(target: "hyperspace_ethereum", "encoding': {}", hex::encode(&bts.to_vec()));

				let tok = ethers::abi::decode(
					&[ParamType::Tuple(vec![
						ParamType::String,
						ParamType::Bytes,
						ParamType::Bytes,
					])],
					&bts.to_vec(), // [4..],
				)
				.unwrap()
				.pop()
				.unwrap();
				assert_eq!(tok, token);
				let (client_id, tx_id) = self.yui.create_client(token).await;
				dbg!(&client_id);
				// self.set_client_id();

				thread::sleep(Duration::from_secs(5));

				//update mutex
				let mut update_mutex = self.prev_state.lock().unwrap();
				*update_mutex = (client_state_data_vec.clone(), consensus_state_data_vec.clone());

				return Ok(tx_id)
			} else if msg.type_url == ibc::core::ics02_client::msgs::update_client::TYPE_URL {
				let msg = MsgUpdateAnyClient::<LocalClientTypes>::decode_vec(&msg.value).unwrap();
				let AnyClientMessage::Tendermint(client_state) = msg.client_message else {
					//TODO return error support only tendermint client state
					panic!("unsupported")
				};
				let ClientMessage::Header(header) = client_state else { panic!("unsupported") };

				//get abi token to update client
				let tm_header_abi_token = tm_header_abi_token(header);
				let tm_header_bytes = ethers_encode(&[tm_header_abi_token]);

				//todo replace empty vec for prev state clint with an actual client state
				let client_state = self.prev_state.lock().unwrap().0.clone();

				//TODO replace client id. it was genereated when we created the client. use 0 for
				// testing
				let client_id = format!("{}-0", self.config.client_type.clone());

				let token = EthersToken::Tuple(vec![
					//should be the same that we use to create client
					//client id
					EthersToken::String(client_id),
					//tm header
					EthersToken::Bytes(tm_header_bytes),
					//tm header
					EthersToken::Bytes(client_state),
				]);

				let _ = self.yui.update_client(token).await;
				thread::sleep(Duration::from_secs(5));

				return Ok((H256::default(), H256::default())) // TODO: tx hash
			} else if msg.type_url == ibc::core::ics03_connection::msgs::conn_open_init::TYPE_URL {
				let msg = MsgConnectionOpenInit::decode_vec(&msg.value).unwrap();
				let token = msg_connection_open_init_token(msg);
				let (connection_id, tx_id) = self.yui.connection_open_init(token).await;
				dbg!(connection_id);
				return Ok(tx_id)
			} else if msg.type_url == ibc::core::ics03_connection::msgs::conn_open_ack::TYPE_URL {
				let msg = MsgConnectionOpenAck::<LocalClientTypes>::decode_vec(&msg.value).unwrap();

				let client_state = match msg.client_state.clone() {
					Some(m) => {
						let AnyClientState::Tendermint(client_state) = m else {
							//TODO return error support only tendermint client state
							panic!("unsupported")
						};
						client_state
					},
					None => {
						//TODO return error support only tendermint client state
						panic!("unsupported")
					},
				};

				let token = msg_connection_open_ack_token(msg, client_state);
				let connection_id = self.yui.connection_open_ack(token).await;
				return Ok((H256::default(), H256::default())) // TODO: tx hash
			} else if msg.type_url == ibc::core::ics03_connection::msgs::conn_open_try::TYPE_URL {
				let msg = MsgConnectionOpenTry::<LocalClientTypes>::decode_vec(&msg.value).unwrap();
				let client_state = match msg.client_state.clone() {
					Some(m) => {
						let AnyClientState::Tendermint(client_state) = m else {
							//TODO return error support only tendermint client state
							panic!("unsupported")
						};
						client_state
					},
					None => {
						//TODO return error support only tendermint client state
						panic!("unsupported")
					},
				};

				let token = msg_connection_open_try_token(msg, client_state);
				self.yui.connection_open_try(token).await;
				return Ok((H256::default(), H256::default())) // TODO: tx hash
			} else if msg.type_url == ibc::core::ics03_connection::msgs::conn_open_confirm::TYPE_URL
			{
				let msg = MsgConnectionOpenConfirm::decode_vec(&msg.value).unwrap();
				let token = msg_connection_open_confirm_token(msg);
				self.yui.connection_open_confirm(token).await;
				return Ok((H256::default(), H256::default())) // TODO: tx hash
			} else if msg.type_url == channel_msgs::chan_open_init::TYPE_URL {
				let msg = MsgChannelOpenInit::decode_vec(&msg.value).unwrap();
				let token = msg.into_token();
				let (_channel_id, tx_id) = self.yui.channel_open_init(token).await;
				return Ok(tx_id)
			} else if msg.type_url == channel_msgs::chan_open_try::TYPE_URL {
				let msg = MsgChannelOpenTry::decode_vec(&msg.value).unwrap();
				let token = msg.into_token();
				let channel_id = self.yui.channel_open_try(token).await;
				return Ok((H256::default(), H256::default())) // TODO: tx hash
			} else if msg.type_url == channel_msgs::chan_open_ack::TYPE_URL {
				let msg = MsgChannelOpenAck::decode_vec(&msg.value).unwrap();
				let token = msg.into_token();
				self.yui.send_and_get_tuple(token, "channelOpenAck").await;
				return Ok((H256::default(), H256::default())) // TODO: tx hash
			} else if msg.type_url == channel_msgs::chan_open_confirm::TYPE_URL {
				let msg = MsgChannelOpenConfirm::decode_vec(&msg.value).unwrap();
				let token = msg.into_token();
				self.yui.send_and_get_tuple(token, "channelOpenConfirm").await;
				return Ok((H256::default(), H256::default())) // TODO: tx hash
			} else if msg.type_url == channel_msgs::chan_close_init::TYPE_URL {
				let msg = MsgChannelCloseInit::decode_vec(&msg.value).unwrap();
				let token = msg.into_token();
				self.yui.send_and_get_tuple(token, "channelCloseInit").await;
				return Ok((H256::default(), H256::default())) // TODO: tx hash
			} else if msg.type_url == channel_msgs::chan_close_confirm::TYPE_URL {
				let msg = MsgChannelCloseConfirm::decode_vec(&msg.value).unwrap();
				let token = msg.into_token();
				self.yui.send_and_get_tuple(token, "channelCloseConfirm").await;
				return Ok((H256::default(), H256::default())) // TODO: tx hash
			} else if msg.type_url == channel_msgs::timeout_on_close::TYPE_URL {
				let msg = MsgTimeoutOnClose::decode_vec(&msg.value).unwrap();
			} else if msg.type_url == channel_msgs::timeout::TYPE_URL {
				let msg = MsgTimeout::decode_vec(&msg.value).unwrap();
			} else if msg.type_url == channel_msgs::acknowledgement::TYPE_URL {
				let msg = MsgAcknowledgement::decode_vec(&msg.value).unwrap();
				let token = msg.into_token();
				self.yui.send_and_get_tuple(token, "acknowledgePacket").await;
				return Ok((H256::default(), H256::default())) // TODO: tx hash
			} else if msg.type_url == channel_msgs::recv_packet::TYPE_URL {
				let msg = MsgRecvPacket::decode_vec(&msg.value).unwrap();
				let token = msg.into_token();
				self.yui.send_and_get_tuple(token, "recvPacket").await;
				return Ok((H256::default(), H256::default())) // TODO: tx hash
			}
			unimplemented!("does not support this msg type for now: {}", msg.type_url);
		}
		Err(ClientError::Other("no message to submit".to_string()))
	}

	async fn query_client_message(
		&self,
		update: UpdateClient,
	) -> Result<AnyClientMessage, Self::Error> {
		todo!("used for misbehaviour; skip for now")
	}

	async fn get_proof_height(&self, block_height: Height) -> Height {
		block_height
	}

	async fn handle_error(&mut self, error: &anyhow::Error) -> Result<(), anyhow::Error> {
		tracing::error!(?error, "handle-error");
		Ok(())
	}

	fn common_state(&self) -> &CommonClientState {
		&self.common_state
	}

	fn common_state_mut(&mut self) -> &mut CommonClientState {
		&mut self.common_state
	}

	fn rpc_call_delay(&self) -> Duration {
		Duration::from_millis(100)
	}

	fn set_rpc_call_delay(&mut self, delay: Duration) {}

	async fn reconnect(&mut self) -> anyhow::Result<()> {
		// TODO: reconnection logic
		Ok(())
	}
}

#[test]
fn fooo() {
	let xs = hex::encode(&[
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 1, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 250, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 3, 208, 144, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 175, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 114, 47, 235, 123, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 126, 17, 214, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 213,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 18, 99, 101, 110, 116, 97, 117, 114, 105, 45, 116, 101, 115, 116, 110, 101,
		116, 45, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	]);
	let ys = hex::encode(&[
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		100, 249, 5, 242, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		23, 130, 113, 216, 219, 130, 80, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 118, 127, 0, 9, 235, 180, 151,
		199, 145, 16, 123, 132, 67, 44, 202, 236, 69, 233, 46, 77, 5, 229, 121, 154, 197, 91, 45,
		72, 242, 213, 212, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 32, 96, 47, 163, 90, 207, 211, 119, 144, 13, 127, 227, 69, 151, 48,
		217, 100, 21, 238, 243, 105, 189, 3, 60, 9, 35, 178, 210, 226, 121, 106, 151, 217,
	]);
	println!("{xs}\n{ys}");
}
