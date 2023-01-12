apt update
apt install -y cmake curl unzip

PROTOC_ZIP=protoc-3.20.3-linux-aarch_64.zip
curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v3.20.3/${PROTOC_ZIP}
unzip -o ${PROTOC_ZIP} -d ./proto
chmod 755 -R ./proto/bin
BASE=/usr
cp ./proto/bin/protoc ${BASE}/bin/
cp -R ./proto/include/* ${BASE}/include/

export PATH="/usr/local/protoc/bin:$PATH"
export CARGO_TARGET_DIR=target_docker

alias ll="ls -l"
alias la="ls -a"
alias l="ll"

cd proj

bash

# cargo build --bin hyperspace
# cargo run --bin hyperspace -- upload-wasm --config ibcgo-1.toml --wasm-path ./target/wasm32-unknown-unknown/release/ics10_grandpa_cw.wasm
# ./target_docker/debug/hyperspace create-connection --config-a ibcgo-1.toml --config-b rococo-local.toml --config-core core.toml --delay-period 10 --port-id testport --order unordered
