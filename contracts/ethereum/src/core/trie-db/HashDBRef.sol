// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../interfaces/ITrie.sol";
import "../../interfaces/ISpec.sol";
import "../../utils/Blake2b.sol";

contract HashDBRef is ITrie, ISpec {
    using Blake2b for Blake2b.Instance;

    function get(
        DB[] memory KVStore,
        bytes memory hash,
        Slice calldata nibble_key,
        Hasher memory hasher
    ) external view returns (uint8[] memory value) {
        for (uint256 i = 0; i < KVStore.length; i++) {
            bytes memory storeKey;
            // compute the key for the key-value store
            if (hasher.hasherType == HasherType.BLAKE2B) {
                Blake2b.Instance memory instance = Blake2b.init(hex"", 64);
                // Todo: check encoder in parity implementation
                bytes memory input = combine(hash, nibble_key);
                storeKey = instance.finalize(input, input.length);
                require(storeKey.length == 64, "invalid hash length");
            } else {
                // not implemented yet
                // due to keccak256 32-bit platform vs blake2b's 64-bit
            }

            // Look up the value in the key-value store using the key.
            for (uint256 j = 0; j < KVStore.length; j++) {
                if (keccak256(KVStore[j].key) == keccak256(storeKey)) {
                    // Return the value if it is found.
                    if (KVStore[j].value.length > 0) {
                        return KVStore[j].value;
                    }
                }
            }
        }
        // if the value is not found in any of the key-value stores, return an empty value
        return value;
    }

    function combine(bytes memory hash, Slice calldata nibble_key)
        internal
        view
        returns (bytes memory)
    {
        // TODO: figure this out
    }
}
