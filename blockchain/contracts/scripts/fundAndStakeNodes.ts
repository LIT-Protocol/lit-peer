// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const realmId = 1;
import fs from 'fs';
import hre from 'hardhat';
import {
  DeployNodeConfig,
  FundAndStakeNodesOutput,
  NodeOperatorCredentials,
  ParsedNodeContracts,
  WalletManifestItem,
} from './deployConfig';

import { TransactionResponse } from '@ethersproject/abstract-provider';

import { ContractTransactionResponse } from 'ethers';
import {
  Ownable,
  OwnershipFacet,
  StakingFacet,
  StakingValidatorFacet,
  StakingAdminFacet,
} from '../typechain-types';
import { CONTRACT_NAME_TO_JSON_CONTRACT_ADDRESS_KEY } from './constants';
import {
  contractAddressAlreadyExists,
  generateWallets,
  getContractInstance,
  walletManifestItemsToNodeOperatorCredentials,
} from './utils';

const { ethers } = hre;
const wlitAddress = hre.network.config.wlitAddress || false;

// how much gas to send to the nodes, and to the staker addresses.
// note that this will be divided up by the walletCount
const nodeAmount = ethers.parseEther('10');
const stakerAmount = ethers.parseEther('1');

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

const ip2int = (ip: string) => {
  return (
    ip.split('.').reduce(function (ipInt, octet) {
      return (ipInt << 8) + parseInt(octet, 10);
    }, 0) >>> 0
  );
};

const getSigner = async () => {
  const [deployer] = await ethers.getSigners();
  return deployer;
};

const fundWalletsWithGas = async (
  nodeOperatorsCredentials: Array<NodeOperatorCredentials>,
  contracts: ParsedNodeContracts
) => {
  const signer = await getSigner();

  const multisenderContract = await ethers.getContractAt(
    'Multisender',
    contracts.multisenderContractAddress,
    signer
  );
  console.log(
    'multisender contract address is ',
    await multisenderContract.getAddress()
  );

  const nodeTx = await multisenderContract.sendEth(
    nodeOperatorsCredentials.map((w) => w.nodeWallet.address),
    { value: nodeAmount }
  );
  console.log('fundWalletsWithGas nodeTx: ', nodeTx);

  const stakerTx = await multisenderContract.sendEth(
    nodeOperatorsCredentials.map((w) => w.stakerWallet.address),
    { value: stakerAmount }
  );
  console.log('fundWalletsWithGas stakerTx: ', stakerTx);

  await Promise.all([nodeTx.wait(), stakerTx.wait()]);
  console.log('mined nodeTx and stakerTx');
};

const fundWalletsWithTokens = async (
  nodeOperatorsCredentials: Array<NodeOperatorCredentials>,
  contracts: ParsedNodeContracts
) => {
  const signer = await getSigner();

  const stakingFacetContract = await getStakingFacet(contracts, signer);
  // send twice the min stake for each node
  const amountPerWallet =
    (await stakingFacetContract.getMinimumSelfStake()) * 2n;
  console.log('amountPerWallet: ', amountPerWallet);
  const multisenderContract = await ethers.getContractAt(
    'Multisender',
    contracts.multisenderContractAddress,
    signer
  );

  const stakerTx = await multisenderContract.sendTokensExact(
    nodeOperatorsCredentials.map((w) => w.stakerWallet.address),
    contracts.litTokenContractAddress,
    amountPerWallet
  );
  console.log('fundWalletsWithTokens stakerTx: ', stakerTx);
  await stakerTx.wait();
  console.log('stakerTx mined');
};

export const fundAdministratorWallet = async (
  adminWalletAddress: string,
  deployNodeConfig: DeployNodeConfig
) => {
  console.log('fundAdministratorWallet: ', adminWalletAddress);

  const fileName = deployNodeConfig.outputTempFilePath;
  console.log('reading from file: ' + fileName);
  let contractsJsonStr = fs.readFileSync(fileName);
  const contracts: ParsedNodeContracts = JSON.parse(
    contractsJsonStr.toString()
  );

  const signer = await getSigner();

  const multisenderContract = await ethers.getContractAt(
    'Multisender',
    contracts.multisenderContractAddress,
    signer
  );
  console.log(
    'multisender contract address is ',
    await multisenderContract.getAddress()
  );

  const ethTx = await multisenderContract.sendEth([adminWalletAddress], {
    value: nodeAmount,
  });
  console.log('fundAdministratorWallet ethTx: ', ethTx);
  const lit_balance = await hre.ethers.provider.getBalance(
    contracts.litTokenContractAddress
  );
  console.log('lit token contract balance: ', lit_balance);

  await Promise.all([ethTx.wait()]);
  console.log('mined admin wallet eth tx');

  const tokenTx = await multisenderContract.sendTokens(
    [adminWalletAddress],
    contracts.litTokenContractAddress
  );
  console.log('fund admin wallet with LIT tokens: ', tokenTx);
  await tokenTx.wait();
  console.log('tokenTx mined');

  const balance = await hre.ethers.provider.getBalance(adminWalletAddress);
  console.log('admin wallet balance: ', balance);
  const lit_balance2 = await hre.ethers.provider.getBalance(
    contracts.litTokenContractAddress
  );
  console.log('lit token contract balance: ', lit_balance2);

  const stakingAdminContract = await getStakingAdminContract(contracts, signer);
  const epochLength = await stakingAdminContract.setDevopsAdmin(
    adminWalletAddress
  );
};

