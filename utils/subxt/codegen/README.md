## subxt-codegen

Use this as a library or binary to generate subxt types.

### Using the binary

```bash
 # ensure that you have the necessary nodes running in the background, reachable on ports
 cargo run --release -p codegen --bin codegen -- --path ./utils/subxt/generated/src/default
```

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