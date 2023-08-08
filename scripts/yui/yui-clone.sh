#!/bin/bash

# use the already set $YUI_COMMIT or set it to bfd4c30 (v0.3.5) if not set
export YUI_COMMIT=${YUI_COMMIT:-bfd4c30}
# use the already set $YUI_IBC_PATH or set it to /tmp/yui-ibc-solidity if not set
export YUI_IBC_PATH=${YUI_IBC_PATH:-/tmp/yui-ibc-solidity} 
# use the already set $YUI_IBC_REPO or set it to the yui-ibc-solidity-private repo if not set
export YUI_IBC_REPO=${YUI_IBC_REPO:-https://github.com/ComposableFi/yui-ibc-solidity-private}

function yui_clone() {
    # if the /tmp/yui-ibc-solidity directory does not exist, clone the repo
    if [ ! -d $YUI_IBC_PATH ]; then
        git clone $YUI_IBC_REPO $YUI_IBC_PATH

        cd $YUI_IBC_PATH

        if [ ! -z $YUI_COMMIT ]; then
            git checkout $YUI_COMMIT
        fi

        yarn
    else
        cd $YUI_IBC_PATH
    fi
}