const getStakingFacet = async (
  contracts: ParsedNodeContracts,
  signer: any
): Promise<StakingFacet> => {
  return ethers.getContractAt(
    'StakingFacet',
    contracts.stakingContractAddress,
    signer
  );
};

const getStakingValidatorFacet = async (
  contracts: ParsedNodeContracts,
  signer: any
): Promise<StakingValidatorFacet> => {
  return ethers.getContractAt(
    'StakingValidatorFacet',
    contracts.stakingContractAddress,
    signer
  );
};

const getStakingAdminContract = async (
  contracts: ParsedNodeContracts,
  signer: any
): Promise<StakingAdminFacet> => {
  return ethers.getContractAt(
    'StakingAdminFacet',
    contracts.stakingContractAddress,
    signer
  );
};

const stakeTokensAndJoinValidators = async (
  allNodeOperatorsCredentials: Array<NodeOperatorCredentials>,
  numberOfStakedAndJoinedWallets: number,
  numberOfStakedOnlyWallets: number,
  contracts: ParsedNodeContracts
) => {
  const signer = await getSigner();

  const stakingFacetContract = await getStakingFacet(contracts, signer);
  const stakingValidatorFacetContract = await getStakingValidatorFacet(
    contracts,
    signer
  );

  let litTokenContract;
  if (wlitAddress) {
    // use wlit
    litTokenContract = await ethers.getContractAt(
      'WLIT',
      contracts.litTokenContractAddress,
      signer
    );
  } else {
    litTokenContract = await ethers.getContractAt(
      'LITToken',
      contracts.litTokenContractAddress,
      signer
    );
  }

  // approve all stakers
  const amountToStake = await stakingFacetContract.getMinimumSelfStake();
  console.log('amountToStake: ', amountToStake);
  console.log('sending approval txns now');
  const totalStakers =
    numberOfStakedAndJoinedWallets + numberOfStakedOnlyWallets;
  const approvalPromises = [];
  for (let i = 0; i < totalStakers; i++) {
    const nodeOperatorCredential = allNodeOperatorsCredentials[i];

    const connectedStakerWallet = nodeOperatorCredential.stakerWallet.connect(
      ethers.provider
    );

    const litTokenContractAsStaker = litTokenContract.connect(
      connectedStakerWallet
    );

    console.log(
      'stakeTokens - approving tokens for staker: ',
      connectedStakerWallet.address
    );
    const approvalTx = await litTokenContractAsStaker.approve(
      contracts.stakingContractAddress,
      amountToStake
    );
    console.log('approvalTx for wallet ' + i + ': ', approvalTx);

    approvalPromises.push(approvalTx);
  }

  console.log('awaiting approval txns to be mined...');
  await Promise.all(
    approvalPromises.map((tx: ContractTransactionResponse) => {
      return tx.wait();
    })
  );

  // stake all the staker wallets
  console.log('sending staking txns now');
  const stakingPromises = [];
  for (let i = 0; i < totalStakers; i++) {
    const nodeOperatorCredential = allNodeOperatorsCredentials[i];

    const connectedStakerWallet = nodeOperatorCredential.stakerWallet.connect(
      ethers.provider
    );

    const stakingContractAsStaker = stakingFacetContract.connect(
      connectedStakerWallet
    );
    // check balance
    const balance = await litTokenContract.balanceOf(
      connectedStakerWallet.address
    );
    console.log(`balance for ${connectedStakerWallet.address}: `, balance);
    console.log(
      'stakeTokens - staking tokens for staker: ',
      connectedStakerWallet.address
    );
    const timeLock = 86400n * 120n; // 120 days
    const stakerAddress = connectedStakerWallet.address;
    const tx = await stakingContractAsStaker.stake(
      amountToStake,
      timeLock,
      stakerAddress
    );
    console.log('stakeTokens tx for wallet ' + i + ': ', tx);

    stakingPromises.push(tx);
  }
  console.log(`awaiting ${stakingPromises.length} staking txns to be mined...`);
  await Promise.all(
    stakingPromises.map((tx: ContractTransactionResponse) => {
      return tx.wait();
    })
  );

  // request to join for the stakers that are joining
  console.log(
    'sending requestToJoin and setIpPortNodeAddressAndCommunicationPubKeys txns now'
  );
  let setIpAndrequestToJoinPromises = [];
  for (let i = 0; i < totalStakers; i++) {
    const nodeOperatorCredential = allNodeOperatorsCredentials[i];

    const ipAsInt = ip2int(contracts.litNodeDomainName);
    const ip = BigInt(ipAsInt);
    const ipv6 = 0n;
    const basePort = BigInt(contracts.litNodePort);
    const port = basePort + BigInt(i);

    const connectedStakerWallet = nodeOperatorCredential.stakerWallet.connect(
      ethers.provider
    );

    const stakingContractAsStaker = stakingValidatorFacetContract.connect(
      connectedStakerWallet
    );

    // For the wallets that are supposed to be staked and joined, we call requestToJoin additiionally.
    // For all other wallets, we only call setIpPortNodeAddressAndCommunicationPubKeys.
    let tx = await stakingContractAsStaker.setIpPortNodeAddress(
      ip,
      ipv6,
      port,
      nodeOperatorCredential.nodeWallet.address
    );
    console.log(
      'setIpPortNodeAddressAndCommunicationPubKeys tx for wallet ' + i + ': ',
      tx
    );
    setIpAndrequestToJoinPromises.push(tx);
    if (i < numberOfStakedAndJoinedWallets) {
      tx = await stakingContractAsStaker.requestToJoin(realmId);
      console.log('requestToJoin tx for wallet ' + i + ': ', tx);
      setIpAndrequestToJoinPromises.push(tx);
    }
  }

  console.log(
    `awaiting ${setIpAndrequestToJoinPromises.length} requestToJoin txns to be mined...`
  );
  await Promise.all(
    setIpAndrequestToJoinPromises.map((tx: ContractTransactionResponse) => {
      return tx.wait();
    })
  );
};

