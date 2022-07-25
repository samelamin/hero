# Architecture

The scope of this document is to describe the architecture of the `hero`-chain.

It attempts to outline the constraints and goals and cover implementation details.

## Goals

Model legal contracts with simple litigation, disputes and mediation. This goes along with the contract definition and acceptance criteria, as well as submitting
proof.

## Participation

Staking is a mean to secure participation and create incentives to participate in the network.

### Block producer

The block producer is selected by a VRF as part of the hybrid consensus approach. Currently it's planned to re-use [`Nimbus`](https://docs.moonbeam.network/learn/features/consensus/) as used by moonbeam.

## Identity

Contracts require identities of customers entering such a contracts.

The main difficulty is to _not_ store any personal information on-chain that would lead to a potential future extraction of the identity.

### PoC: KYC

Using any KYC provider should suffice for the time being, that means submitting a proof on chain associated with some collateral bond to that governance could seize.

### Prod: Identity Wallet

TBD

---

## Storage

For contract disputes to be resolved, a wealth of data needs to be stored and accessible for later review once the contract is due or expires.

The storage must be distributed and retrievable at any time during the active time of the dispute and a set time period after that, which depends both on disputes and appeals being initiated. The secrecy of the documents is prevalent unless they are required to be disclosed to relevant parties.

### PoC

A CLI provided S3 storage provider using a unique key based on the signature
on the contract id signed by both parties.

### Prod

Full nodes can provide some storage and gain rewards per time unit and gain rewards for providing chunks of data per request. The individual chunks are erasure encoded and `k` out of `n` chunks being sufficient. The merkle proof of the reconstruction must be stored on chain to verify both authenticity and completeness of the provided data.

To retain privacy a layer of encryption must be applied _before_ chunk creation, either on individual files or on the compressed archive of all files.
The encryption should be multi party signature where `2` out of the `p` parties part of the contract suffice to unlock provided data, where there is an additional on-chain key that is revealed to the arbiters to unlock the information. The dispute raising party must reveal their key to the on-chain logic to proceed.

This third key is stored on-chain encrypted with a `1` out of `2` keys being required for decryption. Performance is not very relevant, storage size is since that key remains on-chain.

### Open Questions

* Late filings of 'evidence', allowed, penalized, fixed time slots?
* ...

---

## Runtime

The runtime contains almost all of the logic, besides a few individual functions
that require host access, i.e. artifact chunks retrieval.

### Staking

Currently it's planned to reuse the moonbeam staking pallet, which includes
ensures the security of the parachain participation by utilizing the similar staking mechanics to the relay chain.

#### Staking Rewards

TBD

### Time Oracle

Time measurements are vicious. Assuming we can assume little drift in the clocks.

There are a few basic options:

* GNSS receiver
* NTP servers
* Local server time
* Block time

---

Superposition of clocks using i.e. median as a baseline or some advanced Kalman filters assuming accuracy of each is known.

Block time might be inaccurate due to chain stalls and skipped blocks in the relay chain, but is good enough for a PoC.

### `ServiceAgreement`

The actual agreement logic for binding escrow, holding it for the appropriate time and releasing or extending the binding period based on events.

### Escrow

Is a stash account that holds funds under control of solely the contract logic.

#### Ordinary Arbiter

An ordinary arbiter is an arbiter that does not have any requirements regarding education or qualification beyond its identity being verified and sufficient escrow being put down. Sufficient here determines eligibility for certain contract values.

Ordinary arbiters register themselves to be registered to be available within a certain time frame to take on casting their vote to resolve a dispute one way or another.

### Professional Arbiters

Professionally trained arbiters with i.e. a law degree or similar on the relevant jurisdictions. These must be verified and will only be available for appeals but not regular contract disputes.

### Contract Disputes

Resolving contract disputes requires a wealth of information being available.
This information is retrieved based on an identifier used for the contract.
The content must be reconstructed from its chunks and decrypted.

The audience for the information is a set of ordinary arbiters.

The ordinary arbiters must come to a conclusion within a certain time frame `t_dispute`.
Any arbiters that do not participate in the voting must be slashed. Participating voters on the concluding side must be rewarded, on the wrong side shall be slashed.

Any of the contract parties is entitled to an appeal of the resolution, which has to conclude in the same fasion as the initial vote with the identical consequences within time `t_appeal`.

Note: that these disputes are very separate from relay chain disputes and in no way related. As such, it should always be named more specifically.

### Arbiter Selection

An uneven number, but at least 3 must be picked. If any of the primaries do not show up after 50% of the vote deadline expiry, additional votes are requested. These secondary ordinary arbiters will not replace the initial set, but only be added to the voting set. If the initial voters cast their vote on time, the secondary ones are rewarded a fraction according to their side, but the extra arbiters votes are not counted for the tally.

The selection can be narrowed based on i.e. a minimum stake required for the contract to avoid low stakers voting on high value contractor.

The set of determined by a VRF picking arbiters from the active pool and is re-initialized any `q` blocks.

### Appeal

As part of the appeal, either party can submit additional information to strengthen their argument.

The process itself is analogous to the resolution process of the ordinary arbiters, albeit with professional arbiters and possibly some more time. Unlike in the regular court, not the proceedings are checked, but a second stage of higher order votes.

After the conclusion of an appeal there is further action outside of an governance intervention that can be taken. This should not become the rule, but be a very exceptional proceeding and is yet to be specified.
