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

    enum Node {
        Empty,
        Leaf,
        Extension,
        Branch,
        NibbledBranch
    }

    struct NodeStruct {
        Node nodeType;
        bytes32 info;
    }
}
