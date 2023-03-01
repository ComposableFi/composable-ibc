// SPDX-License-Identifier: Apache2
pragma solidity ^0.8.17;

struct Slice {
    bytes data;
    uint256 offset;
}

library NibbleSlice {
    // Define a constant for the number of bits per nibble
    uint256 constant BITS_PER_NIBBLE = 4;
    // Define a constant for the number of nibbles per byte
    uint256 public constant NIBBLE_PER_BYTE = 2;

    /**
     * @dev Function calculates the new offset for the slice data by adding the given offset to the current offset
     *
     * @param self The data for the slice.
     * @param start The starting position of the slice.
     * @return The updated slice.
     */
    function mid(Slice memory self, uint256 start)
        external
        pure
        returns (Slice memory)
    {
        // Check if the start position is within the bounds of the slice data
        require(start <= len(self), "Start position exceeds slice length");
        // Return a new slice with the updated offset
        return Slice(self.data, self.offset + start);
    }

    /**
     * @dev Function to get the length of the slice data.
     *
     * @return The length of the slice data in nibbles.
     */
    function len(Slice memory nibble) public pure returns (uint256) {
        // Return the length of the slice data in nibbles
        return nibble.data.length * NIBBLE_PER_BYTE - nibble.offset;
    }

    /**
     * @dev Function to check if the given slice is empty.
     *
     * @param slice The slice to check.
     * @return True if the slice is empty, false otherwise.
     */
    function isEmpty(Slice memory slice) public pure returns (bool) {
        // Check if the length of the slice data is 0
        return len(slice) == 0;
    }

    /**
     * @dev Function to check if the given partial slice starts with the given slice.
     *
     * @param self The partial slice to check.
     * @param other The slice to check if it is the prefix of the self slice.
     * @return True if the partial slice starts with the given slice, false otherwise.
     */
    function startWith(Slice memory self, Slice memory other)
        public
        pure
        returns (bool)
    {
        // If the other slice is longer than the self slice, they can't match
        if (len(other) > len(self)) {
            return false;
        }
        // Check if the common prefix between the two slices is equal to the length of the other slice
        return commonPrefix(self, other) == len(other);
    }

    /**
     * @dev function that gets the nibble value at a given index in a given memory slice in a byte array.
     * It first checks that the given index is within the bounds of the slice data.
     * It then calculates the index of the byte containing the nibble at the given index,
     * as well as the nibble's offset within that byte.
     * Finally, it returns the nibble value by applying a padding mask to the byte at the calculated index..
     *
     * @param s The slice to get the nibble value from.
     * @param i The index of the nibble to get.
     * @return The nibble value at the given index.
     */
    function at(Slice memory s, uint256 i) public pure returns (uint256) {
        // check if the given index is within the bounds of the slice data
        require(i < len(s), "Index out of bounds");
        // Calculate the index of the byte containing the nibble at position `i`.
        uint256 ix = (s.offset + i) / NIBBLE_PER_BYTE;
        // Calculate the nibble offset within the byte.
        uint256 pad = (s.offset + i) % NIBBLE_PER_BYTE;
        uint8 data = uint8(s.data[ix]);
        // Return the nibble value by applying the padding mask to the byte at index `ix`.

        return (pad == 1) ? data & 0x0F : data >> BITS_PER_NIBBLE;
    }

    /**
     * @dev Function to calculate the biggest common depth between two left aligned packed nibble slices.
     *
     * @param v1 The first slice to compare.
     * @param v2 The second slice to compare.
     * @return The biggest common depth between the two given slices.
     */
    function biggestDepth(bytes memory v1, bytes memory v2)
        public
        pure
        returns (uint256)
    {
        uint256 upperBound = min(v1.length, v2.length);
        uint256 a = 0;
        while (a < upperBound) {
            if (v1[a] != v2[a]) {
                return a * NIBBLE_PER_BYTE + leftCommon(v1[a], v2[a]);
            }
        }
        return a * NIBBLE_PER_BYTE;
    }

    /**
     * @dev Function to get the minimum of two numbers.
     *
     * @param a The first number.
     * @param b The second number.
     * @return The minimum of the two given numbers.
     */
    function min(uint256 a, uint256 b) private pure returns (uint256) {
        return a < b ? a : b;
    }

    /**
     * @dev Function to calculate the number of common nibble between two left aligned bytes.
     */
    function leftCommon(bytes1 a, bytes1 b) internal pure returns (uint8) {
        if (a == b) {
            return 2;
        } else if (uint8(a) & 0xF0 == uint8(b) & 0xF0) {
            return 1;
        } else {
            return 0;
        }
    }

    /**
     * @dev This function calculates the longest common prefix of two memory slices in a byte array.
     * It does this by first checking the alignment of the slices and then either comparing the slices directly or
     * comparing the nibbles one by one. The function returns the length of the longest common prefix as a uint256 value.
     */
    function commonPrefix(Slice memory self, Slice memory other)
        public
        pure
        returns (uint256)
    {
        // Calculate the nibble alignment for each slice
        uint256 self_align = self.offset % NIBBLE_PER_BYTE;
        uint256 them_align = other.offset % NIBBLE_PER_BYTE;

        if (self_align == them_align) {
            // If the alignments are the same, we can compare the byte slices directly
            uint256 self_start = self.offset / NIBBLE_PER_BYTE;
            uint256 them_start = other.offset / NIBBLE_PER_BYTE;
            uint256 first = 0;

            if (self_align != 0) {
                if (
                    (self.data[self_start] & 0x0F) !=
                    (other.data[them_start] & 0x0F)
                ) {
                    return 0;
                }
                ++self_start;
                ++them_start;
                ++first;
            }
            bytes memory selfSlice = bytesSlice(self.data, self_start);
            bytes memory otherSlice = bytesSlice(other.data, them_start);
            return biggestDepth(selfSlice, otherSlice) + first;
        } else {
            uint256 s = min(len(self), len(other));
            uint256 i = 0;
            while (i < s) {
                if (at(self, i) != at(other, i)) {
                    break;
                }
                ++i;
            }
            return i;
        }
    }

    function bytesSlice(bytes memory _bytes, uint256 _start)
        private
        pure
        returns (bytes memory)
    {
        uint256 bytesLength = _bytes.length;
        uint256 _length = bytesLength - _start;
        require(bytesLength >= _start, "slice_outOfBounds");

        bytes memory tempBytes;

        assembly {
            switch iszero(_length)
            case 0 {
                tempBytes := mload(0x40) // load free memory pointer
                let lengthmod := and(_length, 31)

                let mc := add(
                    add(tempBytes, lengthmod),
                    mul(0x20, iszero(lengthmod))
                )
                let end := add(mc, _length)

                for {
                    let cc := add(
                        add(
                            add(_bytes, lengthmod),
                            mul(0x20, iszero(lengthmod))
                        ),
                        _start
                    )
                } lt(mc, end) {
                    mc := add(mc, 0x20)
                    cc := add(cc, 0x20)
                } {
                    mstore(mc, mload(cc))
                }

                mstore(tempBytes, _length)

                mstore(0x40, and(add(mc, 31), not(31)))
            }
            default {
                tempBytes := mload(0x40)
                mstore(tempBytes, 0)

                mstore(0x40, add(tempBytes, 0x20))
            }
        }
        return tempBytes;
    }

    function eq(Slice memory self, Slice memory other)
        public
        pure
        returns (bool)
    {
        return len(self) == len(other) && startWith(self, other);
    }
}
