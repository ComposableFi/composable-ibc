// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ITrie.sol";
import "../../interfaces/ISpec.sol";
import "./NibbleSlice.sol";
import "./HashDBRef.sol";
import "../node-codec/NodeCodec.sol";
import "../node-codec/NodeBuilder.sol";

contract LookUp is ITrie, ISpec {
    NibbleSlice nibbleSlice;
    HashDBRef db;
    NodeCodec nodeCodec;
    NodeBuilder nodeBuilder;

    constructor(
        address nibbleSliceAddress,
        address hashDbAddress,
        address nodeCodecAddress,
        address nodeBuilderAddress
    ) {
        nibbleSlice = NibbleSlice(nibbleSliceAddress);
        db = HashDBRef(hashDbAddress);
        nodeCodec = NodeCodec(nodeCodecAddress);
        nodeBuilder = NodeBuilder(nodeBuilderAddress);
    }

    /**
     * Function to look up a value in a base-16 patricia merkle trie.
     *
     * @param KVStore An array of key-value stores where the trie data is stored.
     * @param key The key to look up.
     * @param rootHash The root of the trie.
     * @param layout The trie layout.
     * @return A tuple consisting of the following elements:
     *         - bool: A boolean indicating whether the key was found in the trie.
     *         - bytes memory: The value associated with the key if found, or empty bytes otherwise.
     */
    function lookUpWithoutCache(
        DB[] memory KVStore,
        uint8[] calldata key,
        bytes memory rootHash,
        TrieLayout calldata layout
    ) external view returns (bool, uint8[] memory result) {
        // keeps track of the number of nibbles in the key that have been traversed
        uint8 keyNibbles = 0;
        // keeps track of the remaining nibbles in the key to be looked up
        Slice memory nibbleKey = Slice(key, 0);
        Slice memory partialKey = nibbleKey;

        Node memory decoded;
        NodeHandle memory nextNode;
        uint8[] memory nodeData;

        // TODO: verify if this makes sense
        while (keyNibbles < nibbleSlice.len(nibbleKey)) {
            nibbleKey = nibbleSlice.mid(nibbleKey, keyNibbles);
            // get the data from the current node
            nodeData = db.get(
                KVStore,
                rootHash,
                nibbleSlice.left(nibbleKey),
                layout.Hash
            );

            // check if the data is not found in the database
            if (nodeData.length == 0) {
                return (false, result);
            }

            while (true) {
                // decode the node data from the codec instance
                decoded = decodeUsingCodec(nodeData, layout.Hash);

                if (decoded.nodeType == NodeType.Leaf) {
                    // check if the slice matches the partial key
                    if (
                        keccak256(
                            abi.encode(decoded.slice.data, decoded.slice.offset)
                        ) ==
                        keccak256(
                            abi.encode(partialKey.data, partialKey.offset)
                        )
                    ) {
                        // if the key is found, load the value and return
                        result = _loadValue(
                            KVStore,
                            decoded.value,
                            nibbleSlice.originalDataAsPrefix(nibbleKey),
                            rootHash,
                            layout
                        );
                        return (true, result);
                    } else {
                        // if the slice does not match the partial key, move to the next inline node
                        break;
                    }
                } else if (decoded.nodeType == NodeType.Extension) {
                    // check if the partial key to remove the traversed slice
                    if (nibbleSlice.startWith(partialKey, decoded.slice)) {
                        // update the partial key to remove the traversed slice
                        partialKey = nibbleSlice.mid(
                            partialKey,
                            nibbleSlice.len(decoded.slice)
                        );
                        // update the key nibbles counter
                        keyNibbles += nibbleSlice.len(decoded.slice);
                        // set the next node to the item in the extension node
                        nextNode = decoded.child;
                    } else {
                        // if the partial key does not start with the slice, move to the next inline node
                        break;
                    }
                } else if (decoded.nodeType == NodeType.Branch) {
                    if (nibbleSlice.isEmpty(partialKey)) {
                        // if the partial key is empty, load the value from the branch node
                        result = _loadValue(
                            KVStore,
                            decoded.value,
                            nibbleSlice.originalDataAsPrefix(nibbleKey),
                            rootHash,
                            layout
                        );
                        return (true, result);
                    } else {
                        // if the partial key is not empty, update the partial key to remove the first nibble
                        partialKey = nibbleSlice.mid(partialKey, 1);
                        ++keyNibbles;
                        // set the next Node to the child at the first nibble of the partial key
                        nextNode = decoded.children[
                            nibbleSlice.at(partialKey, 0)
                        ];
                    }
                } else if (decoded.nodeType == NodeType.NibbledBranch) {
                    if (!nibbleSlice.startWith(partialKey, decoded.slice)) {
                        // if the partial key does not start with the slice, move to the next inline node
                        break;
                    }
                    if (
                        nibbleSlice.len(partialKey) ==
                        nibbleSlice.len(decoded.slice)
                    ) {
                        // if the partial key has the same length as the slice,
                        // the value in the nibbled branch node is the value of the key
                        result = _loadValue(
                            KVStore,
                            decoded.value,
                            nibbleSlice.originalDataAsPrefix(nibbleKey),
                            rootHash,
                            layout
                        );
                        return (true, result);
                    } else {
                        // if the partial key is longer than the slice,
                        // the next node is the child node at the index of the first nibble
                        // after the slice in the partial key
                        partialKey = nibbleSlice.mid(
                            partialKey,
                            nibbleSlice.len(decoded.slice)
                        );
                        keyNibbles += nibbleSlice.len(decoded.slice);
                        nextNode = decoded.children[
                            nibbleSlice.at(partialKey, 0)
                        ];
                    }
                } else if (decoded.nodeType == NodeType.Empty) {
                    // if the node type is empty, the key is not in the trie
                    return (false, result);
                }
                if (nextNode.isHash) {
                    rootHash = _decodeHash(nextNode.data, layout.Hash);
                    break;
                } else nodeData = nextNode.data;
            }
        }
        return (false, result);
    }

    function _loadValue(
        DB[] memory KVStore,
        Value memory value,
        Slice memory prefix,
        bytes memory hash,
        TrieLayout calldata layout
    ) internal view returns (uint8[] memory) {
        if (value.isInline) {
            // if the value is inline, decode it and return the result
            return _decodeValue(layout.Hash, value.data);
        } else {
            // if the value is a node, get the hash value and lookup the value in the db
            // If a value is found, decode and return the result
            return
                _decodeValue(
                    layout.Hash,
                    db.get(KVStore, hash, prefix, layout.Hash)
                );
        }
    }

    function _decodeValue(Hasher memory, uint8[] memory value)
        internal
        pure
        returns (uint8[] memory)
    {
        return value; // TODO: check if bytes or uint8 array
    }

    function _decodeHash(uint8[] memory data, Hasher memory hasher)
        internal
        pure
        returns (bytes memory hash)
    {
        require(
            data.length == hasher.hasherLength,
            "data length not equal to hasher length"
        );

        // TODO: based on the hashed, implement this when bytes/uint8 array is clear

        return hash;
    }

    function decodeUsingCodec(uint8[] memory node_data, Hasher memory hasher)
        internal
        view
        returns (Node memory)
    {
        return
            nodeBuilder.buildNode(
                nodeCodec.decodePlan(node_data, hasher),
                node_data
            );
    }
}
