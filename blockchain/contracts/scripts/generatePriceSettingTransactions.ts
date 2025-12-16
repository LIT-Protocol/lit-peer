// Script to generate SAFE multisig transaction JSON payloads for setting prices
// Usage: HARDHAT_NETWORK=litMainnet npx ts-node --files scripts/generatePriceSettingTransactions.ts

import hre from 'hardhat';
import { ContractResolver } from '../typechain-types';

const { ethers } = hre;

// ============================================================================
// PRICE MAP - Set your desired USD prices here
// ============================================================================

interface PriceMap {
  // PKP Minting Price
  pkpMintPriceUSD: number;

  // Base Network Prices (in USD)
  basePrices: {
    pkpSign: number;
    encSign: number;
    litAction: number;
    signSessionKey: number;
  };

  // Lit Action Price Components (in USD)
  litActionComponents: {
    baseAmount: number; // perCount
    runtimeLength: number; // perSecond
    memoryUsage: number; // perMegabyte
    codeLength: number; // perMegabyte
    responseLength: number; // perMegabyte
    signatures: number; // perCount
    broadcasts: number; // perCount
    contractCalls: number; // perCount
    callDepth: number; // perCount
    decrypts: number; // perCount
    fetches: number; // perCount
  };
}

// Update these prices to your desired values.  All prices are in USD.
const PRICE_MAP: PriceMap = {
  pkpMintPriceUSD: 0.25, // per PKP mint

  basePrices: {
    pkpSign: 0.05, // per PKP sign
    encSign: 0.01, // per encrypted sign
    litAction: 0.05, // base for lit action
    signSessionKey: 0.25, // per session key sign
  },

  litActionComponents: {
    baseAmount: 0.05, // per lit action
    runtimeLength: 0.001, // per second
    memoryUsage: 0.0001, // per MB
    codeLength: 0.0001, // per MB
    responseLength: 0.0001, // per MB
    signatures: 0.05, // per signature
    broadcasts: 0.001, // per broadcast
    contractCalls: 0.005, // per contract call
    callDepth: 0.001, // per call depth
    decrypts: 0.01, // per decrypt
    fetches: 0.001, // per fetch
  },
};

// ============================================================================
// ENUMS AND CONSTANTS
// ============================================================================

enum ProductId {
  PkpSign = 0,
  EncSign = 1,
  LitAction = 2,
  SignSessionKey = 3,
}

enum LitActionPriceComponent {
  baseAmount = 0,
  runtimeLength = 1,
  memoryUsage = 2,
  codeLength = 3,
  responseLength = 4,
  signatures = 5,
  broadcasts = 6,
  contractCalls = 7,
  callDepth = 8,
  decrypts = 9,
  fetches = 10,
}

enum NodePriceMeasurement {
  perSecond = 0,
  perMegabyte = 1,
  perCount = 2,
}

const PRODUCT_NAMES = {
  [ProductId.PkpSign]: 'PKP Sign',
  [ProductId.EncSign]: 'Encrypted Sign',
  [ProductId.LitAction]: 'Lit Action',
  [ProductId.SignSessionKey]: 'Sign Session Key',
};

const LIT_ACTION_COMPONENT_NAMES = {
  [LitActionPriceComponent.baseAmount]: 'Base Amount',
  [LitActionPriceComponent.runtimeLength]: 'Runtime Length',
  [LitActionPriceComponent.memoryUsage]: 'Memory Usage',
  [LitActionPriceComponent.codeLength]: 'Code Length',
  [LitActionPriceComponent.responseLength]: 'Response Length',
  [LitActionPriceComponent.signatures]: 'Signatures',
  [LitActionPriceComponent.broadcasts]: 'Broadcasts',
  [LitActionPriceComponent.contractCalls]: 'Contract Calls',
  [LitActionPriceComponent.callDepth]: 'Call Depth',
  [LitActionPriceComponent.decrypts]: 'Decrypts',
  [LitActionPriceComponent.fetches]: 'Fetches',
};

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/**
 * Get LITKEY token price in USD from CoinGecko
 */
