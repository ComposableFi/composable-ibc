// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ICodec.sol";
import "../../interfaces/ITrie.sol";

contract NodeBuilder is ICodec, ITrie {
    NodeHandle[] emptyChildren;
    NodeHandle emptyChild;
    Slice emptySlice;
    Value emptyValue;

    function buildNode(NodePlanStruct memory nodePlan, uint8[] memory data)
        external
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
        } else if (nodePlan.planType == NodePlanType.Branch) {} else if (
            nodePlan.planType == NodePlanType.NibbledBranch
        ) {} else {
            revert("Unsupported nodeplantype");
        }
    }

    function buildSlice(SlicePlan memory slicePlan, uint8[] memory data)
        internal
        returns (Slice memory)
    {}

    function buildValue(ValuePlan memory valuePlan, uint8[] memory data)
        internal
        returns (Value memory)
    {}

    function buildNodeHandle(
        NodeHandlePlan memory nodeHandlePlan,
        uint8[] memory data
    ) internal returns (NodeHandle memory) {}
}
