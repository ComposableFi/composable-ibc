//! ICS 07: Tendermint Client implements a client verification algorithm for blockchains which use
//! the Tendermint consensus algorithm.

pub mod client_def;
pub mod client_state;
pub mod consensus_state;
pub mod error;
pub mod header;
mod merkle;
pub mod misbehaviour;
#[cfg(any(test, feature = "mocks"))]
pub mod mock;
mod query;

#[cfg(test)]
mod tests {
    use crate::ics07_tendermint::client_state::test_util::get_dummy_tendermint_client_state;
    use crate::ics07_tendermint::client_state::{
        ClientState as TendermintClientState, ClientState,
    };

    use crate::ics07_tendermint::header::test_util::{
        get_dummy_ics07_header, get_dummy_tendermint_header,
    };
    use crate::ics07_tendermint::mock::{
        AnyClientState, AnyConsensusState, AnyHeader, MockClientTypes,
    };

    use ibc::core::ics02_client::context::ClientReader;
    use ibc::core::ics02_client::handler::{dispatch, ClientResult};
    use ibc::core::ics02_client::msgs::update_client::MsgUpdateAnyClient;
    use ibc::core::ics02_client::msgs::{create_client::MsgCreateAnyClient, ClientMsg};
    use ibc::core::ics02_client::trust_threshold::TrustThreshold;
    use ibc::core::ics23_commitment::specs::ProofSpecs;
    use ibc::core::ics24_host::identifier::ClientId;
    use ibc::events::IbcEvent;
    use ibc::handler::HandlerOutput;
    use ibc::mock::context::MockContext;
    use ibc::prelude::*;
    use ibc::test_utils::get_dummy_account_id;
    use ibc::Height;
    use ibc_proto::ibc::core::client::v1::{MsgCreateClient, MsgUpdateClient};
    use std::time::Duration;
    use test_log::test;

    #[test]
    fn msg_create_client_serialization() {
        let signer = get_dummy_account_id();

        let tm_header = get_dummy_tendermint_header();
        let tm_client_state = get_dummy_tendermint_client_state(tm_header.clone());

        let msg = MsgCreateAnyClient::<MockContext<MockClientTypes>>::new(
            tm_client_state,
            AnyConsensusState::Tendermint(tm_header.try_into().unwrap()),
            signer,
        )
        .unwrap();

        let raw = MsgCreateClient::from(msg.clone());
        let msg_back =
            MsgCreateAnyClient::<MockContext<MockClientTypes>>::try_from(raw.clone()).unwrap();
        let raw_back = MsgCreateClient::from(msg_back.clone());
        assert_eq!(msg, msg_back);
        assert_eq!(raw, raw_back);
    }

    #[test]
    fn test_tm_create_client_ok() {
        let signer = get_dummy_account_id();

        let ctx = MockContext::default();

        let tm_header = get_dummy_tendermint_header();

        let tm_client_state = AnyClientState::Tendermint(
            TendermintClientState::new(
                tm_header.chain_id.clone().into(),
                TrustThreshold::ONE_THIRD,
                Duration::from_secs(64000),
                Duration::from_secs(128000),
                Duration::from_millis(3000),
                Height::new(0, u64::from(tm_header.height)),
                ProofSpecs::default(),
                vec!["".to_string()],
            )
            .unwrap(),
        );

        let msg = MsgCreateAnyClient::<MockContext<MockClientTypes>>::new(
            tm_client_state,
            AnyConsensusState::Tendermint(tm_header.try_into().unwrap()),
            signer,
        )
        .unwrap();

        let output = dispatch(&ctx, ClientMsg::CreateClient(msg.clone()));

        match output {
            Ok(HandlerOutput {
                result, mut events, ..
            }) => {
                assert_eq!(events.len(), 1);
                let event = events.pop().unwrap();
                let expected_client_id =
                    ClientId::new(ClientState::<()>::client_type(), 0).unwrap();
                assert!(
                    matches!(event, IbcEvent::CreateClient(ref e) if e.client_id() == &expected_client_id)
                );
                assert_eq!(event.height(), ctx.host_height());
                match result {
                    ClientResult::Create(create_res) => {
                        assert_eq!(create_res.client_type, ClientState::<()>::client_type());
                        assert_eq!(create_res.client_id, expected_client_id);
                        assert_eq!(create_res.client_state, msg.client_state);
                        assert_eq!(create_res.consensus_state, msg.consensus_state);
                    }
                    _ => {
                        panic!("expected result of type ClientResult::CreateResult");
                    }
                }
            }
            Err(err) => {
                panic!("unexpected error: {}", err);
            }
        }
    }

    #[test]
    fn msg_update_client_serialization() {
        let client_id: ClientId = "tendermint".parse().unwrap();
        let signer = get_dummy_account_id();

        let header = get_dummy_ics07_header();

        let msg = MsgUpdateAnyClient::<MockContext<MockClientTypes>>::new(
            client_id,
            AnyHeader::Tendermint(header),
            signer,
        );
        let raw = MsgUpdateClient::from(msg.clone());
        let msg_back = MsgUpdateAnyClient::try_from(raw.clone()).unwrap();
        let raw_back = MsgUpdateClient::from(msg_back.clone());
        assert_eq!(msg, msg_back);
        assert_eq!(raw, raw_back);
    }
}
