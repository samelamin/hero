// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
// When running the script with `hardhat run <script>` you'll find the Hardhat
// Runtime Environment's members available in the global scope.
import { ethers } from 'hardhat';
import { Contract, ContractFactory } from 'ethers';

async function main(): Promise<void> {
  
  // We get the Incrementer contract to deploy
  const IncrementerFactory: ContractFactory = await ethers.getContractFactory(
    'Incrementer',
  );
  const incrementer: Contract = await IncrementerFactory.deploy();
  await incrementer.deployed();
  console.log('Incrementer deployed to: ', incrementer.address);

  console.log(
      `The transaction that was sent to the network to deploy the Incrementer contract: ${
          incrementer.deployTransaction.hash
      }`
  );

  // print the initial value - value, lastCaller
  console.log(`Initial value: ${await incrementer.getValue()}`);
  console.log(`Last caller: ${await incrementer.getLastCaller()}`);

  // After incrementing, print the initial value - value, lastCaller
  console.log("----------------------------------");
  await incrementer.inc();
  console.log("After inc():");
  console.log(`value: ${await incrementer.getValue()}`);
  console.log(`Last caller: ${await incrementer.getLastCaller()}`);
  
  // After adding, print the initial value - value, lastCaller
  console.log("----------------------------------");
  await incrementer.add(45);
  console.log("After add(45):");
  console.log(`value: ${await incrementer.getValue()}`);
  console.log(`Last caller: ${await incrementer.getLastCaller()}`);

}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.  
main()
  .then(() => process.exit(0))
  .catch((error: Error) => {
    console.error(error);
    process.exit(1);
  });
