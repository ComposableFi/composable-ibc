#!/bin/bash

cargo check -p beefy-light-client-primitives --no-default-features --target=wasm32-unknown-unknown
cargo check -p beefy-light-client --no-default-features --target=wasm32-unknown-unknown
cargo check -p grandpa-light-client-primitives --no-default-features --target=wasm32-unknown-unknown
cargo check -p grandpa-light-client --no-default-features --target=wasm32-unknown-unknown
cargo check -p ibc --no-default-features --target=wasm32-unknown-unknown
cargo check -p ics07-tendermint --no-default-features --target=wasm32-unknown-unknown
cargo check -p ics10-grandpa --no-default-features --target=wasm32-unknown-unknown
cargo check -p ics11-beefy --no-default-features --target=wasm32-unknown-unknown
cargo check -p ics13-beefy --no-default-features --target=wasm32-unknown-unknown
cargo check -p pallet-ibc --no-default-features --target=wasm32-unknown-unknown
