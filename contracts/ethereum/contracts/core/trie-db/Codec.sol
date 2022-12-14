// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ITrie.sol";
import "../../interfaces/ISpec.sol";

contract Codec is ITrie, ISpec {
    function decode(bytes32 node_data, NodeCodec codec)
        external
        returns (NodeStruct memory)
    {}
}
