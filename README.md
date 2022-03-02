# Paid-chain

Tha Paid-chain is Polkadot parachain on which we can deploy and use Paid solidity smart contracts. 


## Running a PaidChain Collator
1.) First things first is to [install docker](https://www.docker.com/products/docker-desktop) for your particular platform.

2.) clone Paidchain repository `git clone https://github.com/PAIDNetwork/paid-chain.git && cd paid-chain`
and checkout the correct branch `git checkout issue19_andrew/docker_deployment`

3.) build docker base `docker build -f Dockerfile.base -t paidnetwork/rust .`

4.) build collator image `docker build -t paidnetwork/collator .`

5.) Open the following ports `30344` `40334` `6968` `8845` `9978` if you would like to connect to your collator remotely open port `80`

6.) run the collator and frontend `docker-compose -f scripts/docker-compose-collator.yaml up -d`

You now should beable to view your collator via [polkadotjs](https://polkadot.js.org/apps/#/explorer) switch your network and under
the `Develoment tab` click `Custom` and paste `ws://localhost:8845` then click `Switch` at the top.

## Paid Dev Environment Docker

1.) Clone paid-chain `git clone https://github.com/PAIDNetwork/paid-chain.git && cd paid-chain`

2.) build docker base `docker build -f Dockerfile.base -t paidnetwork/rust .` 

3.) build collator image `docker build -t paidnetwork/collator .`

4.) Clone Polkadot `cd $HOME && git clone https://github.com/PAIDNetwork/polkadot.git`

5.) checkout correct polkadot version `cd polkadot && git checkout release-v0.9.13`

6.) build relay chain image `docker build -f Dockerfile.relay -t paidnetwork/relay .`

7.) Run containers via docker compose `cd $HOME/paid-chain && docker-compose -f docker-compose-relay.yaml up -d`

You now should beable to view your collator via [polkadotjs](https://polkadot.js.org/apps/#/explorer) switch your network and under
the `Develoment tab` click `Custom` and paste `ws://localhost:8844` then click `Switch` at the top. For the relay chain 
connect to `ws://localhost:9944`

Note** the chainspec which is being used for the parachain is "rococo-local" please reference paid-chain/node/src/chain_spec.rs for more
details on the exact configuration. If you would like to change simply make an edit to the docker-compose or upgrade the chain_spec.rs
i.e.(Sudo key)


Follow instructions on [Cumulus Tutorial](https://docs.substrate.io/tutorials/v3/cumulus/connect-parachain/) for registering your parachain on the local relay chain. Remember the paraid is 2000

## Installation Paid Development
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
