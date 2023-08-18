use super::client_state::ClientState;
use crate::consensus_state::ConsensusState;
#[cfg(feature = "cosmwasm")]
use cosmwasm_schema::cw_serde;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Clone, Debug, PartialEq))]
#[derive(Eq)]
pub struct InstantiateMessage<
	AnyClient: Default,
	AnyClientState: Default,
	AnyConsensusState: Default,
> {
	pub client_state: ClientState<AnyClient, AnyClientState, AnyConsensusState>,
	pub consensus_state: ConsensusState<AnyConsensusState>,
}
