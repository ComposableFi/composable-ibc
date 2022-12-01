// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";

contract NibbleSlice is ITrie {
    uint8 constant NIBBLE_PER_BYTE = 2;

    function mid(Slice memory slice, uint8 keyNibbles)
        external
        pure
        returns (Slice memory)
    {
        slice.offset += keyNibbles;
        return slice;
    }

    function len(Slice memory slice) public pure returns (uint8) {
        // return the length of the slice data in nibbles
        return uint8(slice.data.length * NIBBLE_PER_BYTE - slice.offset);
    }

    function isEmpty(Slice memory slice) public pure returns (bool) {
        // check if the length of the slice data is 0
        return len(slice) == 0;
    }

    function left(Slice memory slice) external pure returns (Slice memory) {
        // check if the slice data is empty
        require(!isEmpty(slice), "Slice is empty");

        uint8 split = slice.offset / NIBBLE_PER_BYTE;
        uint8 ix = uint8(slice.offset % NIBBLE_PER_BYTE);

        // Check if the current nibble index is at the start of a byte
        if (ix == 0) {
            return Slice(slice.data, split);
        } else {
            // Create a new bytes variable with the left side of the original data
            bytes memory leftData = new bytes(split + 1);
            // Copy the left side of the original data into the new variable
            for (uint256 i = 0; i < split; i++) {
                leftData[i] = slice.data[i];
            }
            // Set the last nibble of the new data to the left side of the original data
            leftData[split] = slice.data[split] >> (NIBBLE_PER_BYTE - ix);
            // Return the Prefix instance with the new data
            return Slice(leftData, split + 1);
        }
    }

    function getPrefix() external returns (Slice memory) {}

    function getSlice() external returns (NibbleSlice) {}

    function originalDataAsPrefix(Slice memory slice)
        external
        returns (Slice memory)
    {}

    function startWith(Slice memory partialSlice, Slice memory slice)
        external
        returns (bool)
    {}

    function isEmpty() external returns (bool) {}

    function at(Slice memory slice, uint256 index) external returns (uint256) {}
}
