# HERO

## Code In Depth Documentations
For more detailed documentations about the code itself, [click here](./docs)

Hero is Polkadot parachain on which we can deploy and use solidity smart contracts.


## Running a Hero Collator
1.) First thing is to [install docker](https://www.docker.com/products/docker-desktop) for your particular platform.

2.) In the root hero folder run the below command
  ```
  docker-compose up -d
  ```

After a min or two you now should be able to view your collator producing blocks via [polkadotjs](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9946#/explorer)
