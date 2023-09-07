#!/bin/bash

set -e
block_number=0

until [ $block_number -gt 10 ]
do
  block_number=$(curl -L -s http://${PARA_HOST}:$METRICS_PORT/metrics | grep 'substrate_block_height{status="best"' | awk '{print $2}')
  if [ -z "$block_number" ]
  then
    cat /tmp/logfile.log
    exit 1;
  fi
  echo block number: $block_number
  sleep 10
done
