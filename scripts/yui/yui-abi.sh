#!/bin/bash

set -euxm

export ROOT=`git rev-parse --show-toplevel`

# if the /tmp/yui-ibc-solidity directory does not exist, clone the repo
if [ ! -d "/tmp/yui-ibc-solidity" ]; then
    git clone -q https://github.com/hyperledger-labs/yui-ibc-solidity /tmp/yui-ibc-solidity
fi

cd /tmp/yui-ibc-solidity
git checkout bfd4c30 # v0.3.5

yarn > /dev/null 2>&1
forge install > /dev/null 2>&1
forge inspect OwnableIBCHandler abi
