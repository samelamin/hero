## Frontier Test

---

After upgrading Frontier to a new Substrate version, we should deploy and run some Ethereum smart contract tests to confirm the upgraded Frontier is working properly

### Test upgraded Frontier repository

##### Clone the repository

```
git clone https://github.com/paritytech/frontier
```

or

```
https://github.com/PAIDNetwork/frontier
```

##### Update your Frontier code

Open this file repo/template/runtime/src/lib.rs > chainId

Change your default chainId from 42 to 1942

Build your upgraded Frontier repository:

```
cargo build --release
```

then run it:

```
./target/release/frontier-template-node --dev
```

##### Setup your Polkadot UI

Open Polkadot UI in your browser at <https://polkadot.js.org/apps/#?rpc=ws://127.0.0.1:9944>

Copy the polkadot UI setting at <https://github.com/paritytech/frontier/tree/master/template>

```
{
  "Address": "MultiAddress",
  "LookupSource": "MultiAddress",
  "Account": {
    "nonce": "U256",
    "balance": "U256"
  },
  "Transaction": {
    "nonce": "U256",
    "action": "String",
    "gas_price": "u64",
    "gas_limit": "u64",
    "value": "U256",
    "input": "Vec<u8>",
    "signature": "Signature"
  },
  "Signature": {
    "v": "u64",
    "r": "H256",
    "s": "H256"
  }
}
```

Go to Polkadot UI > Settings > Developer tab:
Paste the copied setting

##### Use Hardhat:

Use the sample [hardhat.config.ts file](./hardhat.config.ts)

##### Use MetaMask:

Open Metamask > add the chain's default account 0x6Be02d's private key:
`0x99b3c12287537e38c90a9219d4cb074a89a16e9cdb20bf85728ebd97c343e342`

Add the new local Frontier Ethereum network to Metamask:

-   Network name: Frontier
-   URL: http://127.0.0.1:9933
-   ChainId: 1942
-   Token: ETH

##### Use Remix IDE:

Use Remix > Injected Web3 > Approve connecting certain MetaMask accounts to Remix

Confirm the approved account shows up in Remix

Deploy Solidity smart contracts from examles/hardhat/multicontracts/contracts

Test those smart contracts according to examles/hardhat/multicontracts/test
