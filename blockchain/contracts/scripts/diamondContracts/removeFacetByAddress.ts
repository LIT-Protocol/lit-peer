// Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/diamondContracts/removeFacetByAddress.ts --facet-contract-address <FACET_CONTRACT_ADDRESS> --diamond-contract-address <DIAMOND_CONTRACT_ADDRESS> --facet-name <FACET_NAME> --diamond-owner-signer-private-key <DIAMOND_OWNER_SIGNER_PRIVATE_KEY>

import hre from 'hardhat';
import yargs from 'yargs';
const { ethers } = hre;
import path from 'path';
import {
  MANIFESTS_DIR,
  appendDiamondCutOperationToManifest,
  executeDiamondCutOperations,
} from './lib/diamondCutManifest';
import { FunctionSelector } from './lib/types';

async function run() {
  const {
    diamondContractAddress,
    facetContractAddress,
    facetName,
    diamondOwnerSignerPrivateKey,
  } = await getInputsFromCliOptions();

  const diamondLoupeFacet = await ethers.getContractAt(
    'DiamondLoupeFacet',
    diamondContractAddress
  );
  const rawSelectors = await diamondLoupeFacet.facetFunctionSelectors(
    facetContractAddress
  );
  const selectors: FunctionSelector[] = rawSelectors.map(
    (rawSelector: string) => {
      return {
        selector: rawSelector,
        signature: 'unknown',
      };
    }
  );
  console.log('selectors we will remove:', selectors);
  const facetContract = await ethers.getContractAt(
    facetName,
    facetContractAddress
  );
  // We will use a custom manifest file for the diamond cut operation.
  // The manifest file will consist of the datetime and the git SHA.
  const manifestFilePath = path.join(
    __dirname,
    `./${MANIFESTS_DIR}`,
    `diamondCutManifest_${new Date().toISOString()}.json`
  );
  await appendDiamondCutOperationToManifest(
    manifestFilePath,
    diamondContractAddress,
    facetContract,
    2,
    facetName,
    selectors
  );
  console.info('Manifest file created at:', manifestFilePath);

  // Execute diamond cut operations
  console.info('Executing diamond cut operations...');
  await executeDiamondCutOperations(
    manifestFilePath,
    diamondOwnerSignerPrivateKey
  );
}

if (require.main === module) {
  run()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });
}

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'facet-contract-address': {
      type: 'string',
      describe: 'Address of the facet contract',
      required: true,
    },
    'diamond-contract-address': {
      type: 'string',
      describe: 'Address of the diamond contract',
      required: true,
    },
    'facet-name': {
      type: 'string',
      describe: 'Name of the facet contract',
      required: true,
    },
    'diamond-owner-signer-private-key': {
      type: 'string',
      describe: 'Private key of the diamond owner',
      required: true,
    },
  }).argv;

  return argv;
}

interface Inputs {
  facetContractAddress: string;
  diamondContractAddress: string;
  facetName: string;
  diamondOwnerSignerPrivateKey: string;
}
