const { BigNumber } = require("ethers");
const { expect } = require("chai");
const { ethers } = require("hardhat");
const { loadFixture } = require("@nomicfoundation/hardhat-network-helpers");

describe("MMR contract", function () {
  let mmp;
  let mmpWrapper;
  let owner;
  let testValues: any;
  let root: any;
  let position_1_0: any;
  let position_1_1: any;
  let position_1_2: any;
  let position_1_3: any;
  let position_1_4: any;
  let position_1_5: any;
  let position_2_0: any;
  let position_2_1: any;
  let position_2_2: any;
  let position_3_0: any;
  let position_3_1: any;


  class Node {
    nodeHash: string;
    index: number;

    public constructor(nodeHash: string, index: number) {
      this.index = index;
      this.nodeHash = nodeHash;
    }
  }

  async function createMMP() {
    const MMPLibContract = await ethers.getContractFactory("MerkleMultiProof");
    const MMPlib = await MMPLibContract.deploy();
    mmpWrapper = await ethers.getContractFactory("MerkleMultiProofWrapper", {
      libraries: {
        MerkleMultiProof: MMPlib.address,
      },
    });
    let mmpNew = await mmpWrapper.deploy();
    return mmpNew;
  }

  async function deployMMRFixture() {
    [owner] = await ethers.getSigners();
    mmp = await createMMP();

    /*               
                     0 
             0              1
                            |
        0         1         2 

     0    1    2    3    4     5  

    0 1  2 3  4 5  6 7  8 9  10 11 
    */
    testValues = [
      new Node(ethers.utils.formatBytes32String("0x0001"), 0),
      new Node(ethers.utils.formatBytes32String("0x0002"), 1),
      new Node(ethers.utils.formatBytes32String("0x0003"), 2),
      new Node(ethers.utils.formatBytes32String("0x0004"), 3),
      new Node(ethers.utils.formatBytes32String("0x0005"), 4),
      new Node(ethers.utils.formatBytes32String("0x0006"), 5),
      new Node(ethers.utils.formatBytes32String("0x0007"), 6),
      new Node(ethers.utils.formatBytes32String("0x0008"), 7),
      new Node(ethers.utils.formatBytes32String("0x0009"), 8),
      new Node(ethers.utils.formatBytes32String("0x000a"), 9),
      new Node(ethers.utils.formatBytes32String("0x000b"), 10),
      new Node(ethers.utils.formatBytes32String("0x000c"), 11),
    ];


    position_1_0 = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [testValues[0].nodeHash, testValues[1].nodeHash]
      )
    );
    position_1_1 = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [testValues[2].nodeHash, testValues[3].nodeHash]
      )
    );

    position_1_2 = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [testValues[4].nodeHash, testValues[5].nodeHash]
      )
    );

    position_1_3 = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [testValues[6].nodeHash, testValues[7].nodeHash]
      )
    );

    position_1_4 = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [testValues[8].nodeHash, testValues[9].nodeHash]
      )
    );

    position_1_5 = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [testValues[10].nodeHash, testValues[11].nodeHash]
      )
    );

    position_2_0 = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [position_1_0, position_1_1]
      )
    );

    position_2_1 = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [position_1_2, position_1_3]
      )
    );

    position_2_2 = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [position_1_4, position_1_5]
      )
    );

    position_3_0 = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [position_2_0, position_2_1]
      )
    );

    position_3_1 = position_2_2;
    root = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [position_3_0, position_3_1]
      )
    );
  }

  it("should create correct proof", async function () {
    await loadFixture(deployMMRFixture);

    /*               
      x leaf
      y proof 
                     0 
             0              1
                            |
        0         1         2 

     0    Y    2    Y    4     Y  

    Y x  2 3  Y x  6 7  Y X  10 11 
    */
    const proofs = [
      [
        {
          node: testValues[0].nodeHash,
          k_index: testValues[0].index,
        },
        {
          node: testValues[4].nodeHash,
          k_index: testValues[4].index,
        },
        {
          node: testValues[8].nodeHash,
          k_index: testValues[8].index,
        },
      ],
      [
        {
          node: position_1_1,
          k_index: 1
        },
        {
          node: position_1_3,
          k_index: 3
        },
        {
          node: position_1_5,
          k_index: 5
        },
      ],
      [],
      []
    ];
    const leaves = [
      {
        node: testValues[1].nodeHash,
        k_index: testValues[1].index,
      },
      {
        node: testValues[5].nodeHash,
        k_index: testValues[5].index,
      },
      {
        node: testValues[9].nodeHash,
        k_index: testValues[9].index,
      },
    ];
    expect(await mmp.verifyProof(root, proofs, leaves)).to.be.true;
  });

  it("should create correct proof 2", async function () {
    await loadFixture(deployMMRFixture);

    /*               
      x leaf
      y proof 
                     0 
             0              1
                            |
        Y         1         2 

     0    1    2    Y    4     Y  

    0 1  2 3  Y x  6 7  Y X  10 11 
    */
    const proofs = [
      [
        {
          node: testValues[4].nodeHash,
          k_index: testValues[4].index,
        },
        {
          node: testValues[8].nodeHash,
          k_index: testValues[8].index,
        },
      ],
      [
        {
          node: position_1_3,
          k_index: 3
        },
        {
          node: position_1_5,
          k_index: 5
        },
      ],
      [
       {
          node: position_2_0,
          k_index: 0
        },

      ],
      []
    ];
    const leaves = [
      {
        node: testValues[5].nodeHash,
        k_index: testValues[5].index,
      },
      {
        node: testValues[9].nodeHash,
        k_index: testValues[9].index,
      },
    ];
    expect(await mmp.verifyProof(root, proofs, leaves)).to.be.true;
  });

  it("should return false on incorrect proof", async function () {
    await loadFixture(deployMMRFixture);

    /*               
      x leaf
      y proof 
                     0 
             0              1
                            |
        0         1         2 

     0    Y    2    Y    4     Y  

    m x  2 3  Y x  6 7  Y X  10 11 
    */
    const proofs = [
      [
        {
          node: testValues[4].nodeHash,
          k_index: testValues[4].index,
        },
        {
          node: testValues[8].nodeHash,
          k_index: testValues[8].index,
        },
      ],
      [
        {
          node: position_1_1,
          k_index: 1
        },
        {
          node: position_1_3,
          k_index: 3
        },
        {
          node: position_1_5,
          k_index: 5
        },
      ],
      [],
      []
    ];
    const leaves = [
      {
        node: testValues[1].nodeHash,
        k_index: testValues[1].index,
      },
      {
        node: testValues[5].nodeHash,
        k_index: testValues[5].index,
      },
      {
        node: testValues[9].nodeHash,
        k_index: testValues[9].index,
      },
    ];
    expect(await mmp.verifyProof(root, proofs, leaves)).to.be.false;
  });
});
