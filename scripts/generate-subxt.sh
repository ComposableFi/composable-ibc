#!/bin/bash

set -e

cargo run --bin codegen -- --path ./utils/subxt/generated/src/composable
cargo fmt -- --emit=files

echo "Subxt types are up to date"