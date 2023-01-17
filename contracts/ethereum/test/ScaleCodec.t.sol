// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../lib/forge-std/src/Test.sol";
import "../src/core/node-codec/ScaleCodec.sol";

contract TestScaleCodec {
    ScaleCodec public scaleCodec;

    function setUp() public {
        scaleCodec = new ScaleCodec();
    }

    // Test that decodeCompactU32 returns the correct value for a single-byte input
    function testDecodeCompactU32SingleByte() public view {
        uint8[] memory input = new uint8[](1);
        input[0] = 0x04;
        uint32 expected = 1;
        uint32 actual = scaleCodec.decodeCompactU32(input);
        console.log(actual);
        assert(actual == expected);
    }

    // Test that decodeCompactU32 returns the correct value for a two-byte input
    function testDecodeCompactU32TwoByte() public view {
        uint8[] memory input = new uint8[](2);
        input[0] = 0x15;
        input[1] = 0x01;
        uint32 expected = 69;
        uint32 actual = scaleCodec.decodeCompactU32(input);
        console.log(actual);
        assert(actual == expected);
    }

    // Test that decodeCompactU32 returns the correct value for a four-byte input
    function testDecodeCompactU32FourByte() public view {
        uint8[] memory input = new uint8[](4);
        input[0] = 0xfe;
        input[1] = 0xff;
        input[2] = 0x03;
        input[3] = 0x00;
        uint32 expected = 65535;
        uint32 actual = scaleCodec.decodeCompactU32(input);
        console.log(actual);
        assert(actual == expected);
    }

    // Test that decodeU16 returns the correct value
    function testDecodeU16() public view {
        uint8[] memory input = new uint8[](2);
        input[0] = 0x2a;
        input[1] = 0x00;
        uint16 expected = 42;
        uint16 actual = scaleCodec.decodeU16(input);
        console.log(actual);
        assert(actual == expected);
    }

    function testDecodeU32() public view {
        uint8[] memory input = new uint8[](4);
        input[0] = 0xff;
        input[1] = 0xff;
        input[2] = 0xff;
        input[3] = 0x00;
        uint32 expected = 16777215;
        uint32 actual = scaleCodec.decodeU32(input);
        console.log(actual);
        assert(actual == expected);
    }
}
