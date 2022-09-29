## Upgrade Substrate Versions

---

### Compile the new Parachain code that has the new substrate dependency version

> NOTE: You need to use a GUI comparison tool like meld from http://meldmerge.org/

---

### Setup the new base repository

##### Copy the old repo into another folder

##### Replace with new Parachain Runtime files

-   runtime: add new weights folder and xcm_config.rs

##### Replace with new Parachain Node files

-   node: remove primitive.rs

##### Replace Cargo.toml and Cargo.lock

##### Replace pallets/template

##### Merge supporting code with tne new version

-   .github/ : new workflow are added
-   .dockerignore: not to be added as it is not needed
-   .editconfig : merge
-   .gitignore: merge
-   .rustfmt.toml: merge

-   polkadot-launch/config.json : merge
-   examples/ : use old hardhat and truffle + README.md

save a backup snapshot of this repo: rebuild0

---

### Merge /Cargo.toml

[note] it is important to add Cargo.lock from the compiled code of parachain template, then let Rust to compile your code with that file!

-   add dependencies from Frontier repo: pallet-evm, pallet-ethereum, pallet-evm-precompile-simple, pallet-evm-precompile-sha3fips, pallet-evm-precompile-modexp, fp-self-contained, fp-rpc, fp-storage, fp-evm

-   comment out unused dependencies: fc-rpc, fc-rpc-core, fc-db, fc-mapping-sync, fp-consensus, fp-dynamic-fee, fc-consensus, also in Node/Cargo.toml

-   fix all versions to polkadot-v0.9.18

-   trigger Rust to auto update Cargo.lock by `cargo check`

save a backup snapshot of this repo: rebuild1

---

### Add Pallets

-   add Crowdloan pallet
-   add erc721 pallet

-   Update their all dependencies
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

-   update Frontier dependencies
-   add redirect code at /Cargo.toml to redirect Frontier dependencies to use our custom repo with custom version branch

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

-   redirect Frontier to custom version

#### fix node/Cargo.toml

-   fix frontier dependencies according to the latest

#### in runtime/src/lib.rs

Update according to Frontier repo/template/runtime/src/lib.rs

```
use pallet_ethereum::{..}
impl pallet_evm::Config for Runtime {...}
impl pallet_ethereum::Config for Runtime {...}

construct_runtime!(
  EVM: pallet_evm
  Ethereum: pallet_ethereum
)

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	fp_self_contained::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = fp_self_contained::CheckedExtrinsic<AccountId, Call, SignedExtra, H160>;

impl fp_rpc::ConvertTransaction<UncheckedExtrinsic> for {..}
impl fp_rpc::ConvertTransaction<opaque::UncheckedExtrinsic> for {..}

impl fp_rpc::EthereumRuntimeRPCApi<Block> for Runtime {..}
impl fp_rpc::ConvertTransactionRuntimeApi<Block> for Runtime {..}

impl fp_self_contained::SelfContainedCall for Call {..}
```

#### in chain_spec.rs

`use fp_evm::GenesisAccount`

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
HeroRuntimeExecutor

  "rococo-local" => Box::new(chain_spec::rococo_local_config()),
  "rococo-live" => Box::new(chain_spec::rococo_live_config()),

"Hero Parachain"

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

---

_(added by Remy Clarke 2022/06/24)_

# Upgrading to new version of **Polkadot**

Hero chain is regularly updated to be up-to-dated with the **relay chain of Polkadot**.
Unfortunately, the upgrade process is not always as simple as updating a single dependancy.
The following are steps to help the next engineer upgrade the to future versions of Polkadot.

## **Patching external dependancies** that depend on Polkadot

For now Patching only concerns Hero's **paritytech/Frontier** dependancy. However the following pattern might be required in the future for other dependancies _(for example Cumulus)_.

The `Cargo.toml` at the **workspace root directory** has a patch mapping the
source of the dependancy _( this would be specified in `Cargo.toml` for each crate in the workspace; NOTE the branch itself may not be used, but it must be a valid branch else it will fail .. see below )_

