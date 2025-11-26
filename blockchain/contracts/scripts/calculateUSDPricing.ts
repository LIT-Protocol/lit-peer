// Script to calculate fees in USD based on LITKEY token price
// Usage: HARDHAT_NETWORK=litMainnet npx ts-node --files scripts/calculateUSDPricing.ts

import hre from 'hardhat';

const { ethers } = hre;

// on Lit Chain Mainnet
const NAGA_PROD_PRICE_FEED_ADDRESS =
  '0x88F5535Fa6dA5C225a3C06489fE4e3405b87608C';

// Product IDs from LibPriceFeedStorage.ProductId enum
enum ProductId {
  PkpSign = 0,
  EncSign = 1,
  LitAction = 2,
  SignSessionKey = 3,
}

// LitActionPriceComponent enum values
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

// NodePriceMeasurement enum values
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

const MEASUREMENT_NAMES = {
  [NodePriceMeasurement.perSecond]: '/second',
  [NodePriceMeasurement.perMegabyte]: '/MB',
  [NodePriceMeasurement.perCount]: '/count',
};

interface LitActionPriceConfig {
  priceComponent: bigint;
  priceMeasurement: bigint;
  price: bigint;
}

/**
 * Get LITKEY token price in USD from CoinGecko
 */
async function getLitKeyPrice(): Promise<number> {
  try {
    // Try to get LIT token price from CoinGecko
    // Note: You may need to adjust the token ID if LITKEY is listed differently
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
    console.log('Falling back to manual price input...');
    // You can set a default price here or throw
    throw new Error(
      'Unable to fetch LITKEY price. Please check CoinGecko API or set manually.'
    );
  }
}

/**
 * Get PriceFeed contract address from networkContext.json or use default
 */
function getPriceFeedAddress(): string {
  // Naga prod address
  return NAGA_PROD_PRICE_FEED_ADDRESS;
}

/**
 * Convert wei to LITKEY tokens (18 decimals)
 */
function weiToTokens(wei: bigint): number {
  return Number(wei) / 1e18;
}

/**
 * Format price for display
 */
function formatPrice(priceInTokens: number, priceInUSD: number): string {
  return `${priceInTokens.toFixed(6)} LITKEY ($${priceInUSD.toFixed(6)})`;
}

async function main() {
  console.log('=== Calculating Fees in USD ===\n');

  // Get network info
  const network = await ethers.provider.getNetwork();
  console.log(`Network: ${network.name} (Chain ID: ${network.chainId})\n`);

  // Get LITKEY price in USD
  console.log('Fetching LITKEY token price from CoinGecko...');
  const litKeyPriceUSD = await getLitKeyPrice();
  console.log(`LITKEY Price: $${litKeyPriceUSD.toFixed(4)} USD\n`);

  // Get PriceFeed contract
  const priceFeedAddress = getPriceFeedAddress();
  console.log(`PriceFeed Contract Address: ${priceFeedAddress}\n`);

  // Use PriceFeedDiamond which includes all facets via hardhat-diamond-abi plugin
  // Alternatively, you can use 'PriceFeedFacet' directly
  const priceFeed = await ethers.getContractAt(
    'PriceFeedDiamond',
    priceFeedAddress
  );

  // Get all product IDs
  const productIds = [
    ProductId.PkpSign,
    ProductId.EncSign,
    ProductId.LitAction,
    ProductId.SignSessionKey,
  ];

  console.log('=== Network Base Prices ===');
  const baseNetworkPrices = await priceFeed.baseNetworkPrices(productIds);
  for (let i = 0; i < productIds.length; i++) {
    const productId = productIds[i];
    const priceInWei = baseNetworkPrices[i];
    const priceInTokens = weiToTokens(priceInWei);
    const priceInUSD = priceInTokens * litKeyPriceUSD;
    console.log(
      `${PRODUCT_NAMES[productId]}: ${formatPrice(priceInTokens, priceInUSD)}`
    );
  }

  console.log('\n=== Network Max Prices ===');
  const maxNetworkPrices = await priceFeed.maxNetworkPrices(productIds);
  for (let i = 0; i < productIds.length; i++) {
    const productId = productIds[i];
    const priceInWei = maxNetworkPrices[i];
    const priceInTokens = weiToTokens(priceInWei);
    const priceInUSD = priceInTokens * litKeyPriceUSD;
    console.log(
      `${PRODUCT_NAMES[productId]}: ${formatPrice(priceInTokens, priceInUSD)}`
    );
  }

  // Get prices at different usage percentages
  console.log('\n=== Prices at Different Usage Percentages ===');
  const usagePercentages = [0, 25, 50, 75, 100];
  for (const usagePercent of usagePercentages) {
    console.log(`\nUsage: ${usagePercent}%`);
    const prices = await priceFeed.usagePercentToPrices(
      usagePercent,
      productIds
    );
    for (let i = 0; i < productIds.length; i++) {
      const productId = productIds[i];
      const priceInWei = prices[i];
      const priceInTokens = weiToTokens(priceInWei);
      const priceInUSD = priceInTokens * litKeyPriceUSD;
      console.log(
        `  ${PRODUCT_NAMES[productId]}: ${formatPrice(
          priceInTokens,
          priceInUSD
        )}`
      );
    }
  }

  // Get LitAction price configs
  console.log('\n=== LitAction Price Components ===');
  const litActionPriceConfigs: LitActionPriceConfig[] =
    await priceFeed.getLitActionPriceConfigs();

  for (const config of litActionPriceConfigs) {
    // Convert bigint to number for enum casting
    const priceComponentNum = Number(config.priceComponent);
    const priceMeasurementNum = Number(config.priceMeasurement);

    const componentName =
      LIT_ACTION_COMPONENT_NAMES[
        priceComponentNum as LitActionPriceComponent
      ] || `Component ${priceComponentNum}`;
    const measurementName =
      MEASUREMENT_NAMES[priceMeasurementNum as NodePriceMeasurement] || '';
    const priceInTokens = weiToTokens(config.price);
    const priceInUSD = priceInTokens * litKeyPriceUSD;
    console.log(
      `${componentName}${measurementName}: ${formatPrice(
        priceInTokens,
        priceInUSD
      )}`
    );
  }

  console.log('\n=== Summary ===');
  console.log(`LITKEY Token Price: $${litKeyPriceUSD.toFixed(4)} USD`);
  console.log(`PriceFeed Contract: ${priceFeedAddress}`);
  console.log(`Network: ${network.name} (Chain ID: ${network.chainId})`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