async function getLitKeyPrice(): Promise<number> {
  try {
    const response = await fetch(
      'https://api.coingecko.com/api/v3/simple/price?ids=lit-protocol&vs_currencies=usd'
    );
    const data = await response.json();

    if (data['lit-protocol'] && data['lit-protocol'].usd) {
      return data['lit-protocol'].usd;
    }

    throw new Error('LIT price not found in CoinGecko response');
  } catch (error) {
    console.error('Error fetching LITKEY price from CoinGecko:', error);
    throw new Error(
      'Unable to fetch LITKEY price. Please check CoinGecko API or set manually.'
    );
  }
}

/**
 * Convert USD price to LITKEY wei amount
 */
function usdToLitKeyWei(usdPrice: number, litKeyPriceUSD: number): bigint {
  // Convert USD to LITKEY tokens (18 decimals)
  const tokens = usdPrice / litKeyPriceUSD;
  return ethers.parseUnits(tokens.toFixed(18), 18);
}

/**
 * Get contract addresses from ContractResolver
 */
async function getContractAddresses(): Promise<{
  pkpNftAddress: string;
  priceFeedAddress: string;
}> {
  // Try to get from environment or use defaults
  const contractResolverAddress =
    process.env.CONTRACT_RESOLVER_ADDRESS ||
    '0x0000000000000000000000000000000000000000'; // Update with actual address

  if (
    contractResolverAddress === '0x0000000000000000000000000000000000000000'
  ) {
    // Fallback to hardcoded addresses if resolver not available
    console.warn(
      'ContractResolver address not set, using hardcoded addresses. Set CONTRACT_RESOLVER_ADDRESS env var for dynamic lookup.'
    );
    return {
      pkpNftAddress:
        process.env.PKPNFT_ADDRESS ||
        '0x0000000000000000000000000000000000000000',
      priceFeedAddress:
        process.env.PRICE_FEED_ADDRESS ||
        '0x88F5535Fa6dA5C225a3C06489fE4e3405b87608C', // From calculateUSDPricing.ts
    };
  }

  try {
    const contractResolver: ContractResolver = await ethers.getContractAt(
      'ContractResolver',
      contractResolverAddress
    );

    const network = await ethers.provider.getNetwork();
    const env =
      network.chainId === 175200n
        ? 0 // Mainnet
        : network.chainId === 987n
        ? 1 // Testnet
        : 0; // Default to mainnet

    const pkpNftAddress = await contractResolver.PKP_NFT_CONTRACT();
    const priceFeedAddress = await contractResolver.PRICE_FEED_CONTRACT();

    return {
      pkpNftAddress,
      priceFeedAddress,
    };
  } catch (error) {
    console.error('Error getting contract addresses from resolver:', error);
    throw error;
  }
}

/**
 * Create a SAFE transaction object
 */
function createSafeTransaction(
  to: string,
  data: string,
  value: string = '0'
): {
  to: string;
  value: string;
  data: string;
  operation: number; // 0 = call, 1 = delegatecall
} {
  return {
    to,
    value,
    data,
    operation: 0, // Standard call
  };
}

// ============================================================================
// MAIN FUNCTION
// ============================================================================

