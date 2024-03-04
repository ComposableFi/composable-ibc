use crate::{
	client::{ClientError, EthereumClient},
	contract::{IbcHandler, UnwrapContractError},
	ibc_provider::{BlockHeight, INDEXER_DELAY_BLOCKS},
	utils::{
		clear_proof_value, handle_gas_usage, send_retrying, Header as EthHeader, ProviderImpl,
	},
	yui_types::{ics03_connection::conn_open_try::YuiMsgConnectionOpenTry, IntoToken},
};
use alloy_sol_types::SolValue;
use channel_msgs::{
	acknowledgement::MsgAcknowledgement, chan_close_confirm::MsgChannelCloseConfirm,
	chan_close_init::MsgChannelCloseInit, chan_open_ack::MsgChannelOpenAck,
	chan_open_confirm::MsgChannelOpenConfirm, chan_open_init::MsgChannelOpenInit,
	chan_open_try::MsgChannelOpenTry, recv_packet::MsgRecvPacket, timeout::MsgTimeout,
	timeout_on_close::MsgTimeoutOnClose,
};
use elliptic_curve::bigint::Pow;
use ethers::{
	abi::{ethabi, ParamType, Token},
	contract::FunctionCall,
	providers::Middleware,
	types::{H256, U256},
	utils::rlp,
};
use ethers_providers::EscalationPolicy;
use futures::{Stream, StreamExt};
use ibc::{
	core::{
		ics02_client::{
			client_state::ClientState as _,
			events::UpdateClient,
			msgs::{create_client::MsgCreateAnyClient, update_client::MsgUpdateAnyClient},
			trust_threshold::TrustThreshold,
		},
		ics03_connection::msgs::{
			conn_open_ack::MsgConnectionOpenAck, conn_open_confirm::MsgConnectionOpenConfirm,
			conn_open_init::MsgConnectionOpenInit, conn_open_try::MsgConnectionOpenTry,
		},
		ics04_channel::msgs as channel_msgs,
		ics23_commitment::{
			commitment::{CommitmentProofBytes, CommitmentRoot},
			merkle::MerkleProof,
		},
		ics24_host::identifier::{ChainId, ClientId},
	},
	proofs::ConsensusProof,
	protobuf::{google::protobuf::Timestamp, Protobuf},
	Height,
};
use ibc_proto::google::protobuf::Any;
use ics07_tendermint::{
	client_message::{ClientMessage, Header},
	client_state::ClientState,
	consensus_state::ConsensusState,
};
use ics23::commitment_proof::Proof;
use icsxx_ethereum::{
	abi::EthereumClientAbi::EthereumClientPrimitivesConsensusStateProof, utils::keccak256,
};
use log::{error, info, debug, trace};
use pallet_ibc::light_clients::{
	AnyClientMessage, AnyClientState, AnyConsensusState, HostFunctionsManager,
};
use primitives::{
	mock::LocalClientTypes, Chain, CommonClientState, IbcProvider, LightClientSync,
	MisbehaviourHandler,
};
use serde::{Deserialize, Serialize, __private::de};
use tracing_subscriber::fmt::format;
use std::{
	fmt::Debug,
	pin::Pin,
	str::FromStr,
	sync::{Arc, Mutex},
	thread,
	time::Duration,
};
use tendermint::{
	account, block::{self, header::Version, signed_header::SignedHeader, CommitSig, Height as TmHeight}, crypto::signature, trust_threshold::TrustThresholdFraction, AppHash, Hash, PublicKey, Time
};
use tokio::time::sleep;

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

pub fn client_state_abi_token<H: Debug>(client: &ClientState<H>) -> Token {
	use ethers::abi::{encode as ethers_encode, Token as EthersToken};
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
					EthersToken::Int(client.trusting_period.subsec_nanos().into()),
				]
				.to_vec(),
			),
			//unbonding_period
			EthersToken::Tuple(
				[
					EthersToken::Int(client.unbonding_period.as_secs().into()),
					EthersToken::Int(client.unbonding_period.subsec_nanos().into()),
				]
				.to_vec(),
			),
			//max_clock_drift
			EthersToken::Tuple(
				[
					EthersToken::Int(client.max_clock_drift.as_secs().into()),
					EthersToken::Int(client.max_clock_drift.subsec_nanos().into()),
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
			EthersToken::Bool(false),
			//allow_update_after_misbehaviour
			EthersToken::Bool(false),
		]
		.to_vec(),
	);
	client_state_data
}

pub fn client_state_from_abi_token<H>(token: Token) -> Result<ClientState<H>, ClientError> {
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

	let Token::Tuple(toks) = ethers::abi::decode(&params, bytes.as_slice())?
		.pop()
		.ok_or(ClientError::Other("invalid client state, .pop() return None'".to_string()))?
	else {
		return Err(ClientError::Other("invalid client state'".to_string()))
	};

	let chain_id = match toks.get(0).cloned().ok_or(ClientError::Other(
		"failed to get index 0 and retrieve chain_id from token".to_string(),
	))? {
		EthersToken::String(chain_id) => chain_id,
		_ => return Err(ClientError::Other("chain_id not found".to_string())),
	};

	let (trust_level_numerator, trust_level_denominator) = match toks.get(1).cloned().ok_or(
		ClientError::Other(
			"failed to get index 1 and retrieve trust_level_numerator and trust_level_denominator"
				.to_string(),
		),
	)? {
		EthersToken::Tuple(toks) => (
			match toks.get(0).cloned().ok_or(ClientError::Other(
				"failed to get index 0 and retrieve numerator".to_string(),
			))? {
				EthersToken::Uint(numerator) => numerator.as_u64(),
				_ => return Err(ClientError::Other("trust_level_numerator not found".to_string())),
			},
			match toks.get(1).cloned().ok_or(ClientError::Other(
				"failed to get index 0 and retrieve denominator".to_string(),
			))? {
				EthersToken::Uint(denominator) => denominator.as_u64(),
				_ =>
					return Err(ClientError::Other("trust_level_denominator not found".to_string())),
			},
		),
		_ => return Err(ClientError::Other("trust_level not found".to_string())),
	};

	let (trusting_period_secs, trusting_period_nanos) = match toks.get(2).cloned().ok_or(
		ClientError::Other(
			"failed to get index 2 and retrieve trusting_period_secs and trusting_period_nanos"
				.to_string(),
		),
	)? {
		EthersToken::Tuple(toks) => (
			match toks.get(0).cloned().ok_or(ClientError::Other(
				"failed to get index 0 and retrieve numerator".to_string(),
			))? {
				EthersToken::Int(numerator) => numerator.as_u64(),
				_ => return Err(ClientError::Other("trusting_period_secs not found".to_string())),
			},
			match toks.get(1).cloned().ok_or(ClientError::Other(
				"failed to get index 1 and retrieve denominator".to_string(),
			))? {
				EthersToken::Int(denominator) => denominator.as_u64(),
				_ => return Err(ClientError::Other("trusting_period_nanos not found".to_string())),
			},
		),
		_ => return Err(ClientError::Other("trusting_period not found".to_string())),
	};

	let (unbonding_period_secs, unbonding_period_nanos) = match toks.get(3).cloned().ok_or(
		ClientError::Other(
			"failed to get index 3 and retrieve unbonding_period_secs and unbonding_period_nanos"
				.to_string(),
		),
	)? {
		EthersToken::Tuple(toks) => (
			match toks.get(0).cloned().ok_or(ClientError::Other(
				"failed to get index 0 and retrieve numerator".to_string(),
			))? {
				EthersToken::Int(numerator) => numerator.as_u64(),
				_ => return Err(ClientError::Other("unbonding_period_secs not found".to_string())),
			},
			match toks.get(1).cloned().ok_or(ClientError::Other(
				"failed to get index 1 and retrieve denominator".to_string(),
			))? {
				EthersToken::Int(denominator) => denominator.as_u64(),
				_ => return Err(ClientError::Other("unbonding_period_nanos not found".to_string())),
			},
		),
		_ => return Err(ClientError::Other("unbonding_period not found".to_string())),
	};

	let (max_clock_drift_secs, max_clock_drift_nanos) = match toks.get(4).cloned().ok_or(
		ClientError::Other(
			"failed to get index 4 and retrieve max_clock_drift_secs and max_clock_drift_nanos"
				.to_string(),
		),
	)? {
		EthersToken::Tuple(toks) => (
			match toks.get(0).cloned().ok_or(ClientError::Other(
				"failed to get index 0 and retrieve numerator".to_string(),
			))? {
				EthersToken::Int(numerator) => numerator.as_u64(),
				_ => return Err(ClientError::Other("max_clock_drift_secs not found".to_string())),
			},
			match toks.get(1).cloned().ok_or(ClientError::Other(
				"failed to get index 1 and retrieve denominator".to_string(),
			))? {
				EthersToken::Int(denominator) => denominator.as_u64(),
				_ => return Err(ClientError::Other("max_clock_drift_nanos not found".to_string())),
			},
		),
		_ => return Err(ClientError::Other("max_clock_drift not found".to_string())),
	};

	let frozen_height =
		match toks.get(5).cloned().ok_or(ClientError::Other(
			"failed to get index 5 and retrieve frozen_height".to_string(),
		))? {
			EthersToken::Int(frozen_height) => frozen_height.as_u64(),
			_ => return Err(ClientError::Other("frozen_height not found".to_string())),
		};

	let latest_height =
		match toks.get(6).cloned().ok_or(ClientError::Other(
			"failed to get index 6 and retrieve latest_height".to_string(),
		))? {
			EthersToken::Int(latest_height) => latest_height.as_u64(),
			_ => return Err(ClientError::Other("latest_height not found".to_string())),
		};

	let chain_id: ChainId = chain_id.parse()?;
	let revision_number = chain_id.version();
	Ok(ClientState {
		chain_id,
		trust_level: TrustThreshold::new(trust_level_numerator, trust_level_denominator)?,
		trusting_period: Duration::new(trusting_period_secs, trusting_period_nanos as u32),
		unbonding_period: Duration::new(unbonding_period_secs, unbonding_period_nanos as u32),
		// unbonding_period: Duration::new(unbonding_period_secs, unbonding_piod_nanos as u32),
		max_clock_drift: Duration::new(max_clock_drift_secs, max_clock_drift_nanos as u32),
		frozen_height: if frozen_height == 0 {
			None
		} else {
			Some(Height::new(revision_number, frozen_height))
		},
		latest_height: Height::new(revision_number, latest_height),
		proof_specs: Default::default(),
		upgrade_path: vec![],
		_phantom: Default::default(),
	})
}

