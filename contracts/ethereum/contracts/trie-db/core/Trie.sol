// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./HashRefDB.sol";
import "./LookUp.sol";
import "./NibbleSlice.sol";
import "./Codec.sol";

contract Trie is ITrie {
    LookUp lookUp;
    NibbleSlice nibbleSlice;
    Codec codec;
    HashRefDB hashDb;

    struct TrieDB {
        HashRefDB db;
        bytes32 root;
        Query query;
        TrieLayout layout;
        Codec codec;
        NibbleSlice nibbleSlice;
    }

    constructor(
        address lookUpAddress,
        address nibbleSliceAddress,
        address codecAddress,
        address hashDbAddress
    ) {
        lookUp = LookUp(lookUpAddress);
        nibbleSlice = NibbleSlice(nibbleSliceAddress);
        codec = Codec(codecAddress);
        hashDb = HashRefDB(hashDbAddress);
    }

    function getWith(
        bytes32 root,
        TrieLayout calldata layout,
        uint8[] calldata key,
        Query query
    ) external {
        lookUp.lookUpWithoutCache(
            TrieDB(hashDb, root, query, layout, codec, nibbleSlice),
            key
        );
    }
}
