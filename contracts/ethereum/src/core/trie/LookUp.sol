// SPDX-License-Identifier: Apache2
pragma solidity ^0.8.17;

import "./Node.sol";
import "./NibbleSlice.sol";
import "./TrieDB.sol";
import "./NodeCodec.sol";

// source: 
// https://github.com/polytope-labs/solidity-merkle-trees/blob/main/src/MerklePatricia.sol
// https://github.com/paritytech/trie/blob/master/trie-db/src/lookup.rs

library LookUp {
    uint256 internal constant MAX_TRIE_DEPTH = 50;

    /**
     * Function to look up a value in a base-16 patricia merkle trie.
     *
     * @param root hash of the merkle patricia trie
     * @param proof a list of proof nodes
     * @param keys a list of keys to verify
     * @return bytes[] a list of values corresponding to the supplied keys.
     */
    function VerifySubstrateProof(
        bytes32 root,
        bytes[] memory proof,
        bytes[] memory keys
    ) public pure returns (bytes[] memory) {
        // Initialize an array of values to store the values corresponding to the keys.
        bytes[] memory values = new bytes[](keys.length);

        // Initialize an array of trie nodes to store the proof nodes.
        TrieNode[] memory nodes = new TrieNode[](proof.length);

        // Convert each proof node into a trie node and add it to the nodes array.
        for (uint256 i = 0; i < proof.length; i++) {
            nodes[i] = TrieNode(keccak256(proof[i]), proof[i]);
        }

        // Iterate over each key to verify.
        for (uint256 i = 0; i < keys.length; i++) {
            // Convert the key into a nibble slice.
            Slice memory keyNibbles = Slice(keys[i], 0);

            // Decode the root node of the trie.
            NodeKind memory node = NodeCodec.decodeNodeKind(TrieDB.get(nodes, root));

            // Search the trie for the value corresponding to the key.
            for (uint256 j = 0; j < MAX_TRIE_DEPTH; j++) {
                NodeHandle memory nextNode;

                // If the node is a leaf, check if it has the same key as the search key and return its value.
                if (TrieDB.isLeaf(node)) {
                    Leaf memory leaf = NodeCodec.decodeLeaf(node);
                    if (NibbleSlice.eq(leaf.key, keyNibbles)) {
                        values[i] = TrieDB.load(nodes, leaf.value);
                    }
                    break;
                }

                // If the node is a nibbled branch, check if its key is a prefix of the search key and traverse its children.
                else if (TrieDB.isNibbledBranch(node)) {
                    NibbledBranch memory nibbled = NodeCodec
                        .decodeNibbledBranch(node);
                    uint256 nibbledBranchKeyLength = NibbleSlice.len(
                        nibbled.key
                    );
                    if (!NibbleSlice.startWith(keyNibbles, nibbled.key)) {
                        break;
                    }

                    if (
                        NibbleSlice.len(keyNibbles) == nibbledBranchKeyLength
                    ) {
                        if (isSome(nibbled.value)) {
                            values[i] = TrieDB.load(nodes, nibbled.value.value);
                        }
                        break;
                    } else {
                        uint256 index = NibbleSlice.at(
                            keyNibbles,
                            nibbledBranchKeyLength
                        );
                        NodeHandleOption memory handle = nibbled.children[
                            index
                        ];
                        if (isSome(handle)) {
                            keyNibbles = NibbleSlice.mid(
                                keyNibbles,
                                nibbledBranchKeyLength + 1
                            );
                            nextNode = handle.value;
                        } else {
                            break;
                        }
                    }
                } else if (TrieDB.isEmpty(node)) {
                    break;
                }

                node = NodeCodec.decodeNodeKind(
                    TrieDB.load(nodes, nextNode)
                );
            }
        }

        return values;
    }

    /**
     * @notice Verify child trie keys
     * @dev substrate specific method in order to verify keys in the child trie.
     * @param root hash of the merkle root
     * @param proof a list of proof nodes
     * @param keys a list of keys to verify
     * @param childInfo data that can be used to compute the root of the child trie
     * @return bytes[], a list of values corresponding to the supplied keys.
     */
    function ReadChildProofCheck(
        bytes32 root,
        bytes[] memory proof,
        bytes[] memory keys,
        bytes memory childInfo
    ) public pure returns (bytes[] memory) {
        // fetch the child trie root hash;
        bytes memory prefix = bytes(":child_storage:default:");
        bytes memory key = bytes.concat(prefix, childInfo);
        bytes[] memory _keys = new bytes[](1);
        _keys[0] = key;
        bytes[] memory values = VerifySubstrateProof(root, proof, _keys);

        bytes32 childRoot = bytes32(values[0]);
        require(childRoot != bytes32(0), "Invalid child trie proof");

        return VerifySubstrateProof(childRoot, proof, keys);
    }

    function isSome(ValueOption memory val) public pure returns (bool) {
        return val.isSome == true;
    }

    function isSome(NodeHandleOption memory val) public pure returns (bool) {
        return val.isSome == true;
    }
}
