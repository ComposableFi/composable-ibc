const { expect } = require("chai");
const { ethers } = require("hardhat");
const { loadFixture } = require("@nomicfoundation/hardhat-network-helpers");

describe("MMR contract", function () {
  let mmr;
  let owner;
  let testValues: any;

  async function deployMMRFixture() {
    [owner] = await ethers.getSigners();
    const MMRLibContract = await ethers.getContractFactory("MMR");
    const MMRlib = await MMRLibContract.deploy();
    const MMRWrapper = await ethers.getContractFactory("MMRWrapper", {
      libraries: {
        MMR: MMRlib.address,
      },
    });
    mmr = await MMRWrapper.deploy();

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
  };

  it("Should construct correct mmr tree", async function () {
    await loadFixture(deployMMRFixture);

    let position0Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32"],
        [testValues[0]]
      )
    );
    expect(position0Value).to.be.equal(await mmr.getNode(0));

    let position1Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32"],
        [testValues[1]]
      )
    );
    expect(position1Value).to.be.equal(await mmr.getNode(1));

    let position3Value = ethers.utils.keccak256(
        ethers.utils.solidityPack(
        ["bytes32"],
        [testValues[2]]
      )
    );
    expect(position3Value).to.be.equal(await mmr.getNode(3));

    let position4Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32"],
        [testValues[3]]
      )
    );
    expect(position4Value).to.be.equal(await mmr.getNode(4));

    let position7Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32"],
        [testValues[4]]
      )
    );
    expect(position7Value).to.be.equal(await mmr.getNode(7));

    let position8Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32"],
        [testValues[5]]
      )
    );
    expect(position8Value).to.be.equal(await mmr.getNode(8));

    let position10Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32"],
        [testValues[6]]
      )
    );
    expect(position10Value).to.be.equal(await mmr.getNode(10));

    let position11Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32"],
        [testValues[7]]
      )
    );
    expect(position11Value).to.be.equal(await mmr.getNode(11));

    let position15Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32"],
        [testValues[8]]
      )
    );
    expect(position15Value).to.be.equal(await mmr.getNode(15));

    let position16Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32"],
        [testValues[9]]
      )
    );
    expect(position16Value).to.be.equal(await mmr.getNode(16));

    let position2Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [position0Value, position1Value]
      )
    );
    expect(position2Value).to.be.equal(await mmr.getNode(2));
    let position5Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [position3Value, position4Value]
      )
    );
    expect(position5Value).to.be.equal(await mmr.getNode(5));
    let position9Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [position7Value, position8Value]
      )
    );
    expect(position9Value).to.be.equal(await mmr.getNode(9));
    let position12Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [position10Value, position11Value]
      )
    );
    expect(position12Value).to.be.equal(await mmr.getNode(12));
    let position17Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [position15Value, position16Value]
      )
    );
    expect(position17Value).to.be.equal(await mmr.getNode(17));
    let position6Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [position2Value, position5Value]
      )
    );
    expect(position6Value).to.be.equal(await mmr.getNode(6));
    let position13Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [position9Value, position12Value]
      )
    );
    expect(position13Value).to.be.equal(await mmr.getNode(13));
    let position14Value = ethers.utils.keccak256(
      ethers.utils.solidityPack(
        ["bytes32", "bytes32"],
        [position6Value, position13Value]
      )
    );
    expect(position14Value).to.be.equal(await mmr.getNode(14));

    const size = 18;
    //keccak256(abi.encodePacked(size, keccak256(abi.encodePacked(size, peaks)))
    let root = ethers.utils.keccak256(
            ethers.utils.solidityPack(
              ["bytes32", "bytes32"],
              [position14Value, position17Value]
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
});
