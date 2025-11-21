const chai = require('chai');

const { BigNumber, utils } = require('ethers-v5');
const { ip2int, int2ip } = require('../../utils');
const {
  Environment,
  setContractResolver,
  setupStakingWithValidatorsAndAdvance,
  StakingState,
  createValidatorAndStake,
  sendMetaTransaction,
} = require('../../utils/contract');
const { expect } = chai;
const { deployDiamond } = require('../../scripts/deployDiamond');
const { toBigInt } = require('ethers');

describe('Staking', function () {
  let deployer;
  let signers;
  let token;
  let routerContract;
  let routerViews;
  let pkpNft;
  let stakingAccount1;
  let nodeAccount1;
  let stakingAccount2;
  let nodeAccount2;
  let stakingValidatorFacet;
  let stakingViewsFacet;
  let stakingVersionFacet;
  let ownershipFacet;
  let contractResolver;
  let minStake;
  let stakingAccounts = [];
  let nodeConnectionInfo;
  const totalTokens = BigInt('1000000000') * BigInt('10') ** BigInt('18'); // create 1,000,000,000 total tokens with 18 decimals
  const stakingAccount1IpAddress = '192.168.1.1';
  const stakingAccount1Port = 7777;
  const stakingAccount2IpAddress = '192.168.1.2';
  const stakingAccount2Port = 7777;
  const stakingAccountCount = 10;
  let snapshotId;
  let realmId;

  before(async function () {
    contractResolver = await ethers.deployContract('ContractResolver', [
      Environment.DEV,
    ]);

    // deploy token
    [
      deployer,
      stakingAccount1,
      nodeAccount1,
      stakingAccount2,
      nodeAccount2,
      ...signers
    ] = await ethers.getSigners();
    token = await ethers.deployContract(
      'LITToken',
      [ethers.parseUnits('1000000000', 18)] // 1b tokens
    );
    token = token.connect(deployer);

    // deploy staking balances
    // deprecated in favor of Thunderhead changes to staking contract

    // deploy staking contract
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
          'StakingKeySetsFacet',
          'StakingAdminFacet',
        ],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );

    stakingFacet = await ethers.getContractAt(
      'StakingFacet',
      await stakingDiamond.getAddress()
    );
    stakingValidatorFacet = await ethers.getContractAt(
      'StakingValidatorFacet',
      await stakingDiamond.getAddress()
    );

    stakingAdminFacet = await ethers.getContractAt(
      'StakingAdminFacet',
      await stakingDiamond.getAddress()
    );

    stakingViewsFacet = await ethers.getContractAt(
      'StakingViewsFacet',
      await stakingDiamond.getAddress()
    );
    stakingVersionFacet = await ethers.getContractAt(
      'StakingVersionFacet',
      await stakingDiamond.getAddress()
    );

    stakingKeySetsFacet = await ethers.getContractAt(
      'StakingKeySetsFacet',
      await stakingDiamond.getAddress()
    );

    ownershipFacet = await ethers.getContractAt(
      'OwnershipFacet',
      await stakingDiamond.getAddress()
    );

    // deploy pkpNft
    const { diamond: pkpDiamond } = await deployDiamond(
      'PKPNFT',
      await contractResolver.getAddress(),
      0,
      {
        additionalFacets: ['PKPNFTFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    pkpNft = await ethers.getContractAt(
      'PKPNFTFacet',
      await pkpDiamond.getAddress()
    );

    // deploy router
    let deployResult = await deployDiamond(
      'PubkeyRouter',
      await contractResolver.getAddress(),
      0,
      {
        additionalFacets: ['PubkeyRouterFacet', 'PubkeyRouterViewsFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    routerDiamond = deployResult.diamond;
    routerContract = await ethers.getContractAt(
      'PubkeyRouterFacet',
      await routerDiamond.getAddress()
    );
    routerViews = await ethers.getContractAt(
      'PubkeyRouterViewsFacet',
      await routerDiamond.getAddress()
    );

    // Deploy forwarder contract and set on Staking
    const forwarder = await ethers.deployContract('Forwarder');
    await stakingFacet.setTrustedForwarder(await forwarder.getAddress());

    await setContractResolver(contractResolver, Environment.DEV, {
      tokenContract: token,
      stakingContract: stakingValidatorFacet,
      pkpContract: pkpNft,
      pubkeyRouterContract: routerContract,
      pubkeyRouterViewsContract: routerViews,
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

    await token.mint(deployer.address, totalTokens);

    console.log('Setting up staking with validators and advancing');
    realmId = await stakingAdminFacet.addRealm(); // this mutates state - when it finished, realmId 1 is created
    realmId = 1;

    stakingAccounts = await setupStakingWithValidatorsAndAdvance(
      ethers,
      stakingFacet,
      stakingValidatorFacet,
      stakingAdminFacet,
      token,
      deployer,
      {
        numValidators: stakingAccountCount,
        startingPort: stakingAccount1Port,
        ipAddress: stakingAccount1IpAddress,
      }
    );

    // fund stakingAccount1, stakingAccount2 with tokens
    minStake = await stakingFacet.getMinimumSelfStake();
    console.log('minStake: %s', minStake);
    const totalToStake = minStake * 10n; // 10 times the minimum stake
    await token.transfer(stakingAccount1.address, totalToStake);
    await token
      .connect(stakingAccount1)
      .approve(await stakingFacet.getAddress(), totalToStake);
    await token.transfer(stakingAccount2.address, totalToStake);
    await token
      .connect(stakingAccount2)
      .approve(await stakingFacet.getAddress(), totalToStake);
  });

  beforeEach(async () => {
    if (snapshotId) {
      await network.provider.send('evm_revert', [snapshotId]);
    }
    snapshotId = await network.provider.send('evm_snapshot');
  });

  describe('Constructor & Settings', function () {
    it('should set owner on constructor', async function () {
      const ownerAddress = await ownershipFacet.owner();
      expect(ownerAddress, deployer).is.equal;
    });
  });

  describe('querying state', () => {
    it('has the default validator set', async () => {
      console.log('realmId: %s', realmId);
      const validators = await stakingViewsFacet.getValidatorsInCurrentEpoch(
        realmId
      );
      console.log('validators: %s', validators);
      expect(validators.length).equal(10);
    });

    it('works with all the batch validator retrieval methods', async () => {
      const validatorsStructs =
        await stakingViewsFacet.getValidatorsStructsInCurrentEpoch(realmId);
      expect(validatorsStructs.length).equal(10);
      const validatorStakingAddresses =
        await stakingViewsFacet.getValidatorsInCurrentEpoch(realmId);
      expect(validatorStakingAddresses.length).equal(10);
      const ipAddress = ip2int(stakingAccount1IpAddress);
      for (let i = 0; i < validatorsStructs.length; i++) {
        const validator = validatorsStructs[i];
        const balance = await stakingFacet.balanceOf(
          validatorStakingAddresses[i]
        );
        expect(validator.nodeAddress).equal(
          await stakingAccounts[i].nodeAddress.address
        );
        expect(validator.ip).equal(ipAddress);
        expect(validator.port).equal(stakingAccount1Port + i + 1);
        expect(balance).equal(minStake);
      }
      nodeConnectionInfo =
        await stakingViewsFacet.getActiveUnkickedValidatorStructsAndCounts(
          realmId
        );
      console.log(
        `nodeConnectionInfo: ${JSON.stringify(
          nodeConnectionInfo,
          (key, value) => (typeof value === 'bigint' ? value.toString() : value)
        )}`
      );

      const epoch = nodeConnectionInfo[0];
      const currentValidatorCountForConsensus = nodeConnectionInfo[1];
      const validators = nodeConnectionInfo[2];
      expect(epoch.number).equal(2);
      expect(currentValidatorCountForConsensus).equal(6);
      expect(validators.length).equal(10);
      for (let i = 0; i < validators.length; i++) {
        const validator = validators[i];
        const balance = await stakingFacet.balanceOf(
          validatorStakingAddresses[i]
        );
        expect(validator.nodeAddress).equal(
          await stakingAccounts[i].nodeAddress.address
        );
        expect(validator.ip).equal(ipAddress);
        expect(validator.port).equal(stakingAccount1Port + i + 1);
        expect(balance).equal(minStake);
      }
    });
  });

  describe('invalid staking scenarios', function () {
    it('cannot stake 0', async () => {
      stakingFacet = stakingFacet.connect(stakingAccount1);
      await expect(
        stakingFacet.stake(0, 24 * 60 * 60 * 120, stakingAccount1.address)
      ).revertedWithCustomError(stakingFacet, 'StakeAmountNotMet');
    });

    it('cannot stake less than the minimum stake', async () => {
      stakingFacet = stakingFacet.connect(stakingAccount2);
      token = token.connect(stakingAccount2);
      await token.approve(await stakingFacet.getAddress(), minStake);

      await expect(
        stakingFacet.stake(
          minStake - 1n,
          24 * 60 * 60 * 120,
          stakingAccount2.address
        )
      ).revertedWithCustomError(stakingFacet, 'StakeAmountNotMet');
    });
  });

  describe('validator joining', function () {
    it('can join as a validator, and cannot leave as a staker if below min validator count', async () => {
      let initialStakeBal = await stakingFacet.balanceOf(
        stakingAccount1.address
      );
      let initialTokenBalance = await token.balanceOf(stakingAccount1.address);
      let initialValidatorEntry = await stakingViewsFacet.validators(
        stakingAccount1.address
      );
      const initialIpAddress = initialValidatorEntry.ip;
      const initialPort = initialValidatorEntry.port;
      const initialNodeAddresss = initialValidatorEntry.nodeAddress;
      const initialSenderPubKey = initialValidatorEntry.senderPubKey;
      const initialReceiverPubKey = initialValidatorEntry.receiverPubKey;
      let initialReward = initialValidatorEntry.reward;
      const initialNodeAddressToStakerAddress =
        await stakingViewsFacet.nodeAddressToStakerAddress(
          nodeAccount1.address
        );

      // generate new unused communication keys
      const communicationSenderPubKey = toBigInt(utils.randomBytes(32));
      const communicationReceiverPubKey = toBigInt(utils.randomBytes(32));
      const timelock = 24 * 60 * 60 * 120; // 120 days

      // permit the staker
      stakingFacet = stakingFacet.connect(deployer);
      await stakingAdminFacet.setPermittedValidators(realmId, [
        stakingAccount1.address,
      ]);

      // set ip port node address and comms keys
      stakingValidatorFacet = stakingValidatorFacet.connect(nodeAccount1);
      await stakingValidatorFacet
        .connect(stakingAccount1)
        .setIpPortNodeAddress(
          ip2int(stakingAccount1IpAddress),
          0,
          stakingAccount1Port,
          nodeAccount1.address
        );

      await stakingFacet
        .connect(stakingAccount1)
        .stake(minStake, timelock, stakingAccount1.address);

      await stakingValidatorFacet.registerAttestedWallet(
        stakingAccount1.address,
        nodeAccount1.address,
        new Uint8Array([
          4, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
          2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
          2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        ]),
        communicationSenderPubKey,
        communicationReceiverPubKey
      );

      stakingValidatorFacet = stakingValidatorFacet.connect(stakingAccount1);
      await stakingValidatorFacet.requestToJoin(realmId);

      let postStakeBal = await stakingFacet.balanceOf(stakingAccount1.address);
      let postTokenBalance = await token.balanceOf(stakingAccount1.address);
      let postValidatorEntry = await stakingViewsFacet.validators(
        stakingAccount1.address
      );
      const postIpAddress = postValidatorEntry.ip;
      const postPort = postValidatorEntry.port;
      const postNodeAddress = postValidatorEntry.nodeAddress;
      const postSenderPubKey = postValidatorEntry.senderPubKey;
      const postReceiverPubKey = postValidatorEntry.receiverPubKey;
      let postBalance = await stakingFacet.balanceOf(stakingAccount1.address);
      let postReward = postValidatorEntry.reward;
      let postNodeAddressToStakerAddress =
        await stakingViewsFacet.nodeAddressToStakerAddress(
          nodeAccount1.address
        );

      expect(postTokenBalance).to.be.lt(initialTokenBalance);
      expect(postStakeBal).to.be.gt(initialStakeBal);
      expect(initialIpAddress).to.equal(0);
      expect(int2ip(postIpAddress)).to.equal(stakingAccount1IpAddress);
      expect(initialPort).to.equal(0);
      expect(postPort).to.equal(stakingAccount1Port);
      expect(initialNodeAddresss).to.equal(
        '0x0000000000000000000000000000000000000000'
      );
      expect(postNodeAddress).to.equal(await nodeAccount1.address);
      expect(initialSenderPubKey).to.equal(0);
      expect(postSenderPubKey).to.be.equal(communicationSenderPubKey);
      expect(initialReceiverPubKey).to.equal(0);
      expect(postReceiverPubKey).to.equal(communicationReceiverPubKey);
      expect(initialStakeBal).to.equal(0);
      expect(postBalance).to.equal(minStake);
      expect(initialReward).to.equal(0);
      expect(postReward).to.equal(0);

      expect(initialNodeAddressToStakerAddress).to.equal(
        '0x0000000000000000000000000000000000000000'
      );
      expect(postNodeAddressToStakerAddress).to.equal(
        await stakingAccount1.address
      );

      // turn off permitted stakers
      await stakingAdminFacet.setPermittedValidatorsOn(realmId, false);

      // set min validator count to 11 which is the current validator count
      stakingValidatorFacet = stakingValidatorFacet.connect(deployer);

      await updateMinimumValidatorCount(
        stakingViewsFacet,
        stakingAdminFacet,
        11n
      );

      // we will leave with stakingAccount1
      token = token.connect(stakingAccount1);
      stakingValidatorFacet = stakingValidatorFacet.connect(stakingAccount1);

      initialStakeBal = await stakingFacet.balanceOf(stakingAccount1.address);
      initialTokenBalance = await token.balanceOf(stakingAccount1.address);
      initialValidatorEntry = await stakingViewsFacet.validators(
        stakingAccount1.address
      );
      const initialBalance = await stakingFacet.balanceOf(
        stakingAccount1.address
      );
      initialReward = initialValidatorEntry.reward;

      await expect(
        stakingValidatorFacet.requestToLeave()
      ).revertedWithCustomError(
        stakingValidatorFacet,
        'NotEnoughValidatorsInNextEpoch'
      );

      stakingValidatorFacet = stakingValidatorFacet.connect(deployer);
      await updateMinimumValidatorCount(
        stakingViewsFacet,
        stakingAdminFacet,
        7n
      );

      postStakeBal = await stakingFacet.balanceOf(stakingAccount1.address);
      postTokenBalance = await token.balanceOf(stakingAccount1.address);
      postValidatorEntry = await stakingViewsFacet.validators(
        stakingAccount1.address
      );

      postBalance = await stakingFacet.balanceOf(stakingAccount1.address);
      postReward = postValidatorEntry.reward;
      postNodeAddressToStakerAddress =
        await stakingViewsFacet.nodeAddressToStakerAddress(
          nodeAccount1.address
        );

      expect(postTokenBalance).to.equal(initialTokenBalance);
      expect(postStakeBal).to.equal(initialStakeBal);
      expect(initialBalance).to.equal(postBalance);
      expect(postBalance).to.equal(minStake);
      expect(initialReward).to.equal(0);
      expect(postReward).to.equal(0);
      expect(postNodeAddressToStakerAddress).to.equal(
        await stakingAccount1.address
      );
    });

    it('cannot reuse the same comms keys', async function () {
      // Bypass disabled flag for registering attested wallets
      await stakingAdminFacet.adminSetValidatorRegisterAttestedWalletDisabled(
        stakingAccounts[0].stakingAddress.address,
        false
      );
      await stakingAdminFacet.adminSetValidatorRegisterAttestedWalletDisabled(
        stakingAccounts[1].stakingAddress.address,
        false
      );

      // Staker1 first registers keys
      await stakingValidatorFacet
        .connect(stakingAccounts[0].nodeAddress)
        .registerAttestedWallet(
          stakingAccounts[0].stakingAddress.address,
          stakingAccounts[0].nodeAddress.address,
          new Uint8Array([4, ...ethers.randomBytes(64)]),
          1n,
          1n
        );

      // Then, Staker2 tries to register the same comms keys
      await expect(
        stakingValidatorFacet
          .connect(stakingAccounts[1].nodeAddress)
          .registerAttestedWallet(
            stakingAccounts[1].stakingAddress.address,
            stakingAccounts[1].nodeAddress.address,
            new Uint8Array([4, ...ethers.randomBytes(64)]),
            1n,
            1n
          )
      )
        .to.be.revertedWithCustomError(
          stakingValidatorFacet,
          'CannotReuseCommsKeys'
        )
        .withArgs(1n, 1n);
    });

    it('works in all scenarios with kicked nodes', async function () {
      const validatorsInNextEpochBeforeTest =
        await stakingViewsFacet.getValidatorsInNextEpoch(realmId);
      expect(validatorsInNextEpochBeforeTest.length).equal(10);

      let [epoch, currentValidatorCountForConsensus, allStructs] =
        await stakingViewsFacet.getActiveUnkickedValidatorStructsAndCounts(
          realmId
        );
      expect(allStructs.length).equal(10);

      const stakingAsAdmin = stakingAdminFacet.connect(deployer);

      // kick the first 3 nodes
      for (let i = 0; i < 3; i++) {
        const stakingAddress =
          await stakingViewsFacet.nodeAddressToStakerAddress(
            allStructs[i].nodeAddress
          );
        await stakingAsAdmin.adminKickValidatorInNextEpoch(stakingAddress);
      }

      [epoch, currentValidatorCountForConsensus, allStructs] =
        await stakingViewsFacet.getActiveUnkickedValidatorStructsAndCounts(
          realmId
        );
      expect(allStructs.length).equal(7);
    });

    it('can signal ready using EIP2771 txns', async function () {
      stakingValidatorFacet = stakingValidatorFacet.connect(deployer);

      // lock the validator set
      await stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.NextValidatorSetLocked);

      // signal that we are ready to advance epoch
      for (let i = 0; i < stakingAccounts.length; i++) {
        const stakingAccount = stakingAccounts[i];
        const { nodeAddress } = stakingAccount;
        stakingValidatorFacet = stakingValidatorFacet.connect(nodeAddress);

        const txData =
          await stakingValidatorFacet.signalReadyForNextEpoch.populateTransaction(
            realmId,
            2
          );

        const forwarderAddress = await stakingFacet.getTrustedForwarder();
        const forwarderContract = (
          await ethers.getContractAt('Forwarder', forwarderAddress)
        ).connect(deployer);

        await sendMetaTransaction(
          ethers,
          txData,
          nodeAddress,
          forwarderContract,
          await stakingValidatorFacet.getAddress()
        );
      }

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.ReadyForNextEpoch);
    });

    it('can join as a validator and can leave', async function () {
      // stakingAccount1 requests to join
      // generate new unused communication keys
      const communicationSenderPubKey = toBigInt(utils.randomBytes(32));
      const communicationReceiverPubKey = toBigInt(utils.randomBytes(32));
      const timelock = 24 * 60 * 60 * 120; // 120 days
      // can only join if permitted
      await stakingAdminFacet.setPermittedValidatorsOn(realmId, true);
      // permit the staker
      stakingFacet = stakingFacet.connect(deployer);
      await stakingAdminFacet.setPermittedValidators(realmId, [
        stakingAccount1.address,
      ]);
      stakingValidatorFacet = stakingValidatorFacet.connect(stakingAccount1);
      minStake = await stakingFacet.getMinimumSelfStake();
      console.log('minStake: %s', minStake);

      await stakingValidatorFacet.setIpPortNodeAddress(
        ip2int(stakingAccount1IpAddress),
        0,
        stakingAccount1Port,
        nodeAccount1.address
      );

      await stakingFacet
        .connect(stakingAccount1)
        .stake(minStake, timelock, stakingAccount1.address);
      await stakingValidatorFacet.requestToJoin(realmId);

      // turn off permitted stakers
      await stakingAdminFacet.setPermittedValidatorsOn(realmId, false);

      const validatorsInNextEpochBeforeTest =
        await stakingViewsFacet.getValidatorsInNextEpoch(realmId);
      expect(validatorsInNextEpochBeforeTest.length).equal(11);
      expect(
        validatorsInNextEpochBeforeTest.includes(await stakingAccount1.address)
      ).is.true;

      const epochBeforeAdvancingEpoch = await stakingViewsFacet.epoch(realmId);

      let currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.Active);

      // make sure that we can't lock if less than min validator count
      stakingValidatorFacet = stakingValidatorFacet.connect(deployer);
      await updateMinimumValidatorCount(
        stakingViewsFacet,
        stakingAdminFacet,
        12n
      );

      await expect(
        stakingValidatorFacet.lockValidatorsForNextEpoch(realmId)
      ).revertedWithCustomError(
        stakingValidatorFacet,
        'NotEnoughValidatorsInNextEpoch'
      );
      // reset it back to 7
      await updateMinimumValidatorCount(
        stakingViewsFacet,
        stakingAdminFacet,
        7n
      );

      // lock new validators
      await stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.NextValidatorSetLocked);

      // validators should be unchanged
      const validators = await stakingViewsFacet.getValidatorsInCurrentEpoch(
        realmId
      );
      expect(validators.length).equal(10);

      // validators in next epoch should include stakingAccount1
      const validatorsInNextEpoch =
        await stakingViewsFacet.getValidatorsInNextEpoch(realmId);
      expect(validatorsInNextEpoch.length).equal(11);
      expect(validatorsInNextEpoch[10]).equal(await stakingAccount1.address);

      // signal that we are ready to advance epoch

      for (let i = 0; i < stakingAccounts.length; i++) {
        const stakingAccount = stakingAccounts[i];
        const { nodeAddress } = stakingAccount;
        stakingValidatorFacet = stakingValidatorFacet.connect(nodeAddress);
        await stakingValidatorFacet.signalReadyForNextEpoch(realmId, 2);
      }

      // try advancing before validators all signalled
      await expect(stakingValidatorFacet.advanceEpoch(realmId))
        .revertedWithCustomError(
          stakingValidatorFacet,
          'MustBeInReadyForNextEpochState'
        )
        .withArgs(1);

      stakingValidatorFacet = stakingValidatorFacet.connect(nodeAccount1);
      await stakingValidatorFacet.signalReadyForNextEpoch(realmId, 2);

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.ReadyForNextEpoch);

      stakingValidatorFacet = stakingValidatorFacet.connect(stakingAccount1);

      // advance the epoch.  this sets the validators to be the new set
      await stakingValidatorFacet.advanceEpoch(realmId);

      const epochAfterAdvancingEpoch = await stakingViewsFacet.epoch(realmId);

      // advancing the epoch should reset validatorsForNextEpochLocked
      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.Active);

      expect(epochAfterAdvancingEpoch.number).to.equal(
        epochBeforeAdvancingEpoch.number + 1n
      );

      // validators should include stakingAccount1
      let validatorsAfterAdvancingEpoch =
        await stakingViewsFacet.getValidatorsInCurrentEpoch(realmId);
      expect(validatorsAfterAdvancingEpoch.length).equal(11);
      expect(
        validatorsAfterAdvancingEpoch.includes(await stakingAccount1.address)
      ).is.true;

      const validatorsInNextEpochBefore =
        await stakingViewsFacet.getValidatorsInNextEpoch(realmId);
      expect(validatorsInNextEpochBefore.length).equal(11);
      expect(
        validatorsInNextEpochBefore.includes(await stakingAccount1.address)
      ).is.true;

      // attempt to leave
      await stakingValidatorFacet.requestToLeave();

      const validatorsInNextEpochAfter =
        await stakingViewsFacet.getValidatorsInNextEpoch(realmId);
      expect(validatorsInNextEpochAfter.length).equal(10);
      expect(validatorsInNextEpochAfter.includes(await stakingAccount1.address))
        .is.false;

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.Active);

      // create the new validator set
      await stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.NextValidatorSetLocked);

      for (let i = 0; i < stakingAccounts.length; i++) {
        const stakingAccount = stakingAccounts[i];
        const nodeAddress = stakingAccount.nodeAddress;
        stakingValidatorFacet = stakingValidatorFacet.connect(nodeAddress);
        await stakingValidatorFacet.signalReadyForNextEpoch(realmId, 3);
      }

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.ReadyForNextEpoch);

      stakingValidatorFacet = stakingValidatorFacet.connect(stakingAccount1);

      // advance the epoch.  this sets the validators to be the new set
      await stakingValidatorFacet.advanceEpoch(realmId);

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.Active);

      validatorsAfterAdvancingEpoch =
        await stakingViewsFacet.getValidatorsInCurrentEpoch(realmId);
      expect(validatorsAfterAdvancingEpoch.length).equal(10);
      expect(
        validatorsAfterAdvancingEpoch.includes(await stakingAccount1.address)
      ).to.be.false;

      const validatorsInNextEpochAfterAdvancingEpoch =
        await stakingViewsFacet.getValidatorsInNextEpoch(realmId);
      expect(validatorsInNextEpochAfterAdvancingEpoch.length).equal(10);
      expect(
        validatorsInNextEpochAfterAdvancingEpoch.includes(
          await stakingAccount1.address
        )
      ).to.be.false;
    });

    it('can join as a validator and the node can request to leave', async function () {
      // stakingAccount1 requests to join
      // generate new unused communication keys
      const communicationSenderPubKey = toBigInt(utils.randomBytes(32));
      const communicationReceiverPubKey = toBigInt(utils.randomBytes(32));
      const timelock = 24 * 60 * 60 * 120; // 120 days
      // can only join if permitted
      await stakingAdminFacet.setPermittedValidatorsOn(realmId, true);
      // permit the staker
      stakingFacet = stakingFacet.connect(deployer);
      await stakingAdminFacet.setPermittedValidators(realmId, [
        stakingAccount1.address,
      ]);
      stakingValidatorFacet = stakingValidatorFacet.connect(stakingAccount1);
      minStake = await stakingFacet.getMinimumSelfStake();

      await stakingValidatorFacet.setIpPortNodeAddress(
        ip2int(stakingAccount1IpAddress),
        0,
        stakingAccount1Port,
        nodeAccount1.address
      );

      await stakingFacet
        .connect(stakingAccount1)
        .stake(minStake, timelock, stakingAccount1.address);
      await stakingValidatorFacet.requestToJoin(realmId);

      // turn off permitted stakers
      await stakingAdminFacet.setPermittedValidatorsOn(realmId, false);

      const validatorsInNextEpochBeforeTest =
        await stakingViewsFacet.getValidatorsInNextEpoch(realmId);
      expect(validatorsInNextEpochBeforeTest.length).equal(11);
      expect(
        validatorsInNextEpochBeforeTest.includes(await stakingAccount1.address)
      ).is.true;

      const epochBeforeAdvancingEpoch = await stakingViewsFacet.epoch(realmId);

      let currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.Active);

      // make sure that we can't lock if less than min validator count
      stakingValidatorFacet = stakingValidatorFacet.connect(deployer);
      await updateMinimumValidatorCount(
        stakingViewsFacet,
        stakingAdminFacet,
        12n
      );

      await expect(
        stakingValidatorFacet.lockValidatorsForNextEpoch(realmId)
      ).revertedWithCustomError(
        stakingValidatorFacet,
        'NotEnoughValidatorsInNextEpoch'
      );
      // reset it back to 7
      await updateMinimumValidatorCount(
        stakingViewsFacet,
        stakingAdminFacet,
        7n
      );

      // lock new validators
      await stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.NextValidatorSetLocked);

      // validators should be unchanged
      const validators = await stakingViewsFacet.getValidatorsInCurrentEpoch(
        realmId
      );
      expect(validators.length).equal(10);

      // validators in next epoch should include stakingAccount1
      const validatorsInNextEpoch =
        await stakingViewsFacet.getValidatorsInNextEpoch(realmId);
      expect(validatorsInNextEpoch.length).equal(11);
      expect(validatorsInNextEpoch[10]).equal(await stakingAccount1.address);

      // signal that we are ready to advance epoch

      for (let i = 0; i < stakingAccounts.length; i++) {
        const stakingAccount = stakingAccounts[i];
        const { nodeAddress } = stakingAccount;
        stakingValidatorFacet = stakingValidatorFacet.connect(nodeAddress);
        await stakingValidatorFacet.signalReadyForNextEpoch(realmId, 2);
      }

      // try advancing before validators all signalled
      await expect(stakingValidatorFacet.advanceEpoch(realmId))
        .revertedWithCustomError(
          stakingValidatorFacet,
          'MustBeInReadyForNextEpochState'
        )
        .withArgs(1);

      stakingValidatorFacet = stakingValidatorFacet.connect(nodeAccount1);
      await stakingValidatorFacet.signalReadyForNextEpoch(realmId, 2);

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.ReadyForNextEpoch);

      stakingValidatorFacet = stakingValidatorFacet.connect(stakingAccount1);

      // advance the epoch.  this sets the validators to be the new set
      await stakingValidatorFacet.advanceEpoch(realmId);

      const epochAfterAdvancingEpoch = await stakingViewsFacet.epoch(realmId);

      // advancing the epoch should reset validatorsForNextEpochLocked
      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.Active);

      expect(epochAfterAdvancingEpoch.number).to.equal(
        epochBeforeAdvancingEpoch.number + 1n
      );

      // validators should include stakingAccount1
      let validatorsAfterAdvancingEpoch =
        await stakingViewsFacet.getValidatorsInCurrentEpoch(realmId);
      expect(validatorsAfterAdvancingEpoch.length).equal(11);
      expect(
        validatorsAfterAdvancingEpoch.includes(await stakingAccount1.address)
      ).is.true;

      const validatorsInNextEpochBefore =
        await stakingViewsFacet.getValidatorsInNextEpoch(realmId);
      expect(validatorsInNextEpochBefore.length).equal(11);
      expect(
        validatorsInNextEpochBefore.includes(await stakingAccount1.address)
      ).is.true;

      // attempt to leave with the node account
      await stakingValidatorFacet
        .connect(nodeAccount1)
        .requestToLeaveAsNode(realmId);

      const validatorsInNextEpochAfter =
        await stakingViewsFacet.getValidatorsInNextEpoch(realmId);
      expect(validatorsInNextEpochAfter.length).equal(10);
      expect(validatorsInNextEpochAfter.includes(await stakingAccount1.address))
        .is.false;

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.Active);

      // create the new validator set
      await stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.NextValidatorSetLocked);

      for (let i = 0; i < stakingAccounts.length; i++) {
        const stakingAccount = stakingAccounts[i];
        const nodeAddress = stakingAccount.nodeAddress;
        stakingValidatorFacet = stakingValidatorFacet.connect(nodeAddress);
        await stakingValidatorFacet.signalReadyForNextEpoch(realmId, 3);
      }

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.ReadyForNextEpoch);

      stakingValidatorFacet = stakingValidatorFacet.connect(stakingAccount1);

      // advance the epoch.  this sets the validators to be the new set
      await stakingValidatorFacet.advanceEpoch(realmId);

      currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.Active);

      validatorsAfterAdvancingEpoch =
        await stakingViewsFacet.getValidatorsInCurrentEpoch(realmId);
      expect(validatorsAfterAdvancingEpoch.length).equal(10);
      expect(
        validatorsAfterAdvancingEpoch.includes(await stakingAccount1.address)
      ).to.be.false;

      const validatorsInNextEpochAfterAdvancingEpoch =
        await stakingViewsFacet.getValidatorsInNextEpoch(realmId);
      expect(validatorsInNextEpochAfterAdvancingEpoch.length).equal(10);
      expect(
        validatorsInNextEpochAfterAdvancingEpoch.includes(
          await stakingAccount1.address
        )
      ).to.be.false;
    });

    it('kicks and demerits validator', async () => {
      const toBeKicked = stakingAccounts[stakingAccounts.length - 1];

      await makeAssertions(realmId, stakingViewsFacet, {
        expectedValidatorsInCurrentEpoch: 10,
        expectedValidatorsInNextEpoch: 10,
        expectedKickedValidatorsInNextEpoch: 0,
      });

      const kickedValidatorDemeritsBefore =
        await stakingViewsFacet.getNodeDemerits(
          toBeKicked.stakingAddress.address
        );

      // get epoch number
      const epoch = await stakingViewsFacet.epoch(realmId);

      // vote to kick the last stakingAccount, at the threshold ( can no longer "overvote"!)
      const kickThreshold =
        await stakingViewsFacet.currentValidatorCountForConsensus(realmId);
      for (let i = 0; i < kickThreshold; i++) {
        const stakingAccountToVoteFrom = stakingAccounts[i];
        await voteToKick(realmId, stakingValidatorFacet, stakingViewsFacet, {
          stakingAccountToVoteFrom,
          stakingAccountToBeKicked: toBeKicked,
          epoch,
          expectedExistingNumVotes: i,
        });
      }

      await makeAssertions(realmId, stakingViewsFacet, {
        expectedValidatorsInNextEpoch: 9,
        expectedValidatorsInNextEpochExcludes: [
          await toBeKicked.stakingAddress.address,
        ],
        expectedKickedValidatorsInNextEpoch: 1,
      });

      // check that they were punished
      const kickedValidatorDemeritsAfter =
        await stakingViewsFacet.getNodeDemerits(
          toBeKicked.stakingAddress.address
        );
      expect(kickedValidatorDemeritsAfter).to.be.gt(
        kickedValidatorDemeritsBefore
      );

      let currentState = await stakingViewsFacet.state(realmId);
      expect(currentState).to.equal(StakingState.NextValidatorSetLocked);

      // Signal and advance epoch.
      await signalAndAdvanceEpoch(
        realmId,
        stakingValidatorFacet,
        stakingViewsFacet,
        {
          stakingAccounts: stakingAccounts.slice(0, stakingAccounts.length - 1),
        }
      );

      // Make assertions after advancing epoch
      await makeAssertions(realmId, stakingViewsFacet, {
        expectedValidatorsInCurrentEpoch: 9,
        expectedValidatorsInCurrentEpochExcludes: [
          await toBeKicked.stakingAddress.address,
        ],
        expectedValidatorsInNextEpoch: 9,
        expectedKickedValidatorsInNextEpoch: 0,
      });
    });

    it('cannot kick such that less than a threshold of validators from current set remain', async () => {
      let stakingAccountsCopy = [...stakingAccounts];

      // Get threshold count
      const thresholdCount = Number(
        await stakingViewsFacet.currentValidatorCountForConsensus(realmId)
      );

      // Get number of validator votes needed.
      const numVotesNeeded = thresholdCount;

      // Calculate number of successful kicks
      const numSuccessfulKicks = stakingAccountsCopy.length - thresholdCount;

      // get epoch number
      const epoch = await stakingViewsFacet.epoch(realmId);

      for (let i = stakingAccountsCopy.length - 1; i > 0; i--) {
        const index = Math.floor(Math.random() * (i + 1));
        let temp = stakingAccountsCopy[index];
        stakingAccountsCopy[index] = stakingAccountsCopy[i];
        stakingAccountsCopy[i] = temp;
      }

      // Track the kicked validators
      let kickedValidators = [];

      for (let i = 0; i < numSuccessfulKicks; i++) {
        const stakingAccountToBeKicked = stakingAccountsCopy.shift();

        // Make assertions before kicking
        await makeAssertions(realmId, stakingViewsFacet, {
          expectedValidatorsInCurrentEpoch: 10,
          expectedValidatorsInNextEpoch: 10 - i,
        });

        // Vote to kick the stakingAccount
        let numVotes = 0;
        for (let j = 0; j < stakingAccountsCopy.length; j++) {
          if (numVotes === numVotesNeeded) {
            break;
          }

          const stakingAccountToVoteFrom = stakingAccountsCopy[j];

          await voteToKick(realmId, stakingValidatorFacet, stakingViewsFacet, {
            stakingAccountToVoteFrom,
            stakingAccountToBeKicked,
            epoch,
            expectedExistingNumVotes: numVotes,
          });

          numVotes++;
        }

        // Make assertions after kicking
        await makeAssertions(realmId, stakingViewsFacet, {
          expectedValidatorsInNextEpoch: 10 - i - 1,
          expectedValidatorsInNextEpochExcludes: [
            stakingAccountToBeKicked.stakingAddress.address,
          ],
          expectedKickedValidatorsInNextEpoch: i + 1,
        });

        kickedValidators.push(stakingAccountToBeKicked);
      }

      // After all the successful kicks, the next kick should result in a revert with custom error.

      // Choose to kick the last staker remaining
      const stakingAccountToBeKicked = stakingAccountsCopy[0];

      // Vote to kick the stakingAccount
      const stakingAccountToVoteFrom = stakingAccountsCopy[1];
      const nodeAddress = stakingAccountToVoteFrom.nodeAddress;
      await expect(
        stakingValidatorFacet
          .connect(nodeAddress)
          .kickValidatorInNextEpoch(
            stakingAccountToBeKicked.stakingAddress.address,
            1,
            '0x'
          )
      ).revertedWithCustomError(
        stakingValidatorFacet,
        'CannotKickBelowCurrentValidatorThreshold'
      );

      await makeAssertions(realmId, stakingViewsFacet, {
        expectedValidatorsInNextEpoch: thresholdCount,
      });

      // Now, try locking, signaling ready for next epoch and advancing epoch
      await lockSignalAndAdvanceEpoch(
        realmId,
        stakingValidatorFacet,
        stakingViewsFacet,
        {
          stakingAccounts: stakingAccountsCopy,
        }
      );

      // Make assertions after advancing epoch
      await makeAssertions(realmId, stakingViewsFacet, {
        expectedValidatorsInCurrentEpoch: thresholdCount,
        expectedValidatorsInCurrentEpochExcludes: kickedValidators,
        expectedValidatorsInNextEpochExcludes: kickedValidators,
        expectedValidatorsInNextEpoch: thresholdCount,
        expectedKickedValidatorsInNextEpoch: 0,
      });
    });
  });

  describe('Validators joining and leaving simultaneously', () => {
    it('2 leaves and 4 joins', async () => {
      const nodesRequestingToLeave = [stakingAccounts[0], stakingAccounts[1]];

      let nodesRequestingToJoin = [];
      for (let i = 0; i < 4; i++) {
        nodesRequestingToJoin.push(
          await createValidatorAndStake(ethers, stakingFacet, token, deployer, {
            ipAddress: stakingAccount1IpAddress,
            port: stakingAccount1Port,
            initialTokens: minStake * 3n,
          })
        );
      }

      // nodes request to leave
      for (let i = 0; i < nodesRequestingToLeave.length; i++) {
        const node = nodesRequestingToLeave[i];
        await stakingValidatorFacet
          .connect(node.stakingAddress)
          .requestToLeave();
      }

      // nodes request to join
      for (let i = 0; i < nodesRequestingToJoin.length; i++) {
        const node = nodesRequestingToJoin[i];
        await stakingValidatorFacet
          .connect(node.stakingAddress)
          .setIpPortNodeAddress(
            ip2int(node.ip),
            0,
            node.port,
            node.nodeAddress.address
          );

        await stakingValidatorFacet
          .connect(node.stakingAddress)
          .requestToJoin(realmId);
      }

      await makeAssertions(realmId, stakingViewsFacet, {
        expectedValidatorsInCurrentEpoch: 10,
        expectedValidatorsInNextEpoch: 12,
      });
    });
  });

  describe('setting new resolver contract address', () => {
    it('sets the new contract address', async () => {
      stakingValidatorFacet = stakingValidatorFacet.connect(deployer);
      const existingResolverContractAddress =
        await stakingViewsFacet.contractResolver();
      const newResolverContractAddress =
        '0xea1762E80ED1C54baCa25C7aF4E435FA1427C99E';
      expect(existingResolverContractAddress).not.equal(
        newResolverContractAddress
      );
      await stakingAdminFacet.setContractResolver(newResolverContractAddress);
      expect(await stakingViewsFacet.contractResolver()).equal(
        newResolverContractAddress
      );

      // revert this change
      await stakingAdminFacet.setContractResolver(
        existingResolverContractAddress
      );
    });
  });

  describe('Version tests', () => {
    it('Can get min and max version', async () => {
      let minVersionString = await stakingVersionFacet.getMinVersionString(
        realmId
      );
      expect(minVersionString).to.equal('0.0.0');

      let maxVersionString = await stakingVersionFacet.getMaxVersionString(
        realmId
      );
      expect(maxVersionString).to.equal('10000.0.0');

      let minVersion = await stakingVersionFacet.getMinVersion(realmId);
      expect(minVersion.major).to.equal(0);
      expect(minVersion.minor).to.equal(0);
      expect(minVersion.patch).to.equal(0);

      let maxVersion = await stakingVersionFacet.getMaxVersion(realmId);
      expect(maxVersion.major).to.equal(10000);
      expect(maxVersion.minor).to.equal(0);
      expect(maxVersion.patch).to.equal(0);
    });

    it('Can set min and max version', async () => {
      let isAllowedVersion = await stakingVersionFacet.checkVersion(
        realmId,
        [20000, 0, 0]
      );
      expect(isAllowedVersion).to.equal(false);
      isAllowedVersion = await stakingVersionFacet.checkVersion(
        realmId,
        [1, 1, 200]
      );
      expect(isAllowedVersion).to.equal(true);

      let minVersionBefore = await stakingVersionFacet.getMinVersion(realmId);
      expect(minVersionBefore.major).to.equal(0);
      expect(minVersionBefore.minor).to.equal(0);
      expect(minVersionBefore.patch).to.equal(0);

      let maxVersionBefore = await stakingVersionFacet.getMaxVersion(realmId);
      expect(maxVersionBefore.major).to.equal(10000);
      expect(maxVersionBefore.minor).to.equal(0);
      expect(maxVersionBefore.patch).to.equal(0);

      await stakingVersionFacet.setMinVersion(realmId, [1, 2, 3]);
      let minVersionAfter = await stakingVersionFacet.getMinVersion(realmId);
      expect(minVersionAfter.major).to.equal(1);
      expect(minVersionAfter.minor).to.equal(2);
      expect(minVersionAfter.patch).to.equal(3);

      await stakingVersionFacet.setMaxVersion(realmId, [4, 5, 6]);
      let maxVersionAfter = await stakingVersionFacet.getMaxVersion(realmId);
      expect(maxVersionAfter.major).to.equal(4);
      expect(maxVersionAfter.minor).to.equal(5);
      expect(maxVersionAfter.patch).to.equal(6);

      isAllowedVersion = await stakingVersionFacet.checkVersion(
        realmId,
        [1, 2, 3]
      );
      expect(isAllowedVersion).to.equal(true);

      isAllowedVersion = await stakingVersionFacet.checkVersion(
        realmId,
        [4, 5, 6]
      );
      expect(isAllowedVersion).to.equal(true);

      isAllowedVersion = await stakingVersionFacet.checkVersion(
        realmId,
        [1, 1, 200]
      );
      expect(isAllowedVersion).to.equal(false);
    });
  });

  describe('only the admin can call admin functions', () => {
    it('tries to call the admin functions as a non admin and fails', async () => {
      stakingValidatorFacet = stakingValidatorFacet.connect(nodeAccount1);
      stakingFacet = stakingFacet.connect(nodeAccount1);
      stakingAdminFacet = stakingAdminFacet.connect(nodeAccount1);

      await expect(
        stakingAdminFacet.setEpochLength(realmId, 25)
      ).revertedWithCustomError(stakingAdminFacet, 'CallerNotOwner');

      await expect(
        stakingAdminFacet.setEpochTimeout(realmId, 25)
      ).revertedWithCustomError(stakingAdminFacet, 'CallerNotOwner');

      await expect(
        stakingAdminFacet.setConfig({
          tokenRewardPerTokenPerEpoch: 1,
          minimumValidatorCount: 1,
          rewardEpochDuration: 1,
          maxTimeLock: 1,
          minTimeLock: 1,
          bmin: 1,
          bmax: 1,
          k: 1,
          p: 1,
          enableStakeAutolock: true,
          tokenPrice: 1,
          profitMultiplier: 1,
          usdCostPerMonth: 1,
          maxEmissionRate: 1,
          minStakeAmount: 1,
          maxStakeAmount: 1,
          minSelfStake: 1,
          minSelfStakeTimelock: 1,
          minValidatorCountToClampMinimumThreshold: 1,
          minThresholdToClampAt: 1,
          voteToAdvanceTimeOut: 60,
        })
      ).revertedWithCustomError(stakingAdminFacet, 'CallerNotOwner');

      await expect(
        stakingAdminFacet.setContractResolver(await routerContract.getAddress())
      ).revertedWithCustomError(stakingAdminFacet, 'CallerNotOwner');

      await expect(
        stakingAdminFacet.setEpochState(
          realmId,
          StakingState.NextValidatorSetLocked
        )
      ).revertedWithCustomError(
        stakingAdminFacet,
        'CallerNotOwnerOrDevopsAdmin'
      );

      await expect(
        stakingAdminFacet.setEpochState(realmId, StakingState.Paused)
      ).revertedWithCustomError(
        stakingAdminFacet,
        'CallerNotOwnerOrDevopsAdmin'
      );

      await expect(
        stakingAdminFacet.adminKickValidatorInNextEpoch(
          await stakingAccount1.getAddress()
        )
      ).revertedWithCustomError(
        stakingAdminFacet,
        'CallerNotOwnerOrDevopsAdmin'
      );

      await expect(
        stakingAdminFacet.adminSlashValidator(
          100n,
          await stakingAccount1.getAddress()
        )
      ).revertedWithCustomError(stakingAdminFacet, 'CallerNotOwner');
    });
  });

  describe('the admin can pause', () => {
    it('tries to pause then unpause as admin', async () => {
      stakingAdminFacet = stakingAdminFacet.connect(deployer);

      const currentState = await stakingViewsFacet.state(realmId);
      await stakingAdminFacet.setEpochState(realmId, StakingState.Paused);
      expect(await stakingViewsFacet.state(realmId)).to.equal(
        StakingState.Paused
      );

      // move the state back
      await stakingAdminFacet.setEpochState(realmId, currentState);
      expect(await stakingViewsFacet.state(realmId)).to.equal(currentState);
    });
  });

  describe('when paused', () => {
    let stateBeforePause;

    beforeEach(async () => {
      stateBeforePause = await stakingViewsFacet.state(realmId);
      await stakingAdminFacet
        .connect(deployer)
        .setEpochState(realmId, StakingState.Paused);
    });

    afterEach(async () => {
      await stakingAdminFacet
        .connect(deployer)
        .setEpochState(realmId, stateBeforePause);
    });

    describe('can call mutative functions', function () {
      it('cannot lock validators for next epoch', async () => {
        await expect(
          stakingValidatorFacet
            .connect(stakingAccounts[0].stakingAddress)
            .lockValidatorsForNextEpoch(realmId)
        ).revertedWithCustomError(
          stakingValidatorFacet,
          'MustBeInActiveOrUnlockedState'
        );
      });

      it('cannot signal ready for next epoch', async () => {
        await expect(
          stakingValidatorFacet
            .connect(stakingAccounts[0].stakingAddress)
            .signalReadyForNextEpoch(realmId, 5)
        )
          .revertedWithCustomError(
            stakingValidatorFacet,
            'SignaledReadyForWrongEpochNumber'
          )
          .withArgs(2, 5);
      });

      it('cannot advance epoch', async () => {
        // Get current block time
        const blockTimestamp = (
          await deployer.provider.getBlock(
            await deployer.provider.getBlockNumber()
          )
        ).timestamp;

        // Get current epoch end time and timeout
        const epoch = await stakingViewsFacet.epoch(realmId);
        const epochEndTime = epoch.endTime;

        if (epochEndTime > BigInt(blockTimestamp)) {
          // Fast forward time to when next epoch can be reached.
          await deployer.provider.send('evm_setNextBlockTimestamp', [
            Number(epochEndTime),
          ]);
          await deployer.provider.send('evm_mine');
        }

        await expect(
          stakingValidatorFacet
            .connect(stakingAccounts[0].stakingAddress)
            .advanceEpoch(realmId)
        ).revertedWithCustomError(
          stakingValidatorFacet,
          'MustBeInReadyForNextEpochState'
        );
      });

      it('new stakers can stake and join', async () => {
        // Create new stakers
        const newStakingAddress = ethers.Wallet.createRandom().connect(
          deployer.provider
        );
        const newNodeAddress = ethers.Wallet.createRandom().connect(
          deployer.provider
        );

        // Send them some gas
        const ethForGas = ethers.parseEther('1.0');
        await deployer.sendTransaction({
          to: newStakingAddress.address,
          value: ethForGas,
        });
        await deployer.sendTransaction({
          to: newNodeAddress.address,
          value: ethForGas,
        });

        // Send them some tokens
        const totalToStake = minStake * 3n; // 3 times the minimum stake
        await token
          .connect(deployer)
          .transfer(newStakingAddress.address, totalToStake);
        await token
          .connect(newStakingAddress)
          .approve(await stakingFacet.getAddress(), totalToStake);

        // Prepare joining parameters
        const ipAddress = ip2int(stakingAccount1IpAddress);
        const port = 1337;
        const nodeAddress = newNodeAddress.address;
        const comsKeys = toBigInt(utils.randomBytes(32));

        minStake = await stakingFacet.getMinimumSelfStake();
        const timelock = 24 * 60 * 60 * 120; // 120 days

        await stakingValidatorFacet
          .connect(newStakingAddress)
          .setIpPortNodeAddress(ipAddress, 0, port, nodeAddress);

        // Stake and join
        await stakingFacet
          .connect(newStakingAddress)
          .stake(minStake, timelock, newStakingAddress.address);
        await stakingValidatorFacet
          .connect(newStakingAddress)
          .requestToJoin(realmId);
      });

      it('cannot exit for a reason unrelated to paused state', async () => {
        await expect(
          stakingValidatorFacet
            .connect(stakingAccounts[0].stakingAddress)
            .exit()
        ).revertedWithCustomError(
          stakingValidatorFacet,
          'ActiveValidatorsCannotLeave'
        );
      });

      it('can request to leave', async () => {
        await stakingValidatorFacet
          .connect(stakingAccounts[0].stakingAddress)
          .requestToLeave();
      });

      it('can kick validator in next epoch', async () => {
        await stakingValidatorFacet
          .connect(stakingAccounts[2].nodeAddress)
          .kickValidatorInNextEpoch(
            stakingAccounts[1].stakingAddress.address,
            1,
            '0x'
          );
      });
    });
  });
});

