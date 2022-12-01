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
        Codec codec;
    }

    // TODO: not sure about this
    uint256 MAX_TRIE_DEPTH = 1000;

    // define a trieDB variable to store trie info
    TrieDB _trie;

    // define the function to set the trie information
    function setTrieInfo(
        HashRefDB db,
        bytes32 root,
        Query query,
        TrieLayout calldata layout,
        Codec codec
    ) external {
        _trie = TrieDB(db, root, query, layout, codec);
    }

    function lookUpWithoutCache(bytes calldata key, NibbleSlice nibbleKey)
        external
        returns (bool)
    {
        // keeps track of the number of nibbles in the key that have been traversed
        uint8 keyNibbles = 0;
        // keeps track of the remaining nibbles in the key to be looked up
        NibbleSlice partialKey = nibbleKey;

        NibbleSlice slice;
        Node decoded;
        LookUpStruct memory lookUp;

        while (keyNibbles < nibbleKey.len()) {
            nibbleKey.mid(keyNibbles);
            // get the data from the current node
            lookUp.nodeData = _trie.db.get(
                key,
                nibbleKey.left(),
                _trie.layout.Hash
            );

            // check if the data is not found in the database
            if (lookUp.nodeData.length == 0) {
                // if root node is not found, return an error
                require(keyNibbles == 0, "Invalid state root");
                revert("incomplete database");
            }

            uint256 nodeDataIdx = 0;

            while (nodeDataIdx < lookUp.nodeData.length) {
                // decode the node data from the codec instance
                decoded = _trie.codec.decode(
                    lookUp.nodeData[nodeDataIdx],
                    _trie.layout.Codec
                );

                if (decoded.getNodeType() == NodeType.Leaf) {
                    (slice, lookUp.value) = decoded.Leaf();
                    // check if the slice matches the partial key
                    if (slice.getSlice() == partialKey.getSlice()) {
                        // if the key is found, load the value and return
                        lookUp.nextNode = loadValue(
                            lookUp.value,
                            nibbleKey.originalDataAsPrefix(),
                            key
                        );
                        return true;
                    } else {
                        // if the slice does not match the partial key, move to the next inline node
                        break;
                    }
                } else if (decoded.getNodeType() == NodeType.Extension) {
                    (slice, lookUp.item) = decoded.Extension();
                    // check if the partial key to remove the traversed slice
                    if (partialKey.startWith(slice)) {
                        // update the partial key to remove the traversed slice
                        partialKey.mid(slice.len());
                        // update the key nibbles counter
                        keyNibbles += slice.len();
                        // set the next node to the item in the extension node
                        lookUp.nextNode = lookUp.item;
                    } else {
                        // if the partial key does not start with the slice, move to the next inline node
                        break;
                    }
                } else if (decoded.getNodeType() == NodeType.Branch) {
                    // get the children and value from the branch node
                    (lookUp.children, lookUp.value) = decoded.Branch();
                    // check if the partial key is empty
                    if (partialKey.isEmpty()) {
                        // if the partial key is empty, load the value from the branch node
                        lookUp.nextNode = loadValue(
                            lookUp.value,
                            nibbleKey.originalDataAsPrefix(),
                            key
                        );
                    } else {
                        // if the partial key is not empty, update the partial key to remove the first nibble
                        partialKey.mid(1);
                        // increment the key nibbles counter
                        ++keyNibbles;
                        // set the next Node to the child at the first nibble of the partial key
                        lookUp.nextNode = lookUp.children[partialKey.at(0)];
                    }
                } else if (decoded.getNodeType() == NodeType.NibbledBranch) {
                    (slice, lookUp.children, lookUp.value) = decoded
                        .NibbledBranch();
                    if (!partialKey.startWith(slice)) {
                        // if the partial key does not start with the slice, move to the next inline node
                        break;
                    }
                    if (partialKey.len() == slice.len()) {
                        // if the partial key has the same length as the slice,
                        // the value in the nibbled branch node is the value of the key
                        lookUp.nextNode = loadValue(
                            lookUp.value,
                            nibbleKey.originalDataAsPrefix(),
                            key
                        );
                    } else {
                        // if the partial key is longer than the slice,
                        // the next node is the child node at the index of the first nibble
                        // after the slice in the partial key
                        partialKey.mid(slice.len());
                        keyNibbles += slice.len();
                        lookUp.nextNode = lookUp.children[partialKey.at(0)];
                    }
                } else if (decoded.getNodeType() == NodeType.Empty) {
                    // if the node type is empty, the key is not in the trie
                    return false;
                }
                // if (lookUp.nextNode.nodeHandleType == NodeHandleType.Hash) {
                //     hash = decodeHash(_trie.layout.Hash, lookUp.nextNode.data);
                //     break;
                // } else lookUp.nodeData = lookUp.nextNode.data;
            }
            // if (partialKey.isEmpty()) {
            //     // end of the trie reached
            //     break;
            // }
        }
        return true;
    }

    function loadValue(
        Value memory value,
        Prefix memory prefix,
        bytes calldata key
    ) internal returns (NodeHandle memory) {
        // Check if the valueType is Inline or Node
        require(
            value.valueType == ValueType.Inline ||
                value.valueType == ValueType.Node,
            "Invalid valueType"
        );
        if (value.valueType == ValueType.Inline) {
            // if the value is inline, decode it and return the result
            return _trie.query.decode(_trie.layout.Hash, value.data);
        } else {
            // if the value is a node, get the hash value and lookup the value in the db
            bytes memory v = _trie.db.get(key, prefix, _trie.layout.Hash);
            // If a value is found, decode and return the result
            return _trie.query.decode(_trie.layout.Hash, v);
        }
    }

    function decodeHash(Hasher hasher, bytes memory data)
        internal
        returns (bytes32)
    {}
}
