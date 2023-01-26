// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ICodec.sol";

contract NodeHeader is ICodec {
    uint256 constant FIRST_PREFIX = 0x00 << 6;
    uint256 constant LEAF_PREFIX_MASK = 0x01 << 6;
    uint256 constant BRANCH_WITH_MASK = 0x11 << 6;
    uint256 constant BRANCH_WITHOUT_MASK = 0x10 << 6;
    uint256 constant EMPTY_TRIE = FIRST_PREFIX | (0x00 << 4);
    uint256 constant ALT_HASHING_LEAF_PREFIX_MASK = FIRST_PREFIX | (0x1 << 5);
    uint256 constant ALT_HASHING_BRANCH_WITH_MASK = FIRST_PREFIX | (0x01 << 4);

    function decode(ByteSlice memory input)
        external
        pure
        returns (NodeHeaderStruct memory)
    {
        uint8 prefix;
        (prefix, input) = _readByte(input);
        if (prefix == EMPTY_TRIE) {
            return NodeHeaderStruct(NodeHeaderType.Null, false, 0);
        }

        uint256 prefixMask = (0x11 << 6);

        if ((prefix & prefixMask) == LEAF_PREFIX_MASK) {
            return
                NodeHeaderStruct(
                    NodeHeaderType.Leaf,
                    false,
                    decodeSize(prefix, input, 2)
                );
        } else if ((prefix & prefixMask) == BRANCH_WITH_MASK) {
            return
                NodeHeaderStruct(
                    NodeHeaderType.Branch,
                    true,
                    decodeSize(prefix, input, 2)
                );
        } else if ((prefix & prefixMask) == BRANCH_WITHOUT_MASK) {
            return
                NodeHeaderStruct(
                    NodeHeaderType.Branch,
                    false,
                    decodeSize(prefix, input, 2)
                );
        } else {
            prefixMask = (0x111 << 5);
            if ((prefix & prefixMask) == ALT_HASHING_LEAF_PREFIX_MASK) {
                return
                    NodeHeaderStruct(
                        NodeHeaderType.HashedValueLeaf,
                        false,
                        decodeSize(prefix, input, 3)
                    );
            }
            prefixMask = (0x1111 << 4);
            if ((prefix & prefixMask) == ALT_HASHING_BRANCH_WITH_MASK) {
                return
                    NodeHeaderStruct(
                        NodeHeaderType.HashedValueBranch,
                        false,
                        decodeSize(prefix, input, 4)
                    );
            }
            // do not allow any special encoding
            revert("Unsupported encoding");
        }
    }

    function decodeSize(
        uint256 first,
        ByteSlice memory input,
        uint256 prefixMask
    ) internal pure returns (uint256 result) {
        uint256 maxValue = 255 >> prefixMask;
        result = first & maxValue;
        if (result < maxValue) {
            return result;
        }
        result -= 1;
        while (true) {
            uint256 n;
            (n, input) = _readByte(input);
            if (n < 255) {
                return result + n + 1;
            }
            result += 255;
        }
    }

    function _readByte(ByteSlice memory input)
        internal
        pure
        returns (uint8, ByteSlice memory)
    {
        require(input.offset + 1 <= input.data.length, "out of data");
        uint8 byteAtOffset = input.data[input.offset];
        input.offset += 1;
        return (byteAtOffset, input);
    }
}
