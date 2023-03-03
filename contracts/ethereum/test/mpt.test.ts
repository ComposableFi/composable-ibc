const { expect } = require("chai");
const { ethers } = require("hardhat");
const { loadFixture } = require("@nomicfoundation/hardhat-network-helpers");

describe("MMR contract", function () {
  let mpt;
  let MPTWrapper;
  let owner;
  let testValues: any;

  async function createMPT() {
    //const MPTLibContract = await ethers.getContractFactory("MPT");
    //const MPTlib = await MPTLibContract.deploy();
    //MPTWrapper = await ethers.getContractFactory("MPTWrapper", {
    //});
    //const mptNew = await MPTWrapper.deploy();
    //return mptNew;
    /*
    const TrieProofsLibContract = await ethers.getContractFactory("TrieProofs");
    const TrieProofsLib = await TrieProofsLibContract.deploy();
    const TrieProofWrapper = await ethers.getContractFactory("TrieProofsWrapper", {
      libraries: {
      }
    });
    const mptNew = await TrieProofWrapper.deploy();
    return mptNew;
    */
    const NibbleSliceOps = await ethers.getContractFactory("NibbleSliceOps");
    const nibbleSliceOps = await NibbleSliceOps.deploy();
    const Option = await ethers.getContractFactory("Option");
    console.log(11)
    const option = await Option.deploy();
    const TrieDB = await ethers.getContractFactory("TrieDB");
    console.log(11)
    const trieDB = await TrieDB.deploy();
    console.log(11)
    const EthereumTrieDB = await ethers.getContractFactory("EthereumTrieDB");
    console.log(11)
    const ethereumTrieDB = await EthereumTrieDB.deploy();
    console.log(11)
    const SubstrateTrieDB = await ethers.getContractFactory("SubstrateTrieDB");
    const substrateTrieDB = await SubstrateTrieDB.deploy();

    const MerklePatricia = await ethers.getContractFactory("MerklePatricia",
    {
      libraries: {
        NibbleSliceOps:  nibbleSliceOps.address,
        Option: option.address,
        TrieDB: trieDB.address,
        EthereumTrieDB: ethereumTrieDB.address,
        SubstrateTrieDB: substrateTrieDB.address
      }
    });
    const merklePatricia = await MerklePatricia.deploy();
    const MPTWrapper = await ethers.getContractFactory("TrieProofWrapper", {
      libraries: {
        MerklePatricia: merklePatricia.address
      }
    });
    const mptNew = await MPTWrapper.deploy();
    return mptNew
  }

  it.only("Should verify correctly mpt tree", async function () {
    mpt = await createMPT();
    let hexKey = "0x646f66"
    let hexProof = ["0x802e98809b03c6ae83e3b70aa89acfe0947b3a18b5d35569662335df7127ab8fcb88c88780e5d1b21c5ecc2891e3467f6273f27ce2e73a292d6b8306197edfa97b3d965bd080c51e5f53a03d92ea8b2792218f152da738b9340c6eeb08581145825348bbdba480ad103a9320581c7747895a01d79d2fa5f103c4b83c5af10b0a13bc1749749523806eea23c0854ced8445a3338833e2401753fdcfadb3b56277f8f1af4004f73719806d990657a5b5c3c97b8a917d9f153cafc463acd90592f881bc071d6ba64e90b380346031472f91f7c44631224cb5e61fb29d530a9fafd5253551cbf43b7e97e79a",
                "0x9f00c365c3cf59d671eb72da0e7a4113c41002505f0e7b9012096b41c4eb3aaf947f6ea429080000685f0f1f0515f462cdcf84e0f1d6045dfcbb2035e90c7f86010000"]
    let state_root = "0x6b5710000eccbd59b6351fc2eb53ff2c1df8e0f816f7186ddd309ca85e8798dd"
    await mpt.verify(state_root, hexProof, [hexKey])
  });
});
