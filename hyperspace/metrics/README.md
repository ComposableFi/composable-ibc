## Hyperspace Metrics

This crate provides a prometheus server for collecting useful information about the relayer's operation.

### Server

The server can be spawned by calling `init_prometheus` with the server address and a prometheus registry.  
Metrics can be requested on the `/metrics` route via an http get request.

**Setting up the prometheus server**  

```rust 
    let registry = Registry::new_custom(Some("hyperspace".to_string()), None).expect("this can only fail if the prefix is empty");
    let metrics_a = Metrics::register(any_chain_a.name(), &registry)?;
    let metrics_b = Metrics::register(any_chain_b.name(), &registry)?;
    let mut metrics_handler_a = MetricsHandler::new(registry.clone(), metrics_a);
    let mut metrics_handler_b = MetricsHandler::new(registry.clone(), metrics_b);
    metrics_handler_a.link_with_counterparty(&mut metrics_handler_b);
    let addr = "127.0.0.1:8080".parse()?;
    tokio::spawn(init_prometheus(addr, registry.clone()));
```

### Data Collection

The data collected from the relayer for each chain handler includes the following:  

- `number_of_received_send_packets` - Total number of "send packet" events received.
- `number_of_received_receive_packets` - Total number of "receive packet" events received.
- `number_of_received_acknowledge_packets` - Total number of "acknowledge packet" events received.
- `number_of_received_timeouts` - Total number of "timeout packet" events received.
- `number_of_sent_packets` - Total number of sent packets.
- `number_of_sent_acknowledgments` - Total number of sent acknowledgments.
- `number_of_sent_timeout_packets` - Total number of timed out packets.
- `number_of_undelivered_packets` - Number of undelivered packets over time.
- `number_of_undelivered_acknowledgements` - Number of undelivered acknowledgements over time.
- `gas_cost_for_sent_tx_bundle` - Gas cost for every sent transaction.
- `transaction_length_for_sent_tx_bundle` - Transaction length (in bytes) for every sent tx bundle.
- `light_client_height` - Light client's latest height.
- `send_packet_event_time` - Average time between "send packet" events.
- `receive_packet_event_time` - Average time between "receive packet" events.
- `acknowledge_packet_event_time` - Average time between "acknowledge packet" events.
- `sent_packet_time` - Average time between sending and receiving packets.
- `sent_acknowledgment_time` - Average time between sending and receiving acknowledgments.
- `sent_timeout_packet_time` - Average time between sending and receiving timeout packets.
- `sent_update_client_time` - Average time between client updates.
