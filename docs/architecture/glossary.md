# Glossary

## Staking

Staking is a means of securing actions on chain by freezing funds for a defined perdiod and/or until a predefined condition is fulfilled.

### Staking Rewards

Rewards paid out to stakers for participating in the protocol as expected.

## Block producer

Any of a set of validators selecteby the evaluated VRF for the parachain slot

<https://research.web3.foundation/en/latest/polkadot/block-production/Babe.html>

if there are multiple candidates there might be a short living fork. The block producer
is in this context not clearly defined. If there is only a single block submitted, that's
then ok.

## Identity

Is unique identifier that is irrefutable 1:1 to a person or thing.

### Rating

Rating is the mean of annotating a scalar to an [identity](#Identity) in order to determine trustworthyness. Generally, a lower rating is worse.

## Storage

A mean of persisting data across blocks. This can mean both on-chain or off-chain, which has to be explicitly annotated.

## Runtime

An upgradable piece of logic, that contains all on-chain logic for the chain.

## ServiceAgreement

The actual agreement logic for binding escrow, holding it for the appropriate time and releasing or extending the binding period based on events.

## Escrow

Is a stash account that holds funds under control of solely the contract logic. Similar to a staking account.

## Arbiter

Something akin to a juror, a humon being that casts a vote on a [contract](#disputes) that was raised.

Arbiters must have their identity verified, and have sufficient stake as required by the contrac [SmartAgreement™](#SmartAgreement).

### Ordinary Arbiter

An ordinary arbiter is an arbiter that does not have any requirements regarding education or qualification.

Arbiters register themselves as available and commit to their availability.

### Professional Arbiters

Professionally trained arbiters with i.e. a law degree or similar on the relevant jurisdictions.
These must be verified and will only be available for appeals but not regular contract disputes unless they opt-in.

## Disputes

A dispute arising as part of a smart agreement. Not related to relay chain block validity disputes.

### Dispute Resolution

Arbiters voted based on evidence submitted by both parties. The consequences are depending on the outcome of the vote, but include a reputation change as well as a monetary penalty up to their escrow.

## Arbiter Selection

The selection process to assure a sufficient number of arbiters is assigned to a dispute to assure swift [dispute resolution](#dispute-resolution).

## Appeal

If a smart agreement participant challenges the outcome of the dispute resolution, an appeal can be started, which is in fact a dispute of the dispute. At this point only professional arbiters will be involved and additional charges apply.
An appeal resolution is final.
