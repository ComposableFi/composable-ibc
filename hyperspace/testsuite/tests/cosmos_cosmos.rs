use std::{marker::PhantomData, str::FromStr};

use futures::StreamExt;
use hyperspace_core::logging;
use hyperspace_cosmos::{CosmosClient, CosmosClientConfig};
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

	// Create client configurations
	// Parameters have been set up to work with local nodes according to https://hermes.informal.systems/tutorials
	let config_a = CosmosClientConfig {
		name: "chain_a".to_string(),
		chain_id: "ibc-0".to_string(),
		rpc_url: Url::from_str("http://127.0.0.1:27010").unwrap(),
		grpc_url: Url::from_str("http://127.0.0.1:27012").unwrap(),
		websocket_url: Url::from_str("ws://127.0.0.1:27010/websocket").unwrap(),
		client_id: Some("7-tendermint".to_string()),
		connection_id: None,
		account_prefix: "cosmos".to_string(),
		store_prefix: "ibc".to_string(),
		key_name: "testkey".to_string(),
	};

	let config_b = CosmosClientConfig {
		name: "chain_b".to_string(),
		chain_id: "ibc-1".to_string(),
		rpc_url: Url::from_str("http://127.0.0.1:27020").unwrap(),
		grpc_url: Url::from_str("http://127.0.0.1:27022").unwrap(),
		websocket_url: Url::from_str("ws://127.0.0.1:27020/websocket").unwrap(),
		client_id: Some("7-tendermint".to_string()),
		connection_id: None,
		account_prefix: "cosmos".to_string(),
		store_prefix: "ibc".to_string(),
		key_name: "testkey".to_string(),
	};

	let mut chain_a = CosmosClient::<H>::new(config_a).await.unwrap();
	let mut chain_b = CosmosClient::<H>::new(config_b).await.unwrap();

	// Wait until for cosmos chains to start producing blocks
	log::info!(target: "hyperspace", "Waiting for block production from cosmos chains");
	chain_a.rpc_client.health().await.unwrap();
	chain_a.rpc_client.status().await.unwrap();
	chain_b.rpc_client.health().await.unwrap();
	chain_b.rpc_client.status().await.unwrap();
	log::info!(target: "hyperspace", "Cosmos chains are ready");

	// Check if the clients are already created
	let clients_on_a = chain_a.query_clients().await.unwrap();
	log::info!(target: "hyperspace", "Clients on chain_a: {:?}", clients_on_a);
	let clients_on_b = chain_b.query_clients().await.unwrap();
	log::info!(target: "hyperspace", "Clients on chain_b: {:?}", clients_on_b);

	if !clients_on_a.is_empty() && !clients_on_b.is_empty() {
		chain_a.set_client_id(clients_on_b[0].clone());
		chain_b.set_client_id(clients_on_b[0].clone());
		return (chain_a, chain_b);
	}
	(chain_a, chain_b)
}

#[tokio::test]
async fn cosmos_to_cosmos_ibc_messaging_full_integration_test() {
	logging::setup_logging();
	let (mut chain_a, mut chain_b) = setup_clients::<u32>().await;
	// Run tests sequentially

	// no timeouts + connection delay
	// ibc_messaging_with_connection_delay(&mut chain_a, &mut chain_b).await;

	// // timeouts + connection delay
	// ibc_messaging_packet_height_timeout_with_connection_delay(&mut chain_a, &mut chain_b).await;
	// ibc_messaging_packet_timestamp_timeout_with_connection_delay(&mut chain_a, &mut
	// chain_b).await;

	// // channel closing semantics
	// ibc_messaging_packet_timeout_on_channel_close(&mut chain_a, &mut chain_b).await;
	// ibc_channel_close(&mut chain_a, &mut chain_b).await;
}
