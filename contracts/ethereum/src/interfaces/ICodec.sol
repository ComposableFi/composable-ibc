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
        SlicePlan slicePlan;
        ValuePlan valuePlan;
        NodeHandlePlan[] children;
        NodeHandlePlan child;
    }

    struct ByteSlice {
        uint8[] data;
        uint256 offset;
    }

    struct CodecStruct {
        bool padding;
        uint256 partialRangeStart;
        uint256 partialRangeEnd;
        uint256 partialPadding;
        uint256 bitmapRangeStart;
        uint256 bitmapRangeEnd;
        uint16 bitmapValue;
        ValuePlan valuePlan;
        NodeHandlePlan[] children;
        NodeHandlePlan child;
    }

    struct ValuePlan {
        bool isInline;
        uint256 start;
        uint256 end;
    }

    struct NodeHandlePlan {
        bool isHash;
        uint256 start;
        uint256 end;
    }

    struct SlicePlan {
        uint256 dataStart;
        uint256 dataEnd;
        uint256 offset;
    }
}
