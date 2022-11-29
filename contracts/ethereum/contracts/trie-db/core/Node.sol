// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./NibbleSlice.sol";

contract Node is ITrie {
    NodeType nodeType;

    struct NodeStruct {
        NodeType nodeType;
        bytes32 info;
    }

    function getNodeType() external view returns (NodeType) {
        return nodeType;
    }

    function Leaf() external returns (NibbleSlice slice, Value memory value) {}

    function Extension()
        external
        returns (NibbleSlice slice, NodeHandle memory nodeHandle)
    {}

    function Branch()
        external
        returns (NodeHandle[] memory nodeHandle, Value memory value)
    {}

    function NibbledBranch()
        external
        returns (
            NibbleSlice slice,
            NodeHandle[] memory nodeHandle,
            Value memory value
        )
    {}

    function Empty() external {}
}
