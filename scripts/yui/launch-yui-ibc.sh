#!/bin/bash

set -euxm

export ROOT=`git rev-parse --show-toplevel`

echo "ROOT: $ROOT"

# check if the MNEMONIC environment variable is set
if [ -z ${MNEMONIC+x} ]; then
    echo "MNEMONIC is unset"
    exit 1
else
    echo "MNEMONIC is set to '$MNEMONIC'"
fi

anvil --version
cast --version
forge --version
yarn --version
node --version
jq --version

# if the /tmp/yui-ibc-solidity directory does not exist, clone the repo
if [ ! -d "/tmp/yui-ibc-solidity" ]; then
    git clone https://github.com/hyperledger-labs/yui-ibc-solidity /tmp/yui-ibc-solidity
fi

cd /tmp/yui-ibc-solidity

yarn

if [[ lsof -t -i :8545 ]]; then
    echo "port 8545 is already in use"
    exit 1
fi

anvil --mnemonic "$MNEMONIC" --host 0.0.0.0 --port 8545 &
netstat -an | grep 8545

forge create -r http://localhost:8545 --mnemonic "$MNEMONIC" IBCClient --json > ibc-client.json
export IBC_CLIENT=`cat ibc-client.json | jq '.deployedTo'`

forge create -r http://localhost:8545 --mnemonic "$MNEMONIC" IBCConnection --json > ibc-connection.json
cat ibc-connection.json
export IBC_CONNECTION=`cat ibc-connection.json | jq '.deployedTo'`

forge create -r http://localhost:8545 --mnemonic "$MNEMONIC" IBCChannelHandshake --json > ibc-channel-handshake.json
cat ibc-channel-handshake.json
export IBC_CHANNEL_HANDSHAKE=`cat ibc-channel-handshake.json | jq '.deployedTo'`

forge create -r http://localhost:8545 --mnemonic "$MNEMONIC" IBCPacket --json > ibc-packet.json
cat ibc-packet.json
export IBC_PACKET=`cat ibc-packet.json | jq '.deployedTo'`

forge create -r http://localhost:8545 --mnemonic=$MNEMONIC OwnableIBCHandler --constructor-args $IBC_CLIENT $IBC_CONNECTION $IBC_CHANNEL_HANDSHAKE $IBC_PACKET --json > ownable-ibc-handler.json
cat ownable-ibc-handler.json
export OWNABLE_IBC_HANDLER=`cat ownable-ibc-handler.json | jq '.deployedTo'`

cast rpc --rpc-url http://127.0.0.1:8545 eth_accounts

trap "kill -9 `jobs -p`" SIGHUP
fg anvil
