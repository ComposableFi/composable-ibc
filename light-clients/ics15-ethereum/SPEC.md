## ICS15-Ethereum Light Client Specification


```rust
// the beacon block header
// https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#beaconblockheader
pub struct BeaconBlockHeader {
    slot: Slot,
    proposer_index: ValidatorIndex,
    parent_root: H256,
    state_root: H256,
    body_root: H256,
}

// The sync aggregate; 
// https://github.com/ethereum/consensus-specs/blob/dev/specs/altair/beacon-chain.md#syncaggregate
pub struct SyncAggregate {
    sync_committee_bits: [SYNC_COMMITTEE_SIZE; u8],
    sync_committee_signature: BLSSignature,
}

// The sync committee responsible for signing blocks
// https://github.com/ethereum/consensus-specs/blob/dev/specs/altair/beacon-chain.md#synccommittee
pub struct SyncCommittee {
    pubkeys: [BLSPubkey; SYNC_COMMITTEE_SIZE],
    aggregate_pubkey: BLSPubkey,
}

// the beacon state
// https://github.com/ethereum/consensus-specs/blob/dev/specs/altair/beacon-chain.md#synccommittee
pub struct BeaconState {
    genesis_time: u64,
    genesis_validators_root: H256,
    slot: Slot,
    fork: Fork,
    latest_block_header: BeaconBlockHeader,
    block_roots: [H256; SLOTS_PER_HISTORICAL_ROOT],
    state_roots: [H256; SLOTS_PER_HISTORICAL_ROOT],
    historical_roots: [H256; HISTORICAL_ROOTS_LIMIT],
    eth1_data: Eth1Data,
    eth1_data_votes: [Eth1Data; EPOCHS_PER_ETH1_VOTING_PERIOD * SLOTS_PER_EPOCH],
    eth1_deposit_index: u64,
    validators: [Validator; VALIDATOR_REGISTRY_LIMIT],
    balances: [Gwei; VALIDATOR_REGISTRY_LIMIT],
    randao_mixes: [H256; EPOCHS_PER_HISTORICAL_VECTOR],
    slashings: [Gwei; EPOCHS_PER_SLASHINGS_VECTOR],
    previous_epoch_participation: [ParticipationFlags; VALIDATOR_REGISTRY_LIMIT],
    current_epoch_participation: [ParticipationFlags; VALIDATOR_REGISTRY_LIMIT],
    justification_bits: Bitvector[JUSTIFICATION_BITS_LENGTH],
    previous_justified_checkpoint: Checkpoint,
    current_justified_checkpoint: Checkpoint,
    finalized_checkpoint: Checkpoint,
    inactivity_scores: [u64; VALIDATOR_REGISTRY_LIMIT],
    current_sync_committee: SyncCommittee,
    next_sync_committee: SyncCommittee,
}

pub struct LightClientUpdate {
    attested_header: BeaconBlockHeader,
    next_sync_committee: SyncCommittee,
    next_sync_committee_branch: [H256; floorlog2(NEXT_SYNC_COMMITTEE_INDEX)],
    finalized_header: BeaconBlockHeader,
    finality_branch: [H256; floorlog2(FINALIZED_ROOT_INDEX)],
    sync_aggregate: SyncAggregate,
    signature_slot: Slot,
}

// algorithm for verifying the sync committee's commitment.
// https://github.com/ethereum/consensus-specs/blob/dev/specs/altair/beacon-chain.md#sync-aggregate-processing
pub fn verify_sync_aggregate_commitment(state: BeaconState, sync_aggregate: SyncAggregate) -> Result<(), BlsVerifyError> {
    let committee_pubkeys = state.current_sync_committee.pubkeys;
    let participant_pubkeys = committee_pubkeys
        .into_iter()
        .zip(sync_aggregate.sync_committee_bits)
        .filter(|(pubkey, bit)| {
            if bit {
                Some(pubkey)
            } else {
                None
            }
        })
        .collect::<Vec<BLSPubkey>>();
    
    let previous_slot = std::cmp::max(state.slot, Slot(1)) - Slot(1)
    let domain = get_domain(state, DOMAIN_SYNC_COMMITTEE, compute_epoch_at_slot(previous_slot))
    let signing_root = compute_signing_root(get_block_root_at_slot(state, previous_slot), domain)
    
    bls_fast_aggregate_verify(participant_pubkeys, signing_root, sync_aggregate.sync_committee_signature)?;

    Ok(())
}

pub fn process_light_client_finality_update(
    store: LightClientStore,
    finality_update: LightClientFinalityUpdate,
    current_slot: Slot,
    genesis_validators_root: Root,
) -> Result<(), BlsError> {
    let update = LightClientUpdate {
        attested_header: finality_update.attested_header,
        next_sync_committee: SyncCommittee(),
        next_sync_committee_branch: [Bytes32() for _ in range(floorlog2(NEXT_SYNC_COMMITTEE_INDEX))],
        finalized_header: finality_update.finalized_header,
        finality_branch: finality_update.finality_branch,
        sync_aggregate: finality_update.sync_aggregate,
        signature_slot: finality_update.signature_slot,
    };
    process_light_client_update(store, update, current_slot, genesis_validators_root)
}

const MIN_SYNC_COMMITTEE_PARTICIPANTS: u32 = 1;

pub fn process_light_client_update(
    store: LightClientStore,
    update: LightClientUpdate,
    current_slot: Slot,
    genesis_validators_root: Root
) -> Result<(), BlsError> {
    validate_light_client_update(store, update, current_slot, genesis_validators_root)

    // Verify sync committee has sufficient participants
    let sync_aggregate_participants = update.sync_aggregate.sync_committee_bits.iter().sum();
    if sync_aggregate_participants < MIN_SYNC_COMMITTEE_PARTICIPANTS {
        Err(UpdateError::SyncCommitteeParticiapntsTooLow)?
    }

    // Verify update does not skip a sync committee period
    let is_valid_update =
        current_slot >= update.signature_slot > update.attested_header.slot >= update.finalized_header.slot;

    if !is_valid_update {
        Err(UpdateError::InvalidUpdate)?
    }

    let store_period = compute_sync_committee_period_at_slot(store.finalized_header.slot)
    let update_signature_period = compute_sync_committee_period_at_slot(update.signature_slot)
    if is_next_sync_committee_known(store) {
        assert update_signature_period in (store_period, store_period + 1)
    else:
        assert update_signature_period == store_period

    // Verify update is relevant
    let update_attested_period = compute_sync_committee_period_at_slot(update.attested_header.slot)
    update_has_next_sync_committee = not is_next_sync_committee_known(store) and (
        is_sync_committee_update(update) and update_attested_period == store_period
    )
    assert (
        update.attested_header.slot > store.finalized_header.slot
        or update_has_next_sync_committee
    )


    let sync_committee_bits = update.sync_aggregate.sync_committee_bits;


    # Update finalized header
    update_has_finalized_next_sync_committee = (
        not is_next_sync_committee_known(store)
        and is_sync_committee_update(update) and is_finality_update(update) and (
            compute_sync_committee_period_at_slot(update.finalized_header.slot)
            == compute_sync_committee_period_at_slot(update.attested_header.slot)
        )
    )
    if (
        sum(sync_committee_bits) * 3 >= len(sync_committee_bits) * 2
        and (
            update.finalized_header.slot > store.finalized_header.slot
            or update_has_finalized_next_sync_committee
        )
    ):
        # Normal update through 2/3 threshold
        apply_light_client_update(store, update)
        store.best_valid_update = None
}



```