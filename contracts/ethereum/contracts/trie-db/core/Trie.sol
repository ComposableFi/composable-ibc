// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./HashRefDB.sol";
import "./LookUp.sol";
import "./NibbleSlice.sol";

contract Trie is ITrie {
    LookUp lookUp;
    NibbleSlice nibbleKey;

    constructor(address lookUpAddress, address nibbleKeyAddress) {
        lookUp = LookUp(lookUpAddress);
        nibbleKey = NibbleSlice(nibbleKeyAddress);
    }

    function getWith(
        HashRefDB db,
        bytes32 root,
        TrieLayout calldata layout,
        bytes calldata key,
        Query query
    ) external {
        nibbleKey.setData(key);
        lookUp.setTrieInfo(db, root, query, layout);
        lookUp.lookUpWithoutCache(key, nibbleKey);
    }
}
