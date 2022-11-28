// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";

contract HashRefDB is ITrie {
    function get(
        bytes32 key,
        bytes32 prefix,
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

contract NibbleSlice {
    bytes32 nibbleKey;

    constructor(bytes32 key) {
        nibbleKey = key;
    }

    function newNibbleSlice(bytes32 key) external {}

    function mid(uint256 keyNibbles) external {}

    function left() external {}

    function getPrefix() external returns (bytes32) {}
}
