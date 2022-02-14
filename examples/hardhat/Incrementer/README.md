# Sample Smart contract for PAID chain
A sample Solidity contract on PAID Chain using Hardhat

## About
* It's a Token contract.

## Installation
```console
$ npm i
```

## Usage

### Build
```console
$ npx hardhat compile
```

### Test
```console
$ npx hardhat test
```

### Deploying contracts to localhost Hardhat EVM
#### localhost
```console
// on terminal-1
$ npx hardhat node

// on terminal-2
$ npx hardhat run deployment/hardhat/deploy.ts --network localhost
```


### Deploying contracts to Testnet
#### Testnet - Paid Chain
* Run the Paid Chain as per this [repo url](https://github.com/PAIDNetwork/paid-chain#build--run).
```
$ git clone https://github.com/PAIDNetwork/paid-chain.git
$ cd paid-chain
$ cargo build --release

// to resume a chain
$ ./target/release/paid-chain --dev --tmp

OR

// to start a fresh chain
$ ./target/release/paid-chain --dev
```
* Setup Metamask wallet & get Faucets for development
	1. Add Metamask extension into browser
	2. Follow this [documentation](https://masterventures.slab.com/posts/paid-chain-test-document-v5iivb3t)
	3. Now, you have 
		+ __Faucet__: for smart contract deployment & testing
		+ __Chain URL__: for connecting to the network
* Environment variables
	- Create a `.env` file with its values:
```
PAID_CHAIN_URL=[URL_starts_with_http]
INFURA_API_KEY=[YOUR_INFURA_API_KEY_HERE]
DEPLOYER_PRIVATE_KEY=[YOUR_DEPLOYER_PRIVATE_KEY_without_0x]
REPORT_GAS=<true_or_false>
```
* Deploy the contracts
```console
$ npx hardhat run deployment/testnet/paidchain/deploy.ts  --network paidchain
```

### Deploying contracts to Mainnet
#### ETH Mainnet
* Environment variables
	- Create a `.env` file with its values:
```
INFURA_API_KEY=[YOUR_INFURA_API_KEY_HERE]
DEPLOYER_PRIVATE_KEY=[YOUR_DEPLOYER_PRIVATE_KEY_without_0x]
REPORT_GAS=<true_or_false>
```

* Deploy the token on one-chain
```console
$ npx hardhat run deployment/testnet/ETH/deploy.ts  --network mainnet
```
