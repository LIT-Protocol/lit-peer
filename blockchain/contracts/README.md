These contracts govern the Lit Nodes and various PKP things. Currently in testnet only.

Learn more here: https://developer.litprotocol.com/docs/litactionsandpkps/whatarelitactionsandpkps/

# Running Tests

## Forge Tests

We have some tests written in Solidity that can be run using Forge.

- Run tests with `forge test --ffi`
- Run tests with `forge test -vvvv --ffi` to make logs go BRRRRRRRR.

## Gas Reports

Forge snapshots are used to determine gas usage for various functions. We also measure custom snapshots such as `claimRewardsOver3Months`, which is output to the `snapshots` directory when you run `forge snapshot --ffi -vvvv`.

Alternatively, you can also use the `--gas-report` option to record the gas report for all the functions that were traced and used across all the tests that were run. For example, you can run `forge test --ffi --no-match-test 'Skip|invariant' --gas-report > gas_costs.txt`

## Replaying Test Failures

We currently only keep track of failures from the `StakingInvariantsTest` invariant test suite inside `testruns` directory. In order to only replay the failed tests, you can do `forge test --rerun`.

## Locally Against Anvil

In order to run the contract tests, you will need to follow these instrucitons:

1. Start an instance of our forked Anvil locally. The forked Anvil contains a precompile for the key derivation function and can be found here: https://github.com/LIT-Protocol/foundry
2. Run `npm run test`

## Locally Against Arbitrum

We currently support running a small number of contract tests against a local Arbitrum node. Specifically, only the `PKPNFT` suite of tests are supported.

First, clone the Arbitrum test node project as a sibling repo to `lit-assets` using `git clone -b release --recurse-submodules https://github.com/LIT-Protocol/nitro-testnode.git && cd nitro-testnode`

Then, these are the instructions to run the test:

