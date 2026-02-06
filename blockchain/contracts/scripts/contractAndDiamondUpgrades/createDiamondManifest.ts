// Full command:
// HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/contractAndDiamondUpgrades/createDiamondManifest.ts \
//   --deployed-multiple-contracts-json deployedMultipleContracts.json \
//   --previous-deploy-json deployed-lit-node-contracts-temp.json \
//   [--manifest-file scripts/diamondContracts/manifests/diamondCutManifest.json] \
//   [--reset-manifest true]

import hre from 'hardhat';
import yargs from 'yargs';
import fs from 'fs/promises';
import path from 'path';

import {
  MANIFESTS_DIR,
  appendDiamondCutOperationToManifest,
} from '../diamondContracts/lib/diamondCutManifest';
import { FacetCutAction } from '../diamondContracts/lib/types';

const { ethers } = hre;

type Address = string;

type PreviousDeployJson = {
  stakingContractAddress?: Address;
  pubkeyRouterContractAddress?: Address;
  pkpNftContractAddress?: Address;
  pkpPermissionsContractAddress?: Address;
  backupRecoveryContractAddress?: Address;
  priceFeedContractAddress?: Address;
  facets?: Record<string, Array<{ facetName: string; facetAddress: Address }>>;
};

type DiamondAddressKey = Exclude<keyof PreviousDeployJson, 'facets'>;

const DIAMOND_GROUP_TO_ADDRESS_KEY: Record<string, DiamondAddressKey> = {
  Staking: 'stakingContractAddress',
  PubkeyRouter: 'pubkeyRouterContractAddress',
  PKPNFT: 'pkpNftContractAddress',
  PKPPermissions: 'pkpPermissionsContractAddress',
  BackupRecovery: 'backupRecoveryContractAddress',
  PriceFeed: 'priceFeedContractAddress',
};

/**
 * Infer the diamond group for a facet that doesn't exist in the previous deploy
 * (i.e. a brand-new facet). We check if the contract name starts with any known
 * diamond group name (e.g. "PubkeyRouterViewsFacet" → "PubkeyRouter").
 * Returns the matching group name, or undefined if none match.
 */
function inferDiamondGroup(contractName: string): string | undefined {
  // Sort by longest group name first so that more specific prefixes match first
  const sortedGroups = Object.keys(DIAMOND_GROUP_TO_ADDRESS_KEY).sort(
    (a, b) => b.length - a.length
  );
  for (const group of sortedGroups) {
    if (contractName.startsWith(group)) {
      return group;
    }
  }
  return undefined;
}

