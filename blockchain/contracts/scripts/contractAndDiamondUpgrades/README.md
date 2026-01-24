**IMPORTANT** You must run `npx hardhat clean` then `npx hardhat compile` before running the scripts in this directory.  Otherwise, you'll deploy all the old contracts.

If you're upgrading a bunch of diamond facets, you can use the `deployMultipleContracts.ts` script to deploy them all in parallel.  This will create a JSON file called `deployedMultipleContracts.json` in the root of the project.

Go to the networks repo and grab the `deployed-lit-node-contracts-temp.json` file for the network you're upgrading, and put it in the root of this project (the contracts folder).  

You can then run ```HARDHAT_NETWORK=yellowstone npx ts-node --files scripts/contractAndDiamondUpgrades/createDiamondManifest.ts \
  --deployed-multiple-contracts-json deployedMultipleContracts.json \
  --previous-deploy-json deployed-lit-node-contracts-temp.json``` for example to create the diamond manifest.  

  You can then run `HARDHAT_NETWORK=yellowstone npx ts-node --files scripts/diamondContracts/diamondCut.ts --diamond-owner-signer-private-key <KEY>` to cut the diamond from the manifest.  

  After this process, make sure you copy the `deployedMultipleContracts.json` new facet addresses into the correct `deployed-lit-node-contracts-temp.json` file in the Networks repo.