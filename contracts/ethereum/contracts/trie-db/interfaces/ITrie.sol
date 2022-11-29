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
        bytes32 data;
    }

    enum NodeHandleType {
        Hash,
        Inline
    }

    struct NodeHandle {
        NodeHandleType nodeHandleType;
        bytes32[] data;
    }
}
