#!/bin/bash

set -euxm

export ROOT=`git rev-parse --show-toplevel`

source $ROOT/scripts/yui/yui-clone.sh
yui_clone;

forge install > /dev/null 2>&1
forge inspect OwnableIBCHandler abi
