// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./Codec.sol";
import "./Node.sol";
import "./NibbleSlice.sol";
import "./HashRefDB.sol";

contract LookUp is ITrie {
    struct TrieDB {
        HashRefDB db;
        bytes32 root;
        Query query;
        TrieLayout layout;
    }

    TrieDB _trie;

    constructor(
        HashRefDB db,
        bytes32 root,
        Query query,
        TrieLayout memory layout
    ) {
        _trie = TrieDB(db, root, query, layout);
    }

    function lookUpWithoutCache(bytes32 key, NibbleSlice nibbleKey)
        external
        returns (bool)
    {
        uint256 keyNibbles = 0;
        NibbleSlice partialKey = nibbleKey;
        bytes32 hash = _trie.root;
        Codec codec = new Codec(_trie.layout.Codec);

        NibbleSlice slice;
        Node decoded;
        LookUpStruct memory lookUp;

        while (true) {
            nibbleKey.mid(keyNibbles);
            nibbleKey.left();

            lookUp.nodeData = _trie.db.get(
                key,
                nibbleKey.getPrefix(),
                _trie.layout.Hash
            );

            for (uint256 nData; nData < lookUp.nodeData.length; ++nData) {
                decoded = codec.decode(lookUp.nodeData[nData]);
                lookUp.nodeType = decoded.getNodeType();

                if (lookUp.nodeType == NodeType.Leaf) {
                    (slice, lookUp.value) = decoded.Leaf();
                    if (slice.getSlice() == partialKey.getSlice()) {
                        lookUp.nextNode = loadValue(
                            lookUp.value,
                            nibbleKey.originalDataAsPrefix(),
                            key,
                            _trie.db,
                            _trie.query
                        );
                    } else continue;
                } else if (lookUp.nodeType == NodeType.Extension) {
                    (slice, lookUp.item) = decoded.Extension();
                    if (partialKey.startWith(slice)) {
                        partialKey = partialKey.mid(slice.len());
                        keyNibbles += slice.len();
                        lookUp.nextNode = lookUp.item;
                    } else continue;
                } else if (lookUp.nodeType == NodeType.Branch) {
                    (lookUp.children, lookUp.value) = decoded.Branch();
                    if (partialKey.isEmpty()) {
                        lookUp.nextNode = loadValue(
                            lookUp.value,
                            nibbleKey.originalDataAsPrefix(),
                            key,
                            _trie.db,
                            _trie.query
                        );
                    } else {
                        partialKey = partialKey.mid(1);
                        ++keyNibbles;
                        lookUp.nextNode = lookUp.children[partialKey.at(0)];
                    }
                } else if (lookUp.nodeType == NodeType.NibbledBranch) {
                    (slice, lookUp.children, lookUp.value) = decoded
                        .NibbledBranch();
                    if (!partialKey.startWith(slice)) {
                        continue;
                    }
                    if (partialKey.len() == slice.len()) {
                        lookUp.nextNode = loadValue(
                            lookUp.value,
                            nibbleKey.originalDataAsPrefix(),
                            key,
                            _trie.db,
                            _trie.query
                        );
                    } else {
                        partialKey = partialKey.mid(slice.len() + 1);
                        keyNibbles += slice.len() + 1;
                        lookUp.nextNode = lookUp.children[
                            partialKey.at(slice.len())
                        ];
                    }
                } else if (lookUp.nodeType == NodeType.Empty) {
                    continue;
                }

                if (lookUp.nextNode.nodeHandleType == NodeHandleType.Hash) {
                    hash = decodeHash(_trie.layout.Hash);
                    break;
                } else lookUp.nodeData = lookUp.nextNode.data;
            }
        }
        return true;
    }

    function loadValue(
        Value memory value,
        bytes32 prefix,
        bytes32 key,
        HashRefDB db,
        Query query
    ) internal returns (NodeHandle memory) {}

    function decodeHash(Hasher hasher) internal returns (bytes32) {}
}
