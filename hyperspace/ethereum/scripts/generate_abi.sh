SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

cd $SCRIPTPATH/../yui-ibc-solidity/node_modules || (echo "Error: yui-ibc-solidity/node_modules does not exist" && exit 1)

forge inspect --via-ir ../contracts/diamond/Diamond.sol:Diamond abi > ../../src/abi/diamond-abi.json
forge inspect --via-ir ../contracts/diamond/facets/DiamondCutFacet.sol:DiamondCutFacet abi > ../../src/abi/diamond-cut-facet-abi.json
forge inspect --via-ir ../contracts/diamond/facets/DiamondLoupeFacet.sol:DiamondLoupeFacet abi > ../../src/abi/diamond-loupe-facet-abi.json
forge inspect --via-ir ../contracts/diamond/facets/OwnershipFacet.sol:OwnershipFacet abi > ../../src/abi/ownership-facet-abi.json
forge inspect --via-ir ../contracts/diamond/facets/GovernanceFacet.sol:GovernanceFacet abi > ../../src/abi/governance-facet-abi.json
forge inspect --via-ir ../contracts/utils/GovernanceProxy.sol:GovernanceProxy abi > ../../src/abi/governance-proxy-abi.json
forge inspect --via-ir ../contracts/diamond/facets/RelayerWhitelistFacet.sol:RelayerWhitelistFacet abi > ../../src/abi/relayer-whitelist-facet-abi.json
forge inspect --via-ir ../contracts/diamond/facets/CallBatchFacet.sol:CallBatchFacet abi > ../../src/abi/call-batch-facet-abi.json

forge inspect --via-ir ../contracts/core/02-client/IBCClient.sol:IBCClient abi > ../../src/abi/ibc-client-abi.json
forge inspect --via-ir ../contracts/core/03-connection/IBCConnection.sol:IBCConnection abi > ../../src/abi/ibc-connection-abi.json
forge inspect --via-ir ../contracts/core/04-channel/IBCPacket.sol:IBCPacket abi > ../../src/abi/ibc-packet-abi.json
forge inspect --via-ir ../contracts/core/04-channel/IBCChannelHandshake.sol:IBCChannelHandshake abi > ../../src/abi/ibc-channel-abi.json
forge inspect --via-ir ../contracts/core/25-handler/IBCQuerier.sol:IBCQuerier abi > ../../src/abi/ibc-querier-abi.json

forge inspect --via-ir ../contracts/apps/20-transfer/ICS20TransferBank.sol:ICS20TransferBank abi > ../../src/abi/ics20-transfer-bank-abi.json
forge inspect --via-ir ../contracts/apps/20-transfer/ICS20Bank.sol:ICS20Bank abi > ../../src/abi/ics20-bank-abi.json
forge inspect --via-ir ../contracts/clients/TendermintLightClientZK.sol:TendermintLightClientZK abi > ../../src/abi/tendermint-client-abi.json

forge inspect --via-ir ../contracts/clients/ethereum/EthereumLightClient.sol:EthereumLightClient abi > ../../src/abi/ethereum-client-abi.json
forge inspect --via-ir ../contracts/apps/20-transfer/ERC20Token.sol:ERC20Token abi > ../../src/abi/erc20-abi.json

# remove all "."s inside the abi files to make them compatible with the derive macro
sed -i 's/\.//g' ../../src/abi/*.json