const lockValidatorSet = async (contracts: ParsedNodeContracts) => {
  const signer = await getSigner();
  const stakingValidatorFacetContract = await getStakingValidatorFacet(
    contracts,
    signer
  );

  const lockTx = await stakingValidatorFacetContract.lockValidatorsForNextEpoch(
    realmId
  );
  console.log('lockTx: ', lockTx.hash);
  await lockTx.wait();
  console.log('lockTx mined');
};

const setEpochLength = async (contracts: ParsedNodeContracts, signer: any) => {
  console.log('setting epoch length to 5 mins');
  const stakingAdminContract = await getStakingAdminContract(contracts, signer);
  const epochLength = 300n;
  const setEpochLengthTx = await stakingAdminContract.setEpochLength(
    realmId,
    epochLength
  );
  console.log('setEpochLengthTx: ', setEpochLengthTx.hash);
  await setEpochLengthTx.wait();
  console.log('setEpochLengthTx mined');
};

async function setIpAddresses(
  ipAddresses: string[],
  nodeOperatorsCredentials: Array<NodeOperatorCredentials>,
  contracts: ParsedNodeContracts
) {
  for (let i = 0; i < nodeOperatorsCredentials.length; i++) {
    const nodeOperatorCredential = nodeOperatorsCredentials[i];
    const signer = new ethers.Wallet(
      nodeOperatorCredential.stakerWallet.privateKey,
      ethers.provider
    );
    const stakingValidatorFacet = await getStakingValidatorFacet(
      contracts,
      signer
    );

    // prompt for ip address to set for the node
    const ipAddress = ipAddresses[i];
    let ip = ipAddress;
    let port = 443;
    if (ipAddress.includes(':')) {
      const parts = ipAddress.split(':');
      ip = parts[0];
      port = parseInt(parts[1]);
    }

    const ipBn = BigInt(ip2int(ip));
    const ipv6 = 0n;
    const portBn = BigInt(port);

    console.log('setting ip address for node: ', {
      ipBn,
      ipv6,
      portBn,
      nodeOperatorCredential,
    });

    const txn = await stakingValidatorFacet.setIpPortNodeAddress(
      ipBn,
      ipv6,
      portBn,
      nodeOperatorCredential.nodeWallet.address
    );
    console.log(`Transaction hash: ${txn.hash}`);
    await txn.wait();
    console.log(
      `Transaction mined: https://chain.litprotocol.com/tx/${txn.hash}`
    );
  }
}

