use crate::any::client_def::AnyClient;
use crate::ics07_tendermint::client_state;
#[cfg(any(test, feature = "ics11_beefy"))]
use crate::ics11_beefy::client_state as beefy_client_state;
#[cfg(any(test, feature = "ics11_beefy"))]
use crate::ics13_near::client_state as near_client_state;
use crate::AnyHostFunctionsTrait;
use core::fmt::{Debug, Display};
use core::time::Duration;
use ibc::core::ics02_client::client_state::ClientState;
use ibc::core::ics02_client::client_state::ClientType;
use ibc::core::ics02_client::error::Error;
use ibc::core::ics24_host::error::ValidationError;
use ibc::core::ics24_host::identifier::{ChainId, ClientId};
#[cfg(any(test, feature = "mocks"))]
use ibc::mock::client_state::MockClientState;
use ibc::prelude::*;
use ibc::{downcast, Height};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::IdentifiedClientState;
use serde::{Deserialize, Serialize};
use tendermint_proto::Protobuf;

pub const TENDERMINT_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.tendermint.v1.ClientState";
pub const BEEFY_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.beefy.v1.ClientState";
pub const NEAR_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.near.v1.ClientState";
pub const MOCK_CLIENT_STATE_TYPE_URL: &str = "/ibc.mock.ClientState";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnyUpgradeOptions {
    Tendermint(client_state::UpgradeOptions),
    #[cfg(any(test, feature = "ics11_beefy"))]
    Beefy(beefy_client_state::UpgradeOptions),
    #[cfg(any(test, feature = "ics11_beefy"))]
    Near(near_client_state::NearUpgradeOptions),
    #[cfg(any(test, feature = "mocks"))]
    Mock(()),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, ClientState, Protobuf)]
#[serde(tag = "type")]
pub enum AnyClientState<H>
where
    H: AnyHostFunctionsTrait,
{
    #[ibc(proto_url = "TENDERMINT_CLIENT_STATE_TYPE_URL")]
    Tendermint(client_state::ClientState<H>),
    #[cfg(any(test, feature = "ics11_beefy"))]
    #[serde(skip)]
    #[ibc(proto_url = "BEEFY_CLIENT_STATE_TYPE_URL")]
    Beefy(beefy_client_state::ClientState<H>),
    #[cfg(any(test, feature = "ics11_beefy"))]
    #[serde(skip)]
    #[ibc(proto_url = "NEAR_CLIENT_STATE_TYPE_URL")]
    Near(near_client_state::NearClientState<H>),
    #[cfg(any(test, feature = "mocks"))]
    #[ibc(proto_url = "MOCK_CLIENT_STATE_TYPE_URL")]
    Mock(MockClientState),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct IdentifiedAnyClientState<H: AnyHostFunctionsTrait> {
    pub client_id: ClientId,
    pub client_state: AnyClientState<H>,
}

impl<H: AnyHostFunctionsTrait> IdentifiedAnyClientState<H> {
    pub fn new(client_id: ClientId, client_state: AnyClientState<H>) -> Self {
        IdentifiedAnyClientState {
            client_id,
            client_state,
        }
    }
}

impl<H: AnyHostFunctionsTrait> Protobuf<IdentifiedClientState> for IdentifiedAnyClientState<H>
where
    Self: TryFrom<IdentifiedClientState>,
    <Self as TryFrom<IdentifiedClientState>>::Error: Display,
{
}

impl<H: AnyHostFunctionsTrait> TryFrom<IdentifiedClientState> for IdentifiedAnyClientState<H>
where
    AnyClientState<H>: TryFrom<Any>,
    Error: From<<AnyClientState<H> as TryFrom<Any>>::Error>,
{
    type Error = Error;

    fn try_from(raw: IdentifiedClientState) -> Result<Self, Self::Error> {
        Ok(IdentifiedAnyClientState {
            client_id: raw.client_id.parse().map_err(|e: ValidationError| {
                Error::invalid_raw_client_id(raw.client_id.clone(), e)
            })?,
            client_state: raw
                .client_state
                .ok_or_else(Error::missing_raw_client_state)?
                .try_into()?,
        })
    }
}

impl<H: AnyHostFunctionsTrait> From<IdentifiedAnyClientState<H>> for IdentifiedClientState {
    fn from(value: IdentifiedAnyClientState<H>) -> Self {
        IdentifiedClientState {
            client_id: value.client_id.to_string(),
            client_state: Some(value.client_state.into()),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::any::client_state::AnyClientState;
    use crate::any::mock::context::get_dummy_tendermint_client_state;
    use crate::ics07_tendermint::header::test_util::get_dummy_tendermint_header;
    use ibc_proto::google::protobuf::Any;
    use test_log::test;

    #[test]
    fn any_client_state_serialization() {
        let tm_client_state = get_dummy_tendermint_client_state(get_dummy_tendermint_header());

        let raw: Any = tm_client_state.clone().into();
        let tm_client_state_back = AnyClientState::try_from(raw).unwrap();
        assert_eq!(tm_client_state, tm_client_state_back);
    }
}