async function updateMinimumValidatorCount(
  stakingViewsFacet,
  stakingAdminFacet,
  newMinimumValidatorCount
) {
  // Get currrent config
  const currentConfig = await stakingViewsFacet.globalConfig();

  // Update config

  await stakingAdminFacet.setConfig({
    tokenRewardPerTokenPerEpoch: currentConfig.tokenRewardPerTokenPerEpoch,
    minimumValidatorCount: newMinimumValidatorCount,
    rewardEpochDuration: currentConfig.rewardEpochDuration,
    maxTimeLock: currentConfig.maxTimeLock,
    minTimeLock: currentConfig.minTimeLock,
    bmin: currentConfig.bmin,
    bmax: currentConfig.bmax,
    k: currentConfig.k,
    p: currentConfig.p,
    enableStakeAutolock: currentConfig.enableStakeAutolock,
    permittedValidatorsOn: currentConfig.permittedValidatorsOn,
    tokenPrice: currentConfig.tokenPrice,
    profitMultiplier: currentConfig.profitMultiplier,
    usdCostPerMonth: currentConfig.usdCostPerMonth,
    maxEmissionRate: currentConfig.maxEmissionRate,
    minStakeAmount: currentConfig.minStakeAmount,
    maxStakeAmount: currentConfig.maxStakeAmount,
    minSelfStake: currentConfig.minSelfStake,
    minSelfStakeTimelock: currentConfig.minSelfStakeTimelock,
    minValidatorCountToClampMinimumThreshold:
      currentConfig.minValidatorCountToClampMinimumThreshold,
    minThresholdToClampAt: currentConfig.minThresholdToClampAt,
    voteToAdvanceTimeOut: currentConfig.voteToAdvanceTimeOut,
  });
}

