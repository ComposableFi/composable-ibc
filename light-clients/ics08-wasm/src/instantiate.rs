use crate::Bytes;
#[cfg(feature = "cosmwasm")]
use cosmwasm_schema::cw_serde;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Clone, Debug, PartialEq))]
#[derive(Eq)]
pub struct InstantiateMessage {
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub client_state: Bytes,
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub consensus_state: Bytes,
	#[schemars(with = "String")]
	#[serde(with = "Base64", default)]
	pub checksum: Bytes
}
