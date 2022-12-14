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

    struct NodeHandle {
        bytes key;
        bool isHash;
        bytes value;
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
        bytes value;
    }
}