async function voteToKick(
  realmId,
  stakingContract,
  stakingViewsFacet,
  {
    stakingAccountToVoteFrom,
    stakingAccountToBeKicked,
    epoch,
    expectedExistingNumVotes,
  }
) {
  const nodeAddress = stakingAccountToVoteFrom.nodeAddress;
  stakingContract = stakingContract.connect(nodeAddress);
  await stakingContract.kickValidatorInNextEpoch(
    stakingAccountToBeKicked.stakingAddress.address,
    1,
    '0x'
  );

  // assert votesToKickValidatorsInNextEpoch state
  const [numVotes, didStakerVote] =
    await stakingViewsFacet.getVotingStatusToKickValidator(
      realmId,
      epoch.number,
      stakingAccountToBeKicked.stakingAddress.address,
      stakingAccountToVoteFrom.stakingAddress.address
    );
  expect(numVotes).equal(expectedExistingNumVotes + 1);
  expect(didStakerVote).is.true;
}

async function makeAssertions(
  realmId,
  stakingViewsFacet,
  {
    expectedValidatorsInCurrentEpoch = undefined,
    expectedValidatorsInNextEpoch = undefined,
    expectedValidatorsInCurrentEpochExcludes = [], // array of addresses
    expectedValidatorsInNextEpochExcludes = [], // array of addresses
    expectedKickedValidatorsInNextEpoch = undefined,
  }
) {
  if (!!expectedValidatorsInCurrentEpoch) {
    expect(
      (await stakingViewsFacet.getValidatorsInCurrentEpoch(realmId)).length
    ).equal(expectedValidatorsInCurrentEpoch);
  }

  if (!!expectedValidatorsInNextEpoch) {
    expect(
      (await stakingViewsFacet.getValidatorsInNextEpoch(realmId)).length
    ).equal(expectedValidatorsInNextEpoch);
  }

  if (expectedValidatorsInCurrentEpochExcludes.length > 0) {
    for (const excludedValidatorAddress of expectedValidatorsInCurrentEpochExcludes) {
      expect(
        (await stakingViewsFacet.getValidatorsInCurrentEpoch(realmId)).includes(
          excludedValidatorAddress
        )
      ).is.false;
    }
  }

  if (expectedValidatorsInNextEpochExcludes.length > 0) {
    for (const excludedValidatorAddress of expectedValidatorsInNextEpochExcludes) {
      expect(
        (await stakingViewsFacet.getValidatorsInNextEpoch(realmId)).includes(
          excludedValidatorAddress
        )
      ).is.false;
    }
  }

  if (!!expectedKickedValidatorsInNextEpoch) {
    expect((await stakingViewsFacet.getKickedValidators(realmId)).length).equal(
      expectedKickedValidatorsInNextEpoch
    );
  }
}

