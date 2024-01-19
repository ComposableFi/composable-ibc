## Running Solana-Cosmos Integration tests on Localnet

Clone [composable-ibc](https://github.com/ComposableFi/composable-ibc) with branch `hyperspace-cosmos`.

### i. Setting up solana program

1. Install [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools). Used for running the local validator
2. Install [Anchor CLI](https://www.anchor-lang.com/docs/installation) . Used for buliding and deploying the program.
3. Clone the [solana-ibc program](https://github.com/ComposableFi/emulated-light-client) with branch `test-relayer`.
4. Start local validator with block subscription
```
solana-test-validator -r --rpc-pubsub-enable-block-subscription
```
5. Build and deploy the program on localnet
```
anchor test --skip-local-validator -- --features mocks
```

### ii. Setting up local centauri chain

1. Set up a cosmos node
   Clone the repo (with the tag): https://github.com/notional-labs/composable-centauri/tree/v4.5.0
2. Run `go mod vendor`
3. Open file `vendor/github.com/cosmos/ibc-go/v7/modules/light-clients/08-wasm/keeper/msg_server.go` on line 20 and comment the if statement. Also remove the `govtypes "github.com/cosmos/cosmos-sdk/x/gov/types" `import
4. Open `app/ante/ibc_ante.go` on line 63 and comment the if statement there. Remove unused imports
5. Run `mkdir centaurid-nogov`
6. Build the node using g`o build -mod vendor -o centaurid-nogov ./...`
7. Run the network using the script attached below 
```
./scripts/testnode.sh
```

### iii. Running the testsuite
1. Run the command below to compile the wasm contract.
```
cargo +nightly-2023-02-07 build -p icsxx-solana-cw --release --target wasm32-unknown-unknown --lib --no-default-features
```
2. Set the appropriate rpc url for solana and cosmos.
3. Run the following command to run the integration test.
```
RUST_BACKTRACE=1 cargo test --package hyperspace-testsuite --test solana_cosmos --  --nocapture
```

