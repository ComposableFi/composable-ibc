// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "./Trie.sol";
import "./Codec.sol";
import "./Node.sol";
import "./NibbleSlice.sol";
import "./HashRefDB.sol";

contract LookUp is ITrie {
    struct TrieDB {
        HashRefDB db;
        bytes32 root;
    }

    TrieDB _trie;
    Query _query;
    TrieLayout _layout;

    constructor(
        HashRefDB db,
        bytes32 root,
        Query query,
        TrieLayout memory layout
    ) {
        _trie = TrieDB(db, root);
        _query = query;
        _layout = layout;
    }

    function lookUpWithoutCache(bytes32 key, NibbleSlice nibbleKey)
        external
        returns (bool)
    {
        uint256 keyNibbles = 0;
        NibbleSlice partialKey = nibbleKey;
        bytes32 hash = _trie.root;
        Codec codec = new Codec(_layout.Codec);

        NodeHandle memory nextNode;
        NibbleSlice slice;
        Value memory value;
        NodeHandle memory item;
        NodeHandle[] memory children;
        bytes32[] memory nodeData;
        Node decoded;
        NodeType nodeType;

        while (true) {
            nibbleKey.mid(keyNibbles);
            nibbleKey.left();

            nodeData = _trie.db.get(key, nibbleKey.getPrefix(), _layout.Hash);

            for (uint256 nData; nData < nodeData.length; ++nData) {
                decoded = codec.decode(nodeData[nData]);
                nodeType = decoded.getNodeType();

                if (nodeType == NodeType.Leaf) {
                    (slice, value) = decoded.Leaf();
                    if (slice.getSlice() == partialKey.getSlice()) {
                        nextNode = loadValue(
                            value,
                            nibbleKey.originalDataAsPrefix(),
                            key,
                            _trie.db,
                            _query
                        );
                    } else continue;
                } else if (nodeType == NodeType.Extension) {
                    (slice, item) = decoded.Extension();
                    if (partialKey.startWith(slice)) {
                        partialKey = partialKey.mid(slice.len());
                        keyNibbles += slice.len();
                        nextNode = item;
                    } else continue;
                } else if (nodeType == NodeType.Branch) {
                    (children, value) = decoded.Branch();
                    if (partialKey.isEmpty()) {
                        nextNode = loadValue(
                            value,
                            nibbleKey.originalDataAsPrefix(),
                            key,
                            _trie.db,
                            _query
                        );
                    } else {
                        partialKey = partialKey.mid(1);
                        ++keyNibbles;
                        nextNode = children[partialKey.at(0)];
                    }
                } else if (nodeType == NodeType.NibbledBranch) {
                    (slice, children, value) = decoded.NibbledBranch();
                    if (!partialKey.startWith(slice)) {
                        continue;
                    }
                    if (partialKey.len() == slice.len()) {
                        nextNode = loadValue(
                            value,
                            nibbleKey.originalDataAsPrefix(),
                            key,
                            _trie.db,
                            _query
                        );
                    } else {
                        partialKey = partialKey.mid(slice.len() + 1);
                        keyNibbles += slice.len() + 1;
                        nextNode = children[partialKey.at(slice.len())];
                    }
                } else if (nodeType == NodeType.Empty) {
                    continue;
                }

                if (nextNode.nodeHandleType == NodeHandleType.Hash) {
                    hash = decodeHash(_layout.Hash);
                    break;
                } else nodeData = nextNode.data;
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
