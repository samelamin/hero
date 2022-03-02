#!/bin/sh

WORKDIR="/builds/collator"

nohup ${WORKDIR}/target/release/parachain-collator --alice \
--collator --force-authoring --chain rococo-local --unsafe-ws-external \
--base-path /tmp/parachain/alice --port 40333 --ws-port 8844 --rpc-port 6969 \
--rpc-cors all -- --execution wasm --chain ${WORKDIR}/relay-chain-spec.json --port 30343 \
--ws-port 9977 > collator.out 2>&1 & 