pub fn consensus_state_abi_token(consensus_state: &ConsensusState) -> Token {
	use ethers::abi::{encode as ethers_encode, Token as EthersToken};
	let timestamp = Timestamp::from(consensus_state.timestamp);

	let consensus_state_data = EthersToken::Tuple(
		[
			//timestamp
			EthersToken::Tuple(
				[
					EthersToken::Int(timestamp.seconds.into()),
					EthersToken::Int(timestamp.nanos.into()),
				]
				.to_vec(),
			),
			//root
			EthersToken::Tuple([EthersToken::Bytes(consensus_state.root.bytes.clone())].to_vec()),
			//next_validators_hash
			EthersToken::Bytes(consensus_state.next_validators_hash.as_ref().into()),
		]
		.to_vec(),
	);
	consensus_state_data
}

pub(crate) fn consensus_state_from_abi_token(token: Token) -> Result<ConsensusState, ClientError> {
	use ethers::abi::Token as EthersToken;

	let Token::Bytes(bytes) = token else {
		return Err(ClientError::Other("invalid consensus state".to_string()))
	};
	let params = vec![ParamType::Tuple(vec![
		ParamType::Tuple(vec![ParamType::Int(256), ParamType::Int(256)]),
		ParamType::Tuple(vec![ParamType::Bytes]),
		ParamType::Bytes,
	])];

	let Token::Tuple(toks) = ethers::abi::decode(&params, bytes.as_slice())?
		.pop()
		.ok_or(ClientError::Other("invalid consensus state pop() return None'".to_string()))?
	else {
		return Err(ClientError::Other("invalid consensus state'".to_string()))
	};

	let (timestamp_secs, timestamp_nanos) = match toks.get(0).cloned().ok_or(ClientError::Other(
		"failed to get index 0 and retrieve timestamp_secs and timestamp_nanos".to_string(),
	))? {
		EthersToken::Tuple(toks) => (
			match toks.get(0).cloned().ok_or(ClientError::Other(
				"failed to get index 0 and retrieve numerator".to_string(),
			))? {
				EthersToken::Int(numerator) => numerator.as_u64(),
				_ => return Err(ClientError::Other("timestamp_secs not found".to_string())),
			},
			match toks.get(1).cloned().ok_or(ClientError::Other(
				"failed to get index 1 and retrieve denominator".to_string(),
			))? {
				EthersToken::Int(denominator) => denominator.as_u64(),
				_ => return Err(ClientError::Other("timestamp_nanos not found".to_string())),
			},
		),
		_ => return Err(ClientError::Other("timestamp not found".to_string())),
	};

	let (root,) = match toks
		.get(1)
		.cloned()
		.ok_or(ClientError::Other("failed to get index 1 and retrieve root".to_string()))?
	{
		EthersToken::Tuple(toks) =>
			(
				match toks.get(0).cloned().ok_or(ClientError::Other(
					"failed to get index 0 and retrieve root".to_string(),
				))? {
					EthersToken::Bytes(root) => root,
					_ => return Err(ClientError::Other("root not found'".to_string())),
				},
			),
		_ => return Err(ClientError::Other("root not found".to_string())),
	};

	let next_validators_hash = match toks.get(2).cloned().ok_or(ClientError::Other(
		"failed to get index 2 and retrieve next_validators_hash".to_string(),
	))? {
		EthersToken::Bytes(next_validators_hash) => next_validators_hash,
		_ => return Err(ClientError::Other("next_validators_hash not found".to_string())),
	};

	Ok(ConsensusState {
		timestamp: Time::try_from(Timestamp {
			seconds: timestamp_secs as _,
			nanos: timestamp_nanos as _,
		})?,
		root: CommitmentRoot::from_bytes(root.as_slice()),
		next_validators_hash: Hash::Sha256(next_validators_hash.as_slice().try_into().map_err(
			|e| {
				ClientError::Other(format!(
					"failed to convert next_validators_hash into : Hash::Sha256{}",
					e
				))
			},
		)?),
	})
}

