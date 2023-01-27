//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.17;

import "../MerkleMultiProof.sol";

contract MerkleMultiProofWrapper {
 function verifyProof(bytes32 root, Node[][] memory proof, Node[] memory leaves)
        public
        pure
        returns (bool)
    {
        return MerkleMultiProof.verifyProof(root, proof, leaves);
    }
}