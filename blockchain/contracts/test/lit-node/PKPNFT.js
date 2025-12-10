const { expect } = require('chai');
const { ethers } = require('hardhat');
const { getBytesFromMultihash, getParamsFromPKPMint } = require('../../utils');
const { deployDiamond } = require('../../scripts/deployDiamond');
const {
  Environment,
  setContractResolver,
  setupStakingWithValidatorsAndAdvance,
  allNodesVoteForRootKeys,
} = require('../../utils/contract');

describe('PKPNFT', function () {
  let deployer;
  let signers;
  let pkpContract;
  let router;
  let routerViews;
  let pkpPermissions;
  let pkpNftMetadata;
  let contractResolver;
  let stakingContract;
  let tokenContract;
  let stakingAccounts = [];
  let realmId;
  const totalTokens = BigInt('1000000000') * BigInt('10') ** BigInt('18'); // create 1,000,000,000 total tokens with 18 decimals

  before(async () => {
    // Validation
    if (
      supportsArbitrumStylus(hre.network.config) &&
      (!hre.network.config.stylusContractsForTests.p256 ||
        !hre.network.config.stylusContractsForTests.k256)
    ) {
      console.log(
        'Please set the Stylus contract addresses using env variables defined in the hardhat.config.ts file'
      );
      process.exit(1);
    }
  });

  beforeEach(async () => {
    [deployer, ...signers] = await ethers.getSigners();

    contractResolver = await ethers.deployContract('ContractResolver', [
      Environment.DEV,
    ]);

    const { diamond: pkpDiamond } = await deployDiamond(
      'PKPNFT',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['PKPNFTFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    pkpContract = await ethers.getContractAt(
      'PKPNFTFacet',
      await pkpDiamond.getAddress()
    );
    const { diamond: routerDiamond } = await deployDiamond(
      'PubkeyRouter',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['PubkeyRouterFacet', 'PubkeyRouterViewsFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    router = await ethers.getContractAt(
      'PubkeyRouterFacet',
      await routerDiamond.getAddress()
    );
    routerViews = await ethers.getContractAt(
      'PubkeyRouterViewsFacet',
      await routerDiamond.getAddress()
    );
    const { diamond: pkpPermissionsDiamond } = await deployDiamond(
      'PKPPermissions',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['PKPPermissionsFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    pkpPermissions = await ethers.getContractAt(
      'PKPPermissionsFacet',
      await pkpPermissionsDiamond.getAddress()
    );
    pkpNftMetadata = await ethers.deployContract('PKPNFTMetadata', [
      await contractResolver.getAddress(),
      Environment.DEV,
    ]);
    tokenContract = await ethers.deployContract(
      'LITToken',
      [ethers.parseUnits('1000000000', 18)] // 1b tokens
    );

    const { diamond: stakingDiamond } = await deployDiamond(
      'Staking',
      await contractResolver.getAddress(),
      0,
      {
        additionalFacets: [
          'StakingFacet',
          'StakingValidatorFacet',
          'StakingViewsFacet',
          'StakingVersionFacet',
          'StakingAcrossRealmsFacet',
          'StakingAdminFacet',
          'StakingKeySetsFacet',
        ],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    stakingContract = await ethers.getContractAt(
      'StakingFacet',
      await stakingDiamond.getAddress()
    );

    stakingAdminContract = await ethers.getContractAt(
      'StakingAdminFacet',
      await stakingDiamond.getAddress()
    );

    stakingValidatorFacet = await ethers.getContractAt(
      'StakingValidatorFacet',
      await stakingDiamond.getAddress()
    );

    stakingAdminContract = await ethers.getContractAt(
      'StakingAdminFacet',
      await stakingDiamond.getAddress()
    );

    stakingKeySetsFacet = await ethers.getContractAt(
      'StakingKeySetsFacet',
      await stakingDiamond.getAddress()
    );

    if (hre.network.name === 'localchainArbitrum') {
      keyDeriver = await ethers.deployContract('ArbitrumKeyDeriver', [
        await contractResolver.getAddress(),
        Environment.DEV,
      ]);
    } else {
      keyDeriver = await ethers.deployContract('KeyDeriver');
    }

    await setContractResolver(contractResolver, Environment.DEV, {
      tokenContract,
      stakingContract,
      pkpContract,
      pkpPermissionsContract: pkpPermissions,
      pkpNftMetadataContract: pkpNftMetadata,
      hdKeyDeriverContract: keyDeriver,
      pubkeyRouterContract: router,
      pubkeyRouterViewsContract: routerViews,
      stylusContractP256: supportsArbitrumStylus(hre.network.config)
        ? hre.network.config.stylusContractsForTests.p256
        : undefined,
      stylusContractK256: supportsArbitrumStylus(hre.network.config)
        ? hre.network.config.stylusContractsForTests.k256
        : undefined,
    });

    await stakingKeySetsFacet.setKeySet({
      minimumThreshold: 3,
      monetaryValue: 0,
      completeIsolation: false,
      identifier: 'naga-keyset1',
      description: '',
      realms: [1],
      curves: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
      counts: [1, 2, 2, 2, 2, 2, 2, 2, 2, 2],
      recoverySessionId: '0x',
    });

    // Mint enough tokens for the deployer
    await tokenContract.mint(deployer.address, totalTokens);
    realmId = await stakingAdminContract.addRealm(); // this mutates state - when it finished, realmId 1 is created
    realmId = 1;
    stakingAccounts = await setupStakingWithValidatorsAndAdvance(
      ethers,
      stakingContract,
      stakingValidatorFacet,
      stakingAdminContract,
      tokenContract,
      deployer,
      {
        numValidators: 3,
        startingPort: 7777,
        ipAddress: '192.168.1.1',
      }
    );
    await allNodesVoteForRootKeys(
      ethers,
      router,
      stakingAdminContract,
      stakingAccounts,
      deployer
    );
  });

  describe('Attempt to Mint PKP NFT', async () => {
    let minter;

    beforeEach(async function () {
      [minter, ...signers] = signers;
      pkpContract = pkpContract.connect(minter);
    });

    it('refuses to mint for free', async () => {
      await expect(pkpContract.mintNext(2, 'naga-keyset1')).revertedWith(
        'You must pay exactly mint cost'
      );
    });

    it('mints successfully', async () => {
      // send eth with the txn
      const mintCost = await pkpContract.mintCost();
      const transaction = {
        value: mintCost,
      };

      console.log('mintCost', mintCost);

      const tx = await pkpContract.mintNext(2, 'naga-keyset1', transaction);
      expect(tx).to.emit(pkpContract, 'PKPMinted');
      const { tokenId, pubkey } = await getParamsFromPKPMint(tx, pkpContract);

      console.log('tokenId', tokenId);
      console.log('pubkey', pubkey);

      // check the token was minted
      const owner = await pkpContract.ownerOf(tokenId);
      expect(owner).to.equal(minter.address);

      // check the metadata
      const pkpEthAddress = await pkpContract.getEthAddress(tokenId);

      const tokenUri = await pkpContract.tokenURI(tokenId);
      const metadata = tokenUri.substring(29);
      const decodedUint8Array = ethers.decodeBase64(metadata);
      const decoded = ethers.toUtf8String(decodedUint8Array);
      const parsed = JSON.parse(decoded);

      expect(parsed['name']).to.equal('Lit PKP #' + tokenId.toString());
      expect(parsed['attributes'][0]['value']).to.equal(pubkey);
      expect(parsed['attributes'][1]['value'].toLowerCase()).to.equal(
        pkpEthAddress.toLowerCase()
      );
      expect(parsed['attributes'][2]['value']).to.equal(tokenId.toString());
    });

    it('mints 50 multiple tokens successfully', async () => {
      // send eth with the txn
      const mintCost = await pkpContract.mintCost();
      const transaction = {
        value: mintCost,
      };

      const batchSize = 50;
      console.log('mintCost', mintCost);

      const tokenIds = [];

      for (let i = 0; i < batchSize; i++) {
        const tx = await pkpContract.mintNext(2, 'naga-keyset1', transaction);
        expect(tx).to.emit(pkpContract, 'PKPMinted');
        const { tokenId, pubkey } = await getParamsFromPKPMint(tx, pkpContract);

        console.log('tokenId', tokenId);
        console.log('pubkey', pubkey);

        // check the token was minted
        const owner = await pkpContract.ownerOf(tokenId);

        tokenIds.push(tokenId);
      }

      let owner_address = await pkpContract.ownerOf(tokenIds[0]);

      const pkpInfos3 = await pkpContract.getPkpInfoFromOwnerAddress(
        owner_address,
        batchSize,
        0
      );

      expect(pkpInfos3.length).to.equal(batchSize);
      for (let i = 0; i < pkpInfos3.length; i++) {
        expect(pkpInfos3[i].tokenId).to.equal(tokenIds[i]);
      }

      const pkpInfos4 = await pkpContract.getPkpInfoFromOwnerTokenId(
        tokenIds[0],
        batchSize,
        0
      );
      expect(pkpInfos4.length).to.equal(batchSize);
      for (let i = 0; i < pkpInfos4.length; i++) {
        expect(pkpInfos4[i].tokenId).to.equal(tokenIds[i]);
      }

      const pkpInfos5 = await pkpContract.getPkpInfoFromOwnerTokenId(
        tokenIds[0],
        10,
        1
      );

      expect(pkpInfos5.length).to.equal(10);
      for (let i = 0; i < pkpInfos5.length; i++) {
        expect(pkpInfos5[i].tokenId).to.equal(tokenIds[i + 10]);
      }

      // to be efficient, we can get the eth addresses from the token ids from the first validation...
      const ethAddresses = [];
      const pkpInfos = await pkpContract.getPkpInfoFromTokenIds(tokenIds);
      expect(pkpInfos.length).to.equal(batchSize);
      for (let i = 0; i < pkpInfos.length; i++) {
        ethAddresses.push(pkpInfos[i].ethAddress);
        expect(pkpInfos[i].tokenId).to.equal(tokenIds[i]);
      }

      const pkpInfos2 = await pkpContract.getPkpInfoFromEthAddresses(
        ethAddresses
      );
      expect(pkpInfos2.length).to.equal(batchSize);
      for (let i = 0; i < pkpInfos2.length; i++) {
        expect(pkpInfos2[i].tokenId).to.equal(pkpInfos[i].tokenId);
        expect(pkpInfos2[i].pubkey).to.equal(pkpInfos[i].pubkey);
        expect(pkpInfos2[i].ethAddress).to.equal(pkpInfos[i].ethAddress);
      }
    });
  });

  describe('Attempt to claim derived PKP NFT', async () => {
    let minter;

    beforeEach(async function () {
      [minter, ...signers] = signers;
      pkpContract = pkpContract.connect(minter);
    });

    it('mints successfully', async () => {
      // send eth with the txn
      const mintCost = await pkpContract.mintCost();
      const transaction = {
        value: mintCost,
      };

      const derivedKeyId = ethers.randomBytes(32);
      const sigs = await Promise.all(
        stakingAccounts.map(async (stakingAccount) =>
          ethers.Signature.from(
            await stakingAccount.nodeAddress.signMessage(derivedKeyId)
          )
        )
      );
      const tx = await pkpContract.claimAndMint(
        realmId,
        2,
        'naga-keyset1',
        derivedKeyId,
        sigs,
        stakingContract.getAddress(),
        transaction
      );

      expect(tx).to.emit(pkpContract, 'PKPMinted');
      const { tokenId, pubkey } = await getParamsFromPKPMint(tx, pkpContract);

      // check the token was minted
      const owner = await pkpContract.ownerOf(tokenId);
      expect(owner).to.equal(minter.address);

      // check the metadata
      const pkpEthAddress = await pkpContract.getEthAddress(tokenId);

      const tokenUri = await pkpContract.tokenURI(tokenId);
      const metadata = tokenUri.substring(29);
      const decodedUint8Array = ethers.decodeBase64(metadata);
      const decoded = ethers.toUtf8String(decodedUint8Array);
      const parsed = JSON.parse(decoded);

      expect(parsed['name']).to.equal('Lit PKP #' + tokenId.toString());
      expect(parsed['attributes'][0]['value']).to.equal(pubkey);
      expect(parsed['attributes'][1]['value'].toLowerCase()).to.equal(
        pkpEthAddress.toLowerCase()
      );
      expect(parsed['attributes'][2]['value']).to.equal(tokenId.toString());
    });
  });

  describe('Test Mint Grant And Burn', async () => {
    let minter;

    beforeEach(async function () {
      [minter, ...signers] = signers;
      pkpContract = pkpContract.connect(minter);
    });

    it('mints, grants, and burns successfully', async () => {
      // send eth with the txn
      const mintCost = await pkpContract.mintCost();
      const transaction = {
        value: mintCost,
      };

      const ipfsIdToPermit = 'QmW6uH8p17DcfvZroULkdEDAKThWzEDeNtwi9oezURDeXN';
      const ipfsIdBytes = getBytesFromMultihash(ipfsIdToPermit);

      const tx = await pkpContract.mintGrantAndBurnNext(
        2,
        'naga-keyset1',
        ipfsIdBytes,
        transaction
      );
      expect(tx).to.emit(pkpContract, 'PKPMinted');
      const { tokenId, pubkey } = await getParamsFromPKPMint(tx, pkpContract);
      expect(tokenId.toString().length).to.be.greaterThan(0);
      expect(pubkey.length).to.be.equal(132);

      // check the token was minted and burned
      await expect(pkpContract.ownerOf(tokenId)).revertedWith(
        'ERC721: invalid token ID'
      );

      const actionIsPermitted = await pkpPermissions.isPermittedAction(
        tokenId,
        ipfsIdBytes
      );

      expect(actionIsPermitted).to.equal(true);
    });
  });
});

function supportsArbitrumStylus(hreNetworkConfig) {
  return !!hreNetworkConfig.stylusContractsForTests;
}
