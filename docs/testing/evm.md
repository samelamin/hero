## Running EVM Test

#### Start validators and collator

According to the main README, launch the following:

-   2 validators(relayers) ALICE and BOB
-   the compiled Hero collator from this repository

#### Install an Ethereum wallet

For example: Install MetaMask at https://metamask.io/download/

#### Connect wallet to Hero collator EVM network

-   Go to the settings page
-   Click on "Networks", then "Add Network"
-   Key in the following details:

```
Network Name   : Hero
New RPC URL    : http://127.0.0.1:6969
Chain ID       : 1345 ... (set in runtime lib.rs)
Currency Symbol: Hero
```

> NOTE: this will work only after Hero collator is running and communicating with ALICE and BOB validators

#### Import default Hero accounts via its private keys:

Click the circle icon at the top right corner of your wallet pop-up next to the network indicator.

Select “Import Account” on the dropdown menu:
Paste the private keys from below and click “Import”.

```
Account1: 0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b
Private key1: 0x99b3c12287537e38c90a9219d4cb074a89a16e9cdb20bf85728ebd97c343e342

Account2: 0x735113e044BFce4DebA5da7BfBc96A3e6A96F6Da
Private key2: 0x9cdb0ce76f6da6c844b48e2ef16cff35a9b593921e5b88032bf40d39631c26d8
```

-   Approve connecting 0x6Be02d account1 from the wallet to Remix
-   Confirm that the newly added accounts appear in the dropdown menu with an "Imported" tag next to the account.

#### Clear Account1 and Account2 Nounce

-   Go to the account1 at the top right of your wallet and click Settings.
-   Go to the Advanced settings, click on the Reset Account button.
-   Confirm that any previous Account1 transactions have been removed in your account1's transaction activities section
-   Do the same for Account2

#### Transfer Hero Tokens from Accout1 to Account2

-   Confirm both Account1 and Account2 have zero Hero token balance
-   Go to Account1 and click on "Send" -> click on "Transfer Between my accounts" -> click on Account2. Choose the Hero Token as the asset. Enter amount of 1000 (Hero) Tokens
-   Wait for successful transaction, confirm the Hero token balance now in Account2 is 1000.

#### Deploy Ethereum smart contracts via Hardhat

-   Clone our EVM-Test-Hardhat repository from:
    `$ git clone https://github.com/layerhero/evm-test-hardhat`

-   Use Solidity 0.8.16 according to Hardhat's support:
    https://hardhat.org/hardhat-runner/docs/reference/solidity-support#supported-versions

-   Install Node JS v16.17.0 according to your platform: https://nodejs.org/en/download/

-   Install NodeJs dependencies
    `$ npm install`

-   Implement the env.template variables in a .env file.

```
HERO_CHAIN_URL=http://127.0.0.1:6969
HERO_CHAIN_DOCKER_URL=http://127.0.0.1:9998
HERO_CHAIN_ID=1345
INFURA_API_KEY=0
MNEMONIC=patch alter unable artist hospital prize swear know faith steel frog gesture
BURN_ADDRESS=000000000000000000000000000000000000dEaD
OWNER=YOUR_WALLET_ACCOUNT_WITH_0x
OWNERPK=PRIVATE_KEY_OF_YOUR_ACCOUNT_ABOVE
USER1PK=PRIVATE_KEY_OF_YOUR_ACCOUNT1
USER2PK=PRIVATE_KEY_OF_YOUR_ACCOUNT2
NODE_URL=http://127.0.0.1:8545/
REPORT_GAS=<Optional: true_or_false>
```

#### Wait for the Hero parachain collator to be ready

Wait after seeing something like this in the collator console:

```
[Parachain] 💤 Idle (0 peers), best: #5 (0x3c33…905a), finalized #1 (0xa7c5…7278), ⬇ 0 ⬆ 0
```

#### Deploy Ethereum Smart Contracts via Hardhat

Back to this EVM Test Hardhat repository

-   Compile Ethereum smart contracts

    ```
    $ npm run clean
    $ npm run compile
    ```

-   Clean previous deployment records

    ```
    $ npm run reset
    ```

-   Deploy Ethereum smart contracts:

    > Note: `npm run h3` is just to delete previous Hardhat deployment record. If there is no previous deployment record, it will give error like `no file/directory exists`. That is okay and ignore that error message.

    > Note: If you do have previous Hardhat deployment record inside the `.deployedCtrts` file and `.openzeppelin` folder, then you MUST delete them before deploying new upgradeable contracts. Failing to do so will result in error as Hardhat will not be able to know how to deploy upgradeable contracts.

Deploy Demo1 contract

```
npm run h1
```

Deploy SolidityTest1 contract

```
npm run h2
```

Delete previous deployment records

```
npm run h3
```

Deploy upgradeable Box contract

```
npm run h4
```

Deploy ERC20 contract

```
npm run h11
```

Deploy ERC721 contract

```
npm run h12
```

Deploy ERC1155 contract

```
npm run h13
```

Deploy ERC1820 contract

> Note: You MUST deploy ERC1820 before deploying ERC777 contract. If you have deployed ERC1820 previously, this command will detect it and skip re-deployment. So run this command anyway just in case!

```
npm run h14
```

Deploy ERC777 contract

```
npm run h15
```

Deploy ERC4626 contract

```
npm run h16
```

Deploy ERC20 upgradeable contract

```
npm run h21
```

Deploy ERC721 upgradeable contract

```
npm run h22
```

Deploy ERC1155 upgradeable contract

```
npm run h23
```

Deploy ERC1820 contract

> Note: You MUST deploy ERC1820 before deploying ERC777 upgradeable contract. If you have deployed ERC1820 previously, this command will detect it and skip re-deployment. So run this command anyway just in case!

```
npm run h24
```

Deploy ERC777 upgradeable contract

```
npm run h25
```

#### Watch a video demo on Ethereum smart contract deployment

Part 1: Setup nodes and Connect to MetaMask
https://drive.google.com/file/d/1-Zsaxjaz1rSfUjLs3hRXuvLJbvcRuA3u/view?usp=sharing

Part 2: Deploy Ethereum smart contracts
https://drive.google.com/file/d/1FPs49N2WIofeboGaKJtay2UE5OTe8O0a/view?usp=sharing

---
