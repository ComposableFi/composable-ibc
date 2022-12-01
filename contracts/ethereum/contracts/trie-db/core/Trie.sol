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

    constructor(
        address lookUpAddress,
        address nibbleSliceAddress,
        address codecAddress
    ) {
        lookUp = LookUp(lookUpAddress);
        nibbleSlice = NibbleSlice(nibbleSliceAddress);
        codec = Codec(codecAddress);
    }

    function getWith(
        HashRefDB db,
        bytes32 root,
        TrieLayout calldata layout,
        bytes calldata key,
        Query query
    ) external {
        lookUp.setTrieInfo(db, root, query, layout, codec, nibbleSlice);
        lookUp.lookUpWithoutCache(key);
    }
}
