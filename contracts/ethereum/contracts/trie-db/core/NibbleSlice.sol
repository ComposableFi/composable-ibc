// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";

contract NibbleSlice is ITrie {
    bytes data;
    uint8 offset;
    uint8 constant NIBBLE_PER_BYTE = 2;
    bytes1 constant PADDING_BITMASK = 0x0F;

    function setData(bytes calldata key) external {
        data = key[2:];
        offset = 0;
    }

    function mid(uint8 keyNibbles) external {
        offset += keyNibbles;
    }

    function left() external view returns (Prefix memory) {
        // uint8 split = offset / NIBBLE_PER_BYTE;
        uint8 ix = uint8(offset % NIBBLE_PER_BYTE);
        // TODO: fix returns
        if (ix == 0) {
            return Prefix(data, 0);
        } else {
            return Prefix(data, 0);
        }
    }

    // TODO: implement this
    // function padLeft(bytes1 split) internal returns (bytes1) {
    //     return split & !PADDING_BITMASK;
    // }

    function len() external returns (uint8) {}

    function getPrefix() external returns (Prefix memory) {}

    function getSlice() external returns (uint8[] memory) {}

    function originalDataAsPrefix() external returns (Prefix memory) {}

    function startWith(NibbleSlice slice) external returns (bool) {}

    function isEmpty() external returns (bool) {}

    function at(uint256 index) external returns (uint256) {}
}
