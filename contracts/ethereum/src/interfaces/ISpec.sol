// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

interface ISpec {
    enum HasherType {
        BLAKE2B,
        KECCAK
    }

    struct Hasher {
        HasherType hasherType;
        uint8 hasherLength;
    }

    struct TrieLayout {
        bool USE_EXTENSION;
        bool ALLOW_EMPTY;
        bool MAX_INLINE_VALUE;
        Hasher Hash;
    }
}
