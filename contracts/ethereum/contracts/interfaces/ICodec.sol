// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

interface ICodec {
    enum NodeHeaderType {
        Null,
        Branch,
        Leaf,
        HashedValueBranch,
        HashedValueLeaf
    }

    struct NodeHeaderStruct {
        NodeHeaderType headerType;
        bool isValue;
        uint256 value;
    }

    struct ByteSlice {
        uint8[] data;
        uint8 offset;
    }
}
