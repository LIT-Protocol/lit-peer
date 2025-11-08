import ethers, {
  ContractTransaction,
  Signer,
  keccak256,
  toBigInt,
  toUtf8Bytes,
} from 'ethers';
import {
  BackupRecovery,
  ContractResolver,
  DomainWalletRegistryFacet,
  KeyDeriver,
  PKPNFTFacet,
  PKPNFTMetadata,
  PKPHelper,
  PKPPermissionsFacet,
  PubkeyRouterFacet,
  StakingValidatorFacet,
  StakingKeySetsFacet,
  StakingAdminFacet,
  StakingFacet,
  Forwarder,
} from '../typechain-types';
import { LITToken } from '../typechain-types/contracts/lit-node/LITToken';
import { ip2int } from './index.js';

export enum Environment {
  DEV,
  STAGING,
  PROD,
}

export enum StakingState {
  Active,
  NextValidatorSetLocked,
  ReadyForNextEpoch,
  Unlocked,
  Paused,
}

export async function setContractResolver(
  contractResolver: ContractResolver,
  env: Environment,
  {
    backupRecoveryContract,
    tokenContract,
    stakingContract,
    pkpContract,
    pkpPermissionsContract,
    pkpHelperContract,
    pkpNftMetadataContract,
    domainWalletRegistryContract,
    hdKeyDeriverContract,
    pubkeyRouterContract,
    stylusContractP256,
    stylusContractK256,
  }: {
    backupRecoveryContract?: BackupRecovery;
    tokenContract?: LITToken;
    stakingContract?: StakingFacet;
    pkpContract?: PKPNFTFacet;
    pkpPermissionsContract?: PKPPermissionsFacet;
    pkpHelperContract?: PKPHelper;
    pkpNftMetadataContract?: PKPNFTMetadata;
    domainWalletRegistryContract?: DomainWalletRegistryFacet;
    hdKeyDeriverContract?: KeyDeriver;
    pubkeyRouterContract?: PubkeyRouterFacet;
    stylusContractP256?: string;
    stylusContractK256?: string;
  }
) {
  if (tokenContract) {
    await contractResolver.setContract(
      await contractResolver.LIT_TOKEN_CONTRACT(),
      env,
      await tokenContract.getAddress()
    );
  }

  if (stakingContract) {
    await contractResolver.setContract(
      await contractResolver.STAKING_CONTRACT(),
      env,
      await stakingContract.getAddress()
    );
  }

  if (pkpContract) {
    await contractResolver.setContract(
      await contractResolver.PKP_NFT_CONTRACT(),
      env,
      await pkpContract.getAddress()
    );
  }

  if (pkpPermissionsContract) {
    await contractResolver.setContract(
      await contractResolver.PKP_PERMISSIONS_CONTRACT(),
      env,
      await pkpPermissionsContract.getAddress()
    );
  }

  if (pkpHelperContract) {
    await contractResolver.setContract(
      await contractResolver.PKP_HELPER_CONTRACT(),
      env,
      await pkpHelperContract.getAddress()
    );
  }

  if (pkpNftMetadataContract) {
    await contractResolver.setContract(
      await contractResolver.PKP_NFT_METADATA_CONTRACT(),
      env,
      await pkpNftMetadataContract.getAddress()
    );
  }

  if (domainWalletRegistryContract) {
    await contractResolver.setContract(
      await contractResolver.DOMAIN_WALLET_REGISTRY(),
      env,
      await domainWalletRegistryContract.getAddress()
    );
  }

  if (hdKeyDeriverContract) {
    await contractResolver.setContract(
      await contractResolver.HD_KEY_DERIVER_CONTRACT(),
      env,
      await hdKeyDeriverContract.getAddress()
    );
  }

  if (pubkeyRouterContract) {
    await contractResolver.setContract(
      await contractResolver.PUB_KEY_ROUTER_CONTRACT(),
      env,
      await pubkeyRouterContract.getAddress()
    );
  }

  if (backupRecoveryContract) {
    await contractResolver.setContract(
      await contractResolver.BACKUP_RECOVERY_CONTRACT(),
      env,
      await backupRecoveryContract.getAddress()
    );
  }

  if (stylusContractP256) {
    await contractResolver.setContract(
      keccak256(toUtf8Bytes('HD_KEY_DERIVER_CONTRACT_P256')),
      env,
      stylusContractP256
    );
  }

  if (stylusContractK256) {
    await contractResolver.setContract(
      keccak256(toUtf8Bytes('HD_KEY_DERIVER_CONTRACT_K256')),
      env,
      stylusContractK256
    );
  }
}

export interface SetupStakingOptions {
  numValidators: number;
  ipAddress: string;
  startingPort: number;
}

export interface CreateValidatorOptions {
  ipAddress: string;
  port: number;
  initialTokens: bigint;
}

export interface StakingAccount {
  nodeAddress: ethers.HDNodeWallet;
  stakingAddress: ethers.HDNodeWallet;
  commsKeys: {
    sender: bigint;
    receiver: bigint;
  };
  ip?: string;
  port?: number;
}

