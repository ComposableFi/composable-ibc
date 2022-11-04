use std::{marker::PhantomData, str::FromStr};

use cosmos::{CosmosClient, CosmosClientConfig};
use futures::StreamExt;
use hyperspace::logging;
use hyperspace_primitives::{mock::LocalClientTypes, IbcProvider, KeyProvider};
use hyperspace_testsuite::{
	ibc_channel_close, ibc_messaging_packet_height_timeout_with_connection_delay,
	ibc_messaging_packet_timeout_on_channel_close,
	ibc_messaging_packet_timestamp_timeout_with_connection_delay,
	ibc_messaging_with_connection_delay,
};
use ibc::{core::ics02_client::msgs::create_client::MsgCreateAnyClient, tx_msg::Msg};
use subxt::tx::SubstrateExtrinsicParams;

use tendermint_proto::Protobuf;
use tendermint_rpc::{endpoint::broadcast::tx_sync::Response, Client, HttpClient, Url};

async fn setup_clients<H: Clone + Send + Sync + 'static>() -> (CosmosClient<H>, CosmosClient<H>) {
	log::info!(target: "hyperspace", "=========================== Starting Test ===========================");
	todo!()
}

#[tokio::test]
async fn cosmos_to_cosmos_ibc_messaging_full_integration_test() {
	logging::setup_logging();
	let (mut chain_a, mut chain_b) = setup_clients::<u32>().await;

	// no timeouts + connection delay
	ibc_messaging_with_connection_delay(&mut chain_a, &mut chain_b).await;

	// timeouts + connection delay
	ibc_messaging_packet_height_timeout_with_connection_delay(&mut chain_a, &mut chain_b).await;
	ibc_messaging_packet_timestamp_timeout_with_connection_delay(&mut chain_a, &mut
	chain_b).await;

	// channel closing semantics
	ibc_messaging_packet_timeout_on_channel_close(&mut chain_a, &mut chain_b).await;
	ibc_channel_close(&mut chain_a, &mut chain_b).await;
}
