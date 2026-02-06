// Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/deployMultipleContracts.ts --deployer-private-key <PRIVATE_KEY> [--delay-ms 2000]

import hre from 'hardhat';
import yargs from 'yargs';
import fs from 'fs/promises';

import { hardhatDeployAndVerifySingleContract } from '../utils';

const { ethers } = hre;

const toDeploy: string[] = [
  'PubkeyRouterFacet',
  'PubkeyRouterViewsFacet',
  'BackupRecoveryFacet',
  'PKPNFTFacet',
  'PKPPermissionsFacet',
  'PriceFeedFacet',
  'StakingAcrossRealmsFacet',
  'StakingAdminFacet',
  'StakingFacet',
  'StakingKeySetsFacet',
  'StakingValidatorFacet',
  'StakingViewsFacet',
];

// CONFIGURE THIS //
const args: any[] = [];

// CONFIGURE THIS - delay between deployments in milliseconds (default: 2000ms = 2s)
const DEFAULT_DELAY_MS = 2000;

async function run() {
  const inputs = await getInputsFromCliOptions();

  const deployer = new ethers.Wallet(inputs.deployerPrivateKey).connect(
    ethers.provider
  );

  const delayMs = inputs.delayMs || DEFAULT_DELAY_MS;
  const deploymentPromises: Array<Promise<{ contractName: string; contract: any }>> = [];

  console.log(`\n🚀 Starting deployment of ${toDeploy.length} contracts...`);
  console.log(`⏱️  Delay between starting each deployment: ${delayMs}ms`);
  console.log(`🔄 Deployments will run in parallel after initial delay\n`);

  // Start all deployments with delays between each start
  for (let i = 0; i < toDeploy.length; i++) {
    const contractName = toDeploy[i];
    const progress = `[${i + 1}/${toDeploy.length}]`;

    // Create a promise that starts after the cumulative delay
    const deploymentPromise = (async () => {
      // Wait for the cumulative delay before starting this deployment
      // First one starts immediately (0ms), second after 2s, third after 4s, etc.
      await new Promise((resolve) => setTimeout(resolve, i * delayMs));

      console.log(`${progress} 🚀 Starting deployment of ${contractName}...`);

      try {
        const contract = await hardhatDeployAndVerifySingleContract(
          ethers,
          hre.network.name,
          contractName,
          {
            signer: deployer,
            deploymentArgs: args,
          }
        );

        const contractAddress = await contract.getAddress();
        console.log(
          `${progress} ✅ ${contractName} deployed successfully to ${contractAddress}`
        );

        return { contractName, contract };
      } catch (error) {
        console.error(
          `${progress} ❌ Failed to deploy ${contractName}:`,
          error
        );
        throw error;
      }
    })();

    deploymentPromises.push(deploymentPromise);
  }

  // Wait for all deployments to complete (collecting both successes and failures)
  console.log(`\n⏳ Waiting for all deployments to complete...\n`);
  const settledResults = await Promise.allSettled(deploymentPromises);

  // Extract addresses from all successfully completed contracts
  const deployedContracts: Record<string, string> = {};
  const failedDeployments: Array<{ index: number; reason: unknown }> = [];

  for (let i = 0; i < settledResults.length; i++) {
    const result = settledResults[i];
    if (result.status === 'fulfilled') {
      const { contractName, contract } = result.value;
      const contractAddress = await contract.getAddress();
      deployedContracts[contractName] = contractAddress;
    } else {
      failedDeployments.push({ index: i, reason: result.reason });
    }
  }

  // Save all successfully deployed contract addresses to JSON file
  const outputPath = 'deployedMultipleContracts.json';
  await fs.writeFile(
    outputPath,
    JSON.stringify(deployedContracts, null, 2)
  );

  if (failedDeployments.length === 0) {
    console.log(`\n✨ All contracts deployed successfully!`);
  } else {
    console.log(
      `\n⚠️  Partial deployment: ${settledResults.length - failedDeployments.length} succeeded, ${failedDeployments.length} failed.`
    );
    console.log('Failed deployments:');
    failedDeployments.forEach(({ index, reason }) => {
      const contractName = toDeploy[index] ?? `<index ${index}>`;
      console.log(`  - ${contractName} (index ${index}):`, reason);
    });
  }

  console.log(`📄 Deployed addresses saved to ${outputPath}\n`);
  console.log('Deployed contracts:');
  Object.entries(deployedContracts).forEach(([name, address]) => {
    console.log(`  ${name}: ${address}`);
  });

  // If any deployments failed, signal an overall failure so run().catch(...) can exit non-zero
  if (failedDeployments.length > 0) {
    throw new Error(
      `One or more contract deployments failed. See log above for details.`
    );
  }
}

run().catch((error) => {
  console.error('Deployment failed:', error);
  process.exit(1);
});

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2))
    .options({
      'deployer-private-key': {
        type: 'string',
        describe:
          'Private key of the wallet that will be used to deploy the contracts',
        required: true,
      },
      'delay-ms': {
        type: 'number',
        describe:
          'Delay in milliseconds between each contract deployment (default: 2000)',
        default: DEFAULT_DELAY_MS,
      },
    })
    .argv;

  return argv;
}

interface Inputs {
  deployerPrivateKey: string;
  delayMs?: number;
}
