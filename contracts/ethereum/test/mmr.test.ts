const { BigNumber } = require('ethers')
const { expect } = require("chai");
const { ethers } = require("hardhat");
const { loadFixture } = require("@nomicfoundation/hardhat-network-helpers");

describe("MMR contract", function () {
  let mmr;
  let MMRWrapper;
  let owner;
  let testValues: any;

  async function createMMR() {
    const MMRLibContract = await ethers.getContractFactory("MMR");
    const MMRlib = await MMRLibContract.deploy();
    MMRWrapper = await ethers.getContractFactory("MMRWrapper", {
      libraries: {
        MMR: MMRlib.address,
      },
    });
    let mmrNew = await MMRWrapper.deploy();
    return mmrNew;
  }

  function sortHashes(arr: any) {
    return arr.sort((n1,n2)=> {
      if (BigNumber.from(n1).lte(BigNumber.from(n2))) {
        return -1;
      } else {
        return 1;
      }
    });
  } 

  async function deployMMRFixture() {
    [owner] = await ethers.getSigners();
    mmr = await createMMR();

    testValues = [
      ethers.utils.formatBytes32String("0x0001"), // index 0
      ethers.utils.formatBytes32String("0x0002"), // index 1
      ethers.utils.formatBytes32String("0x0003"), // index 3
      ethers.utils.formatBytes32String("0x0004"), // index 4
      ethers.utils.formatBytes32String("0x0005"), // index 7
      ethers.utils.formatBytes32String("0x0006"), // index 8
      ethers.utils.formatBytes32String("0x0007"), // index 10
      ethers.utils.formatBytes32String("0x0008"), // index 11
      ethers.utils.formatBytes32String("0x0009"), // index 15
      ethers.utils.formatBytes32String("0x000a"), // index 16
    ];
    for (let i = 0; i < testValues.length; i++) {
      await mmr.append(testValues[i]);
    }
  }

  it("Should construct correct mmr tree", async function () {
    await loadFixture(deployMMRFixture);

    let position0Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(["bytes32"], [testValues[0]])
    );
    expect(position0Value).to.be.equal(await mmr.getNode(0));

    let position1Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(["bytes32"], [testValues[1]])
    );
    expect(position1Value).to.be.equal(await mmr.getNode(1));

    let position3Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(["bytes32"], [testValues[2]])
    );
    expect(position3Value).to.be.equal(await mmr.getNode(3));

    let position4Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(["bytes32"], [testValues[3]])
    );
    expect(position4Value).to.be.equal(await mmr.getNode(4));

    let position7Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(["bytes32"], [testValues[4]])
    );
    expect(position7Value).to.be.equal(await mmr.getNode(7));

    let position8Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(["bytes32"], [testValues[5]])
    );
    expect(position8Value).to.be.equal(await mmr.getNode(8));

    let position10Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(["bytes32"], [testValues[6]])
    );
    expect(position10Value).to.be.equal(await mmr.getNode(10));

    let position11Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(["bytes32"], [testValues[7]])
    );
    expect(position11Value).to.be.equal(await mmr.getNode(11));

    let position15Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(["bytes32"], [testValues[8]])
    );
    expect(position15Value).to.be.equal(await mmr.getNode(15));

    let position16Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(["bytes32"], [testValues[9]])
    );
    expect(position16Value).to.be.equal(await mmr.getNode(16));

    let position2Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        sortHashes([position0Value, position1Value])
      )
    );
    expect(position2Value).to.be.equal(await mmr.getNode(2));
    let position5Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        sortHashes([position3Value, position4Value])
      )
    );
    expect(position5Value).to.be.equal(await mmr.getNode(5));
    let position9Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        sortHashes([position7Value, position8Value])
      )
    );
    expect(position9Value).to.be.equal(await mmr.getNode(9));
    let position12Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        sortHashes([position10Value, position11Value])
      )
    );
    expect(position12Value).to.be.equal(await mmr.getNode(12));
    let position17Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        sortHashes([position15Value, position16Value])
      )
    );
    expect(position17Value).to.be.equal(await mmr.getNode(17));
    let position6Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        sortHashes([position2Value, position5Value])
      )
    );
    expect(position6Value).to.be.equal(await mmr.getNode(6));
    let position13Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        sortHashes([position9Value, position12Value])
      )
    );
    expect(position13Value).to.be.equal(await mmr.getNode(13));
    let position14Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        sortHashes([position6Value, position13Value])
      )
    );
    expect(position14Value).to.be.equal(await mmr.getNode(14));

    const size = 18;
    //keccak256(abi.encodePacked(size, keccak256(abi.encodePacked(size, peaks)))
    let root = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        sortHashes([position14Value, position17Value])
      )
    );

    const data = await mmr.getMerkleProof(1);
    expect(data.root).to.be.equal(root);
  });

  it("Should verify correct proof", async function () {
    await loadFixture(deployMMRFixture);

    // create proof
    const index = 16;
    const data = await mmr.getMerkleProof(index);
    const root = data.root;
    const width = data.width;
    const peakBagging = data.peakBagging;
    const siblings = data.siblings;
    await mmr.verify(
      root,
      width,
      index,
      ethers.utils.formatBytes32String("0x000a"),
      peakBagging,
      siblings
    );
  });

  it("Should raise error on incorrect proof", async function () {
    await loadFixture(deployMMRFixture);

    // create proof
    const index = 16;
    const data = await mmr.getMerkleProof(index);
    const root = data.root;
    const width = data.width;
    const peakBagging = data.peakBagging;
    const siblings = data.siblings;
    await expect(
      mmr.verify(
        root,
        width,
        index,
        ethers.utils.formatBytes32String("0x0009"),
        peakBagging,
        siblings
      )
    ).to.be.revertedWith("Hashed peak is invalid");
  });

  // tests from substrate implementation
  // https://github.com/paritytech/substrate/blob/master/frame/beefy-mmr/primitives/src/lib.rs#L404
  describe("substrate tests", function () {
    async function testMMR(dataArray: any, root: string) {
      let mmrNew = await createMMR();

      for (let i = 0; i < dataArray.length; i++) {
        await mmrNew.append(
          ethers.utils.solidityPack(
            ["bytes"],
            [ethers.utils.hexlify(dataArray[i])]
          )
        );
      }

      let data = await mmrNew.getMerkleProof(0);
      expect(data.root).to.be.equal(root);
    }

    it("should_generate_root_pow_2", async function () {
      await testMMR(
        [
          "0xE04CC55ebEE1cBCE552f250e85c57B70B2E2625b",
          "0x25451A4de12dcCc2D166922fA938E900fCc4ED24",
        ],
        "0x697ea2a8fe5b03468548a7a413424a6292ab44a82a6f5cc594c3fa7dda7ce402"
      );
      // change the node order, should generate the same root
      await testMMR(
        [
          "0x25451A4de12dcCc2D166922fA938E900fCc4ED24",
          "0xE04CC55ebEE1cBCE552f250e85c57B70B2E2625b",
        ],
        "0x697ea2a8fe5b03468548a7a413424a6292ab44a82a6f5cc594c3fa7dda7ce402"
      );
    });

    it("should_generate_root_complex", async function () {
      await testMMR(
        [
          ethers.utils.toUtf8Bytes("a"),
          ethers.utils.toUtf8Bytes("b"),
          ethers.utils.toUtf8Bytes("c"),
        ],
        "0x5842148bc6ebeb52af882a317c765fccd3ae80589b21a9b8cbf21abb630e46a7"
      );
      await testMMR(
        [ 
          ethers.utils.toUtf8Bytes("a"),
          ethers.utils.toUtf8Bytes("b"),
          ethers.utils.toUtf8Bytes("c"),
          ethers.utils.toUtf8Bytes("d"),
          ethers.utils.toUtf8Bytes("e"),
          ethers.utils.toUtf8Bytes("f"),
          ethers.utils.toUtf8Bytes("g"),
          ethers.utils.toUtf8Bytes("h"),
          ethers.utils.toUtf8Bytes("i"),
          ethers.utils.toUtf8Bytes("j")
        ],
        "0xcc50382cfd3c9a617741e9a85efee8752b8feb95a2cbecd6365fb21366ce0c8c"
      );
    });
  });
});
