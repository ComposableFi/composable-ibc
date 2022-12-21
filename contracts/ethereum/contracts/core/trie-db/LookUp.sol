// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ITrie.sol";
import "../../interfaces/ISpec.sol";
import "./NibbleSlice.sol";
import "./HashDBRef.sol";

contract LookUp is ITrie, ISpec {
    NibbleSlice nibbleSlice;
    HashDBRef db;

    constructor(address nibbleSliceAddress, address hashDbAddress) {
        nibbleSlice = NibbleSlice(nibbleSliceAddress);
        db = HashDBRef(hashDbAddress);
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
        bytes32 rootHash,
        TrieLayout calldata layout
    ) external returns (bool, uint8[] memory result) {
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

            uint256 nodeDataIdx = 0;

            while (nodeDataIdx < nodeData.length) {
                // decode the node data from the codec instance
                decoded = decodeUsingCodec(nodeData[nodeDataIdx], layout.Codec);
                nodeDataIdx++;

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
        bytes32 hash,
        TrieLayout calldata layout
    ) internal view returns (uint8[] memory) {
        if (value.isInline) {
            // if the value is inline, decode it and return the result
            return _decodeValue(layout.Hash, value.data);
        } else {
            // // if the value is a node, get the hash value and lookup the value in the db
            // bytes memory v =
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
        return value;
    }

    function _decodeHash(uint8[] memory data, Hasher memory hasher)
        internal
        pure
        returns (bytes32)
    {
        if (data.length != hasher.hasherLength) {
            return 0x0;
        }
        bytes32 hash = 0x0;

        // copy the data from the input slice to the hash variable
        for (uint256 i = 0; i < data.length; i++) {
            assembly {
                let b := mload(add(data, i))
                mstore(add(hash, i), b)
            }
        }
        return hash;
    }

    function decodeUsingCodec(uint8 node_data, NodeCodec codec)
        internal
        returns (Node memory)
    {}
}
