#!/bin/bash

function yui_clone() {
    # use the already set $YUI_COMMIT or set it to bfd4c30 (v0.3.5) if not set
    export YUI_COMMIT=${YUI_COMMIT:-bfd4c30}
    
    # if the /tmp/yui-ibc-solidity directory does not exist, clone the repo
    if [ ! -d "/tmp/yui-ibc-solidity" ]; then
        git clone https://github.com/hyperledger-labs/yui-ibc-solidity /tmp/yui-ibc-solidity

        cd /tmp/yui-ibc-solidity

        git checkout $YUI_COMMIT
        
        # apply patches from ../scripts/yui/patches/*.patch
        for patch in $(ls $ROOT/scripts/yui/patches/*.patch); do
            git apply $patch
        done

        yarn
    else
        cd /tmp/yui-ibc-solidity
    fi
}
