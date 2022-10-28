## subxt-codegen

Use this as a library or binary to generate subxt types.

### Using the binary

```bash
 cargo build --release -p codegen --bin codegen 
 # ensure that you have the necessary nodes running in the background, reachable on ports
 ./target/release/codegen --path ./utils/subxt/generated/src
```