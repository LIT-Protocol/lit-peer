const { expect } = require('chai');
const { deployDiamond } = require('../../scripts/deployDiamond');
const {
  Environment,
  setContractResolver,
  setupStakingWithValidatorsAndAdvance,
  sendMetaTransaction,
} = require('../../utils/contract');
const { ip2int } = require('../../utils');

const lac_baseAmount = 0;
const lac_runtimeLength = 1;
const lac_memoryUsage = 2;
const lac_codeLength = 3;
const lac_responseLength = 4;
const lac_signatures = 5;
const lac_broadcasts = 6;
const lac_contractCalls = 7;
const lac_callDepth = 8;
const lac_decrypts = 9;
const lac_fetches = 10;

const perSecond = 0;
const perMegabyte = 1;
const perCount = 2;

describe('PriceFeed', function () {
  let signers;
  let priceFeedDiamond;
  let priceFeedFacet;
  let deployer;
  let tester;
  let contractResolver;
  let stakingFacet;
  let stakingDiamond;
  let tokenContract;
  let realmId;
  let stakingAccounts = [];
  const totalTokens = BigInt('1000000000') * BigInt('10') ** BigInt('18'); // create 1,000,000,000 total tokens with 18 decimals

  before(async () => {
    [deployer, tester, ...signers] = await ethers.getSigners();
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

    stakingValidatorFacet = await ethers.getContractAt(
      'StakingValidatorFacet',
      await stakingDiamond.getAddress()
    );

    deployResult = await deployDiamond(
      'PriceFeed',
      await contractResolver.getAddress(),
      0,
      {
        additionalFacets: ['PriceFeedFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    priceFeedDiamond = deployResult.diamond;
    priceFeedFacet = await ethers.getContractAt(
      'PriceFeedFacet',
      await priceFeedDiamond.getAddress()
    );

    // Deploy forwarder contract and set on PriceFeedFacet
    const forwarder = await ethers.deployContract('Forwarder');
    await priceFeedFacet.setTrustedForwarder(await forwarder.getAddress());

    await setContractResolver(contractResolver, Environment.DEV, {
      tokenContract,
      stakingContract: stakingFacet,
    });

    await tokenContract.mint(deployer.address, totalTokens);
    realmId = await stakingAdminFacet.addRealm(); // this mutates state - when it finished, realmId 1 is created
    realmId = 1;
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

  it('can set the base price', async () => {
    priceFeedFacet = priceFeedFacet.connect(deployer);
    // check that it's set to defaults
    let baseNetworkPrices = await priceFeedFacet.baseNetworkPrices([0, 1, 2]);
    expect(baseNetworkPrices[0]).equal(ethers.parseUnits('0.01', 18));
    expect(baseNetworkPrices[1]).equal(ethers.parseUnits('0.01', 18));
    expect(baseNetworkPrices[2]).equal(ethers.parseUnits('0.01', 18));

    // try setting and getting it
    await priceFeedFacet.setBaseNetworkPrices(
      ethers.parseUnits('10', 18),
      [0, 1, 2]
    );
    baseNetworkPrices = await priceFeedFacet.baseNetworkPrices([0, 1, 2]);
    expect(baseNetworkPrices[0]).equal(ethers.parseUnits('10', 18));
    expect(baseNetworkPrices[1]).equal(ethers.parseUnits('10', 18));
    expect(baseNetworkPrices[2]).equal(ethers.parseUnits('10', 18));
  });

  it('can set the max price', async () => {
    priceFeedFacet = priceFeedFacet.connect(deployer);
    // check that it's set to defaults
    let maxNetworkPrices = await priceFeedFacet.maxNetworkPrices([0, 1, 2]);
    expect(maxNetworkPrices[0]).equal(ethers.parseUnits('1', 18));
    expect(maxNetworkPrices[1]).equal(ethers.parseUnits('1', 18));
    expect(maxNetworkPrices[2]).equal(ethers.parseUnits('1', 18));

    // try setting and getting it
    await priceFeedFacet.setMaxNetworkPrices(
      ethers.parseUnits('1000', 18),
      [0, 1, 2]
    );
    maxNetworkPrices = await priceFeedFacet.maxNetworkPrices([0, 1, 2]);
    expect(maxNetworkPrices[0]).equal(ethers.parseUnits('1000', 18));
    expect(maxNetworkPrices[1]).equal(ethers.parseUnits('1000', 18));
    expect(maxNetworkPrices[2]).equal(ethers.parseUnits('1000', 18));
  });

  it('a node can get their prices based on usage percentage', async () => {
    priceFeedFacet = priceFeedFacet.connect(deployer);

    let baseNetworkPrices = await priceFeedFacet.baseNetworkPrices([0, 1, 2]);
    expect(baseNetworkPrices[0]).equal(ethers.parseUnits('10', 18));
    expect(baseNetworkPrices[1]).equal(ethers.parseUnits('10', 18));
    expect(baseNetworkPrices[2]).equal(ethers.parseUnits('10', 18));
    let maxNetworkPrices = await priceFeedFacet.maxNetworkPrices([0, 1, 2]);
    expect(maxNetworkPrices[0]).equal(ethers.parseUnits('1000', 18));
    expect(maxNetworkPrices[1]).equal(ethers.parseUnits('1000', 18));
    expect(maxNetworkPrices[2]).equal(ethers.parseUnits('1000', 18));

    const expectedResults = [
      {
        usage: 0,
        price: '10000000000000000000',
      },
      {
        usage: 1,
        price: '19900000000000000000',
      },
      {
        usage: 10,
        price: '109000000000000000000',
      },
      {
        usage: 25,
        price: '257500000000000000000',
      },
      {
        usage: 50,
        price: '505000000000000000000',
      },
      {
        usage: 75,
        price: '752500000000000000000',
      },
      {
        usage: 100,
        price: '1000000000000000000000',
      },
    ];

    for (let i = 0; i < expectedResults.length; i++) {
      const { usage, price } = expectedResults[i];
      const results = await priceFeedFacet.usagePercentToPrices(
        usage,
        [0, 1, 2]
      );
      const result_0 = await priceFeedFacet.usagePercentToPrice(usage, 0);
      expect(result_0).equal(results[0]);
      const result_1 = await priceFeedFacet.usagePercentToPrice(usage, 1);
      expect(result_1).equal(results[1]);
      const result_2 = await priceFeedFacet.usagePercentToPrice(usage, 2);
      expect(result_2).equal(results[2]);
      expect(results[0]).equal(price);
      expect(results[1]).equal(price);
      expect(results[2]).equal(price);
    }
  });

  it('a node can set their prices', async () => {
    priceFeedFacet = priceFeedFacet.connect(stakingAccounts[0].nodeAddress);
    // check that it's zero
    let pricesForNode = await priceFeedFacet.price(
      stakingAccounts[0].stakingAddress.address,
      [0]
    );
    expect(pricesForNode[0].price).equal(0);
    expect(pricesForNode[0].timestamp).equal(0);
    expect(pricesForNode[0].stakerAddress).equal(ethers.ZeroAddress);

    const expectedPrice = await priceFeedFacet.usagePercentToPrice(50, 0);
    await priceFeedFacet.setUsage(50, [0]); // 50% usage
    pricesForNode = await priceFeedFacet.price(
      stakingAccounts[0].stakingAddress.address,
      [0]
    );

    expect(pricesForNode[0].price).equal(expectedPrice);
    // check that timestamp is reasonable and within +/- 10 seconds
    const blockNumBefore = await ethers.provider.getBlockNumber();
    const blockBefore = await ethers.provider.getBlock(blockNumBefore);
    const now = blockBefore.timestamp;
    expect(pricesForNode[0].timestamp).greaterThan(now - 10);
    expect(pricesForNode[0].timestamp).lessThan(now + 10);
    expect(pricesForNode[0].stakerAddress).equal(
      stakingAccounts[0].stakingAddress.address
    );

    // get all prices and confirm they're correct
    const allPrices = await priceFeedFacet.prices(0);
    expect(allPrices.length).equal(stakingAccounts.length);
    for (let i = 0; i < allPrices.length; i++) {
      if (
        allPrices[i].stakerAddress == stakingAccounts[0].stakingAddress.address
      ) {
        expect(allPrices[i].price).equal(expectedPrice);
        expect(allPrices[i].timestamp).greaterThan(now - 10);
        expect(allPrices[i].timestamp).lessThan(now + 10);
        expect(allPrices[i].stakerAddress).equal(
          stakingAccounts[0].stakingAddress.address
        );
      } else {
        // expect all zeroes
        expect(allPrices[i].price).equal(0);
        expect(allPrices[i].timestamp).equal(0);
        expect(allPrices[i].stakerAddress).equal(ethers.ZeroAddress);
      }
    }
  });

  it('a node can set their prices using EIP2771 txns', async () => {
    const stakingAccount = stakingAccounts[1];

    priceFeedFacet = priceFeedFacet.connect(stakingAccount.nodeAddress);
    // check that it's zero
    let pricesForNode = await priceFeedFacet.price(
      stakingAccount.stakingAddress.address,
      [0]
    );
    expect(pricesForNode[0].price).equal(0);
    expect(pricesForNode[0].timestamp).equal(0);
    expect(pricesForNode[0].stakerAddress).equal(ethers.ZeroAddress);

    const expectedPrice = await priceFeedFacet.usagePercentToPrice(50, 0);

    const txData = await priceFeedFacet.setUsage.populateTransaction(50, [0]);
    const forwarderAddress = await priceFeedFacet.getTrustedForwarder();
    const forwarderContract = (
      await ethers.getContractAt('Forwarder', forwarderAddress)
    ).connect(deployer);
    await sendMetaTransaction(
      ethers,
      txData,
      stakingAccount.nodeAddress,
      forwarderContract,
      await priceFeedFacet.getAddress()
    );

    pricesForNode = await priceFeedFacet.price(
      stakingAccount.stakingAddress.address,
      [0]
    );

    expect(pricesForNode[0].price).equal(expectedPrice);
    // check that timestamp is reasonable and within +/- 10 seconds
    const blockNumBefore = await ethers.provider.getBlockNumber();
    const blockBefore = await ethers.provider.getBlock(blockNumBefore);
    const now = blockBefore.timestamp;
    expect(pricesForNode[0].timestamp).greaterThan(now - 10);
    expect(pricesForNode[0].timestamp).lessThan(now + 10);
    expect(pricesForNode[0].stakerAddress).equal(
      stakingAccount.stakingAddress.address
    );

    // get all prices and confirm they're correct
    const allPrices = await priceFeedFacet.prices(0);
    expect(allPrices.length).equal(stakingAccounts.length);
    for (let i = 0; i < allPrices.length; i++) {
      if (allPrices[i].stakerAddress == stakingAccount.stakingAddress.address) {
        expect(allPrices[i].price).equal(expectedPrice);
        expect(allPrices[i].timestamp).greaterThan(now - 10);
        expect(allPrices[i].timestamp).lessThan(now + 10);
        expect(allPrices[i].stakerAddress).equal(
          stakingAccount.stakingAddress.address
        );
      }
    }
  });

  it('the SDK can get the nodes and prices for a request', async () => {
    priceFeedFacet = priceFeedFacet.connect(deployer);
    const nodesAndPrices = await priceFeedFacet.getNodesForRequest(realmId, [
      0,
    ]);
    // epoch ID is [0]
    expect(nodesAndPrices[0]).equal(2);
    // min node count for consensus is [1]
    expect(nodesAndPrices[1]).equal(3);
    // node info and prices is [2] and is an array of NodeInfoAndPrices
    /*struct NodeInfoAndPrices {
        LibStakingStorage.Validator validator;
        uint256[] prices;
    }*/
    for (let i = 0; i < 3; i++) {
      expect(nodesAndPrices[2][i].validator.nodeAddress).equal(
        stakingAccounts[i].nodeAddress.address
      );
      expect(nodesAndPrices[2][i].validator.ip).equal(ip2int('192.168.1.1'));
      expect(nodesAndPrices[2][i].validator.port).equal(7778 + i);
      if (i === 0) {
        // in the test above, we set the first node to have a nonzero price.  expect this.
        expect(nodesAndPrices[2][i].prices[0]).not.equal(0);
      }
    }
  });

  it('can set and get lit action component prices', async () => {
    priceFeedFacet = priceFeedFacet.connect(deployer);

    // try setting and getting it
    await priceFeedFacet.setLitActionPriceConfig(
      lac_baseAmount,
      perCount,
      ethers.parseUnits('0.005', 18)
    );
    await priceFeedFacet.setLitActionPriceConfig(
      lac_runtimeLength,
      perSecond,
      ethers.parseUnits('0.001', 18)
    );
    await priceFeedFacet.setLitActionPriceConfig(
      lac_memoryUsage,
      perMegabyte,
      ethers.parseUnits('0.001', 18)
    );
    await priceFeedFacet.setLitActionPriceConfig(
      lac_codeLength,
      perMegabyte,
      ethers.parseUnits('0.001', 18)
    );
    await priceFeedFacet.setLitActionPriceConfig(
      lac_responseLength,
      perMegabyte,
      ethers.parseUnits('0.001', 18)
    );
    await priceFeedFacet.setLitActionPriceConfig(
      lac_signatures,
      perCount,
      ethers.parseUnits('0.01', 18)
    );
    await priceFeedFacet.setLitActionPriceConfig(
      lac_broadcasts,
      perCount,
      ethers.parseUnits('0.001', 18)
    );
    await priceFeedFacet.setLitActionPriceConfig(
      lac_contractCalls,
      perCount,
      ethers.parseUnits('0.001', 18)
    );
    await priceFeedFacet.setLitActionPriceConfig(
      lac_callDepth,
      perCount,
      ethers.parseUnits('0.001', 18)
    );
    await priceFeedFacet.setLitActionPriceConfig(
      lac_decrypts,
      perCount,
      ethers.parseUnits('0.001', 18)
    );
    await priceFeedFacet.setLitActionPriceConfig(
      lac_fetches,
      perCount,
      ethers.parseUnits('0.001', 18)
    );

    // check that it's set to defaults
    let litActionPriceConfigs = await priceFeedFacet.getLitActionPriceConfigs();
    expect(litActionPriceConfigs[lac_baseAmount].price).equal(
      ethers.parseUnits('0.005', 18)
    );
    expect(litActionPriceConfigs[lac_runtimeLength].price).equal(
      ethers.parseUnits('0.001', 18)
    );
    expect(litActionPriceConfigs[lac_memoryUsage].price).equal(
      ethers.parseUnits('0.001', 18)
    );
    expect(litActionPriceConfigs[lac_codeLength].price).equal(
      ethers.parseUnits('0.001', 18)
    );
    expect(litActionPriceConfigs[lac_responseLength].price).equal(
      ethers.parseUnits('0.001', 18)
    );
    expect(litActionPriceConfigs[lac_signatures].price).equal(
      ethers.parseUnits('0.01', 18)
    ); /* not difference - sigs are expensive!! */
    expect(litActionPriceConfigs[lac_broadcasts].price).equal(
      ethers.parseUnits('0.001', 18)
    );
    expect(litActionPriceConfigs[lac_contractCalls].price).equal(
      ethers.parseUnits('0.001', 18)
    );
    expect(litActionPriceConfigs[lac_callDepth].price).equal(
      ethers.parseUnits('0.001', 18)
    );
    expect(litActionPriceConfigs[lac_decrypts].price).equal(
      ethers.parseUnits('0.001', 18)
    );
    expect(litActionPriceConfigs[lac_fetches].price).equal(
      ethers.parseUnits('0.001', 18)
    );
  });
});
