use anchor_client::solana_sdk::{
	ed25519_instruction::SIGNATURE_OFFSETS_SERIALIZED_SIZE, instruction::Instruction,
};
use itertools::izip;
use sigverify::ed25519_program::{new_instruction, Entry, SignatureOffsets};
use tendermint_light_client_verifier_new::types::Commit;
use tendermint_new::{
	block::CommitSig,
	vote::{ValidatorIndex, Vote},
};

pub fn non_absent_vote(
	commit_sig: &CommitSig,
	validator_index: ValidatorIndex,
	commit: &Commit,
) -> Option<Vote> {
	let (validator_address, timestamp, signature, block_id) = match commit_sig {
		CommitSig::BlockIdFlagAbsent { .. } => return None,
		CommitSig::BlockIdFlagCommit { validator_address, timestamp, signature } =>
			(*validator_address, *timestamp, signature, Some(commit.block_id)),
		CommitSig::BlockIdFlagNil { validator_address, timestamp, signature } =>
			(*validator_address, *timestamp, signature, None),
	};

	Some(Vote {
		vote_type: tendermint_new::vote::Type::Precommit,
		height: commit.height,
		round: commit.round,
		block_id,
		timestamp: Some(timestamp),
		validator_address,
		validator_index,
		signature: signature.clone(),
		extension: Default::default(),
		extension_signature: None,
	})
}

/// Solana sdk only accepts a keypair to form ed25519 instruction.
/// Until they implement a method which accepts a pubkey and signature instead of keypair
/// we have to use the below method instead.
///
/// Reference: https://github.com/solana-labs/solana/pull/32806
pub fn new_ed25519_instruction_with_signature(
	pubkeys: Vec<Vec<u8>>,
	signatures: Vec<Vec<u8>>,
	messages: Vec<Vec<u8>>,
) -> Instruction {
	use anchor_client::solana_sdk::ed25519_instruction::{
		DATA_START, PUBKEY_SERIALIZED_SIZE, SIGNATURE_SERIALIZED_SIZE,
	};
	use bytemuck::bytes_of;
	assert_eq!(signatures.len(), messages.len());
	let num_signatures: u8 = signatures.len().try_into().unwrap();
	let mut instruction_data = Vec::new();
	instruction_data.extend_from_slice(&[num_signatures, 0]);
	let mut offset = 0;
	for (index, _) in signatures.iter().enumerate() {
		let signature = &signatures[index];
		let message = &messages[index];
		let pubkey = &pubkeys[index];
		assert_eq!(pubkey.len(), PUBKEY_SERIALIZED_SIZE);
		assert_eq!(signature.len(), SIGNATURE_SERIALIZED_SIZE);

		let public_key_offset = offset + DATA_START;
		let signature_offset = public_key_offset.saturating_add(PUBKEY_SERIALIZED_SIZE);
		let message_data_offset = signature_offset.saturating_add(SIGNATURE_SERIALIZED_SIZE);

		let offsets = SignatureOffsets {
			signature_offset: signature_offset as u16,
			signature_instruction_index: u16::MAX,
			public_key_offset: public_key_offset as u16,
			public_key_instruction_index: u16::MAX,
			message_data_offset: message_data_offset as u16,
			message_data_size: message.len() as u16,
			message_instruction_index: u16::MAX,
		};
		let current_instruction = [bytes_of(&offsets), &pubkey, &signature, &message].concat();
		instruction_data.extend_from_slice(&current_instruction);
		offset += SIGNATURE_OFFSETS_SERIALIZED_SIZE
			.saturating_add(SIGNATURE_SERIALIZED_SIZE)
			.saturating_add(PUBKEY_SERIALIZED_SIZE)
			.saturating_add(message.len())
	}

	// let instruction =
	// 	[&[num_signatures, 0], bytes_of(&offsets), pubkey, signature, message].concat();

	Instruction {
		program_id: anchor_client::solana_sdk::ed25519_program::id(),
		accounts: vec![],
		data: instruction_data,
	}
}

pub fn ed25519_signature_payload(entries: Vec<Entry>) -> Option<Instruction> {
	new_instruction(&entries)
}


/// Displays the error if present, waits for few seconds and
/// retries execution.
///
/// The error is usually due to load on rpc which is solved
/// by waiting a few seconds.
#[macro_export]
macro_rules! skip_fail {
	($res:expr) => {
			match $res {
					Ok(val) => val,
					Err(e) => {
							use crate::Duration;
							use std::thread::sleep;
							log::error!("{:?}", e);
							sleep(Duration::from_secs(2));
							continue;
					}
			}
	};
}

pub(crate) use skip_fail;