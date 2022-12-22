// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

contract NibbleOps {
    // Nibble (half a byte).
    uint8 public constant PADDING_BITMASK = 0x0F;
    uint8 public constant NIBBLE_PER_BYTE = 2;
    uint8 public constant NIBBLE_LENGTH = 16;

    /**
     * @dev Function to mask the given byte, keeping the left nibble.
     *
     * @param b The byte to mask.
     * @return The masked byte with the left nibble.
     */
    function padLeft(uint8 b) public pure returns (uint8) {
        return b & ~PADDING_BITMASK;
    }

    /**
     * @dev Function to mask the given byte, keeping the right nibble.
     *
     * @param b The byte to mask.
     * @return The masked byte with the right nibble.
     */
    function padRight(uint8 b) public pure returns (uint8) {
        return b & PADDING_BITMASK;
    }

    // Calculate the number of needed padding a array of nibble length `i`.
    function numberPadding(uint256 b) public pure returns (uint256) {
        return b % NIBBLE_PER_BYTE;
    }
}
