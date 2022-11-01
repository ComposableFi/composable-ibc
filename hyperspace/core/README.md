# Hyperspace relayer

## Architecture
The relayer architecture is based on two major design choices

**Statelessness**
The relayer is designed to be stateless and does not perform any form of caching. The relayer therefore relies heavily,  
on the nodes it's connected to for sourcing data, this design choice eliminates a whole class of bugs that could come  
from cache invalidation problems.

**Event Driven**
The relayer follows an event driven model, in that it remains idle if no finality events are received from either chain.

## Relayer Loop

The relayer has just one entry point, which is the [`relay`]() function, this function takes two [`Chain`]() implementations  
alongside optional metric handlers and starts the relayer loop.  

The relayer loops awaits finality events from the finality subscription of the chain handlers.  
Whenever a finality event is received, the latest ibc events are queried using `query_latest_ibc_events`.  
These events are then parsed into appropriate messages using the `parse_events` function.  

The `parse_events` function also calls `query_ready_and_timed_out_packets` which produces all packet messages that can   
be submitted at the time after the connection delay has been factored.


## Using the relayer

Using the relayer just requires having a `Chain` implementation for the chains that messages would be relayed between

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
**Note** The relayer depends on correct implementation of the trait methods, check the documentation for each of the trait  
methods for details.
##    