// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./HashRefDB.sol";
import "./LookUp.sol";
import "./NibbleSlice.sol";
import "./Codec.sol";

contract Trie is ITrie {
    struct TrieDB {
        HashRefDB db;
        bytes32 root;
        Query query;
        TrieLayout layout;
        Codec codec;
        NibbleSlice nibbleSlice;
    }

    struct Addresses {
        address lookUpAddress;
        address nibbleSliceAddress;
        address codecAddress;
        address hashDbAddress;
    }

    function getWith(
        Addresses calldata addresses,
        bytes32 root,
        TrieLayout calldata layout,
        uint8[] calldata key,
        Query query
    ) external {
        LookUp lookUp = LookUp(addresses.lookUpAddress);
        lookUp.lookUpWithoutCache(
            TrieDB(
                HashRefDB(addresses.hashDbAddress),
                root,
                query,
                layout,
                Codec(addresses.codecAddress),
                NibbleSlice(addresses.nibbleSliceAddress)
            ),
            key
        );
    }
}
