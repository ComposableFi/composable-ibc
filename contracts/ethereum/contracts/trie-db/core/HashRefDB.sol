// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/ITrie.sol";
import "./Codec.sol";

contract HashRefDB is ITrie {
    Codec codec;

    mapping(bytes32 => bytes) private nodes;

    constructor(address codecAddress) {
        codec = Codec(codecAddress);
    }

    function get(
        bytes calldata key,
        Prefix calldata prefix,
        Hasher hash
    ) external view returns (bytes memory) {}
}

contract Query is ITrie {
    function decode(Hasher hasher, bytes calldata data)
        external
        returns (NodeHandle memory)
    {}
}
