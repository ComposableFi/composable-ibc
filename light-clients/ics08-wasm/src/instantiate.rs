use crate::Bytes;
#[cfg(feature = "cosmwasm")]
use cosmwasm_schema::cw_serde;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Clone, Debug, PartialEq))]
#[derive(Eq)]
pub struct InstantiateMessage {
	pub client_state: Bytes,
	pub consensus_state: Bytes,
	pub checksum: Bytes
}
