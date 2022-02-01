# Paid-chain

Tha Paid-chain is Polkadot parachain on which we can deploy and use Paid solidity smart contracts. 

## Installation
The first thing you will need to do is prepare the system for Substrate development.

Follow this [link](https://docs.substrate.io/v3/getting-started/installation/) for development environment setup.

## Download and install Polkadot

Clone repo:
```bash
git clone https://github.com/paritytech/polkadot.git
```

Change to polkadot directory:
```bash
cd polkadot
```

Checkout the latest working commit:
```bash
git checkout v0.9.13
```

Build the relay chain:
```bash
cargo build --release
```

Check if the build succeeded:
```bash
./target/release/polkadot --help
```


## Download and install paid-collator

Clone repo:
```bash
git clone git@github.com:PAIDNetwork/paid-chain.git
```

Change directory:
```bash
cd paid-chain
```

Build paid parachain collator:
```bash
cargo build --release
```

Check if build succeeded:
```bash
./target/release/parachain-collator --help
```
## Start Local Relay Chain Validators

Open 2 Seperate Terminals:

Start Alice:
```bash
./target/release/polkadot --alice \
--validator \
--base-path /tmp/relay/alice \
--chain rococo-custom-2-raw.json \
--port 30333 \
--ws-port 9944
```

Copy Alices Local Node Identity which looks something like this in Alices terminal output:
```bash
Local node identity is: 12D3KooWGjsmVmZCM1jPtVNp6hRbbkGBK3LADYNniJAKJ19NUYiq
```

Start Bob:
```bash
./target/release/polkadot --bob \
--validator \
--base-path /tmp/relay-bob \
--chain rococo-custom-2-raw.json \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<Alices local node identity from above> \
--port 30334 \
--ws-port 9945
```

## Obtain Wasm runtime validation function and parachain genesis state

```bash
./target/release/parachain-collator export-genesis-wasm --chain rococo-local-parachain-2000-raw.json > para-2000-wasm
```

```bash
./target/release/parachain-collator export-genesis-state --chain rococo-local-parachain-2000-raw.json > para-2000-genesis
```

## Start parachain collator

Start Paid Parachain:
```bash
./target/release/parachain-collator --alice \
--collator \
--force-authoring \
--chain rococo-local-parachain-2000-raw.json \
--base-path /tmp/parachain/alice \
--port 40333 \
--ws-port 8844 \
--rpc-port 6969 \
--rpc-cors all \
-- \
--execution wasm \
--chain relay-chain-spec.json \
--port 30343 \
--ws-port 9977
```

## Register parathread/parachain

Start polkadot js apps:
```
https://polkadot.js.org/apps/#/explorer
#click drop down arrow on the top left by "Rococo Local Testnet"
#select Local Node 127.0.0.1:9944
```

Navigate the ui to register a parathread id
```
Network -> Parachains -> Parathreads -> ParaId(with a plus sign)
#click sign and submit
```

Register the parathread id on your relay chain
On polkadot.js app do the following:
```
Developer -> Sudo -> (under submit the following change) paraSudoWrapper ->
(on the right drop down) sudoScheduleParaInitialize(id, genesis)
#click file upload for both genesisHead and validationCode fields
#upload para-2000-genesis && para-2000-wasm in their respective field.
#change parachain bool to 'yes'
```
You will need to wait 2 minutes for your parachain to be accepted by the relay chain validators.
check this in:
```
Network -> Parachains -> Overview
```

# Complete!
You should see that your collator is collating and you can swap to interact with your paid collator
on the top left corner!! 
Create a custom endpoint:
```
ws://127.0.0.1:8844
```
