#!/bin/bash

cargo build --release -p codegen --bin codegen
./target/release/codegen --path ./utils/subxt/generated/src
cargo +nightly fmt -- --emit=files
