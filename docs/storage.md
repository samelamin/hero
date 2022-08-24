# Storage

A way or storing and retrieving stored assets (proofs, contract materials, etc.)
is required for the system to function.

Previously various pathways were suggested, with IPFS being the most venerable one.

Since IPFS doesn't have an incentive mechanism builtin, it doesn't really fit our bill.
FileCoin is the natural extension of that. However, with XCM/XCMP just being made workable I suggest the following scheme:

We introduce the notion of a storage provider role, where a full node always says how
much storage it will agree on to retain for the sake of network functionality.

To allow for the byzantine recovery, we need two things, very similar to the polkadot
availability: Recovery and availability. Since some of the files to be stored are
anticipated to be large, full copies do not make sense. Storing something like erasure
codes posses itself as a viable alternative.

To be able to detect a malicious actor, we need additional merkle proofs, the error detection capabilities of some implementations of erasure encodings are not sufficient.

Merkle proofs come to mind here. Creating these proofs just like any other merkle proof, this time the leaves are hashes or erasure encoding chunks.

The root of the merkle proof _must_ be stored on-chain, as reference for validation.
The verification and data availability can and should be entirely off-chain. Creating
a new runtime API for the node side with the runtime and vice versa will likely be necessary. Be aware of potential race conditions here.

Recovery must be able to be done from `f` valid chunks, as defined by the byzantine base requirement. Note that `2f+1` is likely not going to work, since the number of offline and malicious actors might change between the storage and the requirements
to have the data available.

The individual chunks are only distributed on demand with their relevant proof via request response. Before that only bitfields should be gossiped regarding the availability of chunks.

It's yet to be defined where the chunks are initially created, i.e. where does the data
get uploaded.
