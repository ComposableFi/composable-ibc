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

    enum NodePlanType {
        Empty,
        Leaf,
        Extension,
        Branch,
        NibbledBranch
    }

    struct NodeHeaderStruct {
        NodeHeaderType headerType;
        bool hasValue;
        uint256 value;
    }

    struct NodePlanStruct {
        NodePlanType planType;
    }

    struct ByteSlice {
        uint8[] data;
        uint8 offset;
    }

    struct CodecStruct {
        bool padding;
    }
}
