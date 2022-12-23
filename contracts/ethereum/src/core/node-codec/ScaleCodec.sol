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
        uint8 prefix = input[0]; // todo: readByte here
        if (prefix % 4 == 0) {
            value = uint32(prefix) >> 2;
        } else if (prefix % 4 == 1) {
            uint16 x = decodeU16(input) >> 2;
            if (x > 0x0011_1111 && x <= 0x0011_1111_1111_1111) {
                value = uint32(x);
            } else {
                // handle error: U32_OUT_OF_RANGE
            }
        } else if (prefix % 4 == 2) {
            uint32 x = decodeU32(input) >> 2;
            if (x > 0x0011_1111_1111_1111 && x <= uint32(0xffffffff) >> 2) {
                value = x;
            } else {
                // handle error: U32_OUT_OF_RANGE
            }
        } else {
            if (prefix >> 2 == 0) {
                uint32 x = decodeU32(input);
                if (x > uint32(0xffffffff) >> 2) {
                    value = x;
                } else {
                    // handle error: U32_OUT_OF_RANGE
                }
            } else {
                // handle error: U32_OUT_OF_RANGE
            }
        }
        return value;
    }

    // Basic integers are encoded using a fixed-width little-endian (LE) format.
    function decodeU16(uint8[] memory input) public pure returns (uint16) {
        // todo: implementation
    }

    function decodeU32(uint8[] memory input) public pure returns (uint32) {
        // todo: implementation
    }

    function readByte(uint8[] memory input)
        external
        pure
        returns (uint8[] memory)
    {
        uint8[] memory buffer;
        buffer[0] = 0;
        return read(input, buffer);
    }

    function read(uint8[] memory input, uint8[] memory into)
        internal
        pure
        returns (uint8[] memory)
    {
        require(into.length <= input.length, "Not enough data to fill buffer");
    }
}
