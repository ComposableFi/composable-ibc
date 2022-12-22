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

    function buildValue(ValuePlan memory valuePlan, uint8[] calldata data)
        internal
        pure
        returns (Value memory)
    {
        if (valuePlan.isInline) {
            return Value(true, data[valuePlan.start:valuePlan.end]);
        }
        return Value(false, data[valuePlan.start:valuePlan.end]);
    }

    function buildNodeHandle(
        NodeHandlePlan memory nodeHandlePlan,
        uint8[] calldata data
    ) internal pure returns (NodeHandle memory) {
        if (nodeHandlePlan.isHash) {
            return
                NodeHandle(true, data[nodeHandlePlan.start:nodeHandlePlan.end]);
        }
        return NodeHandle(false, data[nodeHandlePlan.start:nodeHandlePlan.end]);
    }
}
