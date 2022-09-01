use crate::justification::{AncestryChain, FinalityProof, GrandpaJustification};
use anyhow::anyhow;
use codec::{Decode, Encode};
use finality_grandpa::Chain;
use futures::StreamExt;
use sp_core::storage::StorageKey;
use sp_finality_grandpa::AuthorityList;
use sp_runtime::traits::{BlakeTwo256, Block, Header};
use sp_state_machine::StorageProof;
use sp_trie::LayoutV0;
use std::collections::HashMap;
use std::hash::Hash;

pub mod justification;

#[cfg(test)]
mod kusama;

pub struct ClientState<H> {
    // Current authority set
    current_authorities: AuthorityList,
    // Id of the current authority set.
    current_set_id: u64,
    // latest finalized hash on the relay chain.
    latest_relay_hash: H,
    // para_id of associated parachain
    para_id: u32,
    // latest height of the parachain.
    latest_para_height: u32,
}

/// Holds relavant parachain proofs for both header and timestamp extrinsic.
pub struct ParachainHeaderProofs {
    /// State proofs that prove a parachain header exists at a given relay chain height
    state_proof: Vec<Vec<u8>>,
    /// Timestamp extrinsic for ibc
    extrinsic: Vec<u8>,
    /// Timestamp extrinsic proof for previously proven parachain header.
    extrinsic_proof: Vec<Vec<u8>>,
}

/// Verify a new grandpa justification, given the old state.
pub fn verify_grandpa_finality_proof<B, H>(a
    mut client_state: ClientState<B::Header>,
    proof: FinalityProof<B::Header>,
) -> Result<ClientState<B::Header>, anyhow::Error>
where
    B: Block,  // relay chain block type
    H: Header, // parachain header
{
    let mut parachain_headers: HashMap<B::Hash, ParachainHeaderProofs> = Default::default();

    // 1. first check that target is in proof.unknown_headers.
    let headers = AncestryChain::<B>::new(&proof.unknown_headers);
    let target = headers
        .header(&proof.block)
        .ok_or_else(|| anyhow!("Target header not found!"))?;

    // 2. next check that there exists a route from client.latest_relay_hash to target.
    let finalized = headers.ancestry(&client_state.latest_relay_hash, &proof.block)?;

    // 3. todo: check for authority set change

    // 4. verify justification.
    let mut justification = GrandpaJustification::<B>::decode(&mut &proof.justification[..])?;
    justification.verify(
        client_state.current_set_id,
        &client_state.current_authorities,
    )?;

    // 5. verify state proofs of parachain headers in finalized relay chain headers.
    for hash in finalized {
        let relay_chain_header = headers
            .header(&hash)
            .expect("Headers have been checked by AncestryChain; qed");
        if let Some(proofs) = parachain_headers.remove(&hash) {
            let ParachainHeaderProofs {
                extrinsic_proof,
                extrinsic,
                state_proof,
            } = proofs;
            let proof = StorageProof::new(state_proof);
            let key = parachain_header_storage_key(client_state.para_id);
            let header = sp_state_machine::read_proof_check(
                relay_chain_header.state_root(),
                proof,
                &[key.as_ref()],
            )?
            .remove(key.as_ref())
            .flatten()
            .ok_or_else(|| anyhow!("Invalid proof, parachain header not found"))?;
            let parachain_header = H::decode(&mut &header[..])?;
            // Timestamp extrinsic should be the first inherent and hence the first extrinsic
            let key = codec::Compact(0u32).encode();
            // verify timestamp extrinsic proof
            sp_trie::verify_trie_proof::<LayoutV0<BlakeTwo256>, _, _, _>(
                parachain_header.state_root(),
                &extrinsic_proof,
                &vec![(key, Some(&extrinsic[..]))],
            )
        }
    }

    // 6. set new client state, optionally rotating authorities
    client_state.latest_relay_hash = target.hash();

    Ok(client_state)
}

/// This returns the storage key for a parachain header on the relay chain.
pub fn parachain_header_storage_key(para_id: u32) -> StorageKey {
    let mut storage_key = frame_support::storage::storage_prefix(b"Paras", b"Heads").to_vec();
    let encoded_para_id = para_id.encode();
    storage_key.extend_from_slice(sp_core::hashing::twox_64(&encoded_para_id).as_slice());
    storage_key.extend_from_slice(&encoded_para_id);
    StorageKey(storage_key)
}
