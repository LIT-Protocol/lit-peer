const { expect } = require('chai');
const { getParamsFromPKPMint } = require('../../utils');
const { deployDiamond } = require('../../scripts/deployDiamond');
const {
  Environment,
  setContractResolver,
  setupStakingWithValidatorsAndAdvance,
  allNodesVoteForRootKeys,
  getStaticRootKeys,
  sendMetaTransaction,
} = require('../../utils/contract');
const { hexlify } = require('ethers');
const { ethers } = require('hardhat');

describe('PubkeyRouter', function () {
  let deployer;
  let signers;
  let pkpContract;
  let routerDiamond;
  let router;
  let pkpHelper;
  let pkpPermissions;
  let pkpPermissionsDiamond;
  let contractResolver;
  let staking;
  let stakingDiamond;
  let tokenContract;
  let stakingAdminContract;
  let stakingAccounts = [];
  const totalTokens = BigInt('1000000000') * BigInt('10') ** BigInt('18'); // create 1,000,000,000 total tokens with 18 decimals

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
    pkpHelper = await ethers.deployContract('PKPHelper', [
      await contractResolver.getAddress(),
      Environment.DEV,
    ]);
    let deployResult = await deployDiamond(
      'PubkeyRouter',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['PubkeyRouterFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    routerDiamond = deployResult.diamond;
    router = await ethers.getContractAt(
      'PubkeyRouterFacet',
      await routerDiamond.getAddress()
    );
    deployResult = await deployDiamond(
      'PKPPermissions',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['PKPPermissionsFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    pkpPermissionsDiamond = deployResult.diamond;
    pkpPermissions = await ethers.getContractAt(
      'PKPPermissionsFacet',
      await pkpPermissionsDiamond.getAddress()
    );

    tokenContract = await ethers.deployContract('LITToken', [
      ethers.parseUnits('1000000000', 18), // 1b tokens
    ]);

    deployResult = await deployDiamond(
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
          'StakingNFTFacet',
        ],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    stakingDiamond = deployResult.diamond;
    staking = await ethers.getContractAt(
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

    stakingKeySetsFacet = await ethers.getContractAt(
      'StakingKeySetsFacet',
      await stakingDiamond.getAddress()
    );

    keyDeriver = await ethers.deployContract('KeyDeriver');

    // Deploy forwarder contract and set on PubkeyRouter
    const forwarder = await ethers.deployContract('Forwarder');
    await router.setTrustedForwarder(await forwarder.getAddress());

    await setContractResolver(contractResolver, Environment.DEV, {
      tokenContract,
      stakingContract: staking,
      pkpContract,
      pkpHelperContract: pkpHelper,
      pkpPermissionsContract: pkpPermissions,
      hdKeyDeriverContract: keyDeriver,
      pubkeyRouterContract: router,
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
      recoveryPartyMembers: [],
    });

    // Mint enough tokens for the deployer
    await tokenContract.mint(deployer.address, totalTokens);
    realmId = await stakingAdminContract.addRealm(); // this mutates state - when it finished, realmId 1 is created
    realmId = 1;
    stakingAccounts = await setupStakingWithValidatorsAndAdvance(
      ethers,
      staking,
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
  });

  describe('vote for root keys', async () => {
    it('should work using EIP2771 txns', async () => {
      [deployer, ...signers] = signers;

      // vote for the root keys
      let existingRootKeys = await router.getRootKeys(
        await staking.getAddress(),
        'naga-keyset1'
      );
      expect(existingRootKeys.length).to.be.equal(0);

      const rootKeys = getStaticRootKeys(ethers);

      // Set epoch state to NextValidatorSetLocked
      await stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);

      for (let i = 0; i < stakingAccounts.length; i++) {
        const txData = await router.voteForRootKeys.populateTransaction(
          await staking.getAddress(),
          'naga-keyset1',
          rootKeys
        );

        const forwarderAddress = await router.getTrustedForwarder();
        const forwarderContract = (
          await ethers.getContractAt('Forwarder', forwarderAddress)
        ).connect(deployer);

        await sendMetaTransaction(
          ethers,
          txData,
          stakingAccounts[i].nodeAddress,
          forwarderContract,
          await router.getAddress()
        );
      }
    });
  });

  describe('store and retrieve routing data', async () => {
    context('when routing data is unset', async () => {
      beforeEach(async () => {
        router = router.connect(deployer);
      });

      it('retrieves empty routing data', async () => {
        const fakePubkey =
          '0x0443d46287aa31a62f8319438b6210e169fd9e686a11fad81f6cf375e84ed9ba38a3909e41cc52c0c2f2ad95b4cf32982a6295e410b1ff6d455a7c7a4c44463f48';
        const pubkeyHash = ethers.keccak256(fakePubkey);
        const [pubkey, stakingContract, keyType] = await router.getRoutingData(
          pubkeyHash
        );
        expect(pubkey).equal('0x');
        expect(stakingContract).equal(
          '0x0000000000000000000000000000000000000000'
        );
        expect(parseInt(keyType, 16)).equal(0);
      });
    });

    describe('register a PKP and set routing permissions', async function () {
      context(
        'when the PKP is minted, check the ETH address',
        async function () {
          let tester;
          let creator;
          let tokenId;
          let pubkey;
          let rootKeys = [];

          beforeEach(async () => {
            [creator, tester, ...signers] = signers;

            router = await router.connect(deployer);

            // vote for the root keys
            let existingRootKeys = await router.getRootKeys(
              await staking.getAddress(),
              'naga-keyset1'
            );
            expect(existingRootKeys.length).to.be.equal(0);
            rootKeys = await allNodesVoteForRootKeys(
              ethers,
              router,
              stakingAdminContract,
              stakingAccounts,
              deployer
            );

            // mint the PKP to the tester account
            pkpContract = await pkpContract.connect(tester);
            // send eth with the txn
            const mintCost = await pkpContract.mintCost();
            const transaction = {
              value: mintCost,
            };
            const tx = await pkpContract.mintNext(
              2,
              'naga-keyset1',
              transaction
            );
            expect(tx).to.emit(pkpContract, 'PKPMinted');
            let params = await getParamsFromPKPMint(tx, pkpContract);
            tokenId = params.tokenId;
            pubkey = params.pubkey;
            expect(tokenId.toString().length).to.be.greaterThan(0);
            expect(pubkey.length).to.be.equal(132);

            // validate that it was set
            const [pubkeyAfter, keyTypeAfter, _derivedKeyIdAfter] =
              await router.getRoutingData(tokenId);
            expect(pubkeyAfter).equal(pubkey);
            expect(keyTypeAfter).equal(2);

            const owner = await pkpContract.ownerOf(tokenId);
            expect(owner).equal(tester.address);
          });

          it('checks the PKP eth address and the reverse mapping', async () => {
            // validate that the address matches what ethers calculates
            const ethersResult = ethers.computeAddress(pubkey);
            const pubkeyFromContract = await router.getPubkey(tokenId);
            let ethAddressOfPKP = await router.getEthAddress(tokenId);
            expect(ethAddressOfPKP).equal(ethersResult);
            expect(pubkey).equal(pubkeyFromContract);

            // check the reverse mapping
            const tokenIdFromContract = await router.ethAddressToPkpId(
              ethAddressOfPKP
            );
            expect(tokenIdFromContract).equal(tokenId);
          });

          it('gets and sets root keys', async () => {
            const fetchedRootKeys = await router.getRootKeys(
              await staking.getAddress(),
              'naga-keyset1'
            );
            expect(fetchedRootKeys.length).to.be.equal(10);
            for (let i = 0; i < fetchedRootKeys.length; i++) {
              expect(fetchedRootKeys[i][0]).to.be.equal(
                hexlify(rootKeys[i][0])
              );
              expect(rootKeys[i][1]).to.be.equal(rootKeys[i][1]);
            }

            // show that they can't be written again
            await expect(
              router
                .connect(stakingAccounts[0].nodeAddress)
                .voteForRootKeys(
                  await staking.getAddress(),
                  'naga-keyset1',
                  rootKeys
                )
            ).to.be.revertedWith(
              'PubkeyRouter: root keys already set for this staking contract'
            );
          });
        }
      );
    });
  });
});
