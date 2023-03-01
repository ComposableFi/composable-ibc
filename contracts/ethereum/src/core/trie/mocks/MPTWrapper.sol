// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../LookUp.sol";

contract MPTWrapper {
    function verify(
        bytes32 root,
        bytes[] memory proof,
        bytes[] memory keys
    ) public pure {
        bytes[] memory values = LookUp.VerifySubstrateProof(root, proof, keys); // verifies proofs from state.getReadProof
        // do something with the verified values.
    }
}