fn tm_header_abi_token(header: &ics07_tendermint_zk::client_message::ZkHeader) -> Result<Token, ClientError> {
	use ethers::abi::{encode as ethers_encode, Token as EthersToken};
	let block_header = header.signed_header.header.clone();
	let last_commit = header.signed_header.commit.clone();

	let timestamp = Timestamp::from(block_header.time);

	let last_block_id = block_header
		.last_block_id
		.ok_or(ClientError::Other("last_block_id not found".to_string()))?;

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
				[
					EthersToken::Int(timestamp.seconds.into()),
					EthersToken::Int(timestamp.nanos.into()),
				]
				.to_vec(),
			),
			// //last_block_id
			EthersToken::Tuple(
				[
					EthersToken::Bytes(last_block_id.hash.into()),
					//part_set_header
					EthersToken::Tuple(
						[
							EthersToken::Uint(last_block_id.part_set_header.total.into()),
							EthersToken::Bytes(last_block_id.part_set_header.hash.into()),
						]
						.to_vec(),
					),
				]
				.to_vec(),
			),
			//last_commit_hash
			EthersToken::Bytes(
				block_header
					.last_commit_hash
					.ok_or(ClientError::Other("last_commit_hash not found".to_string()))?
					.into(),
			),
			//data_hash
			EthersToken::Bytes(
				block_header
					.data_hash
					.ok_or(ClientError::Other("data_hash not found".to_string()))?
					.into(),
			),
			//validators_hash
			EthersToken::Bytes(block_header.validators_hash.into()),
			//next_validators_hash
			EthersToken::Bytes(block_header.next_validators_hash.into()),
			//consensus_hash
			EthersToken::Bytes(block_header.consensus_hash.into()),
			//app_hash
			EthersToken::Bytes(block_header.app_hash.into()),
			//last_results_hash
			EthersToken::Bytes(
				block_header
					.last_results_hash
					.ok_or(ClientError::Other("last_results_hash not found".to_string()))?
					.into(),
			),
			//evidence_hash
			EthersToken::Bytes(
				block_header
					.evidence_hash
					.ok_or(ClientError::Other("evidence_hash not found".to_string()))?
					.into(),
			),
			//proposer_address
			EthersToken::Bytes(block_header.proposer_address.into()),
		]
		.to_vec(),
	);

	let mut list_of_commit_sig = vec![];
	
	//enum BlockIDFlag from yui solidity
	const BLOCK_ID_FLAG_UNKNOWN: u32 = 0;
	const BLOCK_ID_FLAG_ABSENT: u32 = 1;
	const BLOCK_ID_FLAG_COMMIT: u32 = 2;
	const BLOCK_ID_FLAG_NIL: u32 = 3;
	
	for i in header.signed_header.commit.signatures.iter() {
		match i{
			CommitSig::BlockIdFlagAbsent => {
				let commit_timestamp = EthersToken::Tuple(
					[
						EthersToken::Int(timestamp.seconds.into()),
						EthersToken::Int(timestamp.nanos.into()),
					]
					.to_vec(),
				);
				let tuple = EthersToken::Tuple(vec![EthersToken::Int(BLOCK_ID_FLAG_ABSENT.into()), EthersToken::Bytes([0;32].into()), commit_timestamp, EthersToken::Bytes([0;32].into())]);
				list_of_commit_sig.push(tuple);
			}
			CommitSig::BlockIdFlagCommit{validator_address, timestamp, signature} => {
			
				let timestamp_t : Timestamp = timestamp.clone().into();
				let commit_timestamp = EthersToken::Tuple(
					[
						EthersToken::Int(timestamp_t.seconds.into()),
						EthersToken::Int(timestamp_t.nanos.into()),
					]
					.to_vec(),
				);

				let s : Vec<u8> = signature.clone().unwrap().into();
				let signature_bytes = EthersToken::Bytes(s.into());

				let validator_address = EthersToken::Bytes(validator_address.as_bytes().into());
				let tuple = EthersToken::Tuple(vec![EthersToken::Int(BLOCK_ID_FLAG_COMMIT.into()), validator_address, commit_timestamp, signature_bytes]);
				list_of_commit_sig.push(tuple);
			}
			CommitSig::BlockIdFlagNil{validator_address, timestamp, signature} => {
				let timestamp_t : Timestamp = timestamp.clone().into();
				let commit_timestamp = EthersToken::Tuple(
					[
						EthersToken::Int(timestamp_t.seconds.into()),
						EthersToken::Int(timestamp_t.nanos.into()),
					]
					.to_vec(),
				);

				let s : Vec<u8> = signature.clone().unwrap().into();
				let signature_bytes = EthersToken::Bytes(s.into());

				let validator_address = EthersToken::Bytes(validator_address.as_bytes().into());
				let tuple = EthersToken::Tuple(vec![EthersToken::Int(BLOCK_ID_FLAG_NIL.into()), validator_address, commit_timestamp, signature_bytes]);
				list_of_commit_sig.push(tuple);
			}
		};
	}

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
			//signatures
			EthersToken::Array(list_of_commit_sig),
		]
		.to_vec(),
	);

	let validators = header
		.validator_set
		.validators()
		.iter()
		.map(|validator| {
			validator_info_token(validator)
		})
		.collect::<Result<Vec<_>, _>>()?;

	let validators_proposer = header.validator_set.proposer().clone().unwrap();
	let validators_proposer = validator_info_token(&validators_proposer)?;

	let validator_set = EthersToken::Tuple(vec![
			EthersToken::Array(validators),
			validators_proposer,
			EthersToken::Int(header.validator_set.total_voting_power().value().into()),
		]
	);

	let trusted_validators = header
		.trusted_validator_set
		.validators()
		.iter()
		.map(|validator| {
			let address = EthersToken::Bytes(validator.address.as_bytes().into());
			let voting_power = validator.power.value();
			let pub_key = validator.pub_key.clone();
			let pub_key = match pub_key {
				PublicKey::Ed25519(pub_key) => EthersToken::Tuple(vec![
					EthersToken::Bytes(pub_key.as_bytes().to_vec()),
					EthersToken::Bytes(vec![]),
					EthersToken::Bytes(vec![]),
				]),
				PublicKey::Secp256k1(pub_key) => EthersToken::Tuple(vec![
					EthersToken::Bytes(vec![]),
					EthersToken::Bytes(pub_key.to_bytes().to_vec()),
					EthersToken::Bytes(vec![]),
				]),
				_ =>
					return Err(ClientError::Other(
						"unsupported public key type, only ed25519 is supported".to_string(),
					)),
			};
			Ok(EthersToken::Tuple(vec![address, pub_key, EthersToken::Int(voting_power.into())]))
		})
		.collect::<Result<Vec<_>, _>>()?;

	let ret = EthersToken::Tuple(
		[
			EthersToken::Tuple([signed_header_header, signed_header_commit].to_vec()),
			validator_set,
			EthersToken::Int(header.trusted_height.revision_height.into()),
			EthersToken::Array(trusted_validators),
		]
		.to_vec(),
	);
	Ok(ret)
}

