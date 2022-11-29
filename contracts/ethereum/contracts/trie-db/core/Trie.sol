// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./HashRefDB.sol";
import "./Codec.sol";
import "./Node.sol";
import "./NibbleSlice.sol";

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
        NibbleSlice partialKey = nibbleKey;
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
                Node decoded = codec.decode(nodeData[nData]);
                NodeType nodeType = decoded.getNodeType();
                if (nodeType == NodeType.Leaf) {
                    (NibbleSlice slice, Value memory value) = decoded.Leaf();
                    if (slice.getSlice() == partialKey.getSlice()) {
                        return
                            loadValue(
                                value,
                                nibbleKey.originalDataAsPrefix(),
                                key,
                                trie,
                                query
                            );
                    } else continue;
                } else if (nodeType == NodeType.Extension) {
                    (NibbleSlice slice, NodeHandle memory item) = decoded
                        .Extension();
                    if (partialKey.startWith(slice)) {
                        partialKey = partialKey.mid(slice.len());
                        keyNibbles += slice.len();
                        // TODO: fix return type
                        return keccak256(abi.encode(item));
                    } else continue;
                } else if (nodeType == NodeType.Branch) {
                    (NodeHandle[] memory children, Value memory value) = decoded
                        .Branch();
                    if (partialKey.isEmpty()) {
                        return
                            loadValue(
                                value,
                                nibbleKey.originalDataAsPrefix(),
                                key,
                                trie,
                                query
                            );
                    } else {
                        NodeHandle memory x = children[partialKey.at(0)];
                        partialKey = partialKey.mid(1);
                        ++keyNibbles;
                        return keccak256(abi.encode(x));
                    }
                } else if (nodeType == NodeType.NibbledBranch) {
                    // loadValue();
                } else if (nodeType == NodeType.Empty) {
                    break;
                }
            }
        }
    }

    function loadValue(
        Value memory value,
        bytes32 prefix,
        bytes32 key,
        TrieDB calldata trie,
        Query query
    ) internal returns (bytes32) {}
}
