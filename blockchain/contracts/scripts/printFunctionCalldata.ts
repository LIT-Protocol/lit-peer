// Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/printFunctionCalldata.ts

import hre from 'hardhat';
import yargs from 'yargs';
import fs from 'fs';

import { hardhatDeployAndVerifySingleContract } from './utils';

const { ethers } = hre;

// CONFIGURE THIS //
// const args: any[] = [];

async function run() {
  //   const inputs = await getInputsFromCliOptions();

  const contract = await ethers.getContractAt(
    'StakingValidatorFacet',
    '0xBC7F8d7864002b6629Ab49781D5199C8dD1DDcE1'
  );
  const args: any = [
    50000000000000000n,
    10,
    900,
    [1, 2],
    2,
    1000,
    0,
    0,
    10,
    1,
    true,
  ];
  const txn = await contract.setConfig.populateTransaction(args);
  console.log(txn.data);

  const abi = fs.readFileSync('./abis/Staking.abi', 'utf8');

  const iface = new ethers.Interface(abi);
  const parsed = iface.parseTransaction({ data: txn.data });
  console.log(parsed);
}

run();

// async function getInputsFromCliOptions(): Promise<Inputs> {
//   const argv = await yargs(process.argv.slice(2)).options({
//     'deployer-private-key': {
//       type: 'string',
//       describe:
//         'Private key of the wallet that will be used to deploy the new contract',
//       required: true,
//     },
//     'new-contract-name': {
//       type: 'string',
//       describe: 'Name of the new contract',
//       required: true,
//     },
//   }).argv;

//   return argv;
// }

// interface Inputs {
//   deployerPrivateKey: string;
//   newContractName: string;
// }
