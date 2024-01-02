use crate::{client_state::ClientState, consensus_state::ConsensusState, error::Error};
use alloc::vec::Vec;
use alloy_sol_macro::sol;
use alloy_sol_types::private::FixedBytes;
use ethabi::Bytes;
use ibc::{
	core::{ics02_client::height::Height, ics23_commitment::commitment::CommitmentRoot},
	timestamp::Timestamp,
};
use ssz_rs::{Node, Vector};
use sync_committee_primitives::{
	consensus_types::{BeaconBlockHeader, SyncCommittee},
	constants::{BlsPublicKey, SYNC_COMMITTEE_SIZE},
	types::VerifierState as LightClientState,
};
use EthereumClientAbi::*;

sol!(EthereumClientAbi, "../../hyperspace/ethereum/src/abi/ethereum-client-abi.json");

impl HeightData {
	fn is_zero(&self) -> bool {
		self.revision_number == 0 && self.revision_height == 0
	}
}

impl From<HeightData> for Height {
	fn from(value: HeightData) -> Self {
		Self {
			revision_number: value.revision_number.into(),
			revision_height: value.revision_height.into(),
		}
	}
}

impl From<Height> for HeightData {
	fn from(value: Height) -> Self {
		Self {
			revision_number: value.revision_number.into(),
			revision_height: value.revision_height.into(),
		}
	}
}

impl From<SyncCommittee<SYNC_COMMITTEE_SIZE>> for EthereumClientPrimitivesSyncCommittee {
	fn from(value: SyncCommittee<SYNC_COMMITTEE_SIZE>) -> Self {
		Self {
			publicKeys: value.public_keys.iter().map(|pk| Bytes::from(pk.as_slice())).collect(),
			aggregatePublicKey: Bytes::from(value.aggregate_public_key.as_slice()),
		}
	}
}

impl TryFrom<EthereumClientPrimitivesSyncCommittee> for SyncCommittee<SYNC_COMMITTEE_SIZE> {
	type Error = Error;

	fn try_from(value: EthereumClientPrimitivesSyncCommittee) -> Result<Self, Self::Error> {
		Ok(Self {
			public_keys: Vector::try_from(
				value
					.publicKeys
					.iter()
					.map(|pk| BlsPublicKey::try_from(pk.as_slice()))
					.collect::<Result<Vec<_>, _>>()?,
			)
			.map_err(|e| e.1)?,
			aggregate_public_key: BlsPublicKey::try_from(value.aggregatePublicKey.as_slice())?,
		})
	}
}

impl From<LightClientState> for EthereumClientPrimitivesLightClientState {
	fn from(value: LightClientState) -> Self {
		Self {
			finalizedHeader: value.finalized_header.into(),
			latestFinalizedEpoch: value.latest_finalized_epoch,
			currentSyncCommittee: value.current_sync_committee.into(),
			nextSyncCommittee: value.next_sync_committee.into(),
			statePeriod: value.state_period,
		}
	}
}

impl TryFrom<EthereumClientPrimitivesLightClientState> for LightClientState {
	type Error = Error;

	fn try_from(value: EthereumClientPrimitivesLightClientState) -> Result<Self, Self::Error> {
		Ok(Self {
			finalized_header: value.finalizedHeader.into(),
			latest_finalized_epoch: value.latestFinalizedEpoch,
			current_sync_committee: value.currentSyncCommittee.try_into()?,
			next_sync_committee: value.nextSyncCommittee.try_into()?,
			state_period: value.statePeriod,
		})
	}
}

impl From<BeaconBlockHeader> for EthereumClientPrimitivesBeaconBlockHeader {
	fn from(value: BeaconBlockHeader) -> Self {
		Self {
			slot: value.slot,
			proposerIndex: value.proposer_index,
			parentRoot: FixedBytes(value.parent_root.0),
			stateRoot: FixedBytes(value.state_root.0),
			bodyRoot: FixedBytes(value.body_root.0),
		}
	}
}

impl From<EthereumClientPrimitivesBeaconBlockHeader> for BeaconBlockHeader {
	fn from(value: EthereumClientPrimitivesBeaconBlockHeader) -> Self {
		Self {
			slot: value.slot,
			proposer_index: value.proposerIndex,
			parent_root: Node(value.parentRoot.0),
			state_root: Node(value.stateRoot.0),
			body_root: Node(value.bodyRoot.0),
		}
	}
}

impl<H> From<ClientState<H>> for EthereumClientPrimitivesClientState {
	fn from(value: ClientState<H>) -> Self {
		Self {
			inner: value.inner.into(),
			frozenHeight: value
				.frozen_height
				.map(Into::into)
				.unwrap_or_else(|| HeightData { revision_number: 0, revision_height: 0 }),
			latestHeight: value.latest_height,
		}
	}
}

impl<H> TryFrom<EthereumClientPrimitivesClientState> for ClientState<H> {
	type Error = Error;

	fn try_from(value: EthereumClientPrimitivesClientState) -> Result<Self, Self::Error> {
		Ok(Self {
			inner: value.inner.try_into()?,
			frozen_height: if value.frozenHeight.is_zero() {
				None
			} else {
				Some(value.frozenHeight.into())
			},
			latest_height: value.latestHeight,
			_phantom: Default::default(),
		})
	}
}

impl From<ConsensusState> for EthereumClientPrimitivesConsensusState {
	fn from(value: ConsensusState) -> Self {
		Self {
			timestamp: Timestamp::from(value.timestamp).nanoseconds(),
			root: FixedBytes(value.root.bytes.try_into().expect("invalid commitment root")),
		}
	}
}

impl From<EthereumClientPrimitivesConsensusState> for ConsensusState {
	fn from(value: EthereumClientPrimitivesConsensusState) -> Self {
		Self {
			timestamp: Timestamp::from_nanoseconds(value.timestamp)
				.expect("invalid timestamp")
				.into_tm_time()
				.expect("invalid timestamp"),
			root: CommitmentRoot::from_bytes(&value.root.0),
		}
	}
}
