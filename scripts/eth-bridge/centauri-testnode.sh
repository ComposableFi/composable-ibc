KEY="mykey"
CHAINID="centauri-testnet-1"
MONIKER="localtestnet"
KEYALGO="secp256k1"
KEYRING="test"
LOGLEVEL="info"
# to trace evm
#TRACE="--trace"
TRACE=""

# validate dependencies are installed
command -v jq > /dev/null 2>&1 || { echo >&2 "jq not installed. More info: https://stedolan.github.io/jq/download/"; exit 1; }

# remove existing daemon
rm -rf ~/.banksy*

centaurid config keyring-backend $KEYRING
centaurid config chain-id $CHAINID

# if $KEY exists it should be deleted
echo "taste shoot adapt slow truly grape gift need suggest midnight burger horn whisper hat vast aspect exit scorpion jewel axis great area awful blind" | centaurid keys add $KEY --keyring-backend $KEYRING --algo $KEYALGO --recover

centaurid init $MONIKER --chain-id $CHAINID 

# Allocate genesis accounts (centauri formatted addresses)
centaurid add-genesis-account $KEY 10000000000000000000stake --keyring-backend $KEYRING

# Sign genesis transaction centauri1594tdya20hxz7kjenkn5w09jljyvdfk8kx5rd6
centaurid gentx $KEY 100000000000000000stake --keyring-backend $KEYRING --chain-id $CHAINID

# Collect genesis tx
centaurid collect-gentxs

# Run this to ensure everything worked and that the genesis file is setup correctly
centaurid validate-genesis

if [[ $1 == "pending" ]]; then
  echo "pending mode is on, please wait for the first block committed."
fi

# update request max size so that we can upload the light client
# '' -e is a must have params on mac, if use linux please delete before run
sed -i'' -e 's/max_body_bytes = /max_body_bytes = 1/g' ~/.banksy/config/config.toml
cat $HOME/.banksy/config/genesis.json | jq '.app_state["gov"]["params"]["voting_period"]="45s"' > $HOME/.banksy/config/tmp_genesis.json && mv $HOME/.banksy/config/tmp_genesis.json $HOME/.banksy/config/genesis.json

# Start the node (remove the --pruning=nothing flag if historical queries are not needed)
centaurid start --pruning=nothing  --minimum-gas-prices=0stake 