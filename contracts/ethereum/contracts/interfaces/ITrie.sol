// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

interface ITrie {
    enum NodeType {
        Empty,
        Leaf,
        Extension,
        Branch,
        NibbledBranch
    }

    struct Value {
        bool isInline;
        bytes data;
    }

    struct Slice {
        uint8[] data;
        uint8 offset;
    }

    struct LookUpStruct {
        NodeHandle nextNode;
        Value value;
        NodeHandle item;
        NodeHandle[] children;
        bytes nodeData;
        Slice slice;
    }

    struct NodeHandle {
        bytes key;
        bool isHash;
        bytes value;
    }

    struct NodeStruct {
        NodeHandle[] children;
        bytes value;
        NodeType nodeType;
    }

    struct DB {
        bytes key;
        bytes value;
    }
}
