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
        uint8[] data;
    }

    struct Slice {
        uint8[] data;
        uint256 offset;
    }

    struct NodeHandle {
        bool isHash;
        uint8[] data;
    }

    struct Node {
        NodeType nodeType;
        Slice slice;
        Value value;
        NodeHandle[] children;
        NodeHandle child;
    }

    struct DB {
        bytes key;
        uint8[] value;
    }
}
