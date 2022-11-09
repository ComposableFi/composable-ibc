# Hyperspace Relayer

## Architecture
The relayer is designed to be:

### 1. Stateless 
This means that the relayer does not perform **any form** of data caching. The relayer therefore relies heavily on  
the nodes it's connected to for sourcing data as needed. This design choice eliminates a class of bugs that could come from cache invalidation.

### 2. Event Driven
The relayer follows an event driven model, where it waits idly until it receives a finality notification from any of the chains it's connected to. The finality notification represents new IBC messages and events that have finalized and ready to be sent to the connected counterparty chain.

## Relayer Loop

The relayer has a single entry point, which is the [`relay`](/hyperspace/core/src/lib.rs#L20) function, this function takes two [`Chain`](/hyperspace/primitives/src/lib.rs#L346) implementations  
alongside optional metric handlers and starts the relayer loop.  

The relayer loops awaits finality events from the finality subscription of the chain handlers.  
Whenever a finality event is received, the latest ibc events are queried using `query_latest_ibc_events`.  
These events are then parsed into the appropriate outgoing IBC messages, and sent off to the counterparty chain. 

### Packet Timeouts

The `query_ready_and_timed_out_packets` which queries a chain and  
produces all packet messages that have passed the connection delay check.
It also returns timed out packet messages that have passed the connection delay check.  

// todo: let's talk more about how we handle timeouts

## Using the relayer

Using the relayer requires having a [`Chain`](/hyperspace/primitives/src/lib.rs#L346) implementation for the chain types  
that packets would be relayed between.

```rust
    // Naive example of how to use the relayer
    pub struct ChainA { ... }

    impl hyperspace_primitives::IbcProvider for ChainA { ... }
    impl hyperspace_primitives::KeyProvider for ChainA { ... }
    impl hyperspace_primitives::Chain for ChainA { ... }

    pub struct ChainB { ... }

    impl hyperspace_primitives::IbcProvider for ChainB { ... }
    impl hyperspace_primitives::KeyProvider for ChainB { ... }
    impl hyperspace_primitives::Chain for ChainB { ... }

    async fn main() -> Result<(), anyhow::Error>{
        let chain_a = ChainA::default();
        let chain_b = ChainB::default();
        hyperspace_core::relay(chain_a, chain_b, None, None).await?;
        Ok(())
    }
```
**Note** Correct functioning of the relayer is dependent on correct implementation of the trait methods, read documentation  
for each of the trait methods for implementation details.

## Gas Awareness

The relayer is gas aware when submitting IBC messages. Messages whose execution cost would exceed block gas limits  
are split into chunks.  
The gas limit tuning is performed by [`flush_message_batch`](/hyperspace/core/src/queue.rs#L6), it achieves this by estimating the weight of the message batch  
using [`estimate_weight`](/hyperspace/primitives/src/lib.rs#L354) and comparing it with the maximum block gas limit provided by [`block_max_weight`](/hyperspace/primitives/src/lib.rs#L351),  
if the estimate exceeds the latter then the ibc messages are split into smaller chunks that fit within the gas limit and  
these chunks are then submitted as individual transactions.  


## CLI Interface

The CLI interface can be used to start the relayer from a config file and also performing the IBC setup on both chains.

- [`relay`](/hyperspace/core/src/command.rs#L24)  
  This command accepts a path to a config file and spawns the relayer alongside a prometheus server for monitoring.  
  The config file must have all the parameters necessary for the chain clients to work correctly.
- [`create-clients`](/hyperspace/core/src/command.rs#L26)  
  This command takes a path to a config file and attempts to create a light clients of each chain on its counterparty.
- [`create-connection`](/hyperspace/core/src/command.rs#L28)  
  This command takes a path to a config file and delay period in seconds and attempts to complete the connection  
  handshake between both chains.
  The config file must have a valid client id
- [`create-channel`](/hyperspace/core/src/command.rs#L30)  
  This command takes a path to a config file, a port id and a version, it attempts to complete the channel handshake  
  between both chains.
  The config file must have a valid client and connection id.
    

### Metrics

The relayer can be spawn with metrics enabled. The [`metrics`](/hyperspace/metrics/README.md) crate provides a prometheus server that collects data  
about the relayer's operation.  

Metrics collected are centered around packets and light client state on either chain and also the cost of transactions submitted on both chains.  

