## Participation

Staking is a mean to secure participation and create incentives to participate in the network.

### Block producer

The block producer is selected by a VRF as part of the hybrid consensus approach on the relay chain.
Technically it's not required to have a specific parachain consensus mechanism other than for efficiency reasons.
For now we omit this optimization such as i.e. [`Nimbus`](https://docs.moonbeam.network/learn/features/consensus/)
and have the block producers on the relay chain bear the load of having all parachain collators submit at once.

## Identity

Contracts require identities of customers entering such a contracts.

The main difficulty is to _not_ store any personal information on-chain that would lead to a potential future extraction of the identity.

Since there is no outlook on getting this done, we skip the identity verification entirely.

### Rating

Since there is no identity, we cannot be sure we're rating the person we want to rate, hence, there will
be no implementation covering rating at this poing.


## Storage

For contract disputes to be resolved, a wealth of data needs to be stored and accessible for later review once the contract is due or expires.

The storage must be distributed and retrievable at any time during the active time of the dispute and a set time period after that, which depends both on disputes and appeals being initiated. The secrecy of the documents is prevalent unless they are required to be disclosed to relevant parties.

For now CLI provided S3 storage provider using a unique key based on the signature
on the `ContractId` signed by both parties.


## Runtime

The runtime contains almost all of the logic, besides a few individual functions
that require host access, i.e. artifact chunks retrieval.

### Staking

Currently it's planned to use `pallet-staking`, which
ensures the security of the parachain participation by utilizing
the similar staking mechanics to the relay chain.

#### Staking Rewards

Setting the precise values for staking remains to be seen.

### Time Oracle

`Babe` uses a user submitted time stamp as part of the inherents data and is checked against by all
validators +-500ms offset to account for delta drifts and package travel delays.

Since that is already available, and is a core part of the relay chain, we don't need to bother to re-invent
the wheel.

### `ServiceAgreement`

The actual agreement logic for binding escrow, holding it for the appropriate time and releasing or extending the binding period based on events.

### Escrow

Is a stash account that holds funds under control of solely the contract logic.

#### Ordinary Arbiter

An ordinary arbiter is an arbiter that does not have any requirements regarding education or qualification beyond its identity being verified and sufficient escrow being put down. Sufficient here determines eligibility for certain contract values.

Ordinary arbiters register themselves to be registered to be available within a certain time frame to take on casting their vote to resolve a dispute one way or another.

### Professional Arbiters

Professionally trained arbiters with i.e. a law degree or similar on the relevant jurisdictions. These must be verified and will only be available for appeals but not regular contract disputes.

No verification is done at this stage, they are flagged as being _professionals_ manually.

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

## Submitting the contract

The contract needs a notion. Parts of it can be formalized. I.e. day until which the completion is due,
deductions for belated delivery, as well as non-delivery.

The actual contract content can be a collection of pdf, image, and video data.

Both parties need to agree on the all-over content of the provided documents. The uploaded automatically agrees
on the uploaded files. The other party has to concur to whatever that content's meaning in respect to the contract is.

## Submitting Evidence

Submitting evidence is the process of providing data within a fixed timeframe. _Evidence_ can come in many forms,
but a emphsis is set on video and image as well as textual explanations as well as pdf documents.

This is only necessary when a dispute arises. The UI of the wallet or a dedicated web2 based entrypoint.
