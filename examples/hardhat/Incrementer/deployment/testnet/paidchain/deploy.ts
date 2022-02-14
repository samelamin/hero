// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
// When running the script with `hardhat run <script>` you'll find the Hardhat
// Runtime Environment's members available in the global scope.
import { ethers } from 'hardhat';
import { Contract, ContractFactory } from 'ethers';
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
import { config as dotenvConfig } from "dotenv";
import { resolve } from "path";
dotenvConfig({ path: resolve(__dirname, "./.env") });

const PAID_CHAIN_URL = process.env.PAID_CHAIN_URL || "";
const DEPLOYER_PRIVATE_KEY = process.env.DEPLOYER_PRIVATE_KEY || "";

async function main(): Promise<void> {
  // Hardhat always runs the compile task when running scripts through it.
  // If this runs in a standalone fashion you may want to call compile manually
  // to make sure everything is compiled
  // await run("compile");
  
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

  // Now, import the account available on paid chain
  const provider = ethers.getDefaultProvider(PAID_CHAIN_URL);
  const acc1/*: SignerWithAddress*/  = new ethers.Wallet(`0x${DEPLOYER_PRIVATE_KEY}`, provider);
  console.log("Deployer: " + acc1.address);
  // console.log(typeof(acc1.address));

  // print the initial value - value, lastCaller
  console.log(`Initial value: ${await incrementer.getValue()}`);
  console.log(`Last caller: ${await incrementer.getLastCaller()}`);

  // After incrementing, print the initial value - value, lastCaller
  console.log("----------------------------------");
  console.log("After inc():");
  await incrementer.connect(acc1).inc().then((tx: any) => {
    return tx.wait().then((receipt: any) => {
        // This is entered if the transaction receipt indicates success
        console.log(`The transaction hash: ${tx.hash}`);
        return true;
    }, (error: any) => {
        // This is entered if the status of the receipt is failure
        return error.checkCall().then((error: any) => {
            console.log("Error", error);
            return false;
        });
    });
  });
  console.log(`value: ${await incrementer.getValue()}`);
  console.log(`Last caller: ${await incrementer.getLastCaller()}`);
  
  // After adding, print the initial value - value, lastCaller
  console.log("----------------------------------");
  console.log("After add(45):");
  await incrementer.connect(acc1).add(45).then((tx: any) => {
    return tx.wait().then((receipt: any) => {
        // This is entered if the transaction receipt indicates success
        console.log(`The transaction hash: ${tx.hash}`);
        return true;
    }, (error: any) => {
        // This is entered if the status of the receipt is failure
        return error.checkCall().then((error: any) => {
            console.log("Error", error);
            return false;
        });
    });
  });
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
