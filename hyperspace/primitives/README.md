# Hyperspace Primitives

This crate defines the traits around which the core relayer logic is built.

## IBC Provider

The [`IbcProvider`](/hyperspace/primitives/src/lib.rs#L83) trait defines methods for querying ibc state from the chain.  

**Associated Types**
- `FinalityEvent` - This should represent the type yielded by the chain's block finality stream.
- `Error` - Errors specific to the chain implementation.
- `TransactionId` - A type that represents the format for transaction ids for the chain.

**Channel Whitelist**
The relayer only relays packets on channels specified in the [`channel_whitelist`](/hyperspace/primitives/src/lib.rs#L219). When the channel whitelist returns  
an empty list, packets will not be relayed.

## Chain 

The [`Chain`](/hyperspace/primitives/src/lib.rs#L346) trait defines methods that centre around subscribing to finality notifications and transaction submission.

- `block_max_weight`  
  This function should return a number that represents the maximum gas a block can consume.
- `estimate_weight`  
  This function should take a vector of IBC messages and return a numerical value that represents the estimated gas    
  it would take to execute these transactions.
- `finality_notifications`  
  This function should return a stream that yields a `FinalityEvent` when a new block has been finalized.
- `submit`  
  This function should take a vector of IBC messages and submit them to the chain.  
  The function should wait until the transaction is included in a block before returning.

## KeyProvider

The [`KeyProvider`](/hyperspace/primitives/src/lib.rs#L346) trait defines a single method for getting the relayer's on-chain account Id.

## TestProvider

The [`TestProvider`](/hyperspace/primitives/src/lib.rs#L346) trait defines methods used by the testsuite for integration tests.  
The trait specifies the following methods:

- `send_transfer` -  This function should submit a transaction that initiates an ics20 token transfer on chain.
- `send_ordered_packet` - This function should submit a transaction that initiates a packet transfer on an ordered channel.
- `set_channel_whitelist` - This function should set the channel whitelist on the chain's client.
- `subscribe_blocks` - This function should return a stream that yields new block numbers.

## Utility Functions

There are a couple utility functions, some of them essential to the core relayer logic.  

- [`query_undelivered_sequences`](/hyperspace/primitives/src/lib.rs#L374)   
  This function returns all packet sequences that have been sent out from the `source` chain, but are yet to be received  
  on the sink chain.
- [`query_undelivered_acks`](/hyperspace/primitives/src/lib.rs#L421)   
  This function returns all the packet acknowledgement sequences on the source that have not been delivered to the sink chain.
- [`find_suitable_proof_height_for_client`](/hyperspace/primitives/src/lib.rs#L478)  
  This function searches for the best available sink light client height on the source chain that can be used to verify a packet timeout  
  proof.
- [`query_maximum_height_for_timeout_proofs`](/hyperspace/primitives/src/lib.rs#L543)  
  This function helps find the maximum height for timeout proofs based on the current undelivered packets, this, coupled  
  with other checks can be used in deciding which client updates are mandatory

- [`create_clients`](/hyperspace/primitives/src/utils.rs#L30)  
  This function takes two chain handles and attempts to creates a light client of each chain on the counterparty.
- [`create_connection`](/hyperspace/primitives/src/utils.rs#L64)  
  This function takes two chain handles and a connection delay and attempts to complete the connection handshake process between both chains.
- [`create_channel`](/hyperspace/primitives/src/utils.rs#L111)  
  This function takes a two chain handles alongside other parameters and attempts to complete the channel handshake between both chains.