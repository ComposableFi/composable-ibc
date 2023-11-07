# Launching a local network

## zombienet

0. Install zombienet using official guide or `nix profile install "github:paritytech/zombienet/refs/pull/1316/merge#default"`
1. Clone [polkadot](https://github.com/paritytech/polkadot) (branch release-v0.9.43) in the parent folder of `centauri` 
   and build it: `cargo build -r --bin polkadot --features fast-runtime`
2. Build `parachain-node` in release (`cargo build -r --bin parachain-node`)
3. `cd scripts` and `run-zombienet.sh`