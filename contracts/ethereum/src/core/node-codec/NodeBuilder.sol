// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ICodec.sol";
import "../../interfaces/ITrie.sol";

contract NodeBuilder is ICodec, ITrie {
    NodeHandle[] emptyChildren;
    NodeHandle emptyChild;
    Slice emptySlice;
    Value emptyValue;

    uint8 constant NIBBLE_LENGTH = 16;

    /**
     * @dev Builds a Node object from a NodePlan object.
     * A NodePlan is a  blueprint for decoding a Node from a byte slice.
     * Stores NodePlanType and additional data needed to build Node.
     * @param nodePlan The NodePlan object specifying how to build the Node.
     * @param data The data needed to build the Node.
     * @return The built Node object.
     */
    function buildNode(NodePlanStruct memory nodePlan, uint8[] calldata data)
        external
        view
        returns (Node memory)
    {
        if (nodePlan.planType == NodePlanType.Empty) {
            return
                Node(
                    NodeType.Empty,
                    emptySlice,
                    emptyValue,
                    emptyChildren,
                    emptyChild
                );
        } else if (nodePlan.planType == NodePlanType.Leaf) {
            return
                Node(
                    NodeType.Leaf,
                    buildSlice(nodePlan.slicePlan, data),
                    buildValue(nodePlan.valuePlan, data),
                    emptyChildren,
                    emptyChild
                );
        } else if (nodePlan.planType == NodePlanType.Extension) {
            return
                Node(
                    NodeType.Extension,
                    buildSlice(nodePlan.slicePlan, data),
                    emptyValue,
                    emptyChildren,
                    buildNodeHandle(nodePlan.child, data)
                );
        } else if (nodePlan.planType == NodePlanType.Branch) {
            NodeHandle[] memory childSlices;
            for (uint256 i = 0; i < NIBBLE_LENGTH; i++) {
                childSlices[i] = buildNodeHandle(nodePlan.children[0], data);
            }
            return
                Node(
                    NodeType.Branch,
                    emptySlice,
                    buildValue(nodePlan.valuePlan, data),
                    childSlices,
                    emptyChild
                );
        } else if (nodePlan.planType == NodePlanType.NibbledBranch) {
            NodeHandle[] memory childSlices;
            for (uint256 i = 0; i < NIBBLE_LENGTH; i++) {
                childSlices[i] = buildNodeHandle(nodePlan.children[0], data);
            }
            return
                Node(
                    NodeType.NibbledBranch,
                    buildSlice(nodePlan.slicePlan, data),
                    buildValue(nodePlan.valuePlan, data),
                    childSlices,
                    emptyChild
                );
        } else {
            revert("Unsupported NodePlanType");
        }
    }

    /**
     * @dev Builds a Slice object from a SlicePlan object.
     * A SlicePlan is a Plan for decoding a nibble slice from a byte slice.
     * Stores start/end indices and offset within the key/value it belongs to.
     * @param slicePlan The SlicePlan object specifying how to build the Slice.
     * @param data The data needed to build the Slice.
     * @return The built Slice object.
     */
    function buildSlice(SlicePlan memory slicePlan, uint8[] calldata data)
        internal
        pure
        returns (Slice memory)
    {
        return
            Slice(
                data[slicePlan.dataStart:slicePlan.dataEnd],
                slicePlan.offset
            );
    }

    /**
     * @dev Builds a Value object from a ValuePlan object.
     * A ValuePlan is a Plan for representing a key/value pair in a NodePlan.
     * Stores start/end indices and boolean indicating whether data is stored inline.
     * @param valuePlan The ValuePlan object specifying how to build the Value.
     * @param data The data needed to build the Value.
     * @return The built Value object.
     */
    function buildValue(ValuePlan memory valuePlan, uint8[] calldata data)
        internal
        pure
        returns (Value memory)
    {
        return Value(valuePlan.isInline, data[valuePlan.start:valuePlan.end]);
    }

    /**
     * @dev Builds a NodeHandle object from a NodeHandlePlan object.
     * A NodeHandlePlan is a decoding plan for constructing a NodeHandle from an encoded trie node.
     * Stores start/end indices and boolean indicating whether data is a hash.
     * @param nodeHandlePlan The NodeHandlePlan object specifying how to build the NodeHandle.
     * @param data The data needed to build the NodeHandle.
     * @return The built NodeHandle object.
     */
    function buildNodeHandle(
        NodeHandlePlan memory nodeHandlePlan,
        uint8[] calldata data
    ) internal pure returns (NodeHandle memory) {
        return
            NodeHandle(
                nodeHandlePlan.isHash,
                data[nodeHandlePlan.start:nodeHandlePlan.end]
            );
    }
}
