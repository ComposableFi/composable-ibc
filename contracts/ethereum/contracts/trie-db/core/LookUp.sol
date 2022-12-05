// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./Codec.sol";
import "./Node.sol";
import "./NibbleSlice.sol";
import "./HashRefDB.sol";

contract LookUp is ITrie {
    NibbleSlice nibbleSlice;
    Codec codec;
    HashRefDB db;
    Query query;
    Node node;

    constructor(
        address nibbleSliceAddress,
        address codecAddress,
        address hashDbAddress,
        address queryAddress,
        address nodeAddress
    ) {
        nibbleSlice = NibbleSlice(nibbleSliceAddress);
        codec = Codec(codecAddress);
        db = HashRefDB(hashDbAddress);
        query = Query(queryAddress);
        node = Node(nodeAddress);
    }

    function lookUpWithoutCache(
        uint8[] calldata key,
        bytes32 root,
        TrieLayout calldata layout
    ) external returns (bool, bytes memory) {
        // keeps track of the number of nibbles in the key that have been traversed
        uint8 keyNibbles = 0;
        // keeps track of the remaining nibbles in the key to be looked up
        Slice memory nibbleKey = Slice(key, 0);
        Slice memory partialKey = nibbleKey;

        NodeStruct memory decoded;
        LookUpStruct memory lookUp;

        // TODO: verify if this makes sense
        while (keyNibbles < nibbleSlice.len(nibbleKey)) {
            nibbleKey = nibbleSlice.mid(nibbleKey, keyNibbles);
            // get the data from the current node
            lookUp.nodeData = db.get(
                key,
                nibbleSlice.left(nibbleKey),
                layout.Hash
            );

            // check if the data is not found in the database
            if (lookUp.nodeData.length == 0) {
                revert("incomplete database");
            }

            uint256 nodeDataIdx = 0;

            while (nodeDataIdx < lookUp.nodeData.length) {
                // decode the node data from the codec instance
                decoded = codec.decode(
                    lookUp.nodeData[nodeDataIdx],
                    layout.Codec
                );

                if (node.getNodeType(decoded) == NodeType.Leaf) {
                    (lookUp.slice, lookUp.value) = node.Leaf(decoded);
                    // check if the slice matches the partial key
                    if (
                        keccak256(abi.encode(lookUp.slice)) ==
                        keccak256(abi.encode(partialKey))
                    ) {
                        // if the key is found, load the value and return
                        lookUp.nextNode = loadValue(
                            lookUp.value,
                            nibbleSlice.originalDataAsPrefix(nibbleKey),
                            key,
                            layout
                        );
                        return (true, lookUp.nextNode.data);
                    } else {
                        // if the slice does not match the partial key, move to the next inline node
                        break;
                    }
                } else if (node.getNodeType(decoded) == NodeType.Extension) {
                    (lookUp.slice, lookUp.item) = node.Extension(decoded);
                    // check if the partial key to remove the traversed slice
                    if (nibbleSlice.startWith(partialKey, lookUp.slice)) {
                        // update the partial key to remove the traversed slice
                        partialKey = nibbleSlice.mid(
                            partialKey,
                            nibbleSlice.len(lookUp.slice)
                        );
                        // update the key nibbles counter
                        keyNibbles += nibbleSlice.len(lookUp.slice);
                        // set the next node to the item in the extension node
                        lookUp.nextNode = lookUp.item;
                    } else {
                        // if the partial key does not start with the slice, move to the next inline node
                        break;
                    }
                } else if (node.getNodeType(decoded) == NodeType.Branch) {
                    // get the children and value from the branch node
                    (lookUp.children, lookUp.value) = node.Branch(decoded);
                    // check if the partial key is empty
                    if (nibbleSlice.isEmpty(partialKey)) {
                        // if the partial key is empty, load the value from the branch node
                        lookUp.nextNode = loadValue(
                            lookUp.value,
                            nibbleSlice.originalDataAsPrefix(nibbleKey),
                            key,
                            layout
                        );
                    } else {
                        // if the partial key is not empty, update the partial key to remove the first nibble
                        partialKey = nibbleSlice.mid(partialKey, 1);
                        ++keyNibbles;
                        // set the next Node to the child at the first nibble of the partial key
                        lookUp.nextNode = lookUp.children[
                            nibbleSlice.at(partialKey, 0)
                        ];
                    }
                } else if (
                    node.getNodeType(decoded) == NodeType.NibbledBranch
                ) {
                    (lookUp.slice, lookUp.children, lookUp.value) = node
                        .NibbledBranch(decoded);
                    if (!nibbleSlice.startWith(partialKey, lookUp.slice)) {
                        // if the partial key does not start with the slice, move to the next inline node
                        break;
                    }
                    if (
                        nibbleSlice.len(partialKey) ==
                        nibbleSlice.len(lookUp.slice)
                    ) {
                        // if the partial key has the same length as the slice,
                        // the value in the nibbled branch node is the value of the key
                        lookUp.nextNode = loadValue(
                            lookUp.value,
                            nibbleSlice.originalDataAsPrefix(nibbleKey),
                            key,
                            layout
                        );
                    } else {
                        // if the partial key is longer than the slice,
                        // the next node is the child node at the index of the first nibble
                        // after the slice in the partial key
                        partialKey = nibbleSlice.mid(
                            partialKey,
                            nibbleSlice.len(lookUp.slice)
                        );
                        keyNibbles += nibbleSlice.len(lookUp.slice);
                        lookUp.nextNode = lookUp.children[
                            nibbleSlice.at(partialKey, 0)
                        ];
                    }
                } else if (node.getNodeType(decoded) == NodeType.Empty) {
                    // if the node type is empty, the key is not in the trie
                    revert("key not found");
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
        revert("key not found");
    }

    function loadValue(
        Value memory value,
        Slice memory prefix,
        uint8[] calldata key,
        TrieLayout calldata layout
    ) internal returns (NodeHandle memory) {
        // Check if the valueType is Inline or Node
        require(
            value.valueType == ValueType.Inline ||
                value.valueType == ValueType.Node,
            "Invalid valueType"
        );
        if (value.valueType == ValueType.Inline) {
            // if the value is inline, decode it and return the result
            return query.decode(layout.Hash, value.data);
        } else {
            // if the value is a node, get the hash value and lookup the value in the db
            bytes memory v = db.get(key, prefix, layout.Hash);
            // If a value is found, decode and return the result
            return query.decode(layout.Hash, v);
        }
    }

    function decodeHash(Hasher hasher, bytes memory data)
        internal
        returns (bytes32)
    {}
}
