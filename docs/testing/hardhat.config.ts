import { task, HardhatUserConfig } from "hardhat/config";

//https://hardhat.org/config/
const config: HardhatUserConfig = {
  defaultNetwork: "hardhat",
  networks: {
    hero: {
      url: `${HERO_CHAIN_URL}`,
      chainId: 1345,
      //gasPrice: 0,
      //gasPrice: ethers.utils.parseUnits("1", "gwei").toNumber(),//causing Payment error
      gas: 25e6,//7e6, 2e6
      accounts: {
        mnemonic: `${MNEMONIC}`,
        path: "m/44'/60'/0'/0",
        initialIndex: 0,
        count: 20,
        passphrase: "",
      },
    },
    hardhatnode: {
      url: "http://127.0.0.1:8545",
      gasPrice: ethers.utils.parseUnits("10", "gwei").toNumber(),
      gas: 7e6,
      accounts: {
        mnemonic: `${MNEMONIC}`,
        path: "m/44'/60'/0'/0",
        initialIndex: 0,
        count: 20,
        passphrase: "",
      }
      //accounts: [privateKey1, privateKey2, ...]
      //accounts: { mnemonic: process.env.MNEMONIC },
    },
    hardhat: {
      gasPrice: ethers.utils.parseUnits("10", "gwei").toNumber(),
      gas: 7e6,
      accounts: {
        mnemonic: `${MNEMONIC}`,
        path: "m/44'/60'/0'/0",
        initialIndex: 0,
        count: 20,
        passphrase: "",
      }
    },
    frontier: {
      url: "http://127.0.0.1:9933",
      chainId: 1942,
      gasPrice: ethers.utils.parseUnits("10", "gwei").toNumber(),
      gas: 25e6,//7e6, 2e6
      accounts: {
        mnemonic: `${MNEMONIC}`,
        path: "m/44'/60'/0'/0",
        initialIndex: 0,
        count: 20,
        passphrase: "",
      }
    },
    rinkeby: {
      url: `https://rinkeby.infura.io/v3/${INFURA_API_KEY}`,
      chainId: 4,
      gasPrice: ethers.utils.parseUnits("20", "gwei").toNumber(),
      gas: 25e6,
      accounts: [`0x${OWNERPK}`, `0x${USER1PK}`, `0x${USER2PK}`],
    },
    goerli: {
      url: `https://goerli.infura.io/v3/${INFURA_API_KEY}`,
      chainId: 5,
      gasPrice: ethers.utils.parseUnits("20", "gwei").toNumber(),
      gas: 25e6,
      accounts: [`0x${OWNERPK}`, `0x${USER1PK}`, `0x${USER2PK}`],
    },
    kovan: {
      url: `https://kovan.infura.io/v3/${INFURA_API_KEY}`,
      chainId: 42,
      gasPrice: ethers.utils.parseUnits("20", "gwei").toNumber(),
      gas: 25e6,
      accounts: [`0x${OWNERPK}`, `0x${USER1PK}`, `0x${USER2PK}`],
    },
  },
  solidity: {
    compilers: [
      {
        version: "0.8.9",
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      },
    ],
    overrides: {
    },
  },
  mocha: {
    timeout: 2000000,
  },
  paths: {
    sources: "contracts",
    artifacts: "./build/artifacts",
    cache: "./build/cache",
  },

  typechain: {
    outDir: "./build/typechain/",
    target: "ethers-v5",
  },
};

export default config;
