## subxt-codegen

Use this as a library or binary to generate subxt types.

### Using the binary

```bash
 # ensure that you have the necessary nodes running in the background, reachable on ports
 cargo run --release -p codegen --bin codegen -- --path ./utils/subxt/generated/src/default
```

You can also specify url for the parachain and relaychain manually using `--para-url` an `--relay-url`, respectively.

#### Examples

Updating types for:

- composable (mainnet): `cargo run -p codegen --bin codegen -- --path ./utils/subxt/generated/src/composable --relay-url wss://composable-unrpc-lb.composablenodes.tech/relay --para-url wss://composable-unrpc-lb.composablenodes.tech`
- picasso (mainnet): `cargo run -p codegen --bin codegen -- --path ./utils/subxt/generated/src/picasso_kusama --relay-url wss://picasso-unrpc-lb.composablenodes.tech/relay --para-url wss://picasso-unrpc-lb.composablenodes.tech`
- picasso (testnet): `cargo run -p codegen --bin codegen -- --path ./utils/subxt/generated/src/picasso_rococo --relay-url wss://rococo-rpc.polkadot.io --para-url wss://picasso-rococo-rpc-lb.composablenodes.tech`

### Adding new chain
1. Create new folder (e.g. `new_chain`) in `utils/subxt/generated/src` with `mod.rs`:
```rust
pub mod parachain;
pub mod relaychain;
```
2. Generate types for the chain:
```bash
 cargo run --release -p codegen --bin codegen -- --path ./utils/subxt/generated/src/new_chain
```
3. Create a corresponding config in `hyperspace/core/substrate/new_chain.rs` (see `default.rs` for example)
4. Add the new variant to the `chains!` macro call.