```toml
# .../hero/node/Cargo.toml
# Frontier
fp-rpc = { git = "https://github.com/paritytech/frontier", branch = "master" }
( ... )
```

to our patched forked version with a specific branch _( we will define the patch name .. see below )_

```toml
# .../hero/Cargo.toml
[patch."https://github.com/paritytech/frontier"]
pallet-evm = { git = "https://github.com/PAIDNetwork/frontier", branch = "patch-polkadot-v0.9.25" }
( ... )
```

### Preparing the **patch**

To make the patch first **clone the fork** _( in this case our current fork )_

```bash
git clone git@github.com:PAIDNetwork/frontier.git
```

move into the repo, then add a **remote of the upstream repo**

```bash
cd frontier/ && git remote add upstream https://github.com/paritytech/frontier
```

**Fetch upstream** changes
_( name the remote `upstream` in the previous command)_

```bash
git fetch upstream
```

You should now see new branches to patch from in your terminal : here is an example

```bash
( ... )
From https://github.com/paritytech/frontier
 * [new branch]        dependabot/cargo/clap-3.2.2 -> upstream/dependabot/cargo/clap-3.2.2
 * [new branch]        dependabot/cargo/jsonrpsee-0.14.0 -> upstream/dependabot/cargo/jsonrpsee-0.14.0
 * [new branch]        gh-pages         -> upstream/gh-pages
 * [new branch]        legacy           -> upstream/legacy
 * [new branch]        master           -> upstream/master
 * [new branch]        polkadot-v0.9.19 -> upstream/polkadot-v0.9.19
 * [new branch]        polkadot-v0.9.22 -> upstream/polkadot-v0.9.22
 * [new branch]        sp-transact-delegatecall -> upstream/sp-transact-delegatecall
```

**Checkout the most recent branch from upstream** to patch that has an explicit
polkadot version _( where possible )_
_( in this case we will show patching from branch `polkadot-v0.9.22` )_

````
git checkout polkadot-v0.9.22
```bash
Checkout the new patch branch
```bash
git checkout -b patch-polkadot-v0.9.25
````

### Update all the dependanies

**Carefully search and replace dependancies** with the correct branch  
**Double check!**

```toml
# here the branch needs a bump in version
sp-core = { version = "6.0.0", git = "https://github.com/paritytech/substrate", branch = "polkadot-v0.9.22", default-features = false }
# to
sp-core = { version = "6.0.0", git = "https://github.com/paritytech/substrate", branch = "polkadot-v0.9.25", default-features = false }
```

Make the apropriate fixes where needed, and **compile the code** to update `Cargo.lock`

```bash
cargo update && cargo build --release
```

After making your changes you can finally push

```bash
git push --set-upstream origin patch-polkadot-v0.9.25
```

A pull request is not required, so long as a branch exists you can use it to patch.

## Updating the Polkadot dependancies on Hero internally

Search and replace dependancies to update Polkadot to new release version the rest to the new Polkadot branch.

```toml
# Polkadot dependancies have ` branch = "release-v<VERSION>" `
polkadot-cli =  { git = "https://github.com/paritytech/polkadot", branch = "release-v0.9.25" }
# Things that depend on polkadot have ` branch = "polkadot-v<VERSION> `
frame-executive =              { git = "https://github.com/paritytech/substrate", default-features = false, branch = "polkadot-v0.9.25" }
```

Many dependancies will change the apis so you will need to fix any
breaking code.

Check the two **templates** at the most apropriate branch ( **paritytech/Frontier** and **paritytech/Cumulus**) for other dependancy changes and changes in the runtime and node.
here are links to the master branches:
_( https://github.com/paritytech/frontier/tree/master/template )_
_( https://github.com/paritytech/cumulus/tree/master/parachain-template )_

**Upgrade up to compatability.**
_( Remember they are still just templates, prioitize the Cumulus template as it is a propper parachain )_

If you are getting errors saying that multiple versions of a dependancy conflicts, try rebuilding from scratch

```bash
cargo clean && cargo update && cargo build --release
```

Remember that the **`runtime` module can compile separately** so get it compiling before trying to compile the `node` module.

Once the Hero compiles be sure to run the **test suite** with `cargo test`
