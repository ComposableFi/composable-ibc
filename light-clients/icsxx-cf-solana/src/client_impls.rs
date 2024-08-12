use alloc::{string::ToString, vec::Vec};

use guestchain::PubKey;
use prost::DecodeError;

use super::{proof, Any, ClientMessage, ClientState, ConsensusState, Header, Misbehaviour};

mod ibc {
	// pub use ibc::core::ics02_client::client_state::{
	//     ClientStateCommon, ClientStateExecution, ClientStateValidation,
	// };
	pub use ibc::core::ics02_client::{
		client_state::Status, error::Error as ClientError, height::Height,
	};
	// pub use ibc::core::ics02_client::context::{
	//     ClientExecutionContext, ClientValidationContext,
	// };
	pub use ibc::core::{
		ics23_commitment::{
			commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
			error::Error as CommitmentError,
		},
		ics24_host::{identifier::ClientId, path, Path::ClientType},
	};
	// pub use ibc::core::ics24_host::{ExecutionContext, ValidationContext};
	pub use ibc::timestamp::Timestamp;
}

type Result<T = (), E = ibc::ClientError> = ::core::result::Result<T, E>;

pub trait CommonContext {
	type ConversionError: ToString;
	type AnyConsensusState: TryInto<ConsensusState, Error = Self::ConversionError>
		+ From<ConsensusState>;

	fn host_metadata(&self) -> Result<(ibc::Timestamp, ibc::Height)>;

	fn consensus_state(
		&self,
		client_id: &ibc::ClientId,
		height: ibc::Height,
	) -> Result<Self::AnyConsensusState>;

	fn store_consensus_state_and_metadata(
		&mut self,
		client_id: &ibc::ClientId,
		height: ibc::Height,
		consensus: Self::AnyConsensusState,
		host_timestamp: ibc::Timestamp,
		host_height: ibc::Height,
	) -> Result;

	fn delete_consensus_state_and_metadata(
		&mut self,
		client_id: &ibc::ClientId,
		height: ibc::Height,
	) -> Result;

	fn sorted_consensus_state_heights(&self, client_id: &ibc::ClientId)
		-> Result<Vec<ibc::Height>>;
}

impl From<proof::VerifyError> for ibc::ClientError {
	fn from(err: proof::VerifyError) -> Self {
		use ::ibc::core::ics23_commitment::error::Error;
		use proof::VerifyError::*;

		Self::invalid_commitment_proof(match err {
			ProofDecodingFailure(msg) =>
				Error::commitment_proof_decoding_failed(DecodeError::new(msg)),
			WrongSequenceNumber(err) => Error::commitment_proof_decoding_failed(err),
			_ => ibc::CommitmentError::invalid_merkle_proof(),
		})
	}
}

impl<PK: PubKey> ClientState<PK> {
	pub fn verify_client_message(
		&self,
		ctx: &impl guestchain::Verifier<PK>,
		_client_id: &ibc::ClientId,
		client_message: ClientMessage<PK>,
	) -> Result<()> {
		match client_message {
			ClientMessage::Header(header) => self.verify_header(ctx, header),
			ClientMessage::Misbehaviour(misbehaviour) =>
				self.verify_misbehaviour(ctx, misbehaviour),
		}
	}

	pub fn check_for_misbehaviour(
		&self,
		ctx: &impl guestchain::Verifier<PK>,
		_client_id: &ibc::ClientId,
		client_message: Any,
	) -> Result<bool> {
		match ClientMessage::<PK>::try_from(client_message)? {
			ClientMessage::Header(header) => self.check_for_misbehaviour_header(ctx, header),
			ClientMessage::Misbehaviour(misbehaviour) =>
				self.check_for_misbehaviour_misbehavior(ctx, misbehaviour),
		}
	}

	fn verify_header(&self, ctx: &impl guestchain::Verifier<PK>, header: Header<PK>) -> Result<()> {
		(|| {
			// panic!("header epoch {:?} and client epoch {:?}", header.epoch_commitment,
			// self.epoch_commitment);
			if header.epoch_commitment != self.epoch_commitment {
				return Err("Unexpected epoch")
			}
			let fp = guestchain::block::Fingerprint::from_hash(
				&header.genesis_hash,
				header.block_header.block_height,
				&header.block_hash,
			);
			let mut quorum_left = header.epoch.quorum_stake().get();
			let mut validators =
				header.epoch.validators().iter().map(Some).collect::<Vec<Option<&_>>>();
			for (idx, sig) in header.signatures {
				let validator = validators
					.get_mut(usize::from(idx))
					.ok_or("Validator index out of bounds")?
					.take()
					.ok_or("Duplicate signature")?;
				if !ctx.verify(fp.as_slice(), &validator.pubkey, &sig) {
					return Err("Bad signature")
				}
				quorum_left = quorum_left.saturating_sub(validator.stake.get());
				if quorum_left == 0 {
					break
				}
			}
			Ok(())
		})()
		.map_err(error)
	}

	fn verify_misbehaviour(
		&self,
		_ctx: &impl guestchain::Verifier<PK>,
		_misbehaviour: Misbehaviour<PK>,
	) -> Result<()> {
		todo!()
	}

	fn check_for_misbehaviour_header(
		&self,
		_ctx: &impl guestchain::Verifier<PK>,
		_header: Header<PK>,
	) -> Result<bool> {
		Ok(false)
	}

	fn check_for_misbehaviour_misbehavior(
		&self,
		_ctx: &impl guestchain::Verifier<PK>,
		_misbehaviour: Misbehaviour<PK>,
	) -> Result<bool> {
		todo!()
	}
}

fn error(msg: impl ToString) -> ibc::ClientError {
	ibc::ClientError::implementation_specific(msg.to_string())
}
