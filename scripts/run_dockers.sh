docker rm -f eth-pg
docker rm -f eth-redis

docker run --name eth-pg -p 5432:5432 -e POSTGRES_PASSWORD=password -d postgres:16.0
#docker run --name eth-pg -p 5432:5432 -e POSTGRES_PASSWORD=7OKx7EAhtcyIObg8ScUkAre -d postgres:16.0
docker run --name eth-redis -p 6379:6379 -p 8001:8001 -d redis
 # --chain=mainnet --start-block=19021572 --rpcs=https://geth-execution-0.ethereum-mainnet.sre-scratchpad-349209.composablenodes.tech --contract-addresses=0x6d99ba872e2d116b0da91954ef6ddf69177efd8b