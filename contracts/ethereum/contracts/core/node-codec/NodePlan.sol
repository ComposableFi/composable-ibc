// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ICodec.sol";

contract NodePlan is ICodec {
    function isEmpty() external returns (NodePlanStruct memory) {}

    function isLeaf() external {}

    function isExtension() external {}
}
