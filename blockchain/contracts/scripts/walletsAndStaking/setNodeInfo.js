/* global ethers */
/* eslint prefer-const: "off" */

const hre = require('hardhat');
const { setInfo } = require('./utils/setInfo');
const yargs = require('yargs/yargs');
const { hideBin } = require('yargs/helpers');
const argv = yargs(hideBin(process.argv)).argv;
const fs = require('fs');
const TOML = require('@iarna/toml');

async function go() {
  const stakingContractAddress = '0x3DAA2A2A1a9ba31D7a4FA2B6e25921E6cef480EE';
  // load all the wallets
  const wallets = JSON.parse(
    fs.readFileSync(
      '../../../SecretsAndKeysAndPrivateKeys/InternalDev/wallets-1718855265771-lit-10.json'
    )
  );
  // load up all the config files with the IP addresses
  const configs = [];
  for (let i = 0; i < 10; i++) {
    const config = TOML.parse(
      fs.readFileSync(
        `../../../SecretsAndKeysAndPrivateKeys/InternalDev/lit_config${i}.toml`,
        'utf8'
      )
    );
    configs.push(config);
  }

  // match them all up
  const walletsAndConfigs = [];
  for (let wallet of wallets) {
    let matchedConfig;
    for (let config of configs) {
      if (config.node.staker_address === wallet.staker.address) {
        matchedConfig = config;
        break;
      }
    }
    walletsAndConfigs.push({
      wallet,
      config: matchedConfig,
    });
  }

  // okay now we have everything we need.  let's run the txns.
  for (let walletAndConfig of walletsAndConfigs) {
    let txHash = await setInfo({
      stakingContractAddress,
      stakerPrivateKey: walletAndConfig.wallet.staker.privateKey,
      nodeAddress: walletAndConfig.wallet.node.address,
      comsKeySender: walletAndConfig.wallet.node.comsKeysSender.publicKey,
      comsKeyReceiver: walletAndConfig.wallet.node.comsKeysReceiver.publicKey,
      ip: walletAndConfig.config.node.domain,
      port: 443,
    });
    console.log(
      `Staking completed for ${walletAndConfig.wallet.staker.address} with txHash ${txHash}`
    );
  }
}

if (require.main === module) {
  go();
}
