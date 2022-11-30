// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";

contract HashRefDB is ITrie {
    function get(
        bytes calldata key,
        Prefix calldata prefix,
        Hasher hasher
    ) external returns (bytes32[] memory) {}

    function contains(
        bytes32 key,
        bytes32 prefix,
        Hasher hasher
    ) external returns (bool) {}
}

contract Query is ITrie {
    function decode(Hasher hasher) external returns (bytes32) {}
}
