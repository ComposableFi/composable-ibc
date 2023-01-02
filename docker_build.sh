apk update
apk add --no-cache musl-dev cmake curl protoc protobuf

export CARGO_TARGET_DIR=target_docker
export PROTOC=/usr/bin/protoc
export PROTOC_INCLUDE=/usr/include/google/protobuf/

cd proj

sh

# cargo build --bin hyperspace
# ./target_docker/debug/hyperspace create-connection --config-a ibcgo-1.toml --config-b rococo-local.toml --config-core core.toml --delay-period 10 --port-id testport --order unordered