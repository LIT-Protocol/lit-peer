/* global ethers */
/* eslint prefer-const: "off" */

const hre = require('hardhat');
const yargs = require('yargs/yargs');
const { hideBin } = require('yargs/helpers');
const argv = yargs(hideBin(process.argv)).argv;
const { ip2int, int2ip } = require('../../utils');

const requiredArgs = ['stakingContractAddress', 'stakerAddress'];

async function getNodeVersionsAndAddresses({ stakingContractAddress }) {
  const staking = await ethers.getContractAt(
    'StakingViewsFacet',
    stakingContractAddress
  );

  const stakers = await staking.getValidatorsInCurrentEpoch();
  console.log(`got stakers: ${JSON.stringify(Array.from(stakers), null, 2)}`);

  const nodeInfos = await staking.getValidatorsStructs(Array.from(stakers));

  const ips = nodeInfos.map((info) => int2ip(info.ip));
  console.log(`got ips: ${ips}`);
}

if (require.main === module) {
  // for (let arg of requiredArgs) {
  //     if (!argv[arg]) {
  //         console.log(`Missing required argument: ${arg}`);
  //         return;
  //     }
  // }
  getNodeVersionsAndAddresses({
    stakingContractAddress: '0x3DAA2A2A1a9ba31D7a4FA2B6e25921E6cef480EE',
  });
}

module.exports = { getNodeVersionsAndAddresses };
