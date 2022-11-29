// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./Node.sol";

contract Codec is ITrie {
    NodeCodec codec;

    constructor(NodeCodec nodeCodec) {
        codec = nodeCodec;
    }

    function decode(bytes32 node_data) external returns (Node) {}
}
