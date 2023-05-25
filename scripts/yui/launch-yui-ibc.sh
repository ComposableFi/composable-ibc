#!/bin/bash

set -euxm

export ROOT=`git rev-parse --show-toplevel`

echo "ROOT: $ROOT"

# check if the MNEMONIC environment variable is set
if [ -z ${MNEMONIC+x} ]; then
    echo "MNEMONIC is unset, using default test-junk mnemonic"
    export MNEMONIC="test test test test test test test test test test test junk"
else
    echo "MNEMONIC is set to '$MNEMONIC'"
fi

anvil --version
cast --version
forge --version
yarn --version
node --version
jq --version


if [ lsof -t -i :8545 ]; then
    echo "port 8545 is already in use"
    exit 1
fi

source $ROOT/scripts/yui/yui-clone.sh
yui_clone;

forge install

# start local ethereum node using anvil
anvil --mnemonic "$MNEMONIC" --host 0.0.0.0 --port 8545 &

# deploy the IBC contracts
forge create -r http://localhost:8545 --mnemonic "$MNEMONIC" IBCClient --json > ibc-client.json

forge create -r http://localhost:8545 --mnemonic "$MNEMONIC" IBCConnection --json > ibc-connection.json
cat ibc-connection.json

forge create -r http://localhost:8545 --mnemonic "$MNEMONIC" IBCChannelHandshake --json > ibc-channel-handshake.json
cat ibc-channel-handshake.json

forge create -r http://localhost:8545 --mnemonic "$MNEMONIC" IBCPacket --json > ibc-packet.json
cat ibc-packet.json

forge create -r http://localhost:8545 --mnemonic "$MNEMONIC" OwnableIBCHandler \
    --constructor-args\
        `cat ibc-client.json | jq '.deployedTo' | tr -d '"'` \
        `cat ibc-connection.json | jq '.deployedTo' | tr -d '"'` \
        `cat ibc-channel-handshake.json | jq '.deployedTo' | tr -d '"'` \
        `cat ibc-packet.json | jq '.deployedTo' | tr -d '"'` \
    --json > ownable-ibc-handler.json

cat ownable-ibc-handler.json

cast rpc --rpc-url http://127.0.0.1:8545 eth_accounts

# setup a trap to kill the anvil process when the script exits
trap "kill -9 `jobs -p`" SIGHUP
fg anvil
