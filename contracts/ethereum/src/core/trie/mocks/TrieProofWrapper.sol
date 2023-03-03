pragma solidity ^0.8.0;

import "@polytope-labs/solidity-merkle-trees/MerklePatricia.sol";

contract TrieProofWrapper {
    function verify(
        bytes32 root,
        bytes[] memory proof,
        bytes[] memory keys
    ) public returns (bytes[] memory values){
        bytes[] memory values = MerklePatricia.VerifySubstrateProof(root, proof, keys); 
    }
}