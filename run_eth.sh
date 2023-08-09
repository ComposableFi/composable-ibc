JWT_PATH=/Users/vmark/work/ethereum/consensus/prysm/jwt.hex
GENESIS_PATH=/Users/vmark/work/ethereum/consensus/prysm/genesis.ssz

geth --goerli --http --http.api eth,net,engine,admin --authrpc.jwtsecret "$JWT_PATH" &
#geth --sepolia --http --http.api eth,net,engine,admin --authrpc.jwtsecret "$JWT_PATH" &

#prysm.sh beacon-chain \
#  --execution-endpoint=http://localhost:8551 --sepolia \
#  --suggested-fee-recipient=0x01234567722E6b0000012BFEBf6177F1D2e9758D9 \
#  --jwt-secret="$JWT_PATH" --genesis-state="$GENESIS_PATH"

prysm.sh beacon-chain \
  --execution-endpoint=http://localhost:8551 --prater --enable-debug-rpc-endpoints \
  --jwt-secret="$JWT_PATH" --genesis-state="$GENESIS_PATH" \
  --checkpoint-sync-url=https://checkpoint-sync.goerli.ethpandaops.io \
  --genesis-beacon-api-url=https://checkpoint-sync.goerli.ethpandaops.io \
  --suggested-fee-recipient=0x01234567722E6b0000012BFEBf6177F1D2e9758D9

# 8545 8551 30303