// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./HashRefDB.sol";
import "./LookUp.sol";
import "./NibbleSlice.sol";
import "./Codec.sol";

contract Trie is ITrie {
    LookUp lookUp;
    NibbleSlice nibbleKey;
    Codec codec;

    constructor(
        address lookUpAddress,
        address nibbleKeyAddress,
        address codecAddress
    ) {
        lookUp = LookUp(lookUpAddress);
        nibbleKey = NibbleSlice(nibbleKeyAddress);
        codec = Codec(codecAddress);
    }

    function getWith(
        HashRefDB db,
        bytes32 root,
        TrieLayout calldata layout,
        bytes calldata key,
        Query query
    ) external {
        nibbleKey.setData(key);
        lookUp.setTrieInfo(db, root, query, layout, codec);
        lookUp.lookUpWithoutCache(key, nibbleKey);
    }
}
