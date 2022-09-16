use crate::any::client_state::AnyClientState;
use crate::any::consensus_state::AnyConsensusState;
use crate::any::mock::MockClientTypes;
use crate::ics07_tendermint::client_state::ClientState as TMClientState;
use crate::ics07_tendermint::client_state::ClientState as TendermintClientState;
use crate::ics07_tendermint::mock::host::MockHostBlock;
use crate::ics11_beefy::client_def::HostFunctions as BeefyHostFunctions;
use crate::ics11_beefy::client_state::ClientState as BeefyClientState;
use crate::ics11_beefy::client_state::{ClientState, RelayChain};
use crate::ics11_beefy::consensus_state::ConsensusState;
use crate::ics13_near::client_def::HostFunctions as NearHostFunctions;
use crate::ics13_near::client_def::HostFunctionsTrait as NearHostFunctionsTrait;
use crate::AnyHostFunctionsTrait;
use beefy_client_primitives::error::BeefyClientError;
use beefy_client_primitives::HostFunctions;
use core::time::Duration;
use frame_support::log::debug;
use ibc::core::ics02_client::client_state::ClientType;
use ibc::core::ics02_client::error::Error;
use ibc::core::ics24_host::identifier::ChainId;
use ibc::core::ics24_host::identifier::ClientId;
use ibc::mock::client_state::{MockClientRecord, MockClientState, MockConsensusState};
use ibc::mock::context::MockContext;
use ibc::mock::header::MockHeader;
use ibc::prelude::*;
use ibc::timestamp::Timestamp;
use ibc::Height;
use primitive_types::H256;
use std::ops::Sub;
use tendermint::block::Header;
use tendermint::Time;
use tendermint_light_client_verifier::host_functions::HostFunctionsProvider;

/// Similar to `with_client`, this function associates a client record to this context, but
/// additionally permits to parametrize two details of the client. If `client_type` is None,
/// then the client will have type Mock, otherwise the specified type. If
/// `consensus_state_height` is None, then the client will be initialized with a consensus
/// state matching the same height as the client state (`client_state_height`).
pub fn with_client_parametrized(
    ctx: MockContext<MockClientTypes>,
    client_id: &ClientId,
    client_state_height: Height,
    client_type: Option<ClientType>,
    consensus_state_height: Option<Height>,
) -> MockContext<MockClientTypes> {
    let cs_height = consensus_state_height.unwrap_or(client_state_height);

    let client_type = client_type.unwrap_or(MockClientState::client_type());
    let (client_state, consensus_state) = match client_type {
        // If it's a mock client, create the corresponding mock states.
        client_type if client_type == MockClientState::client_type() => (
            Some(MockClientState::new(MockHeader::new(client_state_height)).into()),
            MockConsensusState::new(MockHeader::new(cs_height)).into(),
        ),
        client_type if client_type == BeefyClientState::<()>::client_type() => (
            Some(get_dummy_beefy_state()),
            get_dummy_beefy_consensus_state(),
        ),
        // If it's a Tendermint client, we need TM states.
        client_type if client_type == TendermintClientState::<()>::client_type() => {
            let light_block = MockHostBlock::generate_tm_block(
                ctx.host_chain_id.clone(),
                cs_height.revision_height,
                Timestamp::now(),
            );

            let consensus_state = AnyConsensusState::from(light_block.clone());
            let client_state = get_dummy_tendermint_client_state(light_block.signed_header.header);

            // Return the tuple.
            (Some(client_state), consensus_state)
        }
        _ => unimplemented!(),
    };
    let consensus_states = vec![(cs_height, consensus_state)].into_iter().collect();

    debug!("consensus states: {:?}", consensus_states);

    let client_record = MockClientRecord {
        client_type,
        client_state,
        consensus_states,
    };
    ctx.ibc_store
        .lock()
        .unwrap()
        .clients
        .insert(client_id.clone(), client_record);
    ctx
}

pub fn with_client_parametrized_history(
    ctx: MockContext<MockClientTypes>,
    client_id: &ClientId,
    client_state_height: Height,
    client_type: Option<ClientType>,
    consensus_state_height: Option<Height>,
) -> MockContext<MockClientTypes> {
    let cs_height = consensus_state_height.unwrap_or(client_state_height);
    let prev_cs_height = cs_height.clone().sub(1).unwrap_or(client_state_height);

    let client_type = client_type.unwrap_or(MockClientState::client_type());
    let now = Timestamp::now();

    let (client_state, consensus_state) = match client_type {
        // If it's a mock client, create the corresponding mock states.
        client_type if client_type == MockClientState::client_type() => (
            Some(MockClientState::new(MockHeader::new(client_state_height)).into()),
            MockConsensusState::new(MockHeader::new(cs_height)).into(),
        ),

        client_type if client_type == BeefyClientState::<()>::client_type() => (
            Some(get_dummy_beefy_state()),
            get_dummy_beefy_consensus_state(),
        ),
        // If it's a Tendermint client, we need TM states.
        client_type if client_type == TendermintClientState::<()>::client_type() => {
            let light_block = MockHostBlock::generate_tm_block(
                ctx.host_chain_id.clone(),
                cs_height.revision_height,
                now,
            );

            let consensus_state = AnyConsensusState::from(light_block.clone());
            let client_state = get_dummy_tendermint_client_state(light_block.signed_header.header);

            // Return the tuple.
            (Some(client_state), consensus_state)
        }
        _ => unimplemented!(),
    };

    let prev_consensus_state = match client_type {
        // If it's a mock client, create the corresponding mock states.
        client_type if client_type == MockClientState::client_type() => {
            MockConsensusState::new(MockHeader::new(prev_cs_height)).into()
        }
        client_type if client_type == TendermintClientState::<()>::client_type() => {
            let light_block = MockHostBlock::generate_tm_block(
                ctx.host_chain_id.clone(),
                prev_cs_height.revision_height,
                now.sub(ctx.block_time).unwrap(),
            );
            AnyConsensusState::from(light_block)
        }
        _ => unimplemented!(),
    };

    let consensus_states = vec![
        (prev_cs_height, prev_consensus_state),
        (cs_height, consensus_state),
    ]
    .into_iter()
    .collect();

    debug!("consensus states: {:?}", consensus_states);

    let client_record = MockClientRecord {
        client_type,
        client_state,
        consensus_states,
    };

    ctx.ibc_store
        .lock()
        .unwrap()
        .clients
        .insert(client_id.clone(), client_record);
    ctx
}

