#!/bin/bash

DOCKER_BUILDKIT=0 docker build --platform linux/amd64 -f scripts/parachain.Dockerfile . -t parachain-node:latest