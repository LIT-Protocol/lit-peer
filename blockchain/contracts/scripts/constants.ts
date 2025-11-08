export const CONTRACT_NAME_TO_JSON_CONTRACT_ADDRESS_KEY = {
  Staking: 'stakingContractAddress',
  Multisender: 'multisenderContractAddress',
  LITToken: 'litTokenContractAddress',
  AccessControlConditions: 'accessControlConditionsContractAddress',
  PubkeyRouter: 'pubkeyRouterContractAddress',
  PKPNFT: 'pkpNftContractAddress',
  Ledger: 'ledgerContractAddress',
  PriceFeed: 'priceFeedContractAddress',
  PKPHelper: 'pkpHelperContractAddress',
  PKPPermissions: 'pkpPermissionsContractAddress',
  PKPNFTMetadata: 'pkpNftMetadataContractAddress',
  Allowlist: 'allowlistContractAddress',
  ContractResolver: 'resolverContractAddress',
  DomainWalletRegistry: 'domainWalletRegistryAddress',
  KeyDeriver: 'KeyDeriverAddress',
  PaymentDelegation: 'paymentDelegationAddress',
};

export const CONTRACT_NAME_TO_DIAMOND_ABI_PATH = {
  Staking:
    '../artifacts/@lit-protocol/hardhat-diamond-abi/HardhatDiamondABI.sol/StakingDiamond.json',
  PKPPermissions:
    '../artifacts/@lit-protocol/hardhat-diamond-abi/HardhatDiamondABI.sol/PKPPermissionsDiamond.json',
  PubkeyRouter:
    '../artifacts/@lit-protocol/hardhat-diamond-abi/HardhatDiamondABI.sol/PubkeyRouterDiamond.json',
  Ledger:
    '../artifacts/@lit-protocol/hardhat-diamond-abi/HardhatDiamondABI.sol/LedgerDiamond.json',
  PriceFeed:
    '../artifacts/@lit-protocol/hardhat-diamond-abi/HardhatDiamondABI.sol/PriceFeedDiamond.json',
  StakingBalances:
    '../artifacts/@lit-protocol/hardhat-diamond-abi/HardhatDiamondABI.sol/StakingBalancesDiamond.json',
  PKPNFT:
    '../artifacts/@lit-protocol/hardhat-diamond-abi/HardhatDiamondABI.sol/PKPNFTDiamond.json',
  DomainWalletRegistry:
    '../artifacts/@lit-protocol/hardhat-diamond-abi/HardhatDiamondABI.sol/DomainWalletRegistryDiamond.json',
  PaymentDelegation:
    '../artifacts/@lit-protocol/hardhat-diamond-abi/HardhatDiamondABI.sol/PaymentDelegationDiamond.json',
};

export const DEFAULT_KEY_SET_NAME = 'naga-keyset1';
