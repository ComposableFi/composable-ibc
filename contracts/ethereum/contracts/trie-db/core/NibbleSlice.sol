// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

contract NibbleSlice {
    bytes32 nibbleKey;

    constructor(bytes32 key) {
        nibbleKey = key;
    }

    function newNibbleSlice(bytes32 key) external {}

    function mid(uint256 keyNibbles) external returns (NibbleSlice) {}

    function left() external {}

    function len() external returns (uint256) {}

    function getPrefix() external returns (bytes32) {}

    function getSlice() external returns (bytes32) {}

    function originalDataAsPrefix() external returns (bytes32) {}

    function startWith(NibbleSlice slice) external returns (bool) {}

    function isEmpty() external returns (bool) {}

    function at(uint256 index) external returns (uint256) {}
}
