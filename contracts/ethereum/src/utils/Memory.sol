pragma solidity ^0.8.17;

// SPDX-License-Identifier: Apache2

library Memory {
    uint256 internal constant WORD_SIZE = 32;

    /**
     * @dev Compares the 'len' bytes starting at address 'addr' in memory with the 'len'
     * bytes starting at 'addr2'.
     * @param addr The memory address to start comparison from.
     * @param addr2 The memory address to compare against.
     * @param len The number of bytes to compare.
     * @return equal True if the bytes are the same, otherwise false.
     */
    function equals(
        uint256 addr,
        uint256 addr2,
        uint256 len
    ) internal pure returns (bool equal) {
        assembly {
            equal := eq(keccak256(addr, len), keccak256(addr2, len))
        }
    }

    /**
     * @dev Compares the 'len' bytes starting at address 'addr' in memory with the bytes stored in
     * 'bts'. It is allowed to set 'len' to a lower value than 'bts.length', in which case only
     * the first 'len' bytes will be compared.
     * @param addr The memory address to start comparison from.
     * @param len The number of bytes to compare.
     * @param bts The bytes array to compare against.
     * @return equal True if the bytes are the same, otherwise false.
     */
    function equals(
        uint256 addr,
        uint256 len,
        bytes memory bts
    ) internal pure returns (bool equal) {
        require(
            bts.length >= len,
            "Memory: Length of bts must be greater than or equal to len."
        );
        uint256 addr2;
        assembly {
            addr2 := add(bts, 32) // 32 is the size of the bytes array header
        }
        return equals(addr, addr2, len);
    }

    /**
     * @dev Returns a memory pointer to the data portion of the provided bytes array.
     * @param bts The bytes array to get the data pointer for.
     * @return addr The memory pointer to the data portion of the provided bytes array.
     */
    function dataPtr(bytes memory bts) internal pure returns (uint256 addr) {
        assembly {
            addr := add(bts, 32) // 32 is the size of the bytes array header
        }
    }

    /**
     * @dev Creates a 'bytes memory' variable from the memory address 'addr', with the
     * length 'len'. The function will allocate new memory for the bytes array, and
     * the 'len bytes starting at 'addr' will be copied into that new memory.
     * @param addr The memory address to start copying from.
     * @param len The number of bytes to copy.
     * @return bts The newly created 'bytes memory'.
     */
    function toBytes(uint256 addr, uint256 len)
        internal
        pure
        returns (bytes memory bts)
    {
        bts = new bytes(len);
        uint256 btsptr;
        assembly {
            btsptr := add(bts, 32) // 32 is the size of the bytes array header
        }
        copy(addr, btsptr, len);
    }

    /**
     * @dev Copies 'self' into a new 'bytes memory'.
     * @param self The bytes32 value to copy.
     * @return bts The newly created 'bytes memory'.
     */
    function toBytes(bytes32 self) internal pure returns (bytes memory bts) {
        bts = new bytes(32);
        assembly {
            mstore(
                add(
                    bts,
                    /*BYTES_HEADER_SIZE*/
                    32
                ),
                self
            )
        }
    }

    // Copy 'len' bytes from memory address 'src', to address 'dest'.
    // This function does not check the or destination, it only copies
    // the bytes.
    function copy(
        uint256 src,
        uint256 dest,
        uint256 len
    ) internal pure {
        // Copy word-length chunks while possible
        for (; len >= WORD_SIZE; len -= WORD_SIZE) {
            assembly {
                mstore(dest, mload(src))
            }
            dest += WORD_SIZE;
            src += WORD_SIZE;
        }

        // Copy remaining bytes
        uint256 mask = len == 0
            ? 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
            : 256**(WORD_SIZE - len) - 1;
        assembly {
            let srcpart := and(mload(src), not(mask))
            let destpart := and(mload(dest), mask)
            mstore(dest, or(destpart, srcpart))
        }
    }

    // This function does the same as 'dataPtr(bytes memory)', but will also return the
    // length of the provided bytes array.
    function fromBytes(bytes memory bts)
        internal
        pure
        returns (uint256 addr, uint256 len)
    {
        len = bts.length;
        assembly {
            addr := add(
                bts,
                /*BYTES_HEADER_SIZE*/
                32
            )
        }
    }
}
