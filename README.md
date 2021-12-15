# Paid Chain

Tha Paid Chain is substrate based Blockchain on which we can deploy and use our solidity smart contract. 

## Installation
The first thing you will need to do is prepare the system for Substrate development.

Follow this [link](https://docs.substrate.io/v3/getting-started/installation/) for development environment setup.

## Build & Run

To build the chain, execute the following commands:

```
$ cargo build --release
```

To execute the chain, run:

```
$ ./target/release/paid-chain --dev --tmp
```

To start the fresh chain, run:

```
$ ./target/release/paid-chain purge-chain --dev
```

## Test

To build run the test cases of the chain, execute the following commands:

```
$ cargo test
```
