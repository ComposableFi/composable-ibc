// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ICodec.sol";

contract ScaleCodec is ICodec {
    // A "compact" or general integer encoding is sufficient for encoding large integers (up to 2**536)
    // and is more efficient at encoding most values than the fixed-width version.
    // (Though for single-byte values, the fixed-width integer is never worse.)
    function decodeCompactU32(uint8[] memory input)
        external
        pure
        returns (uint32 value)
    {
        uint8 prefix = input[0];
        if (prefix % 4 == 0) {
            value = uint32(prefix) >> 2;
        } else if (prefix % 4 == 1) {
            uint16 x = decodeU16(input) >> 2;
            if (x > 0x0011_1111 && x <= 0x0011_1111_1111_1111) {
                value = uint32(x);
            } else {
                revert("U32_OUT_OF_RANGE");
            }
        } else if (prefix % 4 == 2) {
            uint32 x = decodeU32(input) >> 2;
            if (x > 0x0011_1111_1111_1111 && x <= uint32(0xffffffff) >> 2) {
                value = x;
            } else {
                revert("U32_OUT_OF_RANGE");
            }
        } else {
            if (prefix >> 2 == 0) {
                uint32 x = decodeU32(input);
                if (x > uint32(0xffffffff) >> 2) {
                    value = x;
                } else {
                    revert("U32_OUT_OF_RANGE");
                }
            } else {
                revert("U32_OUT_OF_RANGE");
            }
        }
        return value;
    }

    // Basic integers are encoded using a fixed-width little-endian (LE) format.
    function decodeU16(uint8[] memory input) public pure returns (uint16) {
        // Check that the input is the correct length for a U16
        require(input.length == 2, "Input length must be 2 for U16 decoding");

        // Initialize a variable to hold the result
        uint16 result;

        // Iterate through the input bytes, starting from the least significant byte
        for (uint256 i = 0; i < 2; i++) {
            // Shift the result left by 8 bits and add the current input byte
            result = (result << 8) + input[i];
        }

        // Return the result
        return result;
    }

    function decodeU32(uint8[] memory input) public pure returns (uint32) {
        // Check that the input is the correct length for a U32
        require(input.length == 4, "Input length must be 4 for U32 decoding");

        // Initialize a variable to hold the result
        uint32 result;

        // Iterate through the input bytes, starting from the least significant byte
        for (uint256 i = 0; i < 4; i++) {
            // Shift the result left by 8 bits and add the current input byte
            result = (result << 8) + input[i];
        }

        // Return the result
        return result;
    }
}
