use crate::primitives::BeefyNextAuthoritySet;
use codec::{Decode, Encode};
use sp_core::H256;
use sp_std::prelude::*;

#[derive(sp_std::fmt::Debug, Encode, Decode, PartialEq, Eq, Clone)]
pub struct ClientState {
    pub latest_beefy_height: u32,
    pub mmr_root_hash: H256,
    pub current_authorities: BeefyNextAuthoritySet<H256>,
    pub next_authorities: BeefyNextAuthoritySet<H256>,
}


pub trait HostFunctions {
    fn keccak_256(input: &[u8]) -> [u8; 32];
    fn secp256k1_ecdsa_recover_compressed(
        signature: &[u8; 65],
        value: &[u8; 32],
    ) -> Option<Vec<u8>>;
}
