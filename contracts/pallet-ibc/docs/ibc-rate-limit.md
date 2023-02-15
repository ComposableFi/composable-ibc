# IBC Rate Limit middleware

This module is a safeguard to prevent a massive funds leaving our chain in the event of:
- a bug or hack in {our chain, counterparty chain}
- a bug or hack in the IBC implementation itself

// TODO: safe guarding counterparty chains means also blocking inbound traffic. Is this what we want?

This is rather useful, noting that bridges are large targets for hackers, and regardless of their level
of trust assumptions, there is at least always a smart contract risk involved in them. IBC, which is 
the protocol Centauri is build on has had [vulnerabilities discovered before](https://forum.cosmos.network/t/ibc-security-advisory-dragonberry/7702).

# Implementation details

The first version of this will be rather simple. And we want to mimic Osmosis' implementation, and hence
implement rate limit per denom on non-native assets.

We instantiate the initial parameters upon asset registration on our chain. 

## Parameters

```
base_value_in // this is the base value that's always allowed to bridge into our Chain
base_value_out // this is the base value that's always allowed to bridge from our Chain

epoch_rate_in // the rate by which the next epoch will grow based on the total amount transferred inbound on the current epoch
epoch_rate_out // the rate by which the next epoch will grow based on the total amount transferred outbound on the current epoch

epoch_in_blocks // number of blocks that stands for a rete limit epoch

quota_in // value that's allowed to bridge into our chain 
quota_out // value that's allowed to bridge from our chain 

quota_in_initialization = base_value_in
quota_out_initialization = base_value_out

// invariant: on each epoch 
amount_in <= quota_in
amount_out <= quota_out

```

## Rate limiting formula

```
// on next epoch after initialization
quota_in = amount_in_denom * (1 + epoch_rate_in)
quota_out = amount_out_denom * (1 + epoch_rate_out)

// and then on every new epoch
quota_in = amount_in_denom * (1 + epoch_rate_in)
quota_out = amount_out_denom * (1 + epoch_rate_out)

```

# Further enhancements

We want to improve the UX of users transferring funds cross chains. And not imposing a burden on them having to understand
whether their funds will be bridged or not, an interesting approach would be to partition their original amount
into multiple ones, and queue those transactions that will not be approved for the next _epoch_. In that case,
the user will not have to retry manually, but can either opt-in on this feature to just _fire and forget_ in periods
where the bridge quote is being challenged.

# Reference
Heavily inspired on [Osmosis rate limit implementation](https://github.com/osmosis-labs/osmosis/blob/main/x/ibc-rate-limit/README.md)