export async function fundAndStakeNodes(
  deployNodeConfig: DeployNodeConfig
): Promise<FundAndStakeNodesOutput> {
  const fileName = deployNodeConfig.outputTempFilePath;
  console.log('reading from file: ' + fileName);
  let contractsJsonStr = fs.readFileSync(fileName);
  const contracts: ParsedNodeContracts = JSON.parse(
    contractsJsonStr.toString()
  );

  const totalStakers =
    deployNodeConfig.numberOfStakedOnlyWallets +
    deployNodeConfig.numberOfStakedAndJoinedWallets;
  if (totalStakers === 0) {
    console.log('No node wallets to fund and stake');
    return { nodeOperatorsCredentials: [], contracts };
  }

  const chainName = deployNodeConfig.networkName;

  console.log('Funding and staking to network', {
    deployNodeConfig,
    chainName,
  });

  // *** 1. Generate or use existing wallets
  let nodeOperatorsCredentials: Array<NodeOperatorCredentials> = [];
  if (deployNodeConfig.existingWalletCredentialsPath) {
    // Read from that path.
    console.info(
      'Reading existing wallet credentials from file: ',
      deployNodeConfig.existingWalletCredentialsPath
    );
    const existingWalletCredentialsStr = await fs.promises.readFile(
      deployNodeConfig.existingWalletCredentialsPath
    );
    const existingWalletCredentials: Array<WalletManifestItem> = JSON.parse(
      existingWalletCredentialsStr.toString()
    );
    // Map into node operator credentials.
    nodeOperatorsCredentials = walletManifestItemsToNodeOperatorCredentials(
      existingWalletCredentials,
      ethers.provider
    );
  } else {
    nodeOperatorsCredentials = generateWallets(ethers, totalStakers);
  }

  // *** 2. Fund node and staker wallets with gas
  await fundWalletsWithGas(nodeOperatorsCredentials, contracts);

  // *** 3. Fund staker wallets with LIT
  await fundWalletsWithTokens(nodeOperatorsCredentials, contracts);

  // *** 4. Print balances of node and staker wallets
  await logGasAndTokenBalances(nodeOperatorsCredentials, contracts);

  // *** 5. Stake tokens for nodes
  await stakeTokensAndJoinValidators(
    nodeOperatorsCredentials,
    deployNodeConfig.numberOfStakedAndJoinedWallets,
    deployNodeConfig.numberOfStakedOnlyWallets,
    contracts
  );

  // *** 6. Set epoch length to 5 minutes
  const signer = await getSigner();
  await setEpochLength(contracts, signer);

  // *** 7. Set IP addresses if provided
  if (deployNodeConfig.ipAddresses) {
    console.log(
      'Setting IP addresses for nodes that are joining: ',
      deployNodeConfig.ipAddresses
    );
    await setIpAddresses(
      deployNodeConfig.ipAddresses,
      nodeOperatorsCredentials.slice(
        0,
        deployNodeConfig.numberOfStakedAndJoinedWallets
      ),
      contracts
    );
  }

  // *** 8. if deploying in Active or Unlocked state, lock the validator set
  const stakingViewsFacet = await ethers.getContractAt(
    'StakingViewsFacet',
    contracts.stakingContractAddress,
    signer
  );
  const currentState = await stakingViewsFacet.state(realmId);
  console.log(`Current contract state: ${currentState}`);

  // State enum values: Active=0, NextValidatorSetLocked=1, ReadyForNextEpoch=2, Unlocked=3
  if (currentState === 0n || currentState === 3n) {
    // is Active or Unlocked
    console.log(
      'Contract state is Active or Unlocked. Locking the validator set...'
    );
    await lockValidatorSet(contracts);
  } else {
    console.log(
      'Skipping locking the validator set - contract state is not Active or Unlocked'
    );
  }

  // Finally, transfer ownership of the contracts.
  await transferContractsOwnership(contracts, deployNodeConfig);

  return {
    nodeOperatorsCredentials,
    contracts,
  };
}

