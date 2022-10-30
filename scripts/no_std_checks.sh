#!/bin/bash

cargo +nightly check -p beefy-light-client-primitives --no-default-features --target=wasm32-unknown-unknown
cargo +nightly check -p beefy-light-client --no-default-features --target=wasm32-unknown-unknown
cargo +nightly check -p grandpa-light-client-primitives --no-default-features --target=wasm32-unknown-unknown
cargo +nightly check -p grandpa-light-client --no-default-features --target=wasm32-unknown-unknown
cargo +nightly check -p ibc --no-default-features --target=wasm32-unknown-unknown
cargo +nightly check -p ics07-tendermint --no-default-features --target=wasm32-unknown-unknown
cargo +nightly check -p ics10-grandpa --no-default-features --target=wasm32-unknown-unknown
cargo +nightly check -p ics11-beefy --no-default-features --target=wasm32-unknown-unknown
cargo +nightly check -p ics13-beefy --no-default-features --target=wasm32-unknown-unknown
cargo +nightly check -p pallet-ibc --no-default-features --target=wasm32-unknown-unknown