async function lockSignalAndAdvanceEpoch(
  realmId,
  stakingContract,
  stakingViewsFacet,
  { stakingAccounts }
) {
  // lock the validator set
  await stakingContract.lockValidatorsForNextEpoch(realmId);
  await signalAndAdvanceEpoch(realmId, stakingContract, stakingViewsFacet, {
    stakingAccounts,
  });
}

async function signalAndAdvanceEpoch(
  realmId,
  stakingContract,
  stakingViewsFacet,
  { stakingAccounts }
) {
  // check that the validator set is already locked
  expect(await stakingViewsFacet.state(realmId)).to.equal(
    StakingState.NextValidatorSetLocked
  );

  // signal that we are ready to advance epoch
  const existingEpochNumber = (await stakingViewsFacet.epoch(realmId)).number;

  for (let i = 0; i < stakingAccounts.length; i++) {
    const stakingAccount = stakingAccounts[i];
    const nodeAddress = stakingAccount.nodeAddress;
    stakingContract = stakingContract.connect(nodeAddress);
    await stakingContract
      .connect(nodeAddress)
      .signalReadyForNextEpoch(realmId, existingEpochNumber);
  }
  expect(await stakingViewsFacet.state(realmId)).to.equal(
    StakingState.ReadyForNextEpoch
  );

  // advance the epoch.  this sets the validators to be the new set
  await stakingContract.advanceEpoch(realmId);
  expect(await stakingViewsFacet.state(realmId)).to.equal(StakingState.Active);
}
