// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ITrie.sol";
import "../../utils/NibbleOps.sol";

contract NibbleSlice is ITrie {
    uint8 constant BIT_PER_NIBBLE = 4;

    NibbleOps nibbleOps;

    constructor(address nibbleOpsAddress) {
        nibbleOps = NibbleOps(nibbleOpsAddress);
    }

    /**
     * @dev Function calculates the new offset for the slice data by adding the given offset to the current offset
     *
     * @param slice The data for the slice.
     * @param start The starting position of the slice.
     * @return The updated slice.
     */
    function mid(Slice memory slice, uint8 start)
        external
        view
        returns (Slice memory)
    {
        // check if the start position is within the bounds of the slice data
        require(start < len(slice), "Start position out of bounds");
        // calculate the new offset for the slice data
        slice.offset += start;
        return slice;
    }

    /**
     * @dev Function to get the length of the slice data.
     *
     * @return The length of the slice data in nibbles.
     */
    function len(Slice memory slice) public view returns (uint8) {
        // return the length of the slice data in nibbles
        return
            uint8(
                slice.data.length * nibbleOps.NIBBLE_PER_BYTE() - slice.offset
            );
    }

    /**
     * @dev Function to check if the given slice is empty.
     *
     * @param slice The slice to check.
     * @return True if the slice is empty, false otherwise.
     */
    function isEmpty(Slice memory slice) public view returns (bool) {
        // check if the length of the slice data is 0
        return len(slice) == 0;
    }

    /**
     * @dev Function to get the left nibble of each bytes from a given slice.
     *
     * @param s The slice to check.
     * @return The slice with the left nibble of each byte.
     */
    function left(Slice memory s) public view returns (Slice memory) {
        uint256 split = s.offset / nibbleOps.NIBBLE_PER_BYTE();
        uint256 ix = s.offset % nibbleOps.NIBBLE_PER_BYTE();
        uint8[] memory slicedArray = s.data;
        for (uint8 i; i < split; i++) {
            slicedArray[i] = s.data[i];
        }
        if (ix == 0) {
            s.offset = 0;
        } else {
            s.offset = nibbleOps.padLeft(s.data[split]);
        }
        return Slice(slicedArray, s.offset);
    }

    function originalDataAsPrefix(Slice memory slice)
        external
        pure
        returns (Slice memory)
    {
        // TODO: check if this makes sense
        return Slice(slice.data, 0);
    }

    /**
     * @dev Function to check if the given partial slice starts with the given slice.
     *
     * @param partialSlice The partial slice to check.
     * @param slice The slice to check if it is the prefix of the partial slice.
     * @return True if the partial slice starts with the given slice, false otherwise.
     */
    function startWith(Slice memory partialSlice, Slice memory slice)
        external
        view
        returns (bool)
    {
        return commonPrefix(partialSlice, slice) == slice.data.length;
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
    function at(Slice memory s, uint256 i) public view returns (uint8) {
        // check if the given index is within the bounds of the slice data
        require(i < len(s), "Index out of bounds");
        // Calculate the index of the byte containing the nibble at position `i`.
        uint256 ix = (s.offset + i) / nibbleOps.NIBBLE_PER_BYTE();
        // Calculate the nibble offset within the byte.
        uint256 pad = (s.offset + i) % nibbleOps.NIBBLE_PER_BYTE();
        // Return the nibble value by applying the padding mask to the byte at index `ix`.

        if (pad == 0) {
            return
                (s.data[ix] &
                    (nibbleOps.PADDING_BITMASK() << BIT_PER_NIBBLE)) >>
                BIT_PER_NIBBLE;
        } else {
            return s.data[ix] & nibbleOps.PADDING_BITMASK();
        }
    }

    /**
     * @dev Function to calculate the biggest common depth between two left aligned packed nibble slices.
     *
     * @param v1 The first slice to compare.
     * @param v2 The second slice to compare.
     * @return The biggest common depth between the two given slices.
     */
    function biggestDepth(uint256[] memory v1, uint256[] memory v2)
        public
        view
        returns (uint256)
    {
        uint256 upperBound = min(v1.length, v2.length);
        for (uint256 a = 0; a < upperBound; a++) {
            if (v1[a] != v2[a]) {
                return
                    a * nibbleOps.NIBBLE_PER_BYTE() + leftCommon(v1[a], v2[a]);
            }
        }
        return upperBound * nibbleOps.NIBBLE_PER_BYTE();
    }

    /**
     * @dev Function to get the minimum of two numbers.
     *
     * @param a The first number.
     * @param b The second number.
     * @return The minimum of the two given numbers.
     */
    function min(uint256 a, uint256 b) public pure returns (uint256) {
        return a < b ? a : b;
    }

    /**
     * @dev Function to calculate the number of common nibble between two left aligned bytes.
     */
    function leftCommon(uint256 a, uint256 b) internal pure returns (uint8) {
        if (a == b) {
            return 2;
        } else if (a & 0xF0 == b & 0xF0) {
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
    function commonPrefix(Slice memory slice1, Slice memory slice2)
        public
        view
        returns (uint256)
    {
        // Calculate the nibble alignment for each slice
        uint256 self_align = slice1.offset % nibbleOps.NIBBLE_PER_BYTE();
        uint256 them_align = slice2.offset % nibbleOps.NIBBLE_PER_BYTE();

        if (self_align == them_align) {
            // If the alignments are the same, we can compare the byte slices directly
            uint256 self_start = slice1.offset / nibbleOps.NIBBLE_PER_BYTE();
            uint256 them_start = slice2.offset / nibbleOps.NIBBLE_PER_BYTE();
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
                    nibbleOps.padRight(slice1.data[self_start]) !=
                    nibbleOps.padRight(slice2.data[them_start])
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
