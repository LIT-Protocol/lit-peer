const { expect } = require('chai');
const { deployDiamond } = require('../../scripts/deployDiamond');
const {
  Environment,
  setContractResolver,
  setupStakingWithValidatorsAndAdvance,
  sendMetaTransaction,
} = require('../../utils/contract');

describe('Ledger', function () {
  let signers;
  let ledgerDiamond;
  let ledgerFacet;
  let deployer;
  let tester;
  let tester2;
  let contractResolver;
  let stakingFacet;
  let stakingDiamond;
  let tokenContract;
  let stakingAccounts = [];
  const totalTokens = BigInt('1000000000') * BigInt('10') ** BigInt('18'); // create 1,000,000,000 total tokens with 18 decimals
  const userDepositAmount = ethers.parseUnits('100', 18);
  const toChargeUser = ethers.parseUnits('10', 18);
  let totalExpectedFoundationRewards = BigInt(0);
  let totalChargedToUser = BigInt(0);

  before(async () => {
    [deployer, tester, tester2, ...signers] = await ethers.getSigners();
    contractResolver = await ethers.deployContract('ContractResolver', [
      Environment.DEV,
    ]);
    tokenContract = await ethers.deployContract('LITToken', [
      ethers.parseUnits('1000000000', 18), // 1b tokens
    ]);

    let deployResult = await deployDiamond(
      'Staking',
      await contractResolver.getAddress(),
      0,
      {
        additionalFacets: [
          'StakingFacet',
          'StakingValidatorFacet',
          'StakingViewsFacet',
          'StakingVersionFacet',
          'StakingAdminFacet',
          'StakingAcrossRealmsFacet',
          'StakingNFTFacet',
        ],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    stakingDiamond = deployResult.diamond;
    stakingFacet = await ethers.getContractAt(
      'StakingFacet',
      await stakingDiamond.getAddress()
    );
    const stakingAdminFacet = await ethers.getContractAt(
      'StakingAdminFacet',
      await stakingDiamond.getAddress()
    );

    const stakingValidatorFacet = await ethers.getContractAt(
      'StakingValidatorFacet',
      await stakingDiamond.getAddress()
    );

    deployResult = await deployDiamond(
      'Ledger',
      await contractResolver.getAddress(),
      0,
      {
        additionalFacets: ['LedgerFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    ledgerDiamond = deployResult.diamond;
    ledgerFacet = await ethers.getContractAt(
      'LedgerFacet',
      await ledgerDiamond.getAddress()
    );

    // Deploy forwarder contract and set on Ledger
    const forwarder = await ethers.deployContract('Forwarder');
    await ledgerFacet.setTrustedForwarder(await forwarder.getAddress());

    await setContractResolver(contractResolver, Environment.DEV, {
      tokenContract,
      stakingContract: stakingFacet,
    });

    realmId = await stakingAdminFacet.addRealm(); // this mutates state - when it finished, realmId 1 is created
    realmId = 1;

    await tokenContract.mint(deployer.address, totalTokens);
    stakingAccounts = await setupStakingWithValidatorsAndAdvance(
      ethers,
      stakingFacet,
      stakingValidatorFacet,
      stakingAdminFacet,
      tokenContract,
      deployer,
      {
        numValidators: 3,
        startingPort: 7777,
        ipAddress: '192.168.1.1',
      }
    );
  });

  it('a user can deposit', async () => {
    ledgerFacet = ledgerFacet.connect(tester);
    const testerBalanceBefore = await ethers.provider.getBalance(
      tester.address
    );
    await ledgerFacet.deposit({ value: userDepositAmount });
    const testerBalanceAfter = await ethers.provider.getBalance(tester.address);

    let balance = await ledgerFacet.balance(tester.address);
    expect(balance).equal(userDepositAmount);
    // show that their balance is bounded  the user paid some gas so their balance should be less than what we expect
    expect(testerBalanceAfter).lessThan(
      testerBalanceBefore - userDepositAmount
    );
    expect(testerBalanceAfter).greaterThan(
      testerBalanceBefore - userDepositAmount - ethers.parseUnits('1', 18)
    );
  });

  it('a user can deposit for another user', async () => {
    // we will deposit from tester2 on behalf of tester
    ledgerFacet = ledgerFacet.connect(tester2);
    const tester2BalanceBefore = await ethers.provider.getBalance(
      tester2.address
    );
    const testerDepositBalanceBefore = await ledgerFacet.balance(
      tester.address
    );
    await ledgerFacet.depositForUser(tester.address, {
      value: userDepositAmount,
    });
    const tester2BalanceAfter = await ethers.provider.getBalance(
      tester2.address
    );

    // check the tester balance (the one we deposited for)
    const testerDepositBalanceAfter = await ledgerFacet.balance(tester.address);
    expect(testerDepositBalanceAfter).equal(
      userDepositAmount + testerDepositBalanceBefore
    );
    // show that the depositor's balance is bounded  the user paid some gas so their balance should be less than what we expect
    expect(tester2BalanceAfter).lessThan(
      tester2BalanceBefore - userDepositAmount
    );
    expect(tester2BalanceAfter).greaterThan(
      tester2BalanceBefore - userDepositAmount - ethers.parseUnits('1', 18)
    );
  });

  it('a node can charge a user', async () => {
    ledgerFacet = ledgerFacet.connect(stakingAccounts[0].nodeAddress);

    const balanceBefore = await ledgerFacet.balance(tester.address);
    await ledgerFacet.chargeUser(tester.address, toChargeUser);

    // track total that lit foundation should receive
    let splitPercentage = await ledgerFacet.litFoundationSplitPercentage();
    totalExpectedFoundationRewards +=
      (toChargeUser * splitPercentage) / BigInt(100);
    totalChargedToUser += toChargeUser;
    let balance = await ledgerFacet.balance(tester.address);
    expect(balance).equal(balanceBefore - toChargeUser);
  });

  it('a node can charge a user using EIP2771 txns', async () => {
    ledgerFacet = ledgerFacet.connect(stakingAccounts[0].nodeAddress);

    const balanceBefore = await ledgerFacet.balance(tester.address);
    const txData = await ledgerFacet.chargeUser.populateTransaction(
      tester.address,
      toChargeUser
    );

    const forwarderAddress = await ledgerFacet.getTrustedForwarder();
    const forwarderContract = (
      await ethers.getContractAt('Forwarder', forwarderAddress)
    ).connect(deployer);

    await sendMetaTransaction(
      ethers,
      txData,
      stakingAccounts[0].nodeAddress,
      forwarderContract,
      await ledgerFacet.getAddress()
    );

    // track total that lit foundation should receive
    let splitPercentage = await ledgerFacet.litFoundationSplitPercentage();
    totalExpectedFoundationRewards +=
      (toChargeUser * splitPercentage) / BigInt(100);
    totalChargedToUser += toChargeUser;
    let balance = await ledgerFacet.balance(tester.address);
    expect(balance).equal(balanceBefore - toChargeUser);
  });

  it('an admin can set the lit foundation split percentage', async () => {
    ledgerFacet = ledgerFacet.connect(deployer);

    // Default should be 100%
    let splitPercentage = await ledgerFacet.litFoundationSplitPercentage();
    expect(splitPercentage).equal(100);

    // Set to 20%
    await ledgerFacet.setLitFoundationSplitPercentage(20);
    splitPercentage = await ledgerFacet.litFoundationSplitPercentage();
    expect(splitPercentage).equal(20);

    // Should fail if > 100
    await expect(
      ledgerFacet.setLitFoundationSplitPercentage(101)
    ).to.be.revertedWithCustomError(ledgerFacet, 'PercentageMustBeLessThan100');
  });

  it('rewards are split correctly between node and foundation', async () => {
    ledgerFacet = ledgerFacet.connect(deployer);
    // Set split to 30%
    let newSplitPercentage = BigInt(30);
    await ledgerFacet.setLitFoundationSplitPercentage(newSplitPercentage);

    // Get current rewards
    const foundationRewardsBeforeCharging =
      await ledgerFacet.litFoundationRewards();
    const nodeRewardsBeforeCharging = await ledgerFacet.rewardBalance(
      stakingAccounts[0].stakingAddress.address
    );

    // Charge a user and verify split
    ledgerFacet = ledgerFacet.connect(stakingAccounts[0].nodeAddress);
    const chargeAmount = ethers.parseUnits('100', 18);
    await ledgerFacet.chargeUser(tester.address, chargeAmount);
    totalExpectedFoundationRewards +=
      (chargeAmount * newSplitPercentage) / BigInt(100);
    totalChargedToUser += chargeAmount;

    // Check foundation rewards (30%)
    const foundationRewards = await ledgerFacet.litFoundationRewards();
    expect(foundationRewards - foundationRewardsBeforeCharging).equal(
      (chargeAmount * newSplitPercentage) / BigInt(100)
    );

    // Check node rewards (70%)
    const nodeRewards = await ledgerFacet.rewardBalance(
      stakingAccounts[0].stakingAddress.address
    );
    expect(nodeRewards - nodeRewardsBeforeCharging).equal(
      (chargeAmount * (BigInt(100) - newSplitPercentage)) / BigInt(100)
    );
  });

  it('foundation can withdraw their rewards', async () => {
    ledgerFacet = ledgerFacet.connect(deployer);
    const foundationRewards = await ledgerFacet.litFoundationRewards();
    expect(foundationRewards).to.be.gt(0); // Should have rewards from previous test
    expect(foundationRewards).equal(totalExpectedFoundationRewards);
    // Try to withdraw more than available
    await expect(
      ledgerFacet.withdrawFoundationRewards(foundationRewards + BigInt(1))
    ).to.be.revertedWithCustomError(ledgerFacet, 'InsufficientFunds');

    // Withdraw all rewards
    const deployerBalanceBefore = await ethers.provider.getBalance(
      deployer.address
    );
    await ledgerFacet.withdrawFoundationRewards(foundationRewards);
    const deployerBalanceAfter = await ethers.provider.getBalance(
      deployer.address
    );

    // Verify foundation rewards are now 0
    expect(await ledgerFacet.litFoundationRewards()).equal(0);

    // Verify deployer received the funds (minus gas)
    expect(deployerBalanceAfter).to.be.gt(
      deployerBalanceBefore + foundationRewards - ethers.parseUnits('1', 18)
    );
  });

  it('only owner can withdraw foundation rewards', async () => {
    ledgerFacet = ledgerFacet.connect(tester);
    await expect(
      ledgerFacet.withdrawFoundationRewards(1)
    ).to.be.revertedWithCustomError(ledgerFacet, 'CallerNotOwner');
  });

  it('an admin can set the user withdraw delay', async () => {
    ledgerFacet = ledgerFacet.connect(deployer);

    let userWithdrawDelay = await ledgerFacet.userWithdrawDelay();
    expect(userWithdrawDelay).equal(3600); // default

    await ledgerFacet.setUserWithdrawDelay(30);
    userWithdrawDelay = await ledgerFacet.userWithdrawDelay();
    expect(userWithdrawDelay).equal(30);
  });

  it('a user can withdraw their deposit', async () => {
    ledgerFacet = ledgerFacet.connect(tester);

    let balance = await ledgerFacet.balance(tester.address);

    // if you try to withdraw too much, it should fail
    await expect(
      ledgerFacet.requestWithdraw(balance + BigInt(1))
    ).to.be.revertedWithCustomError(ledgerFacet, 'InsufficientFunds');

    await ledgerFacet.requestWithdraw(balance);
    // try to immediately withdraw, before the window
    await expect(ledgerFacet.withdraw(balance)).to.be.revertedWithCustomError(
      ledgerFacet,
      'WithdrawalDelayNotPassed'
    );
    const blockNumBefore = await ethers.provider.getBlockNumber();
    const blockBefore = await ethers.provider.getBlock(blockNumBefore);
    const now = BigInt(blockBefore.timestamp);
    // confirm the expected withdrawal time
    const userWithdrawDelay = await ledgerFacet.userWithdrawDelay();
    const withdrawRequest = await ledgerFacet.latestWithdrawRequest(
      tester.address
    );
    expect(withdrawRequest.amount).equal(balance);
    // Fast forward time to when withdraw delay has passed.
    await deployer.provider.send('evm_setNextBlockTimestamp', [
      Number(now + userWithdrawDelay + BigInt(10)),
    ]);
    await deployer.provider.send('evm_mine');

    // confirm the withdraw is successful
    const testerBalanceBefore = await ethers.provider.getBalance(
      tester.address
    );
    await ledgerFacet.withdraw(balance);
    const testerBalanceAfter = await ethers.provider.getBalance(tester.address);
    expect(testerBalanceAfter).greaterThanOrEqual(
      testerBalanceBefore + balance - ethers.parseUnits('0.001', 18) // minus some gas
    );
  });

  it('a node can withdraw their rewards', async () => {
    ledgerFacet = ledgerFacet.connect(stakingAccounts[0].stakingAddress);

    const rewards = await ledgerFacet.rewardBalance(
      stakingAccounts[0].stakingAddress.address
    );
    // With 30% split to foundation from previous test, node should have 70% of toChargeUser
    expect(rewards).equal(totalChargedToUser - totalExpectedFoundationRewards);

    // if you try to withdraw too much, it should fail
    await expect(
      ledgerFacet.requestRewardWithdraw(rewards + BigInt(1))
    ).to.be.revertedWithCustomError(ledgerFacet, 'InsufficientFunds');

    await ledgerFacet.requestRewardWithdraw(rewards);
    // try to immediately withdraw, before the window
    await expect(
      ledgerFacet.withdrawRewards(rewards)
    ).to.be.revertedWithCustomError(ledgerFacet, 'WithdrawalDelayNotPassed');
    const blockNumBefore = await ethers.provider.getBlockNumber();
    const blockBefore = await ethers.provider.getBlock(blockNumBefore);
    const now = BigInt(blockBefore.timestamp);
    // confirm the expected withdrawal time
    const rewardWithdrawDelay = await ledgerFacet.rewardWithdrawDelay();
    const withdrawRequest = await ledgerFacet.latestRewardWithdrawRequest(
      stakingAccounts[0].stakingAddress.address
    );
    expect(withdrawRequest.amount).equal(rewards);
    // Fast forward time to when withdraw delay has passed.
    await deployer.provider.send('evm_setNextBlockTimestamp', [
      Number(now + rewardWithdrawDelay + BigInt(10)),
    ]);
    await deployer.provider.send('evm_mine');

    // confirm the withdraw is successful
    const nodeBalanceBefore = await ethers.provider.getBalance(
      stakingAccounts[0].stakingAddress.address
    );
    await ledgerFacet.withdrawRewards(rewards);
    const nodeBalanceAfter = await ethers.provider.getBalance(
      stakingAccounts[0].stakingAddress.address
    );
    // bound us within 1 unit of the expected reward
    expect(nodeBalanceAfter).lessThan(nodeBalanceBefore + rewards);
    expect(nodeBalanceAfter).greaterThan(
      nodeBalanceBefore + rewards - ethers.parseUnits('1', 18)
    );
  });
});
