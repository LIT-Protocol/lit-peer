// Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/diamondContracts/deployMultipleFacets.ts --deployer-private-key <PRIVATE_KEY>

import hre from 'hardhat';
import yargs from 'yargs';
import { hardhatCompile, hardhatDeployAndVerifySingleContract } from '../utils';

const { ethers } = hre;

async function run() {
  const { deployerPrivateKey, facets } = await getInputsFromCliOptions();

  await hardhatCompile();

  const deployer = new ethers.Wallet(deployerPrivateKey).connect(
    ethers.provider
  );
  const deployedFacets = [];
  for (const facetName of facets) {
    console.log('Deploying facet:', facetName);

    const contract = await hardhatDeployAndVerifySingleContract(
      ethers,
      hre.network.name,
      facetName,
      {
        signer: deployer,
        deploymentArgs: [],
      }
    );
    const address = await contract.getAddress();
    deployedFacets.push({ facetName, address });
  }

  console.log('Deployed facets:', deployedFacets);
  process.exit(0);
}
if (require.main === module) {
  run().catch((error) => {
    console.error(error);
    process.exit(1);
  });
}

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'deployer-private-key': {
      type: 'string',
      describe:
        'Private key of the wallet that will be used to deploy the new contracts',
      required: true,
    },
    facets: {
      type: 'array',
      coerce: (arg: string[]) => arg.map((s) => s.trim()),
      describe: 'Array of facets to deploy.  Use --facets Facet1 Facet2 Facet3',
      required: true,
    },
  }).argv;

  return argv;
}

interface Inputs {
  deployerPrivateKey: string;
  facets: string[];
}
