// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./NibbleSlice.sol";

contract Node is ITrie {
    function getNodeType(NodeStruct calldata node)
        public
        pure
        returns (NodeType nodeType)
    {
        // If the node has no children, it is an empty node.
        if (node.children.length == 0) {
            nodeType = NodeType.Empty;
        }
        // If the node has one child and the child is a value node, it is a leaf node.
        else if (node.children.length == 1 && !node.children[0].isHash) {
            nodeType = NodeType.Leaf;
        }
        // If the node has one child and the child is a hash node, it is an extension node.
        else if (node.children.length == 1 && node.children[0].isHash) {
            nodeType = NodeType.Extension;
        }
        // If the node has multiple children and all of the children have the same nibble length, it is a branch node.
        else if (
            node.children.length > 1 &&
            node.children[0].key.length == node.children[1].key.length
        ) {
            nodeType = NodeType.Branch;
        }
        // If the node has multiple children and the children have different nibble lengths, it is a nibbled branch node.
        else {
            nodeType = NodeType.NibbledBranch;
        }

        return nodeType;
    }

    function Leaf(NodeStruct calldata node)
        external
        returns (Slice memory slice, Value memory value)
    {}

    function Extension(NodeStruct calldata node)
        external
        returns (Slice memory slice, NodeHandle memory nodeHandle)
    {}

    function Branch(NodeStruct calldata node)
        external
        returns (NodeHandle[] memory nodeHandle, Value memory value)
    {}

    function NibbledBranch(NodeStruct calldata node)
        external
        returns (
            Slice memory slice,
            NodeHandle[] memory nodeHandle,
            Value memory value
        )
    {}

    function Empty(NodeStruct calldata node) external {}
}
