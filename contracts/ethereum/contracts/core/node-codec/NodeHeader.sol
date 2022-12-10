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

    function decode(bytes memory input)
        external
        returns (NodeHeaderStruct memory)
    {
        uint8 i = _readByte(input);
        if (i == EMPTY_TRIE) {
            return NodeHeaderStruct(NodeHeaderType.Null, false, 0);
        }

        uint256 mask = (0x11 << 6);

        if ((i & mask) == LEAF_PREFIX_MASK) {
            return
                NodeHeaderStruct(
                    NodeHeaderType.Leaf,
                    false,
                    decodeSize(i, input, 2)
                );
        } else if ((i & mask) == BRANCH_WITH_MASK) {
            return
                NodeHeaderStruct(
                    NodeHeaderType.Branch,
                    true,
                    decodeSize(i, input, 2)
                );
        } else if ((i & mask) == BRANCH_WITHOUT_MASK) {
            return
                NodeHeaderStruct(
                    NodeHeaderType.Branch,
                    false,
                    decodeSize(i, input, 2)
                );
        } else {
            mask = (0x111 << 5);
            if ((i & mask) == ALT_HASHING_LEAF_PREFIX_MASK) {
                return
                    NodeHeaderStruct(
                        NodeHeaderType.HashedValueLeaf,
                        false,
                        decodeSize(i, input, 3)
                    );
            }
            mask = (0x1111 << 4);
            if ((i & mask) == ALT_HASHING_BRANCH_WITH_MASK) {
                return
                    NodeHeaderStruct(
                        NodeHeaderType.HashedValueBranch,
                        false,
                        decodeSize(i, input, 4)
                    );
            }
            // do not allow any special encoding
            revert("Unsupported encoding");
        }
    }

    function decodeSize(
        uint256 _first,
        bytes memory input,
        uint256 _prefixMask
    ) internal returns (uint256) {}

    function _readByte(bytes memory input) internal returns (uint8) {}
}