async function run() {
  const inputs = await getInputsFromCliOptions();

  const previousDeploy: PreviousDeployJson = JSON.parse(
    await fs.readFile(inputs.previousDeployJson, 'utf8')
  );
  const deployedMultiple: Record<string, Address> = JSON.parse(
    await fs.readFile(inputs.deployedMultipleContractsJson, 'utf8')
  );

  if (!previousDeploy.facets) {
    throw new Error(
      `Missing "facets" in ${inputs.previousDeployJson}; can't look up old facet addresses`
    );
  }

  const manifestFilePath =
    inputs.manifestFile ??
    path.join(
      __dirname,
      '..',
      'diamondContracts',
      MANIFESTS_DIR,
      'diamondCutManifest.json'
    );

  if (inputs.resetManifest) {
    await fs.mkdir(path.dirname(manifestFilePath), { recursive: true });
    await fs.writeFile(
      manifestFilePath,
      JSON.stringify({ operations: [] }, null, 2)
    );
    console.log(`Reset manifest at ${manifestFilePath}`);
  }

  const facetToGroup: Record<string, string> = {};
  const facetToOldAddress: Record<string, Address> = {};

  for (const [groupName, facets] of Object.entries(previousDeploy.facets)) {
    for (const f of facets) {
      facetToGroup[f.facetName] = groupName;
      facetToOldAddress[f.facetName] = f.facetAddress;
    }
  }

  const facetNames = Object.keys(deployedMultiple);
  console.log(`Preparing manifest for ${facetNames.length} facets...`);

  for (const contractName of facetNames) {
    const newFacetAddress = deployedMultiple[contractName];
    let groupName: string | undefined = facetToGroup[contractName];
    const oldFacetAddress = facetToOldAddress[contractName];
    const isNewFacet = !groupName;

    // If the facet isn't in the previous deploy, it's brand new.
    // Infer its diamond group from the contract name.
    if (isNewFacet) {
      groupName = inferDiamondGroup(contractName);
      if (!groupName) {
        throw new Error(
          `Unable to find diamond group for new facet "${contractName}". ` +
            `It was not in ${inputs.previousDeployJson} and its name doesn't match any known group in DIAMOND_GROUP_TO_ADDRESS_KEY. ` +
            `Known groups: ${Object.keys(DIAMOND_GROUP_TO_ADDRESS_KEY).join(', ')}`
        );
      }
      console.log(
        `\n[NEW FACET] "${contractName}" not found in previous deploy; inferred diamond group "${groupName}". Will add only (no removal).`
      );
    }

    const diamondAddressKey = DIAMOND_GROUP_TO_ADDRESS_KEY[groupName];
    if (!diamondAddressKey) {
      throw new Error(
        `No diamond address mapping for group "${groupName}". Add it to DIAMOND_GROUP_TO_ADDRESS_KEY.`
      );
    }
    const diamondAddress = previousDeploy[diamondAddressKey];
    if (!diamondAddress) {
      throw new Error(
        `Missing "${diamondAddressKey}" in ${inputs.previousDeployJson}; needed for facet "${contractName}"`
      );
    }
    if (!isNewFacet && !oldFacetAddress) {
      throw new Error(
        `Unable to find old facet address for "${contractName}" in ${inputs.previousDeployJson}`
      );
    }
    if (!newFacetAddress) {
      throw new Error(
        `Missing new facet address for "${contractName}" in ${inputs.deployedMultipleContractsJson}`
      );
    }

    const newFacet = await ethers.getContractAt(contractName, newFacetAddress);

    if (isNewFacet) {
      console.log(
        `\n${contractName}\n  diamond: ${diamondAddress}\n  old:    (none — new facet)\n  new:    ${newFacetAddress}`
      );

      // Only add — nothing to remove for a brand-new facet
      await appendDiamondCutOperationToManifest(
        manifestFilePath,
        diamondAddress,
        newFacet,
        FacetCutAction.Add,
        contractName
      );
    } else {
      console.log(
        `\n${contractName}\n  diamond: ${diamondAddress}\n  old:    ${oldFacetAddress}\n  new:    ${newFacetAddress}`
      );

      // 1) Remove old facet selectors from the diamond (uses oldFacetAddress)
      await appendDiamondCutOperationToManifest(
        manifestFilePath,
        diamondAddress,
        newFacet,
        FacetCutAction.Remove,
        contractName,
        oldFacetAddress
      );

      // 2) Add new facet selectors to the diamond
      await appendDiamondCutOperationToManifest(
        manifestFilePath,
        diamondAddress,
        newFacet,
        FacetCutAction.Add,
        contractName
      );
    }
  }

  console.log(`\nDiamond cut manifest updated at ${manifestFilePath}`);
}

run()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'deployed-multiple-contracts-json': {
      type: 'string',
      describe:
        'Path to deployedMultipleContracts.json (new facet addresses keyed by facet name)',
      default: 'deployedMultipleContracts.json',
    },
    'previous-deploy-json': {
      type: 'string',
      describe:
        'Path to previous deployment JSON (must contain top-level diamond addresses + facets map)',
      default: 'deployed-lit-node-contracts-temp.json',
    },
    'manifest-file': {
      type: 'string',
      describe:
        'Output manifest file path (defaults to scripts/diamondContracts/manifests/diamondCutManifest.json)',
      required: false,
    },
    'reset-manifest': {
      type: 'boolean',
      describe:
        'If true, overwrite manifest with an empty operations list before appending',
      default: true,
    },
  }).argv;

  return {
    deployedMultipleContractsJson: argv['deployed-multiple-contracts-json'],
    previousDeployJson: argv['previous-deploy-json'],
    manifestFile: argv['manifest-file'],
    resetManifest: argv['reset-manifest'],
  };
}

interface Inputs {
  deployedMultipleContractsJson: string;
  previousDeployJson: string;
  manifestFile?: string;
  resetManifest: boolean;
}
