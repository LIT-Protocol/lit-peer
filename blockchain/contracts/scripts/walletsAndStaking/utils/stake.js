/* global ethers */
/* eslint prefer-const: "off" */

const hre = require('hardhat');
const yargs = require('yargs/yargs');
const { hideBin } = require('yargs/helpers');
const argv = yargs(hideBin(process.argv)).argv;
const { ip2int, int2ip } = require('../../../utils');

const requiredArgs = [
  'stakingContractAddress',
  'stakerPrivateKey',
  'nodeAddress',
  'comsKeySender',
  'comsKeyReceiver',
  'ip',
  'port',
];

async function stake({
  stakingContractAddress,
  stakerPrivateKey,
  nodeAddress,
  comsKeySender,
  comsKeyReceiver,
  ip,
  port,
}) {
  const wallet = new ethers.Wallet(stakerPrivateKey, ethers.provider);

  const stakingValidator = await ethers.getContractAt(
    'StakingValidatorFacet',
    stakingContractAddress,
    wallet
  );
  const stakingViews = await ethers.getContractAt(
    'StakingViewsFacet',
    stakingContractAddress,
    wallet
  );
  const contractResolver = await ethers.getContractAt(
    'ContractResolver',
    await stakingViews.contractResolver(),
    wallet
  );
  // 0 for dev, 1 for staging, 2 for prod
  const env =
    '0x0000000000000000000000000000000000000000000000000000000000000000';

  const litTokenResolverKey = await contractResolver.LIT_TOKEN_CONTRACT();
  const litToken = await ethers.getContractAt(
    'WLIT',
    await contractResolver.getContract(litTokenResolverKey, env),
    wallet
  );
  console.log('Approving staking balances contract to spend all of your LIT');

  let tx = await litToken.approve(
    await stakingFacet.getAddress(),
    ethers.parseEther('1')
  );
  await tx.wait();
  console.log('Approved!');

  tx = await stakingValidator.stake(ethers.parseEther('1'));
  await tx.wait();
  console.log('Staked!');

  tx = await stakingValidator.setIpPortNodeAddress(
    ip2int(ip),
    0,
    port,
    nodeAddress
  );
  await tx.wait();
  console.log('Set IP, port, node address, and communication public keys');
}

if (require.main === module) {
  for (let arg of requiredArgs) {
    if (!argv[arg]) {
      console.log(`Missing required argument: ${arg}`);
      return;
    }
  }
  stake(argv);
}

module.exports = { stake };