fn validator_info_token(validator: &tendermint::validator::Info) -> Result<Token, ClientError> {
	use ethers::abi::{encode as ethers_encode, Token as EthersToken};
    let address = EthersToken::Bytes(validator.address.as_bytes().into());
    let pub_key = validator.pub_key.clone();
    let voting_power = EthersToken::Int(validator.power.value().into());
    let proposer_priority = EthersToken::Int(validator.proposer_priority.value().into());
    let pub_key = match pub_key {
				    PublicKey::Ed25519(pub_key) => EthersToken::Tuple(vec![
					    EthersToken::Bytes(pub_key.as_bytes().to_vec()),
					    EthersToken::Bytes(vec![]),
					    EthersToken::Bytes(vec![]),
				    ]),
				    PublicKey::Secp256k1(pub_key) => EthersToken::Tuple(vec![
					    EthersToken::Bytes(vec![]),
					    EthersToken::Bytes(pub_key.to_bytes().to_vec()),
					    EthersToken::Bytes(vec![]),
				    ]),
				    _ =>
					    return Err(ClientError::Other(
						    "unsupported public key type, only ed25519 is supported".to_string(),
					    )),
			    };
    Ok(EthersToken::Tuple(vec![address, pub_key,voting_power, proposer_priority ]))
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

	let Token::Tuple(toks) = ethers::abi::decode(&params, bytes.as_slice())?
		.pop()
		.ok_or(ClientError::Other("invalid header pop() return None'".to_string()))?
	else {
		return Err(ClientError::Other("invalid header'".to_string()))
	};

	let signed_header_header = match toks.get(0).cloned().ok_or(ClientError::Other(
		"failed to get index 0 and retrieve signed_header_header".to_string(),
	))? {
		EthersToken::Tuple(toks) => match toks.get(0).cloned().ok_or(ClientError::Other(
			"failed to get index 0 and retrieve signed_header_header".to_string(),
		))? {
			EthersToken::Tuple(toks) => toks,
			_ => return Err(ClientError::Other("signed_header_header' not found".to_string())),
		},
		_ => return Err(ClientError::Other("signed_header_header not found".to_string())),
	};

	let version = match signed_header_header
		.get(0)
		.cloned()
		.ok_or(ClientError::Other("failed to get index 0 and retrieve version".to_string()))?
	{
		EthersToken::Tuple(toks) => toks,
		_ => return Err(ClientError::Other("version not found".to_string())),
	};

	let chain_id = match signed_header_header
		.get(1)
		.cloned()
		.ok_or(ClientError::Other("failed to get index 1 and retrieve chain_id".to_string()))?
	{
		EthersToken::String(chain_id) => chain_id,
		_ => return Err(ClientError::Other("chain_id not found".to_string())),
	};

	let height = match signed_header_header
		.get(2)
		.cloned()
		.ok_or(ClientError::Other("failed to get index 2 and retrieve height".to_string()))?
	{
		EthersToken::Int(height) => height,
		_ => return Err(ClientError::Other("height not found".to_string())),
	};

	let time = match signed_header_header
		.get(3)
		.cloned()
		.ok_or(ClientError::Other("failed to get index 3 and retrieve time".to_string()))?
	{
		EthersToken::Tuple(toks) => toks,
		_ => return Err(ClientError::Other("time not found".to_string())),
	};

	let last_block_id =
		match signed_header_header.get(4).cloned().ok_or(ClientError::Other(
			"failed to get index 4 and retrieve last_block_id".to_string(),
		))? {
			EthersToken::Tuple(toks) => toks,
			_ => return Err(ClientError::Other("last_block_id not found".to_string())),
		};

	let last_commit_hash = match signed_header_header.get(5).cloned().ok_or(ClientError::Other(
		"failed to get index 5 and retrieve last_commit_hash".to_string(),
	))? {
		EthersToken::Bytes(last_commit_hash) => last_commit_hash,
		_ => return Err(ClientError::Other("last_commit_hash not found".to_string())),
	};

	let data_hash = match signed_header_header
		.get(6)
		.cloned()
		.ok_or(ClientError::Other("failed to get index 6 and retrieve data_hash".to_string()))?
	{
		EthersToken::Bytes(data_hash) => data_hash,
		_ => return Err(ClientError::Other("data_hash not found".to_string())),
	};

	let validators_hash = match signed_header_header.get(7).cloned().ok_or(ClientError::Other(
		"failed to get index 7 and retrieve validators_hash".to_string(),
	))? {
		EthersToken::Bytes(validators_hash) => validators_hash,
		_ => return Err(ClientError::Other("validators_hash not found".to_string())),
	};

	let next_validators_hash = match signed_header_header.get(8).cloned().ok_or(
		ClientError::Other("failed to get index 8 and retrieve next_validators_hash".to_string()),
	)? {
		EthersToken::Bytes(next_validators_hash) => next_validators_hash,
		_ => return Err(ClientError::Other("next_validators_hash not found".to_string())),
	};

	let consensus_hash = match signed_header_header.get(9).cloned().ok_or(ClientError::Other(
		"failed to get index 9 and retrieve consensus_hash".to_string(),
	))? {
		EthersToken::Bytes(consensus_hash) => consensus_hash,
		_ => return Err(ClientError::Other("consensus_hash not found".to_string())),
	};

	let app_hash = match signed_header_header
		.get(10)
		.cloned()
		.ok_or(ClientError::Other("failed to get index 10 and retrieve app_hash".to_string()))?
	{
		EthersToken::Bytes(app_hash) => app_hash,
		_ => return Err(ClientError::Other("app_hash not found".to_string())),
	};

	let last_results_hash = match signed_header_header.get(11).cloned().ok_or(
		ClientError::Other("failed to get index 11 and retrieve last_results_hash".to_string()),
	)? {
		EthersToken::Bytes(last_results_hash) => last_results_hash,
		_ => return Err(ClientError::Other("last_results_hash not found".to_string())),
	};

	let evidence_hash = match signed_header_header.get(12).cloned().ok_or(ClientError::Other(
		"failed to get index 12 and retrieve evidence_hash".to_string(),
	))? {
		EthersToken::Bytes(evidence_hash) => evidence_hash,
		_ => return Err(ClientError::Other("evidence_hash not found".to_string())),
	};

	let proposer_address = match signed_header_header.get(13).cloned().ok_or(ClientError::Other(
		"failed to get index 13 and retrieve proposer_address".to_string(),
	))? {
		EthersToken::Bytes(proposer_address) => proposer_address,
		_ => return Err(ClientError::Other("proposer_address not found".to_string())),
	};

	let version_block =
		match version.get(0).cloned().ok_or(ClientError::Other(
			"failed to get index 0 and retrieve version_block".to_string(),
		))? {
			EthersToken::Uint(version_block) => version_block,
			_ => return Err(ClientError::Other("version_block not found".to_string())),
		};

	let version_app =
		match version.get(1).cloned().ok_or(ClientError::Other(
			"failed to get index 1 and retrieve version_app".to_string(),
		))? {
			EthersToken::Uint(version_app) => version_app,
			_ => return Err(ClientError::Other("version_app not found".to_string())),
		};

	let time_secs = match time
		.get(0)
		.cloned()
		.ok_or(ClientError::Other("failed to get index 0 and retrieve time_secs".to_string()))?
	{
		EthersToken::Int(time_secs) => time_secs,
		_ => return Err(ClientError::Other("time_secs not found".to_string())),
	};

	let time_nanos =
		match time.get(1).cloned().ok_or(ClientError::Other(
			"failed to get index 1 and retrieve time_nanos".to_string(),
		))? {
			EthersToken::Int(time_nanos) => time_nanos,
			_ => return Err(ClientError::Other("time_nanos not found".to_string())),
		};

	let last_block_id_hash = match last_block_id.get(0).cloned().ok_or(ClientError::Other(
		"failed to get index 0 and retrieve last_block_id_hash".to_string(),
	))? {
		EthersToken::Bytes(last_block_id_hash) => last_block_id_hash,
		_ => return Err(ClientError::Other("last_block_id_hash not found".to_string())),
	};

	let last_block_id_part_set_header = match last_block_id.get(1).cloned().ok_or(
		ClientError::Other(
			"failed to get index 1 and retrieve last_block_id_part_set_header".to_string(),
		),
	)? {
		EthersToken::Tuple(toks) => toks,
		_ => return Err(ClientError::Other("last_block_id_part_set_header not found".to_string())),
	};

	let last_block_id_part_set_header_total =
		match last_block_id_part_set_header.get(0).cloned().ok_or(ClientError::Other(
			"failed to get index 0 and retrieve last_block_id_part_set_header_total".to_string(),
		))? {
			EthersToken::Uint(last_block_id_part_set_header_total) =>
				last_block_id_part_set_header_total,
			_ =>
				return Err(ClientError::Other(
					"last_block_id_part_set_header_total not found".to_string(),
				)),
		};

	let last_block_id_part_set_header_hash =
		match last_block_id_part_set_header.get(1).cloned().ok_or(ClientError::Other(
			"failed to get index 1 and retrieve last_block_id_part_set_header_hash".to_string(),
		))? {
			EthersToken::Bytes(last_block_id_part_set_header_hash) =>
				last_block_id_part_set_header_hash,
			_ =>
				return Err(ClientError::Other(
					"last_block_id_part_set_header_hash not found".to_string(),
				)),
		};

	let signed_header_commit = match toks.get(0).cloned().ok_or(ClientError::Other(
		"failed to get index 0 and retrieve signed_header_commit".to_string(),
	))? {
		EthersToken::Tuple(toks) => match toks.get(1).cloned().ok_or(ClientError::Other(
			"failed to get index 1 and retrieve signed_header_commit".to_string(),
		))? {
			EthersToken::Tuple(toks) => toks,
			_ => return Err(ClientError::Other("signed_header_commit' not found".to_string())),
		},
		_ => return Err(ClientError::Other("signed_header_commit not found".to_string())),
	};

	let last_commit_height = match signed_header_commit.get(0).cloned().ok_or(
		ClientError::Other("failed to get index 0 and retrieve last_commit_height".to_string()),
	)? {
		EthersToken::Int(last_commit_height) => last_commit_height,
		_ => return Err(ClientError::Other("last_commit_height not found".to_string())),
	};

	let last_commit_round = match signed_header_commit.get(1).cloned().ok_or(ClientError::Other(
		"failed to get index 1 and retrieve last_commit_round".to_string(),
	))? {
		EthersToken::Int(last_commit_round) => last_commit_round,
		_ => return Err(ClientError::Other("last_commit_round not found".to_string())),
	};

	let last_commit_block_id = match signed_header_commit.get(2).cloned().ok_or(
		ClientError::Other("failed to get index 2 and retrieve last_commit_block_id".to_string()),
	)? {
		EthersToken::Tuple(toks) => toks,
		_ => return Err(ClientError::Other("last_commit_block_id not found".to_string())),
	};

	let last_commit_block_id_hash =
		match last_commit_block_id.get(0).cloned().ok_or(ClientError::Other(
			"failed to get index 0 and retrieve last_commit_block_id_hash".to_string(),
		))? {
			EthersToken::Bytes(last_commit_block_id_hash) => last_commit_block_id_hash,
			_ => return Err(ClientError::Other("last_commit_block_id_hash not found".to_string())),
		};

	let last_commit_block_id_part_set_header =
		match last_commit_block_id.get(1).cloned().ok_or(ClientError::Other(
			"failed to get index 1 and retrieve last_commit_block_id_part_set_header".to_string(),
		))? {
			EthersToken::Tuple(toks) => toks,
			_ =>
				return Err(ClientError::Other(
					"last_commit_block_id_part_set_header not found".to_string(),
				)),
		};

	let last_commit_block_id_part_set_header_total =
		match last_commit_block_id_part_set_header.get(0).cloned().ok_or(ClientError::Other(
			"failed to get index 0 and retrieve last_commit_block_id_part_set_header_total"
				.to_string(),
		))? {
			EthersToken::Uint(last_commit_block_id_part_set_header_total) =>
				last_commit_block_id_part_set_header_total,
			_ =>
				return Err(ClientError::Other(
					"last_commit_block_id_part_set_header_total not found".to_string(),
				)),
		};

	let last_commit_block_id_part_set_header_hash =
		match last_commit_block_id_part_set_header.get(1).cloned().ok_or(ClientError::Other(
			"failed to get index 1 and retrieve last_commit_block_id_part_set_header_hash"
				.to_string(),
		))? {
			EthersToken::Bytes(last_commit_block_id_part_set_header_hash) =>
				last_commit_block_id_part_set_header_hash,
			_ =>
				return Err(ClientError::Other(
					"last_commit_block_id_part_set_header_hash not found".to_string(),
				)),
		};

	let trusted_height = match toks.get(1).cloned().ok_or(ClientError::Other(
		"failed to get index 1 and retrieve trusted_height".to_string(),
	))? {
		EthersToken::Int(trusted_height) => trusted_height,
		_ => return Err(ClientError::Other("trusted_height not found".to_string())),
	};

	let chain_id: ChainId = chain_id.parse()?;
	let revision_number = chain_id.version();
	Ok(Header {
		signed_header: SignedHeader::new(
			block::Header {
				version: Version { block: version_block.as_u64(), app: version_app.as_u64() },
				chain_id: chain_id.into(),
				height: TmHeight::try_from(height.as_u64()).map_err(|e| {
					ClientError::Other(format!("failed to convert height into TmHeight: {}", e))
				})?,
				time: Time::try_from(Timestamp {
					seconds: time_secs.as_u64() as _,
					nanos: time_nanos.as_u32() as _,
				})?,
				last_block_id: Some(block::Id {
					hash: Hash::Sha256(last_block_id_hash.as_slice().try_into().map_err(|e| {
						ClientError::Other(format!("failed to convert last_block_id_hash into : Hash::Sha256 {:?}", e))
					})?),
					part_set_header: block::parts::Header::new(
						last_block_id_part_set_header_total.as_u32(),
						Hash::Sha256(
							last_block_id_part_set_header_hash.to_vec().try_into().map_err(|e| {
								ClientError::Other(format!("failed to convert last_block_id_part_set_header_hash into : Hash::Sha256 {:?}", e))
							})?
						),
					)
					.unwrap(),
				}),
				last_commit_hash: Some(Hash::Sha256(last_commit_hash.to_vec().try_into().map_err(|e| {
					ClientError::Other(format!("failed to convert last_commit_hash into : Hash::Sha256 {:?}", e))
				})?)),
				data_hash: Some(Hash::Sha256(data_hash.to_vec().try_into().map_err(|e| {
					ClientError::Other(format!("failed to convert data_hash into : Hash::Sha256 {:?}", e))
				})?)),
				validators_hash: Hash::Sha256(validators_hash.to_vec().try_into().map_err(|e| {
					ClientError::Other(format!("failed to convert validators_hash into : Hash::Sha256 {:?}", e))
				})?),
				next_validators_hash: Hash::Sha256(
					next_validators_hash.to_vec().try_into().map_err(|e| {
						ClientError::Other(format!("failed to convert next_validators_hash into : Hash::Sha256 {:?}", e))
					})?
				),
				consensus_hash: Hash::Sha256(consensus_hash.to_vec().try_into().map_err(|e| {
					ClientError::Other(format!("failed to convert consensus_hash into : Hash::Sha256 {:?}", e))
				})?),
				app_hash: AppHash::try_from(app_hash.to_vec()).map_err(|e| {
					ClientError::Other(format!("failed to convert app_hash into : AppHash {:?}", e))
				})?,
				last_results_hash: Some(Hash::Sha256(
					last_results_hash.to_vec().try_into().map_err(|e| {
						ClientError::Other(format!("failed to convert last_results_hash into : Hash::Sha256 {:?}", e))
					})?
				)),
				evidence_hash: Some(Hash::Sha256(evidence_hash.to_vec().try_into().map_err(|e| {
					ClientError::Other(format!("failed to convert evidence_hash into : Hash::Sha256 {:?}", e))
				})?
				)),
				proposer_address: account::Id::try_from(proposer_address.to_vec()).map_err(|e| {
					ClientError::Other(format!("failed to convert proposer_address into : account::Id {:?}", e))
				})?
			},
			block::Commit {
				height: TmHeight::try_from(last_commit_height.as_u64()).map_err(|e| {
					ClientError::Other(format!("failed to convert last_commit_height into TmHeight: {}", e))
				})?,
				round: last_commit_round.as_u32().try_into().map_err(|e| {
					ClientError::Other(format!("failed to convert last_commit_round into u32: {}", e))
				})?,
				block_id: block::Id {
					hash: Hash::Sha256(last_commit_block_id_hash.to_vec().try_into().map_err(
						|e| {
							ClientError::Other(format!("failed to convert last_commit_block_id_hash into : Hash::Sha256 {:?}", e))
						},
					)?),
					part_set_header: block::parts::Header::new(
						last_commit_block_id_part_set_header_total.as_u32(),
						Hash::Sha256(
							last_commit_block_id_part_set_header_hash.to_vec().try_into().map_err(
								|e| {
									ClientError::Other(format!("failed to convert last_commit_block_id_part_set_header_hash into : Hash::Sha256 {:?}", e))
								},
							)?,
						),
					)
					.map_err(
						|e| ClientError::Other(format!("failed to create part_set_header: {}", e)),
					)?,
				},
				signatures: vec![],
			},
		)
		.map_err(|e| ClientError::Other(format!("failed to create SignedHeader: {:?}", e)))?,
		validator_set: tendermint::validator::Set::new(vec![], None),
		trusted_height: Height::new(revision_number, trusted_height.as_u64()),
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
			// version
			EthersToken::Tuple(
				[
					// identifier
					EthersToken::String(
						x.clone().version.map(|x| x.identifier().to_owned()).unwrap_or_default(),
					),
					// features
					EthersToken::Array(
						x.version
							.clone()
							.map(|x| {
								x.features()
									.clone()
									.into_iter()
									.map(EthersToken::String)
									.collect::<Vec<_>>()
							})
							.unwrap_or_default(),
					),
				]
				.to_vec(),
			),
			// delay_period
			EthersToken::Uint(x.delay_period.as_nanos().into()),
		]
		.to_vec(),
	);
	consensus_state_data
}

