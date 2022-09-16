use crate::any::consensus_state::AnyConsensusState;
use crate::any::header::AnyHeader;
use crate::any::mock::context::{with_client_parametrized, with_client_parametrized_history};
use crate::any::mock::MockClientTypes;
use crate::ics07_tendermint::client_state::ClientState as TendermintClientState;
use crate::ics07_tendermint::mock::host::MockHostType;
use ibc::core::ics02_client::client_state::ClientState;

use ibc::core::ics02_client::context::ClientReader;
use ibc::core::ics02_client::handler::dispatch;
use ibc::core::ics02_client::handler::ClientResult::Update;
use ibc::core::ics02_client::header::Header;
use ibc::core::ics02_client::msgs::update_client::MsgUpdateAnyClient;
use ibc::core::ics02_client::msgs::ClientMsg;
use ibc::core::ics24_host::identifier::{ChainId, ClientId};
use ibc::events::IbcEvent;
use ibc::handler::HandlerOutput;
use ibc::mock::context::MockContext;
use ibc::test_utils::get_dummy_account_id;
use ibc::Height;

#[test]
fn test_update_synthetic_tendermint_client_adjacent_ok() {
    let client_id = ClientId::new(TendermintClientState::<()>::client_type(), 0).unwrap();
    let client_height = Height::new(1, 20);
    let update_height = Height::new(1, 21);

    let ctx = MockContext::<MockClientTypes>::new(
        ChainId::new("mockgaiaA".to_string(), 1),
        MockHostType::Mock,
        5,
        Height::new(1, 1),
    );
    let ctx = with_client_parametrized(
        ctx,
        &client_id,
        client_height,
        Some(TendermintClientState::<()>::client_type()), // The target host chain (B) is synthetic TM.
        Some(client_height),
    );

    let ctx_b = MockContext::<MockClientTypes>::new(
        ChainId::new("mockgaiaB".to_string(), 1),
        MockHostType::SyntheticTendermint,
        5,
        update_height,
    );

    let signer = get_dummy_account_id();

    let block_ref = ctx_b.host_block(update_height);
    let mut latest_header: AnyHeader = block_ref.cloned().map(Into::into).unwrap();

    latest_header = match latest_header {
        AnyHeader::Tendermint(mut theader) => {
            theader.trusted_height = client_height;
            AnyHeader::Tendermint(theader)
        }
        AnyHeader::Beefy(h) => AnyHeader::Beefy(h),
        AnyHeader::Mock(m) => AnyHeader::Mock(m),
        AnyHeader::Near(h) => AnyHeader::Near(h),
    };

    let msg = MsgUpdateAnyClient {
        client_id: client_id.clone(),
        header: latest_header,
        signer,
    };

    let output = dispatch(&ctx, ClientMsg::UpdateClient(msg.clone()));

    match output {
        Ok(HandlerOutput {
            result,
            mut events,
            log,
        }) => {
            assert_eq!(events.len(), 1);
            let event = events.pop().unwrap();
            assert!(
                matches!(event, IbcEvent::UpdateClient(ref e) if e.client_id() == &msg.client_id)
            );
            assert_eq!(event.height(), ctx.host_height());
            assert!(log.is_empty());
            // Check the result
            match result {
                Update(upd_res) => {
                    assert_eq!(upd_res.client_id, client_id);
                    assert!(!upd_res.client_state.is_frozen());
                    assert_eq!(upd_res.client_state.latest_height(), msg.header.height(),)
                }
                _ => panic!("update handler result has incorrect type"),
            }
        }
        Err(err) => {
            panic!("unexpected error: {}", err);
        }
    }
}

