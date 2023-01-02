#!/bin/bash

set -e
set -x

# x86_64
DOCKER_BUILDKIT=0 docker build --platform linux/amd64 -f scripts/parachain.Dockerfile . -t parachain-node:local
# arm64
#DOCKER_BUILDKIT=0 docker build --platform linux/arm64 -f scripts/parachain.aarch64.Dockerfile . -t parachain-node:local