pub fn msg_connection_open_ack_token(
	msg: MsgConnectionOpenAck<LocalClientTypes>,
) -> Result<Token, ClientError> {
	use ethers::abi::Token as EthersToken;

	let client_state = msg
		.client_state
		.ok_or_else(|| {
			ClientError::Other(
				"client state not found in msg_connection_open_ack_token".to_string(),
			)
		})?
		.encode_to_vec()
		.map_err(|e| ClientError::Other(format!("failed to encode client state: {:?}", e)))?;

	let consensus_state_data = EthersToken::Tuple(
		[
			// connectionId
			EthersToken::String(msg.connection_id.as_str().to_owned()),
			//clientStateBytes
			EthersToken::Bytes(client_state),
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
						msg.proofs
							.consensus_proof()
							.ok_or(ClientError::Other("consensus_proof not found".to_string()))?
							.height()
							.revision_number
							.into(),
					),
					//revisionHeight
					EthersToken::Uint(
						msg.proofs
							.consensus_proof()
							.ok_or(ClientError::Other("consensus_proof not found".to_string()))?
							.height()
							.revision_height
							.into(),
					),
				]
				.to_vec(),
			),
		]
		.to_vec(),
	);
	Ok(consensus_state_data)
}

fn msg_connection_open_try_token(
	msg: MsgConnectionOpenTry<LocalClientTypes>,
) -> Result<Token, ClientError> {
	let client_state_data_vec = msg
		.client_state
		.ok_or_else(|| {
			ClientError::Other(
				"client state not found in msg_connection_open_try_token".to_string(),
			)
		})?
		.encode_to_vec()
		.map_err(|e| ClientError::Other(format!("failed to encode client state: {:?}", e)))?;
	let conn_open_try = YuiMsgConnectionOpenTry {
		counterparty: msg.counterparty.into(),
		delay_period: msg.delay_period.as_nanos() as _,
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
		consensus_height: msg
			.proofs
			.consensus_proof()
			.ok_or(ClientError::Other("consensus_proof not found".to_string()))?
			.height()
			.into(),
	};
	let conn_open_try_token = conn_open_try.into_token();
	Ok(conn_open_try_token)
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

	async fn estimate_weight(&self, msg: Vec<Any>) -> Result<u64, Self::Error> {
		let method = self.ibc_messages_to_contract_call(msg).await?;
		let gas = method
			.estimate_gas()
			.await
			.map_err(|e| ClientError::Other(format!("failed to estimate gas for tx: {:?}", e)))?;
		Ok(gas.as_u64())
	}

	async fn finality_notifications(
		&self,
	) -> Result<Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send>>, Self::Error> {
		let ws = self.websocket_provider().await?;

		let delay = self.expected_block_time() * INDEXER_DELAY_BLOCKS as u32;
		let stream = async_stream::stream! {
			// TODO: is it really finalized blocks stream?
			let mut stream = ws.subscribe_blocks().await.expect("failed to subscribe to blocks");

			while let Some(block) = stream.next().await {
				// Add delay for the indexer to be able to add the block in DB
				sleep(delay).await;
				yield block
			}
		};

		Ok(stream.boxed())
	}

	async fn submit(&self, messages: Vec<Any>) -> Result<Self::TransactionId, Self::Error> {
		let method = self.ibc_messages_to_contract_call(messages).await?;
		let data = method.call().await?;
		let receipt = method
			.send()
			.await?
			.await?
			.ok_or_else(|| ClientError::Other("tx failed".into()))?;
		handle_gas_usage(&receipt);
		if receipt.status != Some(1.into()) {
			return Err(ClientError::Other(format!("tx failed: {:?}", receipt)))
		}

		tokio::time::sleep(Duration::from_secs(15)).await;

		for (i, (success, result)) in
			data.0.into_iter().zip(data.1.into_iter()).into_iter().enumerate()
		{
			if !success {
				log::error!(target: "hyperspace_ethereum", "tx failed {i}: {}", hex::encode(&result));
			}
		}

		let block_hash = receipt.block_hash.ok_or(ClientError::Other(format!(
			"Block hash is missing for tx hash: {:?}",
			receipt.transaction_hash
		)))?;
		Ok((block_hash, receipt.transaction_hash))
		// Ok(Default::default())
	}

	async fn query_client_message(
		&self,
		update: UpdateClient,
	) -> Result<AnyClientMessage, Self::Error> {
		unimplemented!("query_client_message")
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

	fn set_client_id_ref(&mut self, client_id: Arc<Mutex<Option<ClientId>>>) {
		self.counterparty_client_id = client_id;
	}

	fn get_counterparty_client_id_ref(&self) -> Arc<Mutex<Option<ClientId>>> {
		self.client_id.clone()
	}
}

impl EthereumClient {
	async fn ibc_messages_to_contract_call(
		&self,
		messages: Vec<Any>,
	) -> Result<FunctionCall<Arc<ProviderImpl>, ProviderImpl, (Vec<bool>, Vec<Vec<u8>>)>, ClientError>
	{
		use ethers::abi::{encode as ethers_encode, Token as EthersToken};

		if messages.is_empty() {
			return Err(ClientError::Other("messages are empty".into()))
		}

		sleep(self.expected_block_time() * INDEXER_DELAY_BLOCKS as u32).await;

		info!(target: "hyperspace_ethereum", "Submitting messages: {:?}", messages.iter().map(|x| x.type_url.clone()).collect::<Vec<_>>().join(", "));

		let mut temp_client_state: Option<ClientState<()>> = None;

		let mut calls = vec![];
		for msg in messages {
			if msg.type_url == ibc::core::ics02_client::msgs::create_client::TYPE_URL {
				let msg = MsgCreateAnyClient::<LocalClientTypes>::decode_vec(&msg.value).map_err(
					|_| ClientError::Other("create_client: unsupported client state".into()),
				)?;
				let AnyClientState::Tendermint(client_state) = msg.client_state.unpack_recursive()
				else {
					return Err(ClientError::Other("create_client: unsupported client state".into()))
				};
				let AnyConsensusState::Tendermint(client_consensus) =
					msg.consensus_state.unpack_recursive()
				else {
					return Err(ClientError::Other(
						"create_client: unsupported consensus state".into(),
					))
				};
				let client_state_abi_token = client_state_abi_token(client_state);
				let consensus_state_abi_token = consensus_state_abi_token(client_consensus);

				let client_state_data_vec = ethers_encode(&[client_state_abi_token]);
				let consensus_state_data_vec = ethers_encode(&[consensus_state_abi_token]);

				let token = EthersToken::Tuple(vec![
					//should be the same that we use to register client
					//client type
					EthersToken::String(self.config.client_type.clone()),
					//clientStateBytes
					EthersToken::Bytes(client_state_data_vec.clone()),
					//consensusStateBytes
					EthersToken::Bytes(consensus_state_data_vec.clone()),
				]);

				let bts = ethers::abi::encode(&vec![token.clone()]);
				let bytes = self.yui.create_client_calldata(token).await;

				calls.push(bytes);
			} else if msg.type_url == ibc::core::ics02_client::msgs::update_client::TYPE_URL {
				let msg = MsgUpdateAnyClient::<LocalClientTypes>::decode_vec(&msg.value).map_err(
					|_| ClientError::Other("update_client: failed to decode_vec".into()),
				)?;

				let AnyClientMessage::TendermintZk(client_state) =
					msg.client_message.unpack_recursive()
				else {
					return Err(ClientError::Other(format!("update_client: unsupported client_state. Should be AnyClientMessage::Tendermint")))
				};
				let ics07_tendermint_zk::client_message::ZkClientMessage::Header(header) = client_state else {
					return Err(ClientError::Other("update_client: unsupported Client Message. Should be ClientMessage::Header".into()))
				};

				let client_id = msg.client_id.clone();
				{
					//update arc mutex with a new client id
					let mut m = self.counterparty_client_id.lock().unwrap();
					*m = Some(client_id.clone());
				}

				let latest_client_state = if let Some(cs) = &temp_client_state {
					Token::Bytes(ethabi::encode(&[client_state_abi_token(cs)]))
				} else {
					let (state, _) =
						self.get_latest_client_state_exact_token(client_id.clone()).await?;
					let x = client_state_from_abi_token::<()>(state.clone())?;
					temp_client_state = Some(x.clone());
					Token::Bytes(ethabi::encode(&[client_state_abi_token(&x)]))
				};

				if let Some(cs) = &mut temp_client_state {
					cs.latest_height.revision_height = header.signed_header.header.height.value();
				}

				if header.zk_proof.len() <= 0 {
					let error_message = format!("zk_proof is missing for update_client for client_id: {:?} height: {:?}", client_id, header.height());
					error!(target: "hyperspace_ethereum", "{}", error_message);
					return Err(ClientError::Other(error_message));
				}

				if header.zk_bitmask <= 0 {
					let error_message = format!("zk_bitmask is missing for update_client for client_id: {:?} height: {:?}", client_id, header.height());
					error!(target: "hyperspace_ethereum", "{}", error_message);
					return Err(ClientError::Other(error_message));
				}

				//get abi token to update client
				let tm_header_abi_token = tm_header_abi_token(header)?;
				let tm_header_bytes = ethers_encode(&[tm_header_abi_token]);

				let zk_proof = header.zk_proof.clone();
				let s = String::from_utf8(zk_proof).map_err(|e| {
					let error_message = format!("failed to convert zk_proof into string: {:?} for client_id: {:?} height: {:?}", e, client_id, header.height());
					error!(target: "hyperspace_ethereum", "{}", error_message);
					ClientError::Other(error_message)
				})?;

				let s = format!(" [{}]", s);


				//todo. add map error here once i get a new zk curctui strucutre with a data from remote ZK api service
				let json_data: serde_json::Value = serde_json::from_str(s.as_str()).unwrap();
				//take a first element of the array as a array of strings
				let sa = json_data[0].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect::<Vec<_>>();
				//take second element of the array as a array of arrays of strings
				let sb = json_data[1].as_array().unwrap().iter().map(|x| x.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect::<Vec<_>>()).collect::<Vec<_>>();
				//take third element of the array as a array of strings
				let sc = json_data[2].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect::<Vec<_>>();

				trace!(target: "hyperspace_ethereum", "json_data: {:?}", json_data.clone());
				trace!(target: "hyperspace_ethereum", "____________________________________");
				trace!(target: "hyperspace_ethereum", "json   sa: {:?}", sa);
				trace!(target: "hyperspace_ethereum", "____________________________________");
				trace!(target: "hyperspace_ethereum", "json   sb: {:?}", sb);
				trace!(target: "hyperspace_ethereum", "____________________________________");
				trace!(target: "hyperspace_ethereum", "json   sc: {:?}", sc);

				fn get_u256(s0x : &str) -> U256{
					let s : &'static str = Box::leak(s0x.to_string().into_boxed_str());
					let s = s.trim_start_matches("0x");
					let pi_a_0 = U256::from(s);
					return pi_a_0;
				}

				let pi_a_0 = sa[0].clone();
				let pi_a_1 = sa[1].clone();

				let pi_b_0 = sb[0][0].clone();
				let pi_b_1 = sb[0][1].clone();
				let pi_b_2 = sb[1][0].clone();
				let pi_b_3 = sb[1][1].clone();

				let pi_c_0 = sc[0].clone();
				let pi_c_1 = sc[1].clone();
				
				let pi_a_0 = get_u256(pi_a_0.as_str());
				let pi_a_1 = get_u256(pi_a_1.as_str());

				let pi_b_0 = get_u256(pi_b_0.as_str());
				let pi_b_1 = get_u256(pi_b_1.as_str());
				let pi_b_2 = get_u256(pi_b_2.as_str());
				let pi_b_3 = get_u256(pi_b_3.as_str());

				let pi_c_0 = get_u256(pi_c_0.as_str());
				let pi_c_1 = get_u256(pi_c_1.as_str());

				/*
					uint[2] _pA;
					uint[2][2] _pB;
					uint[2] _pC;
					uint[3] _pubSignals;
				*/

				let token = EthersToken::Tuple(vec![
					//should be the same that we use to create client
					//client id
					EthersToken::String(client_id.to_string()),
					//tm header
					EthersToken::Bytes(tm_header_bytes),
					// previous client state
					latest_client_state, // EthersToken::Bytes(latest_client_state),
					//zk proof
					EthersToken::Bytes(header.zk_proof.clone()),
					//zk bitmask
					EthersToken::Uint(header.zk_bitmask.into()),
					//uint[2] _pA;
					EthersToken::FixedArray(vec![
						EthersToken::Uint(pi_a_0),
						EthersToken::Uint(pi_a_1),
					]),
					//uint[2][2] _pB;
					EthersToken::FixedArray(vec![
						EthersToken::FixedArray(vec![
							EthersToken::Uint(pi_b_0),
							EthersToken::Uint(pi_b_1),
						]),
						EthersToken::FixedArray(vec![
							EthersToken::Uint(pi_b_2),
							EthersToken::Uint(pi_b_3),
						]),
					]),
					//uint[2] _pC;
					EthersToken::FixedArray(vec![
						EthersToken::Uint(pi_c_0),
						EthersToken::Uint(pi_c_1),
					]),
				]);
				

				calls.push(self.yui.update_client_calldata(token).await);
			} else if msg.type_url == ibc::core::ics03_connection::msgs::conn_open_init::TYPE_URL {
				let msg = MsgConnectionOpenInit::decode_vec(&msg.value).map_err(|_| {
					ClientError::Other("conn_open_init: failed to decode_vec".into())
				})?;
				let token = msg_connection_open_init_token(msg);
				calls.push(self.yui.connection_open_init_calldata(token).await);
			} else if msg.type_url == ibc::core::ics03_connection::msgs::conn_open_ack::TYPE_URL {
				let mut msg = MsgConnectionOpenAck::<LocalClientTypes>::decode_vec(&msg.value)
					.map_err(|_| {
						ClientError::Other("conn_open_ack: failed to decode_vec".into())
					})?;

				let mut proof = msg.proofs.consensus_proof().ok_or_else(|| {
					ClientError::Other("conn_open_ack: consensus_proof not found".into())
				})?;
				let bytes = proof.proof();

				info!("Using proof height at: {}", proof.height());
				let block = self
					.client()
					.get_block(proof.height().revision_height)
					.await
					.map_err(|e| {
						ClientError::Other(format!("conn_open_ack: failed to get block: {:?}", e))
					})?
					.ok_or_else(|| {
						ClientError::Other("conn_open_ack: failed to get block".into())
					})?;
				let header: EthHeader = block.clone().into();
				let rlp_encoded_header = rlp::encode(&header).to_vec();

				if block.hash.clone().unwrap().0 != keccak256(&rlp_encoded_header) {
					info!(
						"{:?} != {:?}\n{}",
						block.hash.clone().unwrap(),
						H256(keccak256(&rlp_encoded_header)),
						hex::encode(rlp_encoded_header)
					);
					return Err(ClientError::Other("conn_open_ack: block hash doesn't match".into()))
				}
				let new_proof = EthereumClientPrimitivesConsensusStateProof {
					header: rlp_encoded_header,
					merkleProof: clear_proof_value(&bytes)?.as_bytes().to_vec(),
					isWasm: true, // TODO: replace with the actual value
				}
				.abi_encode();
				msg.proofs.consensus_proof = Some(
					ConsensusProof::new(
						CommitmentProofBytes::try_from(new_proof.to_vec())
							.expect("proof can't be empty"),
						proof.height(),
					)
					.expect("proof can't be empty"),
				);
				let commitment_proof = msg.proofs.client_proof().as_ref().expect("no proof");
				msg.proofs.client_proof = Some(clear_proof_value(commitment_proof)?);

				let mut token = msg_connection_open_ack_token(msg)?;
				// log::info!("tok={token:?}");
				std::fs::write("token.txt", format!("{:?}", token)).unwrap();

				calls.push(self.yui.connection_open_ack_calldata(token).await);
			} else if msg.type_url == ibc::core::ics03_connection::msgs::conn_open_try::TYPE_URL {
				let msg = MsgConnectionOpenTry::<LocalClientTypes>::decode_vec(&msg.value)
					.map_err(|_| {
						ClientError::Other("conn_open_try: failed to decode_vec".into())
					})?;
				let mut token = msg_connection_open_try_token(msg.clone())?;
				let Token::Tuple(ref mut tokens) = token else {
					return Err(ClientError::Other(format!("Token should be tuple")))
				};

				calls.push(self.yui.connection_open_try_calldata(token).await);
			} else if msg.type_url == ibc::core::ics03_connection::msgs::conn_open_confirm::TYPE_URL
			{
				let msg = MsgConnectionOpenConfirm::decode_vec(&msg.value).map_err(|_| {
					ClientError::Other("conn_open_confirm: failed to decode_vec".into())
				})?;
				let mut token = msg_connection_open_confirm_token(msg);
				let Token::Tuple(ref mut tokens) = token else {
					return Err(ClientError::Other(format!("Token should be tuple")))
				};

				calls.push(self.yui.connection_open_confirm_calldata(token).await);
			} else if msg.type_url == channel_msgs::chan_open_init::TYPE_URL {
				let msg = MsgChannelOpenInit::decode_vec(&msg.value).map_err(|_| {
					ClientError::Other("chan_open_init: failed to decode_vec".into())
				})?;
				log::info!("msg.channel.ordering = {}", msg.channel.ordering);
				let token = msg.into_token();
				calls.push(self.yui.channel_open_init_calldata(token).await);
			} else if msg.type_url == channel_msgs::chan_open_try::TYPE_URL {
				let msg = MsgChannelOpenTry::decode_vec(&msg.value).map_err(|_| {
					ClientError::Other("chan_open_try: failed to decode_vec".into())
				})?;
				let mut token = msg.into_token();

				let Token::Tuple(ref mut tokens) = token else {
					return Err(ClientError::Other(format!("Token should be tuple")))
				};

				calls.push(self.yui.channel_open_try_calldata(token).await);
			} else if msg.type_url == channel_msgs::chan_open_ack::TYPE_URL {
				let msg = MsgChannelOpenAck::decode_vec(&msg.value).map_err(|e| {
					ClientError::Other(format!("chan_open_ack: failed to decode_vec: {:?}", e))
				})?;
				// log::info!("msg = {msg:#?}");
				let mut token = msg.into_token();
				let Token::Tuple(ref mut tokens) = token else {
					return Err(ClientError::Other(format!("Token should be tuple")))
				};

				calls.push(self.yui.send_and_get_tuple_calldata(token, "channelOpenAck").await);
			} else if msg.type_url == channel_msgs::chan_open_confirm::TYPE_URL {
				let msg = MsgChannelOpenConfirm::decode_vec(&msg.value).map_err(|e| {
					ClientError::Other(format!(
						"chann_open_confirm: failed to decode_vec : {:?}",
						e
					))
				})?;
				let mut token = msg.into_token();

				let Token::Tuple(ref mut tokens) = token else {
					return Err(ClientError::Other(format!("Token should be tuple")))
				};

				calls.push(self.yui.send_and_get_tuple_calldata(token, "channelOpenConfirm").await);
			} else if msg.type_url == channel_msgs::chan_close_init::TYPE_URL {
				let msg = MsgChannelCloseInit::decode_vec(&msg.value).map_err(|e| {
					ClientError::Other(format!("chan_close_init: failed to decode_vec : {:?}", e))
				})?;
				let token = msg.into_token();
				calls.push(self.yui.send_and_get_tuple_calldata(token, "channelCloseInit").await);
			} else if msg.type_url == channel_msgs::chan_close_confirm::TYPE_URL {
				let msg = MsgChannelCloseConfirm::decode_vec(&msg.value).map_err(|e| {
					ClientError::Other(format!(
						"chan_close_confirm: failed to decode_vec : {:?}",
						e
					))
				})?;
				let mut token = msg.into_token();
				let Token::Tuple(ref mut tokens) = token else {
					return Err(ClientError::Other(format!("Token should be tuple")))
				};

				let client_id = (*self.counterparty_client_id.lock().unwrap()).clone().unwrap();
				// let (latest_client_state, latest_height) =
				// 	self.get_latest_client_state_exact_token(client_id.clone()).await?;
				// let latest_consensus_state = self
				// 	.get_latest_consensus_state_encoded_abi_token(client_id.clone(), latest_height)
				// 	.await?;

				calls
					.push(self.yui.send_and_get_tuple_calldata(token, "channelCloseConfirm").await);
			} else if msg.type_url == channel_msgs::timeout_on_close::TYPE_URL {
				let msg = MsgTimeoutOnClose::decode_vec(&msg.value).map_err(|e| {
					ClientError::Other(format!("timeout_on_close: failed to decode_vec : {:?}", e))
				})?;
				let token = msg.into_token();
				calls.push(self.yui.send_and_get_tuple_calldata(token, "timeoutOnClose").await);
			} else if msg.type_url == channel_msgs::timeout::TYPE_URL {
				let msg = MsgTimeout::decode_vec(&msg.value).map_err(|e| {
					ClientError::Other(format!("timeout: failed to decode_vec : {:?}", e))
				})?;
				let token = msg.into_token();
				calls.push(self.yui.send_and_get_tuple_calldata(token, "timeoutPacket").await);
			} else if msg.type_url == channel_msgs::acknowledgement::TYPE_URL {
				let msg = MsgAcknowledgement::decode_vec(&msg.value).map_err(|e| {
					ClientError::Other(format!("acknowledgement: failed to decode_vec : {:?}", e))
				})?;
				let mut token = msg.into_token();

				let Token::Tuple(ref mut tokens) = token else {
					return Err(ClientError::Other(format!("Token should be tuple")))
				};

				let client_id = (*self.counterparty_client_id.lock().unwrap()).clone().unwrap();
				// let (latest_client_state, latest_height) =
				// 	self.get_latest_client_state_exact_token(client_id.clone()).await?;
				// let latest_consensus_state = self
				// 	.get_latest_consensus_state_encoded_abi_token(client_id.clone(), latest_height)
				// 	.await?;

				calls.push(self.yui.send_and_get_tuple_calldata(token, "acknowledgePacket").await);
			} else if msg.type_url == channel_msgs::recv_packet::TYPE_URL {
				let msg = MsgRecvPacket::decode_vec(&msg.value).map_err(|e| {
					ClientError::Other(format!("recv_packet: failed to decode_vec : {:?}", e))
				})?;
				let mut token = msg.into_token();

				let Token::Tuple(ref mut tokens) = token else {
					return Err(ClientError::Other(format!("Token should be tuple")))
				};

				let client_id = (*self.counterparty_client_id.lock().unwrap()).clone().unwrap();
				// let (latest_client_state, latest_height) =
				// 	self.get_latest_client_state_exact_token(client_id.clone()).await?;
				// let latest_consensus_state = self
				// 	.get_latest_consensus_state_encoded_abi_token(client_id.clone(), latest_height)
				// 	.await?;

				calls.push(self.yui.send_and_get_tuple_calldata(token, "recvPacket").await);
			} else {
				return Err(ClientError::Other(format!(
					"does not support this msg type for now: {}",
					msg.type_url,
				)))
			}
		}

		let method = self.yui.method::<_, (Vec<bool>, Vec<Vec<u8>>)>("callBatch", calls)?;
		Ok(method)
	}
}
