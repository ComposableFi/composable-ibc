//! Light client error definition
use sp_std::prelude::*;
#[derive(sp_std::fmt::Debug, PartialEq, Eq, derive_more::From)]
/// Error definition for the light client
pub enum BeefyClientError {
    /// Failed to read a value from storage
    StorageReadError,
    /// Failed to write a value to storage
    StorageWriteError,
    /// Error decoding some value
    DecodingError,
    /// Invalid Mmr Update
    InvalidMmrUpdate,
    /// Outdated commitment
    OutdatedCommitment {
        /// Latest beefy height stored
        latest_beefy_height: u32,
        /// Commitment block number received
        commitment_block_number: u32,
    },
    /// Mmr root hash not found in commitment
    MmrRootHashNotFound,
    /// Invalid Authority set id received
    AuthoritySetMismatch {
        /// Current authority set id
        current_set_id: u64,
        /// Next authority set id
        next_set_id: u64,
        /// Authority set id in commitment
        commitment_set_id: u64,
    },
    /// Incomplete Signature threshold
    IncompleteSignatureThreshold,
    /// Error recovering public key from signature
    InvalidSignature,
    /// Some invalid merkle root hash
    InvalidRootHash {
        /// Root hash
        root_hash: Vec<u8>,
        /// Root hash length
        len: u64,
    },
    /// Some invalid mmr proof
    InvalidMmrProof,
    /// Invalid authority proof
    InvalidAuthorityProof,
    /// Invalid merkle proof
    InvalidMerkleProof,
    /// Mmr Error
    MmrVerificationError(mmr_lib::Error),
}
