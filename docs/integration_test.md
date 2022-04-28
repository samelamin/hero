## Running Integration Test

This testing steps are based on the [Cumulus tutorial](https://docs.substrate.io/tutorials/v3/cumulus/start-relay/)


#### Software versioning
This steps have been tested on:
* [Polkadot official repository](https://github.com/paritytech/polkadot), branch = polkadot-v0.9.18
* This Paidchain repository, branch = polkadot-v0.9.18
* Polkadot-JS Apps v0.112.2-37. It is generally expected that the [hosted Polkadot-JS Apps](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer) should work. 

> NOTE: Exact Versions Matter
You must use the exact versions set forth in this document to ensure that you do not run into conflicts.

---
#### Build the relay chain nodes

Clone the Polkadot Repository:
`git clone https://github.com/paritytech/polkadot.git`

Switch into the Polkadot directory:
`cd polkadot`

Checkout the proper commit:
`git checkout release-v0.9.18`

Build the relay chain Node:
`cargo build --release`
wait for about 15 mins to 60 mins depending on your hardware

Check if the help page prints to ensure the node is built correctly:
`./target/release/polkadot --help`
If the help page is printed, you have succeeded in building a Polkadot node.

---
#### Building the Paidchain node

Clone this Paidchain
`git clone git@github.com:PAIDNetwork/paid-chain.git`

Switch into the parachain template directory
`cd substrate-parachain-template`

Checkout the proper commit
`git checkout polkadot-v0.9.18`

Build the parachain template collator
`cargo build --release`

Check if the help page prints to ensure the node is built correctly
`./target/release/parachain-collator --help`

this will take 15 to 60 mins to complete.
If the help page is printed, you have succeeded in building a Cumulus-based parachain collator.

---
#### Relay chain specification
Use Pre-configured chain spec files, which include a two-validator relay chain with Alice and Bob as authorities chain spec file:
* Plain rococo-local relay chain spec ... Paste this rococo-custom-2-plain.json file inside the relay chain project folder/chainspec/
* Raw rococo-local relay chain spec ... Paste this rococo-custom-2-raw.json file inside the relay chain project folder/chainspec/

Plain chain spec files are in a more human readable and modifiable format for your inspection. You will need to convert it to a SCALE encoded raw chain spec to use when starting your nodes. Jump to the raw chainspec generation section to see how to do that.

----
#### Start your relay chain
Start Relay `Alice` node
```
./target/release/polkadot \
--alice \
--validator \
--base-path /tmp/relay/alice \
--chain ./chainspec/rococo-custom-2-raw.json \
--port 30333 \
--ws-port 9944
```

Copy Alice node's Peer ID in the logs.
🏷 Local node identity is: 12D3KooWDMNzD1X8okKULRjxi5MzsLjB2JPtwXkFrbuWk1U2bPt1

---
#### Start the bob validator
```
./target/release/polkadot \
--bob \
--validator \
--base-path /tmp/relay-bob \
--chain ./chainspec/rococo-custom-2-raw.json \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWDMNzD1X8okKULRjxi5MzsLjB2JPtwXkFrbuWk1U2bPt1 \
--port 30334 \
--ws-port 9945
```

Confirm both nodes have found 1 peer!

---
#### Connect a Parachain
Open Polkadot.js.org > network dropdown > development > click on "local node" > Switch

Under the Network > Parachains > click on Parathreads tab and use the "+ ParaId button" > choose Alice as it has currency to pay according to "My Accounts" page



---
####  Configure a parachain for a specific relay chain & para ID
Go to your parachain
// Assumes that `rococo-local` is in `node/chan_spec.rs` as the relay you registered with
```
./target/release/parachain-collator build-spec --disable-default-bootnode > rococo-local-parachain-plain.json
```

Open rococo-local-parachain-plain.json and modify two fields:
```
// --snip--
  "para_id": 2000, // <--- replace 2000 with your registered ID

  /* genesis: runtime: system: code... */

      "parachainInfo": {
        "parachainId": 2000 // <--- replace with your registered ID
      },
  // --snip--
```

Then generate a raw chain spec derived from your modified plain chain spec:
```
./target/release/parachain-collator build-spec --chain rococo-local-parachain-plain.json --raw --disable-default-bootnode > rococo-local-parachain-2000-raw.json
```


---
####  Generate Wasm runtime validation and genesis state
in parachain folder:
```
./target/release/parachain-collator export-genesis-wasm --chain rococo-local-parachain-2000-raw.json > para-2000-wasm

./target/release/parachain-collator export-genesis-state --chain rococo-local-parachain-2000-raw.json > para-2000-genesis
```


---
####  Start the collator node
Assume rococo-local-parachain-2000-raw.json is iniside your parachain project folder, and rococo-custom-2-raw.json is inside your relay chain project folder

```
./target/release/parachain-collator \
--alice \
--collator \
--force-authoring \
--chain rococo-local-parachain-2000-raw.json \
--base-path /tmp/parachain/alice \
--port 40333 \
--ws-port 8844 \
-- \
--execution wasm \
--chain ../polkadot0918/chainspec/rococo-custom-2-raw.json \
--port 30343 \
--ws-port 9977
```

You should see your collator node running (alone) and peering with the already running relay chain nodes.

---
####  Parachain Registration
Go to Polkadot Apps UI, connecting to your relay chain.

Go to Developer -> Extrinsics > sudo

Pick paraSudoWrapper -> sudoScheduleParaInitialize(id, genesis) as the extrinsic type, shown below.
In the extrinsics parameters, specify:
- Set the id: ParaId to 2,000
- genesisHead: upload the file para-2000-genesis (from the previous step)
- validationCode: upload the file para-2000-wasm (from the previous step)
- Set the parachain: Bool option to Yes

This dispatch, if successful, will emit the sudo.Sudid event, viewable in the relay chain explorer page.
