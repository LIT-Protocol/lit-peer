// Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/diamondContracts/listFacets.ts --diamond-contract-address <DIAMOND_CONTRACT_ADDRESS>

import hre from 'hardhat';
import yargs from 'yargs';
const { ethers } = hre;

async function run() {
  const { diamondContractAddress } = await getInputsFromCliOptions();

  const diamondLoupeFacet = await ethers.getContractAt(
    'DiamondLoupeFacet',
    diamondContractAddress
  );
  const facets = await diamondLoupeFacet.facets();
  console.log('Facets:', facets);
}

run();

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'diamond-contract-address': {
      type: 'string',
      describe: 'Address of the diamond contract',
      required: true,
    },
  }).argv;

  return argv;
}

interface Inputs {
  diamondContractAddress: string;
}
