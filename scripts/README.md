# Launching a local network

## Polkadot-Launch

### Pre-requisites

1. Clone [polkadot](https://github.com/paritytech/polkadot) (branch release-v0.9.39) in the parent folder of `centauri` 
   and build it: `cargo build -r --bin polkadot --features fast-runtime`
2. Build `parachain-node` in release (`cargo build -r --bin parachain-node`)
3. Run script: `cd scripts/polkadot-launch`, `yarn install`

#### Run local network

In the same folder run `yarn dev`

#### Run composable network

Clone `composable` to the parent directory of `centauri`. Build node binary: `scripts/build-composable.sh`

In the same folder run `yarn picasso.json` or `yarn composable.json`. It's also possible to launch the networks together.

You can freely change the script and the JSON file to adjust the network configuration.