1. Spin up local Arbitrum test node with `./test-node.bash --init`
2. Fund Anvil wallets using `./fund-wallets.bash`
3. Deploy the precompiles using `cargo stylus deploy --private-key <PRIV_KEY> --endpoint http://127.0.0.1:8547` in both the `p256` and the `k256` directories of [lit-precompiles](https://github.com/LIT-Protocol/lit-precompiles). Make a note of the deployed addresses.
4. Run the `PKPNFT` test against local node using `LIT_STYLUS_P256_CONTRACT_ADDRESS=<ADDR> LIT_STYLUS_K256_CONTRACT_ADDRESS=<ADDR>  npx hardhat test --network localchainArbitrum --grep PKPNFT`

# How to verify contracts

```shell
npx hardhat verify --network celo 0x5Ef8A5e3b74DE013d608740F934c14109ae12a81 \
  "0x0008a7B1Ce657E78b4eDC6FC40078ce8bf08329A"
```

The second param is any constructor params.

# Deploying

1. Run `npm install` to install project dependencies.
2. Run `npm run test` to test the smart contracts.
3. Export the private key to the environment depending on your deployment target - refer to `hardhat.config.ts` for more details. For example, if deploying to Polygon Mumbia, export `LIT_MUMBAI_DEPLOYER_PRIVATE_KEY=<YOUR-PRIVATE-KEY>`. If deploying to LIT Rollup (Chronicle), export `LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY=<YOUR-PRIVATE-KEY>`.
4. Export the API key for IPFS to the environment variable `IPFS_API_KEY`. You can also declare it inside a `.env` - refer to the `.env.example`.
5. `npm run deploy -- --network <NETWORK>`

- If you know exactly which deployment full config file you would like to use, you can do `npm run deploy -- --deploy-config <DEPLOY_FULL_CONFIG_PATH>`. The `--network` option is not needed here as the deploy config file contains that parameter.

**Note**: The wallet you provide should have at least 10 LIT for the gas to complete the entire deployment process which includes funding & staking the nodes which is called internally in the deploy script. If you don't have that much LIT you may ask Chris on Slack for it. You could also get some from the Chronicle Faucet but it only gives out a tiny amount so you would have to modify the deploy scripts to fund each of the node wallets with less tokens.

- [Chronicle Faucet](https://faucet.litprotocol.com/)

**Note**: The deploy script will set the ownership of each contract to the `newOwner` address defined in scripts/deploy_lit_node_contracts.js. If you need to call owner / admin functions on the contracts after they're deployed, you can set that `newOwner` address to something you control. If you're just using the contracts with the nodes you probably don't need to do this.

Once this script is done running, if you answered "y" to "Should we copy the node config files into the node folder", there will be config files for you generated in /node_configs of this repo. You can copy these to the /config folder of the lit_node_rust repo.

## Local Deployment

These are the instructions for deploying + the smart contracts locally:

1. Run a local blockchain / testnet using [Hardhat](https://hardhat.org/hardhat-network/docs/overview#running-stand-alone-in-order-to-support-wallets-and-other-software), [Anvil](https://book.getfoundry.sh/anvil/) or other software. It must be listening on port `8545` on `localhost`.
2. `npm run deploy -- --network localchain` and follow the interactive prompts.
3. Select `dev` for the environment.
4. Specify a wallet address that you own / have access to when specifying the `newOwnerAddress`.
5. Accept to copy the node configs to the Rust project.
6. Choose anywhere from 3 to 10 for the number of node wallets.
7. Use the default IP addresses as suggested.

Here is an example deployment configuration:

```json
{
  "deploymentSelection": "lit-core + lit-node",
  "deployNodeConfig": {
    "environment": "dev",
    "networkName": "localchain",
    "newOwnerAddress": "0x4259E44670053491E7b4FE4A120C70be1eAD646b",
    "numberOfStakedOnlyWallets": 3,
    "resolverContractAddress": "TBD",
    "useLitCoreDeploymentResolverContractAddress": true,
    "outputTempFilePath": "./deployed-lit-node-contracts-temp.json",
    "copyNodeConfigsToRustProject": true,
    "ipAddresses": ["127.0.0.1:7470", "127.0.0.1:7471", "127.0.0.1:7472"]
  },
  "deployCoreConfig": {
    "environment": "dev",
    "networkName": "localchain",
    "subnetOwnerAddress": "0xB77AEBbC262Bb809933D991A919A0e4A6A3b2f65",
    "subnetAdminPublicKey": "0x045f96e860435fccf287d9c2592fa129edfca7159c8dd2260cf2def38a9d5ee627ba73afef636467bc95fe551f10c862e910f18eafb751226d6901eab7d5b2794a",
    "subnetProvAddress": "0x3324439C8b9181eF07D54030E32d2CD22FF0C6A7",
    "outputTempFilePath": "./deployed-lit-core-contracts-temp.json"
  }
}
```

# Contract Deployment Tooling

We have developed a tool that makes it convenient to deploy and configure our suite of smart contracts before spinning up a network of nodes against them. Specifically, our tool helps with:

- Deploying Lit Core and/or Lit Node smart contracts to any supported chain
- After deploying smart contracts, configure the smart contract parameters and settings per each of the node operators

## Technical Details

- The tool is available as a `npm` script - `npm run deploy -- --network <NETWORK>`
  - The currently supported network names are:
    - `celo`
    - `mumbai`
    - `alfajores`
    - `polygon`
    - `litTestnet`
    - `lit`
    - `localchain`
- At a high-level, the tool consists of 2 main steps:
  1. An **interactive** step that determines the entire set of deployment configurations that will be used.
  2. A **non-interactive** step that takes a deployment configuration and deploys and configures a set of smart contracts accodingly.
- Running the entire tool as it is will involve an interactive experience (eg. command-line experience). If you wish to have a non-interactive experience, that is only available by running the tool and specifying exactly which deployment configuration you would like to use, ie. `npm run deploy -- --deploy-config <DEPLOY_FULL_CONFIG_PATH>`. The `--network` option is not needed here as the deploy config file contains that parameter.
- The non-interactive deployment step is run as a child process that is spawned. All environment variables are inherited in the spawned environment. We pass in additional environment variables.
- When running the interactive first step,
  - you will have the option to choose whether to deploy:
    1. Only the Lit Core contracts
    2. Only the Lit Node contracts
    3. Both the Lit Core and Lit Node contracts, and in that order.
  - you will have the option to choose a previously generated deployment configuration.
    - **This is the true power of this tool, in allowing you to reference EXACTLY a deployment configuration that has worked well for your needs previously**.
- This tool is environment-aware and will ask for confirmation before proceeding to use parameters specified in your shell session. Refer to the Deployment Configuration Reference section below for more details.
- This tool uses [`inquirer.js`](https://github.com/SBoudrias/Inquirer.js) to create the interactive experience.

## Deployment Configuration

The deployment configuration refers to the entire set of parameters that will be used for deploying and configuring the smart contracts.

### Persistence

- Each deployment configuration is persisted to disk locally under the `scripts/deployConfig/configs` directory.
- In order for a persisted deployment configuration to be detected by the tool, it must match the following pattern `deploy-config-*.json`

### Reference

Here is an explanation for each of the fields in the deployment configuration:

| Key Path                                                       | Type       | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| -------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `deploymentSelection`                                          | `string`   | An enum of either `lit-core`, `lit-core + lit-node` or `lit-node` describing which set of smart contracts should be deployed.                                                                                                                                                                                                                                                                                                                                                                  |
| `deployNodeConfig`                                             | `object`   | The deployment configuration parameters that relate to deploying the Lit Node smart contracts.                                                                                                                                                                                                                                                                                                                                                                                                 |
| `deployNodeConfig.environment`                                 | `string`   | An enum of either `dev`, `staging` or `prod` describing which deployment environment the Lit Node contracts should be deployed to.                                                                                                                                                                                                                                                                                                                                                             |
| `deployNodeConfig.networkName`                                 | `string`   | The name of the network (chain) the Lit Node contracts should be deployed to.                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `deployNodeConfig.newOwnerAddress`                             | `string`   | The EVM-compatible address that will be given ownership and configuration permissions once the tool finishes. While the tool uses the deployer address to configure the smart contract parameters after deployment, you would most likely want to revoke admin / owner permissions from the deployer and grant your own address such permissions after the tool finishes. If `LIT_OWNER_WALLET_ID` is set in your environment, the tool will ask for confirmation before using this parameter. |
| `deployNodeConfig.numberOfStakedOnlyWallets`                   | `number`   | The number of nodes (and node operators) that only stake on the network.                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `deployNodeConfig.numberOfStakedAndJoinedWallets`              | `number`   | The number of nodes (and node operators) that stake and request to join the network. If a node is already accounted for in `deployNodeConfig.numberOfStakedOnlyWallets`, do not account for that node here, as the total number of stakers will be the sum of the two. The nodes that are joining will be the first `numberOfStakedAndJoinedWallets` entries from the overall node wallets list that is generated.                                                                             |
| `deployNodeConfig.resolverContractAddress`                     | `string`   | The Lit Core `ContractResolver` contract address that will be referenced. It will be marked as `TBD` when `deployNodeConfig.useLitCoreDeploymentResolverContractAddress` is set to `true`, since we won't know the smart contract address until the non-interactive deployment step of the tool. If `LIT_RESOLVER_CONTRACT_ADDRESS` is set in your environment, the tool will ask for confirmation before using this parameter.                                                                |
| `deployNodeConfig.useLitCoreDeploymentResolverContractAddress` | `boolean`  | Whether to use the `ContractResolver` contract address from deploying the Lit Core contracts.                                                                                                                                                                                                                                                                                                                                                                                                  |
| `deployNodeConfig.outputTempFilePath`                          | `string`   | The path to the file containing the addresses of the deployed Lit Node smart contracts.                                                                                                                                                                                                                                                                                                                                                                                                        |
| `deployNodeConfig.copyNodeConfigsToRustProject`                | `boolean`  | Whether to copy the generated node configs over to the Rust project. You will likely need this when spinning up a network locally on your machine.                                                                                                                                                                                                                                                                                                                                             |
| `deployNodeConfig.ipAddresses`                                 | `string[]` | An array of strings representing the IP addresses of the node operators. You will likely need this when spinning up a network locally on your machine. If `IP_ADDRESSES` is set in your environment, the tool will ask for confirmation before using this parameter.                                                                                                                                                                                                                           |
| `deployNodeConfig.existingRouterAndPkpContracts`               | `object`   | An object containing the addresses of smart contracts from a prior deployment to be referenced again in this current deployment.                                                                                                                                                                                                                                                                                                                                                               |
| `deployCoreConfig`                                             | `object`   | The deployment configuration parameters that relate to deploying the Lit Core smart contracts.                                                                                                                                                                                                                                                                                                                                                                                                 |
| `deployCoreConfig.environment`                                 | `string`   | An enum of either `dev`, `staging` or `prod` describing which deployment environment the Lit Core contracts should be deployed to.                                                                                                                                                                                                                                                                                                                                                             |
| `deployCoreConfig.networkName`                                 | `string`   | The name of the network (chain) the Lit Core contracts should be deployed to.                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `deployCoreConfig.subnetOwnerAddress`                          | `string`   | The address of the subnet owner.                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `deployCoreConfig.subnetAdminPublicKey`                        | `string`   | The public key of the subnet admin.                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `deployCoreConfig.subnetProvAddress`                           | `string`   | The address of the wallet that provisions the subnet.                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `deployCoreConfig.outputTempFilePath`                          | `string`   | The path to the file containing the addresses of the deployed Lit Core smart contracts.                                                                                                                                                                                                                                                                                                                                                                                                        |
| `deploySensitiveConfig`                                        | `string`   | The deployment configuration parameters that are sensitive. These parameters are never stored to disk and are only provided via the environment.                                                                                                                                                                                                                                                                                                                                               |
| `deploySensitiveConfig.ipfsApiKey`                             | `string`   | The IPFS API key                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

# Deployed Contract Addresses

Deployed contract addresses are listed by network in this repo: https://github.com/LIT-Protocol/networks

# Staking Mechanism

## Overview

These staking contracts facilitate the validators of the Lit Protocol network. Validators are responsible for being online so they can facilitate decentralized threshold cryptography for various use cases. These validators need to be held accountable for their performance otherwise they will be slashed. They should be compensated for uptime in the form of rewards. Validator operators and delegators can also stake tokens to a validator and earn the rewards generated from the validators. The three main components of this system are the VoteEscrow-esque staking mechanics, slashing, and the validator epoch loop. Note that these contracts will be on a Conduit L3 so gas usage is not of concern. We also have a mechanism design paper that describes how this system should work (note the paper is "theoretical" and the implementation details differ in practice but the end effect is the same).

## VoteEscrow

These contracts take inspiration from [veCRV](https://etherscan.deth.net/token/0x5f3b5DfEb7B28CDbD7FAba78963EE202a494e2A2#code), the 3CRV reward [distributor](https://etherscan.deth.net/address/0xa464e6dcda8ac41e03616f95f4bc98a13b8922dc#code), and [Prisma VE](https://github.com/prisma-fi/prisma-contracts/blob/main/contracts/dao/TokenLocker.sol). No code is actually taken from these, but the principles of how they work is much the same. These existing solutions work with more complicated "Points" which are stored at the end of each interaction with the protocol; however, since here you can't edit unfrozen locks or refreeze locks, we are able to have a much simpler system.

### StakeWeight

A user who stakes 1 token for 10 days will have a third as much StakeWeight as someone who stakes 1 token for 30 days. Someone who stakes 1 token for 10 days will have a third as much StakeWeight as someone who stakes 3 tokens for 10 days. StakeWeight for a given amount of tokens is in the range (0, 1] depending on how long the user locks for. Reward allocation amongst stakers within a validator is proportional to staker's StakeWeights. Reward allocation amongst validators is proportional to the aggregate StakeWeight of all the delegations. Users can `stake` to create a `StakeRecord` and lock their tokens. Users will earn rewards pro-rata to their percentage of the total stake and can claim these via `ClaimStakeRewards`. Users can increase the length of their lock, split their lock into multiple, and increase the stake amount of their lock. Users can unfreeze their stake, which causes the stakeweight to start to linearly decay; once it reaches zero, users can withdraw their tokens.

### RewardEpoch and RewardEpochGlobalStats

Operating VE in a continous manner is near-impossible, so we opt for a version discretized into daily RewardEpochs. Each validator has a set of RewardEpochs. The RewardEpoch stores information such as the total stake weight (sum of all the stakeweights from delegators), totalStakeRewards (how many rewards the validator has earned for being in the set), slope (how much the totalStakeWeight should decrease going into the next epoch due to stakeWeights unfreezing). `RewardEpochGlobalStats` stores these metrics but globally across validators for the purpose of calculating validator share for reward distribution.

## Slashing

When validators aren't online enough, or fail signings etc, they are eligible to be slashed by other validators. When a slashing occurs, we decrease all values in `RewardEpoch` accordingly (updating global stats as well of course). There are some intricacies though. For example, if a slashing occurs to a validator that has a Stake unfreezing, then that stake's slope increase will be too high. We must adjust these on the fly. We must also recalculate how much a user is eligible to withdraw from a StakeRecord once their time finishes.

### UpdateRewardEpoch

This is the function called in `advanceEpoch` that performs these updates when a new RewardEpoch starts (i.e when a period of 86400s finishes). Keep in mind that `advanceEpoch` is called every hour to adjust the regular epochs that determine validator uptime, inclusion, and reward distribution. Every 86400s, `updateRewardEpoch` within `advanceEpoch` will perform the previously mentioned updates and copy the staking information into a new `RewardEpoch`.

### Validator Operations

Right now validators are permissioned - the network will launch with 10. Validators must self-stake a certain amount of LITKEY for a certain period of time to be eligible to enter the active set. Validator operators earn a fixed USD amount of LITKEY to cover costs and a percentage of emissions relative to how much they have staked.

Validators must `requestToJoin` and signal that they're ready to join the next epoch every hour. Validators will be the ones to call `advanceEpoch` but anyone can, although there must be a certain number of validators in the set for this to happen.

## Contracts

`Staking` is a diamond proxy with various facets (the name explains the purpose of each).

## Realms

A realm is a logical grouping of validators within a set. A realm has its own storage in `RealmStorage` as differentiated via distinct storage locations, and all realms share the same `GlobalStorage`.

## Attribution Semantics

A staked validator must be part of a realm in order to earn rewards. Below we outline how staking details are attributed to various reward epoch numbers:

- Staking is a separate action from joining / leaving a realm. Because of this, a validator proceeding to stake does not provide sufficient information regarding which realm they will join - after all, they have yet to call `requestToJoin` against a particular realm.
- When a validator calls `requestToJoin` to a particular realm, ALL of their stake records will be attributed to the next (upcoming) reward epoch number for that realm. For example, if Realm 1 is at reward epoch 3 at the time of Validator 1 requesting to join it, then Validator 1's stake records will attribute towards updating the reward epoch and global stats information for the next reward epoch number (eg. 4).
  - Note that a validator can only be part of one single realm at any point in time. This is why we can attribute all of their stake records towards a particular realm's reward epoch details.
- When a realm's epoch advances, the current epoch active validators' reward epochs and global stats are attributed to the next reward epoch and global stats details. In other words, the current epoch number's reward epoch and global stats are almost always determined by the data from the previous epoch number - the one exception is if any slashing occurs. Furthermore, when a realm's epoch advances, the total rewards for the current reward epoch number is calculated and stored in the reward epoch and global stats data structures.
- When a validator is slashed, their validator share price for the current reward epoch number is immediately updated, which in turn affects the total rewards made available to that validator as the epoch advances and rewards are calculated.
- Since the next reward epoch's details are calculated based on the current epoch's details and which validators are currently active, validators that have requested to leave will be dealt out of the validator set and naturally they will not be contributing to the reward epoch and global stats in the upcoming epochs.
- When a staker migrates their stake from an inactive validator to an active one, the realm's current reward epoch and global stats are immediately updated.
- When a staker increases their stake record or timelock, the current reward epoch and global stats are immediately updated.
