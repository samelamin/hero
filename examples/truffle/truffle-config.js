require("dotenv").config
const HDWalletProvider = require('@truffle/hdwallet-provider');
const privateKeys = [`${process.env.DEPLOYER_PRIVATE_KEY}`];

module.exports = {

  networks: {
      paidchain: {
            provider: () => new HDWalletProvider(
              privateKeys,
              `${process.env.PAID_CHAIN_URL}`
            ),
            network_id: `${process.env.PAID_CHAIN_ID}`,
            skipDryRun: true
      }
  },

  // Set default mocha options here, use special reporters etc.
  mocha: {
    // timeout: 100000
  },

  // Configure your compilers
  compilers: {
    solc: {
      version: "0.8.10",
    }
  },
};