pub fn get_dummy_beefy_state() -> AnyClientState<Crypto> {
    AnyClientState::Beefy(
        ClientState::new(
            RelayChain::Rococo,
            2000,
            0,
            Default::default(),
            0,
            0,
            Default::default(),
            Default::default(),
        )
        .unwrap(),
    )
}

pub fn get_dummy_beefy_consensus_state() -> AnyConsensusState {
    AnyConsensusState::Beefy(ConsensusState {
        timestamp: Time::now(),
        root: vec![0; 32].into(),
    })
}

pub fn get_dummy_tendermint_client_state(tm_header: Header) -> AnyClientState<Crypto> {
    AnyClientState::Tendermint(
        TMClientState::new(
            ChainId::from(tm_header.chain_id.clone()),
            Default::default(),
            Duration::from_secs(64000),
            Duration::from_secs(128000),
            Duration::from_millis(3000),
            Height::new(
                ChainId::chain_version(tm_header.chain_id.as_str()),
                u64::from(tm_header.height),
            ),
            Default::default(),
            vec!["".to_string()],
        )
        .unwrap(),
    )
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Crypto;

impl HostFunctions for Crypto {
    fn keccak_256(_input: &[u8]) -> [u8; 32] {
        unimplemented!()
    }

    fn secp256k1_ecdsa_recover_compressed(
        _signature: &[u8; 65],
        _value: &[u8; 32],
    ) -> Option<Vec<u8>> {
        unimplemented!()
    }

    fn verify_timestamp_extrinsic(
        _root: H256,
        _proof: &[Vec<u8>],
        _value: &[u8],
    ) -> Result<(), BeefyClientError> {
        unimplemented!()
    }
}

impl HostFunctionsProvider for Crypto {
    fn sha2_256(preimage: &[u8]) -> [u8; 32] {
        sp_io::hashing::sha2_256(preimage)
    }

    fn ed25519_verify(_sig: &[u8], _msg: &[u8], _pub_key: &[u8]) -> bool {
        true // TODO: ed25519_verify
    }

    fn secp256k1_verify(_sig: &[u8], _message: &[u8], _public: &[u8]) -> bool {
        unimplemented!()
    }
}

impl ics23::HostFunctionsProvider for Crypto {
    fn sha2_256(_message: &[u8]) -> [u8; 32] {
        unimplemented!()
    }

    fn sha2_512(_message: &[u8]) -> [u8; 64] {
        unimplemented!()
    }

    fn sha2_512_truncated(_message: &[u8]) -> [u8; 32] {
        unimplemented!()
    }

    fn sha3_512(_message: &[u8]) -> [u8; 64] {
        unimplemented!()
    }

    fn ripemd160(_message: &[u8]) -> [u8; 20] {
        unimplemented!()
    }
}

impl NearHostFunctions for Crypto {
    fn keccak_256(_input: &[u8]) -> [u8; 32] {
        unimplemented!()
    }

    fn secp256k1_ecdsa_recover_compressed(
        _signature: &[u8; 65],
        _value: &[u8; 32],
    ) -> Option<Vec<u8>> {
        unimplemented!()
    }

    fn ed25519_verify(_signature: &[u8; 64], _msg: &[u8], _pubkey: &[u8]) -> bool {
        unimplemented!()
    }

    fn verify_membership_trie_proof(
        _root: &[u8; 32],
        _proof: &[Vec<u8>],
        _key: &[u8],
        _value: &[u8],
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn verify_non_membership_trie_proof(
        _root: &[u8; 32],
        _proof: &[Vec<u8>],
        _key: &[u8],
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn verify_timestamp_extrinsic(
        _root: &[u8; 32],
        _proof: &[Vec<u8>],
        _value: &[u8],
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn sha256_digest(_data: &[u8]) -> [u8; 32] {
        unimplemented!()
    }

    fn sha2_256(_message: &[u8]) -> [u8; 32] {
        unimplemented!()
    }

    fn sha2_512(_message: &[u8]) -> [u8; 64] {
        unimplemented!()
    }

    fn sha2_512_truncated(_message: &[u8]) -> [u8; 32] {
        unimplemented!()
    }

    fn sha3_512(_message: &[u8]) -> [u8; 64] {
        unimplemented!()
    }

    fn ripemd160(_message: &[u8]) -> [u8; 20] {
        unimplemented!()
    }
}

impl NearHostFunctionsTrait for Crypto {}

impl BeefyHostFunctions for Crypto {
    fn verify_membership_trie_proof(
        _root: &[u8; 32],
        _proof: &[Vec<u8>],
        _key: &[u8],
        _value: &[u8],
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn verify_non_membership_trie_proof(
        _root: &[u8; 32],
        _proof: &[Vec<u8>],
        _key: &[u8],
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn verify_timestamp_extrinsic(
        _root: &[u8; 32],
        _proof: &[Vec<u8>],
        _value: &[u8],
    ) -> Result<(), Error> {
        unimplemented!()
    }
}

impl AnyHostFunctionsTrait for Crypto {}
