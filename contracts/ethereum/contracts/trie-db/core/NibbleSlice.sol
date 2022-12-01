// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";

contract NibbleSlice is ITrie {
    uint8 constant NIBBLE_PER_BYTE = 2;
    // Nibble (half a byte).
    uint8 constant PADDING_BITMASK = 0x0F;

    uint8 constant BIT_PER_NIBBLE = 4;

    function mid(Slice memory slice, uint8 start)
        external
        pure
        returns (Slice memory)
    {
        // check if the start position is within the bounds of the slice data
        require(start < len(slice), "Start position out of bounds");
        // calculate the new offset for the slice data
        slice.offset += start;
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

    function left(Slice memory s) public pure returns (Slice memory) {
        uint8 split = s.offset / NIBBLE_PER_BYTE;
        uint8 ix = s.offset % NIBBLE_PER_BYTE;
        uint8[] memory slicedArray = s.data;
        if (ix == 0) {
            for (uint8 i; i < split; i++) {
                slicedArray[i] = s.data[i];
            }
            s.offset = 0;
        } else {
            for (uint8 i; i < split; i++) {
                slicedArray[i] = s.data[i];
            }
            s.offset = padLeft(s.data[split]);
        }
        return Slice(slicedArray, s.offset);
    }

    function getPrefix() external returns (Slice memory) {}

    function getSlice() external returns (NibbleSlice) {}

    function originalDataAsPrefix(Slice memory slice)
        external
        returns (Slice memory)
    {}

    function startWith(Slice memory partialSlice, Slice memory slice)
        external
        pure
        returns (bool)
    {
        return commonPrefix(partialSlice, slice) == slice.data.length;
    }

    function isEmpty() external returns (bool) {}

    function at(Slice memory s, uint256 i) public pure returns (uint8) {
        // check if the given index is within the bounds of the slice data
        require(i < len(s), "Index out of bounds");
        // Calculate the index of the byte containing the nibble at position `i`.
        uint256 ix = (s.offset + i) / NIBBLE_PER_BYTE;
        // Calculate the nibble offset within the byte.
        uint256 pad = (s.offset + i) % NIBBLE_PER_BYTE;
        // Return the nibble value by applying the padding mask to the byte at index `ix`.

        if (pad == 1) {
            return s.data[ix] & PADDING_BITMASK;
        } else {
            return s.data[ix] & (PADDING_BITMASK << BIT_PER_NIBBLE);
        }
    }

    // Mask a byte, keeping left nibble.
    function padLeft(uint8 b) public pure returns (uint8) {
        return b & ~PADDING_BITMASK;
    }

    // Mask a byte, keeping right byte.
    function padRight(uint256 b) public pure returns (uint256) {
        return b & PADDING_BITMASK;
    }

    // Count the biggest common depth between two left aligned packed nibble slice.
    function biggestDepth(uint256[] memory v1, uint256[] memory v2)
        public
        pure
        returns (uint256)
    {
        uint256 upperBound = min(v1.length, v2.length);
        for (uint256 a = 0; a < upperBound; a++) {
            if (v1[a] != v2[a]) {
                return a * NIBBLE_PER_BYTE + leftCommon(v1[a], v2[a]);
            }
        }
        return upperBound * NIBBLE_PER_BYTE;
    }

    function min(uint256 a, uint256 b) public pure returns (uint256) {
        return a < b ? a : b;
    }

    function leftCommon(uint256 a, uint256 b) public pure returns (uint8) {
        if (a == b) {
            return 2;
        } else if (a & 0xF0 == b & 0xF0) {
            return 1;
        } else {
            return 0;
        }
    }

    function commonPrefix(Slice memory slice1, Slice memory slice2)
        public
        pure
        returns (uint256)
    {
        // Calculate the nibble alignment for each slice
        uint256 self_align = slice1.offset % NIBBLE_PER_BYTE;
        uint256 them_align = slice2.offset % NIBBLE_PER_BYTE;

        if (self_align == them_align) {
            // If the alignments are the same, we can compare the byte slices directly
            uint256 self_start = slice1.offset / NIBBLE_PER_BYTE;
            uint256 them_start = slice2.offset / NIBBLE_PER_BYTE;
            uint256[] memory array1;
            for (uint256 i = self_start; i < slice1.data.length; i++) {
                array1[i] = slice1.data[i];
            }
            uint256[] memory array2;
            for (uint256 i = them_start; i < slice2.data.length; i++) {
                array2[i] = slice2.data[i];
            }
            uint256 first = 0;

            if (self_align != 0) {
                // If the alignments are not 0, we need to compare the first nibble
                // separately and adjust the start indices accordingly
                if (
                    padRight(slice1.data[self_start]) !=
                    padRight(slice2.data[them_start])
                ) {
                    return 0;
                }
                self_start += 1;
                them_start += 1;
                first += 1;
            }

            // Calculate the biggest depth between the two byte slices
            return biggestDepth(array1, array2) + first;
        } else {
            // If the alignments are different, we need to compare the nibbles one by one
            uint256 s = min(slice1.data.length, slice2.data.length);
            uint256 i = 0;

            while (i < s) {
                if (at(slice1, i) != at(slice2, i)) {
                    break;
                }
                i += 1;
            }

            return i;
        }
    }
}
