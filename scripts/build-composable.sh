#!/bin/bash

wasm-optimizer() {
    wasm-opt $1 -o $2 -Os --strip-dwarf --debuginfo --mvp-features
    subwasm compress $2 $2
}

if ! command -v wasm-opt &> /dev/null
then
    echo "'wasm-opt' couldn't be found. Please install it from https://rustwasm.github.io/wasm-pack/installer/"
    exit 1
fi

if ! command -v subwasm &> /dev/null
then
    echo "'subwasm' couldn't be found. Please install it from https://github.com/chevdor/subwasm#install"
    exit 1
fi

cd "$(dirname "$0")/../../composable/code" || exit 1

cargo build -p composable-runtime-wasm --target wasm32-unknown-unknown -r || exit 1
cargo build -p picasso-runtime-wasm    --target wasm32-unknown-unknown -r || exit 1
cargo build -p picasso-runtime-wasm    --target wasm32-unknown-unknown --features testnet,fastnet -r || exit 1

wasm-optimizer ./target/wasm32-unknown-unknown/release/picasso_runtime.wasm    ./target/wasm32-unknown-unknown/release/picasso_kusama_runtime.optimized.wasm
wasm-optimizer ./target/wasm32-unknown-unknown/release/composable_runtime.wasm ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm
wasm-optimizer ./target/wasm32-unknown-unknown/release/picasso_runtime.wasm    ./target/wasm32-unknown-unknown/release/picasso_rococo_runtime.optimized.wasm

export PICASSO_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/picasso_rococo_runtime.optimized.wasm)
# Uncomment to use Picasso (Kusama) instead of Picasso (Rococo)
#export PICASSO_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/picasso_kusama_runtime.optimized.wasm)
export COMPOSABLE_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm)

cargo build --features builtin-wasm --bin composable -r --features builtin-wasm || exit 1
