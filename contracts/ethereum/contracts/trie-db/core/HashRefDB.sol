// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./Codec.sol";

contract HashRefDB is ITrie {
    mapping(bytes32 => bytes) private values;

    function get(
        bytes32 hash,
        Slice calldata nibble_key,
        Hasher hasher
    ) public view returns (bytes memory value) {
        // // Look up the value in the values mapping using the given hash as the key.
        // value = values[hash];
        // // If the value is not found in the values mapping, return null.
        // // Parse the node data from the found value.
        // Node node = Node(value);
        // // Iterate through the children of the node and find the child that matches the given nibble key.
        // for (uint256 i = 0; i < node.children.length; i++) {
        //     if (node.children[i].key == nibble_key) {
        //         // If the child is a hash node, recursively call the get function to retrieve the value from the child.
        //         if (node.children[i].isHash) {
        //             return get(node.children[i].value, nibble_key);
        //         }
        //         // If the child is a value node, return the value of the child.
        //         else {
        //             return node.children[i].value;
        //         }
        //     }
        // }
        // // If no child is found that matches the given nibble key, return null.
        // return 0x00;
    }
}

contract Query is ITrie {
    function decode(Hasher hasher, bytes calldata data)
        external
        returns (bytes memory)
    {}
}
