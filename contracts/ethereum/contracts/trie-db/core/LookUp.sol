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
        NibbleSlice nibbleSlice;
    }

    // define a trieDB variable to store trie info
    TrieDB _trie;

    // define the function to set the trie information
    function setTrieInfo(
        HashRefDB db,
        bytes32 root,
        Query query,
        TrieLayout calldata layout,
        Codec codec,
        NibbleSlice nibbleSlice
    ) external {
        _trie = TrieDB(db, root, query, layout, codec, nibbleSlice);
    }

    function lookUpWithoutCache(bytes calldata key) external returns (bool) {
        // keeps track of the number of nibbles in the key that have been traversed
        uint8 keyNibbles = 0;
        // keeps track of the remaining nibbles in the key to be looked up
        Slice memory nibbleKey = Slice(key, 0);
        Slice memory partialKey = nibbleKey;

        Node decoded;
        LookUpStruct memory lookUp;

        // TODO: verify if this makes sense
        while (keyNibbles < _trie.nibbleSlice.len(nibbleKey)) {
            nibbleKey = _trie.nibbleSlice.mid(nibbleKey, keyNibbles);
            // get the data from the current node
            lookUp.nodeData = _trie.db.get(
                key,
                _trie.nibbleSlice.left(nibbleKey),
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
                    (lookUp.slice, lookUp.value) = decoded.Leaf();
                    // check if the slice matches the partial key
                    if (
                        keccak256(abi.encode(lookUp.slice)) ==
                        keccak256(abi.encode(partialKey))
                    ) {
                        // if the key is found, load the value and return
                        lookUp.nextNode = loadValue(
                            lookUp.value,
                            _trie.nibbleSlice.originalDataAsPrefix(nibbleKey),
                            key
                        );
                        return true;
                    } else {
                        // if the slice does not match the partial key, move to the next inline node
                        break;
                    }
                } else if (decoded.getNodeType() == NodeType.Extension) {
                    (lookUp.slice, lookUp.item) = decoded.Extension();
                    // check if the partial key to remove the traversed slice
                    if (_trie.nibbleSlice.startWith(partialKey, lookUp.slice)) {
                        // update the partial key to remove the traversed slice
                        partialKey = _trie.nibbleSlice.mid(
                            partialKey,
                            _trie.nibbleSlice.len(lookUp.slice)
                        );
                        // update the key nibbles counter
                        keyNibbles += _trie.nibbleSlice.len(lookUp.slice);
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
                    if (_trie.nibbleSlice.isEmpty(partialKey)) {
                        // if the partial key is empty, load the value from the branch node
                        lookUp.nextNode = loadValue(
                            lookUp.value,
                            _trie.nibbleSlice.originalDataAsPrefix(nibbleKey),
                            key
                        );
                    } else {
                        // if the partial key is not empty, update the partial key to remove the first nibble
                        partialKey = _trie.nibbleSlice.mid(partialKey, 1);
                        // increment the key nibbles counter
                        ++keyNibbles;
                        // set the next Node to the child at the first nibble of the partial key
                        lookUp.nextNode = lookUp.children[
                            _trie.nibbleSlice.at(partialKey, 0)
                        ];
                    }
                } else if (decoded.getNodeType() == NodeType.NibbledBranch) {
                    (lookUp.slice, lookUp.children, lookUp.value) = decoded
                        .NibbledBranch();
                    if (
                        !_trie.nibbleSlice.startWith(partialKey, lookUp.slice)
                    ) {
                        // if the partial key does not start with the slice, move to the next inline node
                        break;
                    }
                    if (
                        _trie.nibbleSlice.len(partialKey) ==
                        _trie.nibbleSlice.len(lookUp.slice)
                    ) {
                        // if the partial key has the same length as the slice,
                        // the value in the nibbled branch node is the value of the key
                        lookUp.nextNode = loadValue(
                            lookUp.value,
                            _trie.nibbleSlice.originalDataAsPrefix(nibbleKey),
                            key
                        );
                    } else {
                        // if the partial key is longer than the slice,
                        // the next node is the child node at the index of the first nibble
                        // after the slice in the partial key
                        partialKey = _trie.nibbleSlice.mid(
                            partialKey,
                            _trie.nibbleSlice.len(lookUp.slice)
                        );
                        keyNibbles += _trie.nibbleSlice.len(lookUp.slice);
                        lookUp.nextNode = lookUp.children[
                            _trie.nibbleSlice.at(partialKey, 0)
                        ];
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
        Slice memory prefix,
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
