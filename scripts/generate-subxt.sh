#!/bin/bash

 cargo build --release -p codegen --bin codegen
 ./target/release/codegen \
    --path ./utils/subxt/generated/src \
    --relay_host $GATEWAY_IP \
    --para_host $GATEWAY_IP
cargo +nightly fmt -- --emit=files