async function logGasAndTokenBalances(
  nodeOperatorsCredentials: Array<NodeOperatorCredentials>,
  contracts: ParsedNodeContracts
) {
  const signer = await getSigner();

  const litTokenContract = await ethers.getContractAt(
    'LITToken',
    contracts.litTokenContractAddress,
    signer
  );

  // Log the gas and token balance of the signer
  const signerEthBalance = await ethers.provider.getBalance(signer.address);
  const signerTokenBalance = await litTokenContract.balanceOf(signer.address);

  console.log(
    `Signer ${
      signer.address
    } - gas balance: ${signerEthBalance.toString()}, token balance: ${signerTokenBalance.toString()}`
  );

  for (let i = 0; i < nodeOperatorsCredentials.length; i++) {
    const nodeOperatorCredential = nodeOperatorsCredentials[i];

    // Get gas and token balance of node wallet
    const nodeWalletEthBalance = await ethers.provider.getBalance(
      nodeOperatorCredential.nodeWallet.address
    );
    const nodeWalletTokenBalance = await litTokenContract.balanceOf(
      nodeOperatorCredential.nodeWallet.address
    );

    // Get gas and token balance of staker wallet
    const stakerWalletEthBalance = await ethers.provider.getBalance(
      nodeOperatorCredential.stakerWallet.address
    );
    const stakerWalletTokenBalance = await litTokenContract.balanceOf(
      nodeOperatorCredential.stakerWallet.address
    );

    // Log
    console.log(
      `Node wallet (${i}) ${
        nodeOperatorCredential.nodeWallet.address
      } - gas balance: ${nodeWalletEthBalance.toString()}, token balance: ${nodeWalletTokenBalance.toString()}`
    );
    console.log(
      `Staker wallet (${i}) ${
        nodeOperatorCredential.stakerWallet.address
      } - gas balance: ${stakerWalletEthBalance.toString()}, token balance: ${stakerWalletTokenBalance.toString()}`
    );
  }
}

async function transferContractsOwnership(
  contracts: ParsedNodeContracts,
  deployNodeConfig: DeployNodeConfig
) {
  const signer = await getSigner();
  const newOwnerAddress = deployNodeConfig.newOwnerAddress;
  if (signer.address === newOwnerAddress) {
    console.info('Signer is already the new owner. Skipping...');
    return;
  }

  const allowlistContract = await ethers.getContractAt(
    'Allowlist',
    contracts.allowlistContractAddress,
    signer
  );
  console.info('Transferring ownership for Allowlist contract...');
  await transferOwnershipToNewOwner(allowlistContract, newOwnerAddress);

  const stakingContract: OwnershipFacet = await getContractInstance(
    ethers,
    'Staking',
    contracts.stakingContractAddress,
    signer,
    'OwnershipFacet'
  );
  console.info('Transferring ownership for Staking contract...');
  await transferOwnershipToNewOwner(stakingContract, newOwnerAddress);

  const remainingContracts = [
    { name: 'PubkeyRouter', isDiamond: true },
    { name: 'Multisender', isDiamond: false },
    { name: 'PKPPermissions', isDiamond: true },
    { name: 'PKPHelper', isDiamond: false },
    { name: 'PKPNFT', isDiamond: true },
    { name: 'DomainWalletRegistry', isDiamond: true },
  ];

  for (const remainingContract of remainingContracts) {
    await transferOwnershipIfNotAlreadyExist(
      contracts,
      deployNodeConfig,
      remainingContract,
      signer
    );
  }
}

async function transferOwnershipIfNotAlreadyExist(
  contracts: ParsedNodeContracts,
  deployNodeConfig: DeployNodeConfig,
  contract: { name: string; isDiamond: boolean },
  signer: any
) {
  if (
    !contractAddressAlreadyExists(
      deployNodeConfig.existingContracts as ParsedNodeContracts,
      contract.name
    )
  ) {
    const contractInstance: OwnershipFacet | Ownable =
      await getContractInstance(
        ethers,
        contract.name,
        // @ts-ignore
        contracts[
          // @ts-ignore
          CONTRACT_NAME_TO_JSON_CONTRACT_ADDRESS_KEY[contract.name]
        ],
        signer,
        contract.isDiamond ? 'OwnershipFacet' : undefined
      );
    await transferOwnershipToNewOwner(
      contractInstance,
      deployNodeConfig.newOwnerAddress
    );
  }
}

async function transferOwnershipToNewOwner(
  contract: Ownable | OwnershipFacet,
  newOwnerAddress: string
) {
  console.log(`Setting new owner to ${newOwnerAddress}`);
  const tx = await contract.transferOwnership(newOwnerAddress);
  const txReceipt = await tx.wait();
  console.log('New owner set.');

  return txReceipt;
}
