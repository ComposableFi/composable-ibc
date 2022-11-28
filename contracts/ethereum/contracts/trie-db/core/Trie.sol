// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./HashRefDB.sol";
import "./Codec.sol";

contract Trie is ITrie {
    struct TrieDB {
        HashRefDB db;
        bytes32 root;
    }

    function getWith(
        TrieDB calldata trie,
        TrieLayout calldata layout,
        bytes32 key,
        Query query
    ) external returns (bytes32) {
        NibbleSlice nibbleKey = new NibbleSlice(key);
        return lookUp(key, nibbleKey, trie, query, layout);
    }

    function lookUp(
        bytes32 key,
        NibbleSlice nibbleKey,
        TrieDB calldata trie,
        Query query,
        TrieLayout calldata layout
    ) internal returns (bytes32) {
        uint256 keyNibbles = 0;
        while (true) {
            nibbleKey.mid(keyNibbles);
            nibbleKey.left();
            Codec codec = new Codec(layout.Codec);
            bytes32[] memory nodeData = trie.db.get(
                key,
                nibbleKey.getPrefix(),
                layout.Hash
            );
            for (uint256 nData; nData < nodeData.length; ++nData) {
                NodeStruct memory decoded = codec.decode(nodeData[nData]);
                if (decoded.nodeType == Node.Leaf) {
                    loadValue();
                } else if (decoded.nodeType == Node.Extension) {
                    break;
                } else if (decoded.nodeType == Node.Branch) {
                    loadValue();
                } else if (decoded.nodeType == Node.NibbledBranch) {
                    loadValue();
                } else if (decoded.nodeType == Node.Empty) {
                    break;
                }
            }
        }
    }

    function loadValue() internal returns (bytes32) {}
}
