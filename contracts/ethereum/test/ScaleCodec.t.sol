// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../lib/forge-std/src/Test.sol";
import "../src/core/node-codec/ScaleCodec.sol";

contract TestScaleCodec is Test {
    ScaleCodec public scaleCodec;

    function setUp() public {
        scaleCodec = new ScaleCodec();
    }

    // Test that decodeCompactU32 reverts for empty input
    function testRevertDecodeCompactU32Empty() public {
        uint8[] memory input = new uint8[](0);
        vm.expectRevert(bytes("Input array must not be empty."));
        scaleCodec.decodeCompactU32(input);
    }

    // Test that decodeCompactU32 returns the correct value for a single-byte input
    function testDecodeCompactU32SingleByte_1() public view {
        uint8[] memory input = new uint8[](1);
        input[0] = 0x00;
        uint32 expected = 0;
        uint32 actual = scaleCodec.decodeCompactU32(input);
        console.log(actual);
        assert(actual == expected);
    }

    // Test that decodeCompactU32 returns the correct value for a single-byte input
    function testDecodeCompactU32SingleByte_2() public view {
        uint8[] memory input = new uint8[](1);
        input[0] = 0x04;
        uint32 expected = 1;
        uint32 actual = scaleCodec.decodeCompactU32(input);
        console.log(actual);
        assert(actual == expected);
    }

    // Test that decodeCompactU32 returns the correct value for a single-byte input
    function testDecodeCompactU32SingleByte_3() public view {
        uint8[] memory input = new uint8[](1);
        input[0] = 0xa8;
        uint32 expected = 42;
        uint32 actual = scaleCodec.decodeCompactU32(input);
        console.log(actual);
        assert(actual == expected);
    }

    // Test that decodeCompactU32 returns the correct value for a two-byte input
    function testDecodeCompactU32TwoByte_1() public view {
        uint8[] memory input = new uint8[](2);
        input[0] = 0x15;
        input[1] = 0x01;
        uint32 expected = 69;
        uint32 actual = scaleCodec.decodeCompactU32(input);
        console.log(actual);
        assert(actual == expected);
    }

    // Test that decodeCompactU32 reverts for three-byte input
    function testRevertDecodeCompactU32ThreeByte() public {
        uint8[] memory input = new uint8[](3);
        input[0] = 0xfe;
        input[1] = 0xff;
        input[2] = 0x03;
        vm.expectRevert(bytes("Input length must be 4 for U32 decoding"));
        scaleCodec.decodeCompactU32(input);
    }

    // Test that decodeCompactU32 returns the correct value for a four-byte input
    function testDecodeCompactU32FourByte_1() public view {
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

    // Test that decodeCompactU32 returns the correct value for a four-byte input
    function testDecodeCompactU32FourByte_2() public view {
        uint8[] memory input = new uint8[](4);
        input[0] = 0x02;
        input[1] = 0x09;
        input[2] = 0x3d;
        input[3] = 0x00;
        uint32 expected = 1000000;
        uint32 actual = scaleCodec.decodeCompactU32(input);
        console.log(actual);
        assert(actual == expected);
    }

    // Test that decodeCompactU32 returns the correct value for a four-byte input
    function testDecodeCompactU32FourByte_3() public view {
        uint8[] memory input = new uint8[](4);
        input[0] = 0xff;
        input[1] = 0xff;
        input[2] = 0xff;
        input[3] = 0xff;
        uint32 expected = 4294967295;
        uint32 actual = scaleCodec.decodeCompactU32(input);
        console.log(actual);
        assert(actual == expected);
    }

    // Test that decodeCompactU32 reverts for a five-byte input
    function testRevertDecodeCompactU32FiveByte() public {
        uint8[] memory input = new uint8[](5);
        input[0] = 0xff;
        input[1] = 0xff;
        input[2] = 0xff;
        input[3] = 0xff;
        input[4] = 0xff;
        vm.expectRevert(bytes("Input length must be 4 for U32 decoding"));
        scaleCodec.decodeCompactU32(input);
    }

    // Test that decodeU16 returns the correct value
    function testDecodeU16_1() public view {
        uint8[] memory input = new uint8[](2);
        input[0] = 0x2a;
        input[1] = 0x00;
        uint16 expected = 42;
        uint16 actual = scaleCodec.decodeU16(input);
        console.log(actual);
        assert(actual == expected);
    }

    // Test that decodeU16 returns the correct value
    function testDecodeU16_2() public view {
        uint8[] memory input = new uint8[](2);
        input[0] = 0x2e;
        input[1] = 0xfb;
        uint16 expected = 64302;
        uint16 actual = scaleCodec.decodeU16(input);
        console.log(actual);
        assert(actual == expected);
    }

    // Test that decodeU16 returns the correct value
    function testDecodeU16_3() public view {
        uint8[] memory input = new uint8[](2);
        input[0] = 0xff;
        input[1] = 0xff;
        uint16 expected = 65535;
        uint16 actual = scaleCodec.decodeU16(input);
        console.log(actual);
        assert(actual == expected);
    }

    // Test that decodeU32 returns the correct value
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
