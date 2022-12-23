// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ICodec.sol";
import "../../interfaces/ISpec.sol";
import "../../utils/NibbleOps.sol";
import "./NodeHeader.sol";
import "./ScaleCodec.sol";

contract NodeCodec is ICodec, ISpec {
    NodeHeader nodeHeader;
    NibbleOps nibbleOps;
    ScaleCodec scaleCodec;

    uint256 constant BITMAP_LENGTH = 2;

    constructor(
        address nodeHeaderAddress,
        address nibbleOpsAddress,
        address scaleCodecAddress
    ) {
        nodeHeader = NodeHeader(nodeHeaderAddress);
        nibbleOps = NibbleOps(nibbleOpsAddress);
        scaleCodec = ScaleCodec(scaleCodecAddress);
    }

    /**
     * @dev Decodes a node in a base-16 Merkle Patricia Trie encoded using SCALE codec.
     * @param data an array of 8-bit unsigned integers representing the encoded node data
     * @param hasher an instance of the Hasher contract
     * @return a NodePlanStruct struct containing the decoded node data
     */
    function decodePlan(uint8[] memory data, Hasher memory hasher)
        external
        view
        returns (NodePlanStruct memory)
    {
        // Initialize variables
        CodecStruct memory codecStruct;
        ByteSlice memory input = ByteSlice(data, 0);
        uint256 rangeStart;
        uint256 rangeEnd;
        uint256 decodeCount;

        // Decode the node header
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
            return
                NodePlanStruct(
                    NodePlanType.Empty,
                    SlicePlan(0, 0, 0),
                    ValuePlan(false, 0, 0),
                    codecStruct.children,
                    codecStruct.child
                );
        } else if (
            header.headerType == NodeHeaderType.HashedValueBranch ||
            header.headerType == NodeHeaderType.Branch
        ) {
            // Calculate padding for the partial value
            codecStruct.padding =
                header.value % nibbleOps.NIBBLE_PER_BYTE() != 0;
            // check that the padding is valid (if any)
            if (
                codecStruct.padding &&
                nibbleOps.padLeft(data[input.offset]) != 0
            ) {
                revert("Bad format");
            }
            // Decode the partial value
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
                input,
                codecStruct.bitmapRangeStart,
                codecStruct.bitmapRangeEnd
            );
            if (branchHasValue) {
                if (containsHash) {
                    (input, rangeStart, rangeEnd) = take(
                        input,
                        hasher.hasherLength
                    );
                    codecStruct.valuePlan = ValuePlan(
                        false,
                        rangeStart,
                        rangeEnd
                    );
                } else {
                    decodeCount = scaleCodec.decodeCompactU32(input.data);
                    (input, rangeStart, rangeEnd) = take(input, decodeCount);
                    codecStruct.valuePlan = ValuePlan(
                        true,
                        rangeStart,
                        rangeEnd
                    );
                }
            } else {
                codecStruct.valuePlan = ValuePlan(false, 0, 0);
            }
            for (uint8 i; i < nibbleOps.NIBBLE_LENGTH(); i++) {
                if (bitmapValueAt(codecStruct.bitmapValue, i)) {
                    decodeCount = scaleCodec.decodeCompactU32(input.data);
                    (input, rangeStart, rangeEnd) = take(input, decodeCount);
                    if (decodeCount == hasher.hasherLength) {
                        codecStruct.children[i] = NodeHandlePlan(
                            true,
                            rangeStart,
                            rangeEnd
                        );
                    } else {
                        codecStruct.children[i] = NodeHandlePlan(
                            false,
                            rangeStart,
                            rangeEnd
                        );
                    }
                }
            }
            return
                NodePlanStruct(
                    NodePlanType.NibbledBranch,
                    SlicePlan(
                        codecStruct.partialRangeStart,
                        codecStruct.partialRangeEnd,
                        codecStruct.partialPadding
                    ),
                    codecStruct.valuePlan,
                    codecStruct.children,
                    codecStruct.child
                );
        } else if (
            header.headerType == NodeHeaderType.HashedValueLeaf ||
            header.headerType == NodeHeaderType.Leaf
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
            if (containsHash) {
                (input, rangeStart, rangeEnd) = take(
                    input,
                    hasher.hasherLength
                );
                codecStruct.valuePlan = ValuePlan(false, rangeStart, rangeEnd);
            } else {
                // todo: fix for compact u32
                decodeCount = scaleCodec.decodeCompactU32(input.data);
                (input, rangeStart, rangeEnd) = take(input, decodeCount);
                codecStruct.valuePlan = ValuePlan(true, rangeStart, rangeEnd);
            }
            return
                NodePlanStruct(
                    NodePlanType.Leaf,
                    SlicePlan(
                        codecStruct.partialRangeStart,
                        codecStruct.partialRangeEnd,
                        codecStruct.partialPadding
                    ),
                    codecStruct.valuePlan,
                    codecStruct.children,
                    codecStruct.child
                );
        } else {
            revert("Unknown NodeHeaderType");
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

    // Radix 16 trie, bitmap decoding implementation
    function decodeBitmap(
        ByteSlice memory input,
        uint256 start,
        uint256 end
    ) public view returns (uint16) {
        uint8[] memory result;
        for (uint256 i = start; i < end; i++) {
            result[i - start] = input.data[i];
        }
        uint16 value = scaleCodec.decodeU16(result);
        require(value != 0, "Bitmap without a child");
        return value;
    }

    function bitmapValueAt(uint16 bitmapValue, uint8 i)
        internal
        pure
        returns (bool)
    {
        return (bitmapValue & (1 << i)) != 0;
    }
}
