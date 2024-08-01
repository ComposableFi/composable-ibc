#[cfg(feature = "cosmwasm")]
use cosmwasm_schema::cw_serde;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Clone, Debug, PartialEq, serde::Deserialize))]
#[derive(Eq)]
pub struct MigrateMsg {}