type RootKey = [Uint8Array, number];

export function getStaticRootKeys(ethers: any): RootKey[] {
  return [
    [
      ethers.getBytes(
        '0x028506cbedca1d12788d6bc74627d99263c93204d2e9565d861b7c1270736b0071'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x02a89cb5090c0aaee9c5831df939abbeab2e0f62b5d54ceae6e816a9fe87c8ca32'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x033e0c9d93b41414c3a8d287bb40ab024fbf176cb45c6616a3bf74e97bb68b5165'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x03a0c18f5d9db21fec597edef52f7a26449cdd90357532704a1ede6c27981a31b8'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x02794db35a0b6a6968ba4ed059630d788d591f083778dac9a45935549ca5f75ea6'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x03b398a663086dc7f1b5948d2195b176a7705fe71b0ad07110f57975254e601598'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x0215f2cddeb89428f74132a84acf7e1a344f2ed9a39768f7006c9b8843e513dc55'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x0297d2a91f5a52e98873b7a4946c47d7736d6661cebace9c160d955999be971492'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x03d2ee101c65ca0d60b5bc27ca1859c968984b1096d742874649bdc4fac6e9498a'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x02bb0deb45aefb171e7117390991c2a230218fda04d9bb3cfd343f56ab61c3e390'
      ),
      2,
    ],
  ];
}

export async function allNodesVoteForRootKeys(
  ethers: any,
  pubkeyRouterContract: PubkeyRouterFacet,
  stakingContract: StakingAdminFacet,
  stakingAccounts: StakingAccount[],
  deployer: Signer
): Promise<RootKey[]> {
  await stakingContract
    .connect(deployer)
    .setEpochState(1, StakingState.NextValidatorSetLocked);

  const rootKeys = getStaticRootKeys(ethers);

  for (let i = 0; i < stakingAccounts.length; i++) {
    // vote with the nodes
    await pubkeyRouterContract
      .connect(stakingAccounts[i].nodeAddress)
      .voteForRootKeys(
        // @ts-ignore
        await stakingContract.getAddress(),
        'naga-keyset1',
        rootKeys
      );
  }

  return rootKeys;
}

export async function createValidator(
  ethers: any,
  stakingFacet: StakingFacet,
  tokenContract: LITToken,
  deployer: Signer,
  options: CreateValidatorOptions
): Promise<StakingAccount> {
  let provider = deployer.provider!;
  console.log(
    `deployer has ${await provider.getBalance(
      deployer
    )} eth.  Funding validator...`
  );

  const ethForGas = ethers.parseEther('1.0');

  const stakingAccount: StakingAccount = {
    stakingAddress: ethers.Wallet.createRandom().connect(provider),
    nodeAddress: ethers.Wallet.createRandom().connect(provider),
    commsKeys: {
      sender: toBigInt(ethers.randomBytes(32)),
      receiver: toBigInt(ethers.randomBytes(32)),
    },
    ip: options.ipAddress,
    port: options.port,
  };

  const stakingAddress = stakingAccount.stakingAddress.address;
  const nodeAddress = stakingAccount.nodeAddress.address;

  // send them some gas
  await deployer.sendTransaction({
    to: stakingAddress,
    value: ethForGas,
  });
  await deployer.sendTransaction({
    to: nodeAddress,
    value: ethForGas,
  });
  // send them some tokens
  tokenContract = tokenContract.connect(deployer);
  await tokenContract.transfer(stakingAddress, options.initialTokens);
  tokenContract = tokenContract.connect(stakingAccount.stakingAddress);
  await tokenContract.approve(
    await stakingFacet.getAddress(),
    options.initialTokens
  );

  return stakingAccount;
}

export async function createValidatorAndStake(
  ethers: any,
  stakingFacet: StakingFacet,
  tokenContract: LITToken,
  deployer: Signer,
  options: CreateValidatorOptions
): Promise<StakingAccount> {
  console.log('Creating validator and staking #1');
  const minStake = await stakingFacet.getMinimumSelfStake();

  const stakingAccount = await createValidator(
    ethers,
    stakingFacet,
    tokenContract,
    deployer,
    options
  );

  stakingFacet = stakingFacet.connect(stakingAccount.stakingAddress);

  const timeLock = 86400n * 120n; // 120 days
  const stakerAddress = stakingAccount.stakingAddress.address;
  await stakingFacet.stake(minStake, timeLock, stakerAddress);

  return stakingAccount;
}

