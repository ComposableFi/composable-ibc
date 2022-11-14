# Hyperspace Test Suite

This package provides utilities required for running integration tests with the relayer.

### Test Provider

[`TestProvider`](/hyperspace/primitives/src/lib.rs#318) should be implemented for each client before running the integration tests.

### Connection and Channel Setup

Every integration test attempts to complete the connection and channel handshake before progressing.  
If an open connection and channel exist, those are used instead.

### Available tests

There are a couple integration tests that can be used directly.  

The following tests are for unordered channels, although we use ICS20. They test the general unordered channel flow:  
- [`ibc_messaging_packet_height_timeout_with_connection_delay`](/hyperspace/testsuite/src/lib.rs#L444)  
  Spawns a test that checks if the packet timeout height rules are obeyed on both chains with connection delay enabled.  
- [`ibc_messaging_packet_timestamp_timeout_with_connection_delay`](/hyperspace/testsuite/src/lib.rs#L473)  
  Spawns a test that checks if the packet timeout timestamp rules are obeyed on both chains with connection delay  
  enabled.
- [`ibc_messaging_with_connection_delay`](/hyperspace/testsuite/src/lib.rs#L503)  
  This spawns a test that checks if the packets are delivered and acknowledged on both chains with connection delay enabled.  
- [`ibc_channel_close`](/hyperspace/testsuite/src/lib.rs#L530)  
  This spawns a test that checks if channel closing rules are obeyed on both chains.
- [`ibc_messaging_packet_timeout_on_channel_close`](/hyperspace/testsuite/src/lib.rs#L557)  
  This spawns a test that checks if packet timeout rules are obeyed when a channel is closed.

The following tests are for ordered channels:

- [`ibc_messaging_ordered_packet_with_connection_delay`](/hyperspace/testsuite/src/ordered_channels.rs#L213)  
  Spawns a test that checks for ordered delivery of packets on the receiving chain.
- [`send_a_packet_on_ordered_channel_and_assert_timeout`](/hyperspace/testsuite/src/ordered_channels.rs#L250)  
  Spawns a test that tests if the rules for packet timeout is obeyed on ordered channels on the connected chains.

### Using the test suite

Using the testsuite is straight forward and the following pseudocode describes the process:  

```rust

    // In actual implementation this function should return concrete types instead of trait objects
    async fn setup_clients() -> (Box<dyn Chain<Error = (), FinalityEvent = ()>>, Box<dyn Chain<Error = (), FinalityEvent = ()>>) {
        log::info!(target: "hyperspace", "=========================== Starting Test ===========================");
        // 1. Initialize chain parameters

        let (chain_a_config, chain_b_config) = initialize_chain_configs();

        // 2. Create chain handlers from config
        let mut chain_a = chain_a_config.to_client().await.unwrap();
        let mut chain_b = chain_b_config.to_client().await.unwrap();
    
        // 3. Check if clients already exist on both chains
        let clients_on_a = chain_a.query_clients().await.unwrap();
        let clients_on_b = chain_b.query_clients().await.unwrap();

        if !clients_on_a.is_empty() && !clients_on_b.is_empty() {
            // 4. If clients are found set the client ids on the chain handlers
            chain_a.set_client_id(clients_on_b[0].clone());
            chain_b.set_client_id(clients_on_b[0].clone());
            return (Box::new(chain_a), Box::new(chain_b))
        }
    
        // 4. If clients do not exist create them
        let (client_a, client_b) = hyperspace_primitives::utils::create_clients(&chain_a, &chain_b).await.unwrap();
        chain_a.set_client_id(client_a);
        chain_b.set_client_id(client_b);
        (Box::new(chain_a), Box::new(chain_b))
    }

    #[tokio::test]
    async fn full_integration_test() {
        hyperspace_core::logging::setup_logging();
        let (mut chain_a, mut chain_b) = setup_clients().await;
        // Run tests sequentially

        // no timeouts + connection delay
        ibc_messaging_with_connection_delay(&mut chain_a, &mut chain_b).await;

        // timeouts + connection delay
        ibc_messaging_packet_height_timeout_with_connection_delay(&mut chain_a, &mut chain_b).await;
        ibc_messaging_packet_timestamp_timeout_with_connection_delay(&mut chain_a, &mut chain_b).await;

        // channel closing semantics
        ibc_messaging_packet_timeout_on_channel_close(&mut chain_a, &mut chain_b).await;
        ibc_channel_close(&mut chain_a, &mut chain_b).await;
    }

```

## Running parachain tests

To run the integration tests between two parachain nodes:
1. Spawn the parachain and relay chain cluster by running [`docker-compose.yml`](/scripts/parachain-launch/docker-compose.yml`)
2. Run the tests with `cargo test -p hyperspace-testsuite`.  
