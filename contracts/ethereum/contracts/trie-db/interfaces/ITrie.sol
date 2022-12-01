// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

interface ITrie {
    enum NodeCodec {
        ParityScaleCodec
    }

    enum Hasher {
        BLAKE2B,
        KECCAK
    }

    struct TrieLayout {
        bool USE_EXTENSION;
        bool ALLOW_EMPTY;
        bool MAX_INLINE_VALUE;
        Hasher Hash;
        NodeCodec Codec;
    }

    enum NodeType {
        Empty,
        Leaf,
        Extension,
        Branch,
        NibbledBranch
    }

    enum ValueType {
        Inline,
        Node
    }

    struct Value {
        ValueType valueType;
        bytes data;
    }

    enum NodeHandleType {
        Hash,
        Inline
    }

    struct NodeHandle {
        NodeHandleType nodeHandleType;
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
}
