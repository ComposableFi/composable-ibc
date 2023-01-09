// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ITrie.sol";
import "../../interfaces/ISpec.sol";
import "./LookUp.sol";

contract Trie is ITrie, ISpec {
    address owner;
    LookUp lookUpContract;

    constructor(address lookUpAddress) {
        owner = msg.sender;
        lookUpContract = LookUp(lookUpAddress);
    }

    function updateLookUp(address lookUpAddress) external {
        require(owner == msg.sender, "Not owner");
        lookUpContract = LookUp(lookUpAddress);
    }

    function getWith(
        bytes memory root,
        TrieLayout calldata layout,
        uint8[] calldata key,
        DB[] calldata KVStore
    ) external view returns (uint8[] memory value) {
        bool ok = false;
        (ok, value) = lookUpContract.lookUpWithoutCache(
            KVStore,
            key,
            root,
            layout
        );
        require(ok, "key not found in the trie");
        return value;
    }
}