async function main() {
  console.log(
    '=== Generating SAFE Transaction Payloads for Price Setting ===\n'
  );

  // Get network info
  const network = await ethers.provider.getNetwork();
  console.log(`Network: ${network.name} (Chain ID: ${network.chainId})\n`);

  // Get LITKEY price in USD
  console.log('Fetching LITKEY token price from CoinGecko...');
  const litKeyPriceUSD = await getLitKeyPrice();
  console.log(`LITKEY Price: $${litKeyPriceUSD.toFixed(4)} USD\n`);

  // Get contract addresses
  const { pkpNftAddress, priceFeedAddress } = await getContractAddresses();
  console.log(`PKPNFT Contract: ${pkpNftAddress}`);
  console.log(`PriceFeed Contract: ${priceFeedAddress}\n`);

  // Get contract instances
  const pkpNft = await ethers.getContractAt('PKPNFTDiamond', pkpNftAddress);
  const priceFeed = await ethers.getContractAt(
    'PriceFeedDiamond',
    priceFeedAddress
  );

  const transactions: Array<{
    to: string;
    value: string;
    data: string;
    operation: number;
    description: string;
    priceUSD: number;
    priceLITKEY: string;
  }> = [];

  // ============================================================================
  // 1. PKP Mint Cost
  // ============================================================================
  console.log('=== PKP Mint Cost ===');
  const pkpMintPriceWei = usdToLitKeyWei(
    PRICE_MAP.pkpMintPriceUSD,
    litKeyPriceUSD
  );
  const pkpMintPriceTokens = parseFloat(
    ethers.formatUnits(pkpMintPriceWei, 18)
  );
  console.log(
    `Setting PKP mint cost to: ${pkpMintPriceTokens.toFixed(
      6
    )} LITKEY ($${PRICE_MAP.pkpMintPriceUSD.toFixed(4)} USD)`
  );

  const setMintCostData = pkpNft.interface.encodeFunctionData('setMintCost', [
    pkpMintPriceWei,
  ]);

  transactions.push({
    ...createSafeTransaction(pkpNftAddress, setMintCostData),
    description: 'Set PKP Mint Cost',
    priceUSD: PRICE_MAP.pkpMintPriceUSD,
    priceLITKEY: pkpMintPriceTokens.toFixed(6),
  });

  // ============================================================================
  // 2. Base Network Prices
  // ============================================================================
  console.log('\n=== Base Network Prices ===');
  const productIds = [
    ProductId.PkpSign,
    ProductId.EncSign,
    ProductId.LitAction,
    ProductId.SignSessionKey,
  ];
  const basePrices = [
    PRICE_MAP.basePrices.pkpSign,
    PRICE_MAP.basePrices.encSign,
    PRICE_MAP.basePrices.litAction,
    PRICE_MAP.basePrices.signSessionKey,
  ];

  const basePriceWeis = basePrices.map((usdPrice) =>
    usdToLitKeyWei(usdPrice, litKeyPriceUSD)
  );

  for (let i = 0; i < productIds.length; i++) {
    const productId = productIds[i];
    const usdPrice = basePrices[i];
    const priceWei = basePriceWeis[i];
    const priceTokens = parseFloat(ethers.formatUnits(priceWei, 18));
    console.log(
      `${PRODUCT_NAMES[productId]}: ${priceTokens.toFixed(
        6
      )} LITKEY ($${usdPrice.toFixed(4)} USD)`
    );

    const setBasePriceData = priceFeed.interface.encodeFunctionData(
      'setBaseNetworkPrices',
      [priceWei, [productId]] as [bigint, bigint[]]
    );

    transactions.push({
      ...createSafeTransaction(priceFeedAddress, setBasePriceData),
      description: `Set Base Network Price - ${PRODUCT_NAMES[productId]}`,
      priceUSD: usdPrice,
      priceLITKEY: priceTokens.toFixed(6),
    });
  }

  // ============================================================================
  // 3. Lit Action Price Components
  // ============================================================================
  console.log('\n=== Lit Action Price Components ===');

  const litActionComponentMap: Array<{
    component: LitActionPriceComponent;
    measurement: NodePriceMeasurement;
    usdPrice: number;
    name: string;
  }> = [
    {
      component: LitActionPriceComponent.baseAmount,
      measurement: NodePriceMeasurement.perCount,
      usdPrice: PRICE_MAP.litActionComponents.baseAmount,
      name: 'Base Amount',
    },
    {
      component: LitActionPriceComponent.runtimeLength,
      measurement: NodePriceMeasurement.perSecond,
      usdPrice: PRICE_MAP.litActionComponents.runtimeLength,
      name: 'Runtime Length',
    },
    {
      component: LitActionPriceComponent.memoryUsage,
      measurement: NodePriceMeasurement.perMegabyte,
      usdPrice: PRICE_MAP.litActionComponents.memoryUsage,
      name: 'Memory Usage',
    },
    {
      component: LitActionPriceComponent.codeLength,
      measurement: NodePriceMeasurement.perMegabyte,
      usdPrice: PRICE_MAP.litActionComponents.codeLength,
      name: 'Code Length',
    },
    {
      component: LitActionPriceComponent.responseLength,
      measurement: NodePriceMeasurement.perMegabyte,
      usdPrice: PRICE_MAP.litActionComponents.responseLength,
      name: 'Response Length',
    },
    {
      component: LitActionPriceComponent.signatures,
      measurement: NodePriceMeasurement.perCount,
      usdPrice: PRICE_MAP.litActionComponents.signatures,
      name: 'Signatures',
    },
    {
      component: LitActionPriceComponent.broadcasts,
      measurement: NodePriceMeasurement.perCount,
      usdPrice: PRICE_MAP.litActionComponents.broadcasts,
      name: 'Broadcasts',
    },
    {
      component: LitActionPriceComponent.contractCalls,
      measurement: NodePriceMeasurement.perCount,
      usdPrice: PRICE_MAP.litActionComponents.contractCalls,
      name: 'Contract Calls',
    },
    {
      component: LitActionPriceComponent.callDepth,
      measurement: NodePriceMeasurement.perCount,
      usdPrice: PRICE_MAP.litActionComponents.callDepth,
      name: 'Call Depth',
    },
    {
      component: LitActionPriceComponent.decrypts,
      measurement: NodePriceMeasurement.perCount,
      usdPrice: PRICE_MAP.litActionComponents.decrypts,
      name: 'Decrypts',
    },
    {
      component: LitActionPriceComponent.fetches,
      measurement: NodePriceMeasurement.perCount,
      usdPrice: PRICE_MAP.litActionComponents.fetches,
      name: 'Fetches',
    },
  ];

  for (const config of litActionComponentMap) {
    const priceWei = usdToLitKeyWei(config.usdPrice, litKeyPriceUSD);
    const priceTokens = parseFloat(ethers.formatUnits(priceWei, 18));
    const measurementName =
      config.measurement === NodePriceMeasurement.perSecond
        ? '/second'
        : config.measurement === NodePriceMeasurement.perMegabyte
        ? '/MB'
        : '/count';
    console.log(
      `${config.name} ${measurementName}: ${priceTokens.toFixed(
        6
      )} LITKEY ($${config.usdPrice.toFixed(4)} USD)`
    );

    const setLitActionPriceData = priceFeed.interface.encodeFunctionData(
      'setLitActionPriceConfig',
      [config.component, config.measurement, priceWei] as [
        number,
        number,
        bigint
      ]
    );

    transactions.push({
      ...createSafeTransaction(priceFeedAddress, setLitActionPriceData),
      description: `Set Lit Action Price - ${config.name} ${measurementName}`,
      priceUSD: config.usdPrice,
      priceLITKEY: priceTokens.toFixed(6),
    });
  }

  // ============================================================================
  // OUTPUT SAFE TRANSACTION PAYLOAD
  // ============================================================================
  console.log('\n=== SAFE Transaction Payload ===\n');

  // Create the SAFE transaction payload
  // This format is compatible with SAFE's Transaction Builder
  const safePayload = {
    version: '1.0',
    chainId: network.chainId.toString(),
    createdAt: new Date().toISOString(),
    meta: {
      name: 'Price Setting Transactions',
      description:
        'Transactions to set prices for PKP minting and PriceFeed products',
      txBuilderVersion: '1.0.0',
    },
    transactions: transactions.map((tx) => ({
      to: tx.to,
      value: tx.value,
      data: tx.data,
      operation: tx.operation,
    })),
  };

  // Also create a simple array format (alternative format for SAFE)
  const simpleSafePayload = transactions.map((tx) => ({
    to: tx.to,
    value: tx.value,
    data: tx.data,
    operation: tx.operation,
  }));

  // Also create a detailed version with descriptions
  const detailedPayload = {
    ...safePayload,
    transactions: transactions,
  };

  // Output JSON
  console.log('=== SAFE Transaction JSON (Transaction Builder format) ===');
  console.log(JSON.stringify(safePayload, null, 2));

  console.log('\n=== Simple SAFE Transaction Array (alternative format) ===');
  console.log(JSON.stringify(simpleSafePayload, null, 2));

  console.log('\n=== Detailed Transaction List (for reference) ===');
  console.log(JSON.stringify(detailedPayload, null, 2));

  // Summary
  console.log('\n=== Summary ===');
  console.log(`Total transactions: ${transactions.length}`);
  console.log(`LITKEY Price: $${litKeyPriceUSD.toFixed(4)} USD`);
  console.log(`PKPNFT Address: ${pkpNftAddress}`);
  console.log(`PriceFeed Address: ${priceFeedAddress}`);
  console.log('\n=== Usage Instructions ===');
  console.log(
    '1. Copy the SAFE Transaction JSON above (Transaction Builder format)'
  );
  console.log('2. Go to your SAFE wallet and use the Transaction Builder');
  console.log('3. Import the JSON or manually add each transaction');
  console.log('4. Review and sign the transactions with your multisig');
  console.log(
    "\nAlternatively, you can use the simple array format with SAFE's API directly."
  );
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
