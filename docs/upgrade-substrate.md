## Upgrade Substrate Versions

---
### Compile the new Parachain code that has the new substrate dependency version

> NOTE: You need to use a GUI comparison tool like meld from http://meldmerge.org/

---
### Setup the new base repository
##### Copy the old repo into another folder

##### Replace with new Parachain Runtime files
* runtime: add new weights folder and xcm_config.rs

##### Replace with new Parachain Node files
* node: remove primitive.rs

##### Replace Cargo.toml and Cargo.lock

##### Replace pallets/template

##### Merge supporting code with tne new version
* .github/ : new workflow are added
* .dockerignore: not to be added as it is not needed
* .editconfig : merge 
* .gitignore: merge
* .rustfmt.toml: merge

* polkadot-launch/config.json : merge
* examples/ : use old hardhat and truffle + README.md

save a backup snapshot of this repo: rebuild0

---
### Merge /Cargo.toml 
[note] it is important to add Cargo.lock from the compiled code of parachain template, then let Rust to compile your code with that file!

* add dependencies from Frontier repo: pallet-evm, pallet-ethereum, pallet-evm-precompile-simple, pallet-evm-precompile-sha3fips, pallet-evm-precompile-modexp, fp-self-contained, fp-rpc, fp-storage, fp-evm

* comment out unused dependencies: fc-rpc, fc-rpc-core, fc-db, fc-mapping-sync, fp-consensus, fp-dynamic-fee, fc-consensus, also in Node/Cargo.toml

* fix all versions to polkadot-v0.9.18

* trigger Rust to auto update Cargo.lock by `cargo check`

save a backup snapshot of this repo: rebuild1

---
### Add Pallets
* add Crowdloan pallet
* add erc721 pallet

* Update their all dependencies
hex-literal = { version = "0.3.4" ...}
polkadot-v0.9.18
release-v0.9.17

save a backup snapshot of this repo: rebuild2

---
### Compile runtime/Cargo.toml
replace "parachain_template_runtime" with "paid_chain_runtime"

save a backup snapshot of this repo: rebuild3

---
### Update runtime/lib.rs and node/src/chain_spec.rs
* update Frontier dependencies
* add redirect code at /Cargo.toml to redirect Frontier dependencies to use our custom repo with custom version branch

#### add crowdloan pallet in runtime/lib.rs
in runtime/lib.rs

```parameter_types! {..}
impl pallet_crowdloan_rewards::Config for Runtime {..}

construct_runtime!(
  ...
  CrowdloanRewards: pallet_crowdloan_rewards::{Pallet, Call, Storage, Config<T>, Event<T>} = 42,
)
```

#### node/src/chain_spec.rs
```use paid_chain_runtime::{
  AccountId, AuraId, Balance, CrowdloanRewardsConfig, ...}
use sp_runtime::{..}
const CROWDLOAN_FUND_POT: u128 = 30_000_000;

pub fn development_config() -> ChainSpec {
  CROWDLOAN_FUND_POT,
}

pub fn local_testnet_config() -> ChainSpec {
CROWDLOAN_FUND_POT,
}

fn testnet_genesis(...
  crowdloan_fund_pot: Balance,
) {
  crowdloan_rewards: CrowdloanRewardsConfig { funded_amount: crowdloan_fund_pot },
}
```

save a backup snapshot of this repo: rebuild4


---
### add pallet_ethereum, pallet_evm pallet
#### in project/Cargo.toml
* redirect Frontier to custom version

#### fix node/Cargo.toml
* fix frontier dependencies according to the latest

#### in runtime/src/lib.rs
```
use pallet_ethereum::{..}
impl pallet_evm::Config for Runtime {...}
impl pallet_ethereum::Config for Runtime {...}

construct_runtime!(
  EVM: pallet_evm
  Ethereum: pallet_ethereum
)

pub type UncheckedExtrinsic =
  fp_self_contained::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;

impl fp_rpc::ConvertTransaction<UncheckedExtrinsic> for {..}
impl fp_rpc::ConvertTransaction<opaque::UncheckedExtrinsic> for {..}

impl fp_rpc::EthereumRuntimeRPCApi<Block> for Runtime {..}
impl fp_rpc::ConvertTransactionRuntimeApi<Block> for Runtime {..}

impl fp_self_contained::SelfContainedCall for Call {..}
```

#### in chain_spec.rs
``` use fp_evm::GenesisAccount ```

save a backup snapshot of this repo: rebuild5


---
### add pallet_sudo pallet and runtime benchmarks
#### in runtime/src/lib.rs
```
impl pallet_sudo::Config for Runtime {
  type Event = Event;
  type Call = Call;
}
construct_runtime!(
    Sudo: pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>} = 25,
)
```

#### in node/src/chain_spec.rs
```
//add root_key
get_account_id_from_seed::<sr25519::Public>("Alice"),

    sudo: SudoConfig {
      // Assign network admin rights.
      key: Some(root_key),
    },

//#[cfg(feature = "std")]
//pub use pallet_evm::GenesisAccount;
```

save a backup snapshot of this repo: rebuild6


---
### in node/src/chain_spec.rs
```
pub fn rococo_live_config() -> ChainSpec {..}
pub fn rococo_local_config() -> ChainSpec {...}
```

save a backup snapshot of this repo: rebuild7


---
### in node/src/command.rs
```
PaidChainRuntimeExecutor

  "rococo-local" => Box::new(chain_spec::rococo_local_config()),
  "rococo-live" => Box::new(chain_spec::rococo_live_config()),

"Paid-chain Parachain"

service.rs: PaidChainRuntimeExecutor
```

save a backup snapshot of this repo: rebuild8


---
### in node/src/service.rs
```
add dependencies: fc-rpc, fc-rpc-core
line10: Index as Nonce -> Index

MappingSyncWorker::new(... add argument at index 5 and 6)
```

save a backup snapshot of this repo: rebuild9


---
### in node/src/rpc.rs
```
  P: TransactionPool<Block = Block> + Sync + Send + 'static,
....
  use fc_rpc::{EthApi, EthApiServer, NetApi, NetApiServer, Web3Api, Web3ApiServer};


  io.extend_with(SystemApi::to_delegate(FullSystem::new(..)
  io.extend_with(Web3ApiServer::to_delegate(Web3Api::new(client.clone())));
  io.extend_with(EthApiServer::to_delegate(EthApi::new(..)
```
save a backup snapshot of this repo: rebuild10

---
#### Fix pallets/crowdloan-rewards test

#### Fix pallets/pallets/erc721 test

save a backup snapshot of this repo: rebuild11 
