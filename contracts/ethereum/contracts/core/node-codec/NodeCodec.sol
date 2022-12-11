// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ICodec.sol";
import "./NodeHeader.sol";
import "./NodePlan.sol";
import "../../utils/NibbleOps.sol";

contract NodeCodec is ICodec {
    NodeHeader nodeHeader;
    NodePlan nodePlan;
    NibbleOps nibbleOps;

    constructor(
        address nodeHeaderAddress,
        address nodePlanAddress,
        address nibbleOpsAddress
    ) {
        nodeHeader = NodeHeader(nodeHeaderAddress);
        nodePlan = NodePlan(nodePlanAddress);
        nibbleOps = NibbleOps(nibbleOpsAddress);
    }

    function decodePlan(uint8[] memory data)
        external
        returns (NodePlanStruct memory)
    {
        CodecStruct memory codecStruct;
        ByteSlice memory input = ByteSlice(data, 0);
        NodeHeaderStruct memory header = nodeHeader.decode(input);
        bool containsHash = header.headerType ==
            NodeHeaderType.HashedValueLeaf ||
            header.headerType == NodeHeaderType.HashedValueBranch
            ? true
            : false;
        bool branchHasValue = header.headerType == NodeHeaderType.Branch
            ? header.hasValue
            : true;
        if (header.headerType == NodeHeaderType.Null) {
            return nodePlan.isEmpty();
        } else if (
            header.headerType == NodeHeaderType.HashedValueBranch ||
            header.headerType == NodeHeaderType.Branch
        ) {
            codecStruct.padding =
                header.value % nibbleOps.NIBBLE_PER_BYTE() != 0;
            // check that the padding is valid (if any)
            if (
                codecStruct.padding &&
                nibbleOps.padLeft(data[input.offset]) != 0
            ) {
                revert("Bad format");
            }
        }
    }
}
