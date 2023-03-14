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
4. Add a variant to all the `Any*` enums in `hyperspace/core/chain.rs`. For example,
```rust
pub enum AnyChain {
	Parachain(ParachainClient<DefaultConfig>),
	NewChain(ParachainClient<NewChainConfig>), // <-- newly added variant
}
```
and implement all the methods in the file for the new variant. Usually it's enough to just copy the `Parachain` branch,
for example:
```rust
impl AnyChain {
    async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>> {
        match self {
            Self::Parachain(chain) => chain.ibc_events().await,
            Self::Dali(chain) => chain.ibc_events().await, // <-- copy-pasted from Parachain and changed to `Dali`
        }
    }
}
```
And don't forget to implement `query_ibc_balance`, since it will panic instead of a compilation error if not all the variants
are handled.
