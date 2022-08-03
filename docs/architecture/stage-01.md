### Block producer

Re-use [`Nimbus`](https://docs.moonbeam.network/learn/features/consensus/) as used by moonbeam to reduce
the load of collators.

## Identity

Contracts require identities of customers entering such a contracts.

The main difficulty is to _not_ store any personal information on-chain that would lead to a potential future extraction of the identity

Use a set of KYC providers to uniquely identify each provider - we can create a proof and submit that on-chain for each entered contract.

## Reputation

Since there is an initial approach to identity, scoring can be implemented.

A completed contract gives aa reputiation bump by $x$.

Let $f$, $p$ and $q$ be factors in range $(0..1)$ (not including bounds).

A raised dispute causes a mild reputation being unchanged for the winning party of the dispute (net zero)
and decreased for the party losing by $\frace1p$.

Note that once we move to biometric data, there might be two proofs for the same person. Given the maturity and
early stage of the parachain, this is still acceptable. There is no intended migration path from KYC to biometric
data based reputiation is not intended at this point.

## Storage

Full nodes can provide some storage and gain rewards per time unit and gain rewards for providing chunks of data per request. The individual chunks are erasure encoded and `k` out of `n` chunks being sufficient. The merkle proof of the reconstruction must be stored on chain to verify both authenticity and completeness of the provided data.

To retain privacy a layer of encryption must be applied _before_ chunk creation, either on individual files or on the compressed archive of all files.
The encryption should be multi party signature where `2` out of the `p` parties part of the contract suffice to unlock provided data, where there is an additional on-chain key that is revealed to the arbiters to unlock the information. The dispute raising party must reveal their key to the on-chain logic to proceed.

This third key is stored on-chain encrypted with a `1` out of `2` keys being required for decryption. Performance is not very relevant, storage size is since that key remains on-chain.

The integration of ipfs is outlined <https://rs-ipfs.github.io/offchain-ipfs-manual/>.

## Extendability

Extendability is a key feature, so other chains can leverage our infrastructure
to model contracts.

### XCMP

It shall be able to use a granular API to be used via a pair of directional XCMP channels.

### EVM

Since the parachain is supposed to also act as a layer one for smart contracts, there is
a certain push to include a smart contract language. At this point this is EVM pulling in
compatibility with a lot of easy to migrate smart contracts.
