// Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/diamondContracts/listSelectors.ts --contract-name <CONTRACT_NAME>

import hre from 'hardhat';
import yargs from 'yargs';
const { ethers } = hre;
import { getSelectors } from './lib/utils';

async function run() {
  const { contractName } = await getInputsFromCliOptions();

  const contract = await ethers.getContractAt(
    contractName,
    '0x0000000000000000000000000000000000000000'
  );
  const functionSelectors = getSelectors(contract);
  console.log('Function selectors:', functionSelectors);
  process.exit(0);
}

run();

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'contract-name': {
      type: 'string',
      describe: 'Name of the contract',
      required: true,
    },
  }).argv;

  return argv;
}

interface Inputs {
  contractName: string;
}
