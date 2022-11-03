#!/bin/bash

PORT=$1 

echo "Waiting $PORT/tcp opening..."

while ! nc -z localhost $PORT; do   
  sleep 0.5
done

echo "$PORT/tcp is open."