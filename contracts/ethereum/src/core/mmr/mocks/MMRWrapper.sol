//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.17;

import "../MMR.sol";

contract MMRWrapper {
  using MMR for MMR.Tree;
  MMR.Tree public mmr;

  bytes32 public rootHash;
  bool public verified;

  function append(bytes memory hash) external {
    mmr.append(hash);
  }

  function commit() external {
    rootHash = mmr.root;
  }

  function verify(
      bytes32 root,
      uint256 width,
      uint256 index,
      bytes memory value,
      bytes32[] memory peaks,
      bytes32[] memory siblings
  ) external {
    verified = MMR.inclusionProof(
        root,
        width,
        index + 1,
        value,
        peaks,
        siblings
    );
  }

  function getMerkleProof(uint256 index) public view returns (
      bytes32 root,
      uint256 width,
      bytes32[] memory peakBagging,
      bytes32[] memory siblings
  ){
    return mmr.getMerkleProof(index + 1);
  }

  function getNode(uint256 index) public view returns (bytes32) {
    return mmr.getNode(index + 1);
  }
}