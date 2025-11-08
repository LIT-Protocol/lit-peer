// Full Command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/verifyDiamondContract.ts --diamond-name <DIAMOND_NAME> --diamond-address <DIAMOND_ADDRESS> --contract-resolver <CONTRACT_RESOLVER> --env <ENV> --diamond-owner-address <DIAMOND_OWNER_ADDRESS> --facets <FACET_0> <FACET_1>

import yargs from 'yargs';

import parsedContracts from '../deployed-lit-node-contracts-datil-dev.json';
import { Environment } from '../utils/contract';
import { verifyDiamond } from './verifyDiamond';

async function run() {
  const {
    diamondName,
    diamondAddress,
    facets,
    contractResolver,
    env,
    diamondOwnerAddress,
  } = await getInputsFromCliOptions();

  // Reformat the JSON data
  const partialFacetMap: {
    [diamondContractName: string]: string;
  } = {};
  // @ts-ignore
  for (const diamondContractName of Object.keys(parsedContracts.facets)) {
    for (
      let facet = 0;
      // @ts-ignore
      facet < parsedContracts.facets[diamondContractName].length;
      facet++
    ) {
      // @ts-ignore
      const facetObject = parsedContracts.facets[diamondContractName][facet];
      if (facetObject.facetName.includes(diamondContractName)) {
        partialFacetMap[facetObject.facetName] = facetObject.facetAddress;
      }
    }
  }

  await verifyDiamond(
    diamondName,
    diamondAddress,
    '0xAA55D08232aA631c5FAcce2805CAACa816d6b5Ed',
    contractResolver,
    env,
    diamondOwnerAddress,
    {
      DiamondCutFacet: '0xbBa57629FB73d72A66e91c06683e31168204EA03',
      DiamondLoupeFacetNoERC165: '0x5f689ad066e8F87359d9B9051fd7Dd25FBB38787',
      OwnershipFacet: '0x032483F6EbFfc0a963cFf5a7Ad0F8140F75d3Cb3',
      ...partialFacetMap,
    },
    {
      additionalFacets: facets,
    }
  );
}

run();

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    facets: {
      type: 'array',
      describe: 'List of facets to deploy',
      required: true,
    },
    'diamond-name': {
      type: 'string',
      describe: 'Name of the diamond contract',
      required: true,
    },
    'diamond-address': {
      type: 'string',
      describe: 'Diamond contract address',
      required: true,
    },
    'contract-resolver': {
      type: 'string',
      describe: 'Resolver contract address',
      required: true,
    },
    env: {
      type: 'number',
      describe: 'Environment',
      required: true,
    },
    'diamond-owner-address': {
      type: 'string',
      describe: 'Diamond contract owner address',
      required: true,
    },
  }).argv;

  for (const facet of argv.facets) {
    if (typeof facet !== 'string') {
      throw new Error('Facets must be strings');
    }
  }

  return argv as Inputs;
}

interface Inputs {
  facets: string[];
  diamondName: string;
  diamondAddress: string;
  contractResolver: string;
  env: Environment;
  diamondOwnerAddress: string;
}
