// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

contract ScaleCodec {
    // A "compact" or general integer encoding is sufficient for encoding large integers (up to 2**536)
    // and is more efficient at encoding most values than the fixed-width version.
    // (Though for single-byte values, the fixed-width integer is never worse.)
    function decodeCompactU32(uint8[] memory input)
        external
        pure
        returns (uint32 value)
    {
        require(input.length > 0, "Input array must not be empty.");
        uint8 prefix = input[0];
        uint32 result;
        if (prefix % 4 == 0) {
            result = uint32(prefix) >> 2;
        } else if (prefix % 4 == 1) {
            result = uint32(decodeU16(input) >> 2);
        } else if (prefix % 4 == 2) {
            result = uint32(decodeU32(input) >> 2);
        } else if (prefix % 4 == 3) {
            result = uint32(decodeU32(input));
            require(result > (4294967295 >> 2), "U32 out of range.");
        } else {
            revert("U32 out of range.");
        }
        return result;
    }

    // Basic integers are encoded using a fixed-width little-endian (LE) format.
    function decodeU16(uint8[] memory input) public pure returns (uint16) {
        // Check that the input is the correct length for a U16
        require(input.length == 2, "Input length must be 2 for U16 decoding");
        return (uint16(input[1]) << 8) | uint16(input[0]);
    }

    function decodeU32(uint8[] memory input) public pure returns (uint32) {
        // Check that the input is the correct length for a U32
        require(input.length == 4, "Input length must be 4 for U32 decoding");
        return
            (uint32(input[3]) << 24) |
            (uint32(input[2]) << 16) |
            (uint32(input[1]) << 8) |
            uint32(input[0]);
    }
}
