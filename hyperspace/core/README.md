# Hyperspace Relayer

## Architecture
The relayer architecture is based on two major design choices

**Statelessness**  
The relayer is designed to be stateless and does not perform any form of caching. The relayer therefore relies heavily on  
the nodes it's connected to for sourcing data, this design choice eliminates the chances of bugs that could come  
from cache invalidation problems.

**Event Driven**  
The relayer follows an event driven model, in that it remains idle if no finality events are received from either chain.

## Relayer Loop

The relayer has just one entry point, which is the [`relay`]() function, this function takes two [`Chain`]() implementations  
alongside optional metric handlers and starts the relayer loop.  

The relayer loops awaits finality events from the finality subscription of the chain handlers.  
Whenever a finality event is received, the latest ibc events are queried using `query_latest_ibc_events`.  
These events are then parsed into appropriate messages using the `parse_events` function.  

The `parse_events` function internally calls `query_ready_and_timed_out_packets` which queries a chain and  
produces all packet messages that have passed the connection delay check.


## Using the relayer

Using the relayer requires having a [`Chain`](/hyperspace/primitives/src/lib.rs#L346) implementation for the chains in question.

```rust
    // Naive example of how to use the relayer
    pub struct ChainA;

    impl IbcProvider for ChainA { ... }
    impl KeyProvider for ChainA { ... }
    impl Chain for ChainA { ... }

    pub struct ChainB;

    impl IbcProvider for ChainB { ... }
    impl KeyProvider for ChainB { ... }
    impl Chain for ChainB { ... }

    async fn main() -> Result<(), anyhow::Error>{
        let chain_a = ChainA::default();
        let chain_b = ChainB::default();
        relay(chain_a, chain_b, None, None).await?;
        Ok(())
    }
```
**Note** Correct functioning of the relayer is dependent on correct implementation of the trait methods, read documentation  
for each of the trait methods for details.

## Gas Awareness

The relayer employs a gas aware logic to submit IBC messages. Messages whose execution cost would exceed block gas limits  
are split into chunks.  
The gas limit tuning is performed by [`flush_message_batch`](/hyperspace/core/src/queue.rs#L6), it achieves this by by estimating the weight of the message batch  
using [`estimate_weight`](/hyperspace/primitives/src/lib.rs#L354) and comparing it with the maximum block gas limit provided by [`block_max_weight`](/hyperspace/primitives/src/lib.rs#L351),  
if the estimate exceeds the latter then the ibc messages are split into smaller chunks that fit within the gas limit and  
these chunks are then submitted as individual transactions.  


## CLI Interface

The CLI interface can be used to start the relayer from a config file and also performing the IBC setup on both chains.

- [`relay`](/hyperspace/core/src/command.rs#L24) 
  This command accepts a path to a config file and spawns the relayer.
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
    
  