#[test]
fn test_update_synthetic_tendermint_client_non_adjacent_ok() {
    let client_id = ClientId::new(TendermintClientState::<()>::client_type(), 0).unwrap();
    let client_height = Height::new(1, 20);
    let update_height = Height::new(1, 21);

    let ctx = MockContext::new(
        ChainId::new("mockgaiaA".to_string(), 1),
        MockHostType::Mock,
        5,
        Height::new(1, 1),
    );
    let ctx = with_client_parametrized_history(
        ctx,
        &client_id,
        client_height,
        Some(TendermintClientState::<()>::client_type()), // The target host chain (B) is synthetic TM.
        Some(client_height),
    );

    let ctx_b = MockContext::<MockClientTypes>::new(
        ChainId::new("mockgaiaB".to_string(), 1),
        MockHostType::SyntheticTendermint,
        5,
        update_height,
    );

    let signer = get_dummy_account_id();

    let block_ref = ctx_b.host_block(update_height);
    let mut latest_header: AnyHeader = block_ref.cloned().map(Into::into).unwrap();

    let trusted_height = client_height.clone().sub(1).unwrap_or_default();

    latest_header = match latest_header {
        AnyHeader::Tendermint(mut theader) => {
            theader.trusted_height = trusted_height;
            AnyHeader::Tendermint(theader)
        }
        AnyHeader::Beefy(h) => AnyHeader::Beefy(h),
        AnyHeader::Mock(m) => AnyHeader::Mock(m),
        AnyHeader::Near(h) => AnyHeader::Near(h),
    };

    let msg = MsgUpdateAnyClient {
        client_id: client_id.clone(),
        header: latest_header,
        signer,
    };

    let output = dispatch(&ctx, ClientMsg::UpdateClient(msg.clone()));

    match output {
        Ok(HandlerOutput {
            result,
            mut events,
            log,
        }) => {
            assert_eq!(events.len(), 1);
            let event = events.pop().unwrap();
            assert!(
                matches!(event, IbcEvent::UpdateClient(ref e) if e.client_id() == &msg.client_id)
            );
            assert_eq!(event.height(), ctx.host_height());
            assert!(log.is_empty());
            // Check the result
            match result {
                Update(upd_res) => {
                    assert_eq!(upd_res.client_id, client_id);
                    assert!(!upd_res.client_state.is_frozen());
                    assert_eq!(upd_res.client_state.latest_height(), msg.header.height(),)
                }
                _ => panic!("update handler result has incorrect type"),
            }
        }
        Err(err) => {
            panic!("unexpected error: {}", err);
        }
    }
}

#[test]
fn test_update_synthetic_tendermint_client_duplicate_ok() {
    let client_id = ClientId::new(TendermintClientState::<()>::client_type(), 0).unwrap();
    let client_height = Height::new(1, 20);

    let chain_start_height = Height::new(1, 11);

    let ctx = MockContext::<MockClientTypes>::new(
        ChainId::new("mockgaiaA".to_string(), 1),
        MockHostType::Mock,
        5,
        chain_start_height,
    );
    let ctx = with_client_parametrized(
        ctx,
        &client_id,
        client_height,
        Some(TendermintClientState::<()>::client_type()), // The target host chain (B) is synthetic TM.
        Some(client_height),
    );

    let ctx_b = MockContext::<MockClientTypes>::new(
        ChainId::new("mockgaiaB".to_string(), 1),
        MockHostType::SyntheticTendermint,
        5,
        client_height,
    );

    let signer = get_dummy_account_id();

    let block_ref = ctx_b.host_block(client_height);
    let latest_header: AnyHeader = match block_ref.cloned().map(Into::into).unwrap() {
        AnyHeader::Tendermint(mut theader) => {
            let cons_state = ctx.latest_consensus_states(&client_id, &client_height);
            if let AnyConsensusState::Tendermint(tcs) = cons_state {
                theader.signed_header.header.time = tcs.timestamp;
                theader.trusted_height = Height::new(1, 11)
            }
            AnyHeader::Tendermint(theader)
        }
        AnyHeader::Beefy(h) => AnyHeader::Beefy(h),
        AnyHeader::Mock(header) => AnyHeader::Mock(header),
        AnyHeader::Near(header) => AnyHeader::Near(header),
    };

    let msg = MsgUpdateAnyClient {
        client_id: client_id.clone(),
        header: latest_header,
        signer,
    };

    let output = dispatch(&ctx, ClientMsg::UpdateClient(msg.clone()));

    match output {
        Ok(HandlerOutput {
            result,
            mut events,
            log,
        }) => {
            assert_eq!(events.len(), 1);
            let event = events.pop().unwrap();
            assert!(
                matches!(event, IbcEvent::UpdateClient(ref e) if e.client_id() == &msg.client_id)
            );
            assert_eq!(event.height(), ctx.host_height());
            assert!(log.is_empty());
            // Check the result
            match result {
                Update(upd_res) => {
                    assert_eq!(upd_res.client_id, client_id);
                    assert!(!upd_res.client_state.is_frozen());
                    assert_eq!(upd_res.client_state, ctx.latest_client_states(&client_id));
                    assert_eq!(upd_res.client_state.latest_height(), msg.header.height())
                }
                _ => panic!("update handler result has incorrect type"),
            }
        }
        Err(err) => {
            panic!("unexpected error: {:?}", err);
        }
    }
}
