// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./Codec.sol";

contract HashRefDB is ITrie {
    mapping(bytes32 => bytes) private nodes;

    function get(
        uint8[] calldata key,
        Slice calldata prefix,
        Hasher hash
    ) external view returns (bytes memory) {}
}

contract Query is ITrie {
    function decode(Hasher hasher, bytes calldata data)
        external
        returns (NodeHandle memory)
    {}
}
