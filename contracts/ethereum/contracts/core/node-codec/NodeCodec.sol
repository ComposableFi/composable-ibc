// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ICodec.sol";
import "../../utils/NibbleOps.sol";
import "./NodeHeader.sol";
import "./NodePlan.sol";
import "./ScaleCodec.sol";

contract NodeCodec is ICodec {
    NodeHeader nodeHeader;
    NodePlan nodePlan;
    NibbleOps nibbleOps;
    ScaleCodec scaleCodec;

    uint256 constant BITMAP_LENGTH = 2;

    constructor(
        address nodeHeaderAddress,
        address nodePlanAddress,
        address nibbleOpsAddress,
        address scaleCodecAddress
    ) {
        nodeHeader = NodeHeader(nodeHeaderAddress);
        nodePlan = NodePlan(nodePlanAddress);
        nibbleOps = NibbleOps(nibbleOpsAddress);
        scaleCodec = ScaleCodec(scaleCodecAddress);
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
            (
                input,
                codecStruct.partialRangeStart,
                codecStruct.partialRangeEnd
            ) = take(
                input,
                (header.value + (nibbleOps.NIBBLE_PER_BYTE() - 1)) /
                    nibbleOps.NIBBLE_PER_BYTE()
            );
            codecStruct.partialPadding = nibbleOps.numberPadding(header.value);
            (
                input,
                codecStruct.bitmapRangeStart,
                codecStruct.bitmapRangeEnd
            ) = take(input, BITMAP_LENGTH);
            codecStruct.bitmapValue = decodeBitmap(
                input.data,
                codecStruct.bitmapRangeStart,
                codecStruct.bitmapRangeEnd
            );
        }
    }

    function take(ByteSlice memory slice, uint256 count)
        internal
        pure
        returns (
            ByteSlice memory,
            uint256,
            uint256
        )
    {
        require(slice.offset + count <= slice.data.length, "out of data");

        uint256 rangeStart = slice.offset;
        slice.offset += count;
        uint256 rangeEnd = slice.offset;

        return (slice, rangeStart, rangeEnd);
    }

    function decodeBitmap(
        uint8[] memory data,
        uint256 start,
        uint256 end
    ) public returns (uint8) {
        uint256[] memory result;
        for (uint256 i = start; i < end; i++) {
            result[i - start] = data[i];
        }
        uint8 value = scaleCodec.decode(result);
        require(value != 0, "Bitmap without a child");
        return value;
    }
}