export async function sendMetaTransaction(
  ethers: any,
  txnData: ContractTransaction,
  metaTransactionSigner: Signer,
  forwarderContractWithFundedWallet: Forwarder,
  recipientContractAddress: string,
  options?: {
    checkMetaTransactionSignerBalance?: boolean;
  }
): Promise<string> {
  // If options are not provided, we default to checking the balance
  if (!options) {
    options = {
      checkMetaTransactionSignerBalance: true,
    };
  }

  // Get the balance of the metaTransactionSigner before the meta-txn
  const metaTransactionSignerBalanceBefore = await ethers.provider.getBalance(
    await metaTransactionSigner.getAddress()
  );

  // Get the nonce from the forwarder
  const nonce = await forwarderContractWithFundedWallet.getNonce(
    await metaTransactionSigner.getAddress()
  );

  const gasLimit = await ethers.provider.estimateGas({
    ...txnData,
    from: await metaTransactionSigner.getAddress(),
  });

  // Construct the EIP-2771 request
  const metaTxn = {
    from: await metaTransactionSigner.getAddress(),
    to: recipientContractAddress,
    value: txnData.value ?? 0n,
    gas: gasLimit.toString(),
    nonce: nonce.toString(),
    data: txnData.data,
  };

  // Create domain for EIP-712 signing, which needs to match the forwarder contract
  const domain = {
    name: 'GSNv2 Forwarder',
    version: '0.0.1',
    chainId: await ethers.provider
      .getNetwork()
      .then((network: any) => network.chainId),
    verifyingContract: await forwarderContractWithFundedWallet.getAddress(),
  };

  const types = {
    ForwardRequest: [
      { name: 'from', type: 'address' },
      { name: 'to', type: 'address' },
      { name: 'value', type: 'uint256' },
      { name: 'gas', type: 'uint256' },
      { name: 'nonce', type: 'uint256' },
      { name: 'data', type: 'bytes' },
    ],
  };

  // Sign the meta-txn with typed data using the ephemeral wallet
  const signature = await metaTransactionSigner.signTypedData(
    domain,
    types,
    metaTxn
  );

  // Execute the txn
  const tx = await forwarderContractWithFundedWallet.execute(
    metaTxn,
    signature,
    {
      value: txnData.value,
    }
  );

  // Now that the meta-txn has been executed, we need to assert that the metaTransactionSigner
  // has not had their balance changed to help prove the function of meta-txn (it should not use
  // any of the funds in the metaTransactionSigner's balance).
  const metaTransactionSignerBalanceAfter = await ethers.provider.getBalance(
    await metaTransactionSigner.getAddress()
  );
  if (
    options.checkMetaTransactionSignerBalance &&
    metaTransactionSignerBalanceAfter !== metaTransactionSignerBalanceBefore
  ) {
    throw new Error('Meta-txn signer balance changed');
  }

  return tx.hash;
}

export async function setupStakingWithValidatorsAndAdvance(
  ethers: any,
  stakingFacet: StakingFacet,
  stakingValidatorFacet: StakingValidatorFacet,
  stakingAdminFacet: StakingAdminFacet,
  tokenContract: LITToken,
  deployer: Signer,
  options: SetupStakingOptions
): Promise<StakingAccount[]> {
  // Validate number of validators is greater than 1
  if (options.numValidators < 2) {
    throw new Error('Must have at least 2 validator');
  }

  // set epoch length to 1 so that we can test quickly
  const realmId = 1;
  await stakingAdminFacet.setEpochLength(realmId, 1);

  console.log(
    'Setup Staking With validators for realmId ',
    realmId,
    ' with ',
    options.numValidators,
    ' validators'
  );

  const minStake = await stakingFacet.getMinimumSelfStake();
  const totalToStake = minStake * 3n; // 3 times the minimum stake

  let stakingAccounts: StakingAccount[] = [];
  for (let i = 0; i < options.numValidators; i++) {
    const stakingAccount = await createValidatorAndStake(
      ethers,
      stakingFacet,
      tokenContract,
      deployer,
      {
        ipAddress: options.ipAddress,
        port: options.startingPort + i + 1,
        initialTokens: totalToStake,
      }
    );

    console.log(
      'Request from ',
      stakingAccount.stakingAddress.address,
      ' (node: ',
      stakingAccount.nodeAddress.address,
      ') to join realmId: ',
      realmId
    );

    // Set the IP and port of the validator
    await stakingValidatorFacet
      .connect(stakingAccount.stakingAddress)
      .setIpPortNodeAddress(
        ip2int(stakingAccount.ip!),
        0,
        stakingAccount.port!,
        stakingAccount.nodeAddress.address
      );

    // Call requestToJoin for each validator
    await stakingValidatorFacet
      .connect(stakingAccount.stakingAddress)
      .requestToJoin(realmId);
    stakingAccounts.push(stakingAccount);
  }

  // set next epoch end time to 10 seconds ago
  await stakingAdminFacet.setEpochEndTime(
    realmId,
    Math.floor(Date.now() / 1000) - 10
  );

  // unpause staking contract
  await stakingAdminFacet
    .connect(deployer)
    .setEpochState(realmId, StakingState.Active);

  // okay now that we're all staked, let's kickoff the first epoch
  await stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);

  const epochNumber = 1;
  for (let i = 0; i < options.numValidators; i++) {
    stakingValidatorFacet = stakingValidatorFacet.connect(
      stakingAccounts[i].nodeAddress
    );
    await stakingValidatorFacet.signalReadyForNextEpoch(realmId, epochNumber);
  }

  await stakingValidatorFacet.advanceEpoch(realmId);

  console.info(
    `Finished setting up staking with ${options.numValidators} validators and advanced epoch.`
  );

  return stakingAccounts;
}
