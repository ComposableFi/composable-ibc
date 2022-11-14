use hyperspace_core::logging;
use hyperspace_cosmos::{CosmosClient, CosmosClientConfig};
use hyperspace_testsuite::{
	ibc_channel_close, ibc_messaging_packet_height_timeout_with_connection_delay,
	ibc_messaging_packet_timeout_on_channel_close,
	ibc_messaging_packet_timestamp_timeout_with_connection_delay,
	ibc_messaging_with_connection_delay,
};
use std::str::FromStr;
use tendermint_rpc::Url;

async fn setup_clients<H: Clone + Send + Sync + 'static>() -> (CosmosClient<H>, CosmosClient<H>) {
	log::info!(target: "hyperspace", "=========================== Starting Test ===========================");

	// Create client configurations
	// Parameters have been set up to work with local nodes according to https://hermes.informal.systems/tutorials
	let config_a = CosmosClientConfig {
		name: "chain_a".to_string(),
		rpc_url: Url::from_str("http://127.0.0.1:27030").unwrap(),
		grpc_url: Url::from_str("http://127.0.0.1:27032").unwrap(),
		websocket_url: Url::from_str("ws://127.0.0.1:27030/websocket").unwrap(),
		chain_id: "ibc-0".to_string(),
		client_id: Some("7-tendermint".to_string()),
		connection_id: None,
		account_prefix: "cosmos".to_string(),
		store_prefix: "ibc".to_string(),
		key_name: "wallet".to_string(),
	};

	let config_b = CosmosClientConfig {
		name: "chain_b".to_string(),
		rpc_url: Url::from_str("http://127.0.0.1:27040").unwrap(),
		grpc_url: Url::from_str("http://127.0.0.1:27042").unwrap(),
		websocket_url: Url::from_str("ws://127.0.0.1:27040/websocket").unwrap(),
		chain_id: "ibc-1".to_string(),
		client_id: Some("7-tendermint".to_string()),
		connection_id: None,
		account_prefix: "cosmos".to_string(),
		store_prefix: "ibc".to_string(),
		key_name: "wallet".to_string(),
	};

	let mut chain_a = CosmosClient::<H>::new(config_a).await.unwrap();
	let mut chain_b = CosmosClient::<H>::new(config_b).await.unwrap();
	todo!()
}

#[tokio::test]
async fn cosmos_to_cosmos_ibc_messaging_full_integration_test() {
	// logging::setup_logging();
	// let (mut chain_a, mut chain_b) = setup_clients::<u32>().await;
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
