// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./HashRefDB.sol";
import "./LookUp.sol";
import "./NibbleSlice.sol";

contract Trie is ITrie {
    function getWith(
        HashRefDB db,
        bytes32 root,
        TrieLayout calldata layout,
        bytes32 key,
        Query query
    ) external {
        NibbleSlice nibbleKey = new NibbleSlice(key);
        LookUp lookUp = new LookUp(db, root, query, layout);
        lookUp.lookUpWithoutCache(key, nibbleKey);
    }
}
