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

For uploading the data here is some research.

1. IPFS: IPFS stands for InterPlanetary File System a peer-to-peer network for storing and accessing files, websites, applications, and data in distributed file.

How IPFS works ?
->  When you add a file to IPFS, your file is split into smaller chunks, cryptographically hashed, and given a unique fingerprint called a content identifier (CID). This CID acts as a permanent record of your file as it exists at that point in time.
->  When other nodes look up your file, they ask their peer nodes who's storing the content referenced by the file's CID. When they view or download your file, they cache a copy — and become another provider of your content until their cache is cleared.
->  A node can pin content in order to keep (and provide) it forever, or discard content it hasn't used in a while to save space. This means each node in the network stores only content it is interested in, plus some indexing information that helps figure out which node is storing what.
->  If you add a new version of your file to IPFS, its cryptographic hash is different, and so it gets a new CID. This means files stored on IPFS are resistant to tampering and censorship — any changes to a file don't overwrite the original,
     and common chunks across files can be reused in order to minimize storage costs.
->  However, this doesn't mean you need to remember a long string of CIDs — IPFS can find the latest version of your file using the IPNS(InterPlanetary Name System) decentralized naming system, and DNSLink can be used to map CIDs to human-readable DNS names.

Cons:
As mentioned above IPFS doesn't have an incentive mechanism builtin, it doesn't really fit our bill.

So for incentive things we have:

2. FileCoin: Like IPFS, FileCoin is a protocol developed by Protocol labs that offers a decentralized storage network. FileCoin's main focus is the storage itself and uses IPFS as a [complementary] back-end protocol.

How FileCoin Works ?
The FileCoin network works on a proof-of-work model. Unlike Bitcoin, FileCoin’s POW is related to storage of data. The work done proves that a miner has stored data for a certain time period.
FileCoin uses two types of proof-of-work proofs to prove the work done: proof-of-replication (PoRep) and proof-of-spacetime (PoSt). Proof-of-replication allows the network to confirm replication of data to a unique location.
Whereas, Proof-of-spacetime allows the network to verify that the data is stored for a specific duration of time.
Combining the two, FileCoin manages large scale storage networks with multiple independent parties. Therefore, removing the possibility of forgery of data storage records to increase mining rewards.
FileCoin’s competitors like Storj ($STORJ) and SiaCoin ($SC) lack this functionality.
The minor proofs used to create a network are based on three methods put, get and manage. Put and get methods are used for storing and accessing data in storage on client’s request.
The manage method is used for managing the marketplace by matching buy and sell orders on the platform. FileCoin has two marketplaces. One is for storage and the other one is for retrieval. Storage miners receive put requests and are responsible for storage of data.
In order to keep the data say storage miners are required to pledge collateral proportional to the data stored. Retrieval miners receive requests and are responsible for fetching the client’s data. Both the miners receive rewards in terms of the network’s native coin, FIL.

Relation between IPFS and FileCoin.
FileCoin and IPFS are two separate, complementary protocols, both created by Protocol Labs. IPFS allows peers to store, request, and transfer verifiable data with each other, while FileCoin is designed to provide a system of persistent data storage.
Under FileCoin's incentive structure, clients pay to store data at specific levels of redundancy and availability, and storage providers earn payments and rewards by continuously storing data and cryptographically proving it.
In short: IPFS addresses and moves content, while FileCoin is an incentive layer to persist data.

3. Crust Network: Crust implements the incentive layer protocol for decentralized storage. It is adaptable to multiple storage layer protocols such as IPFS, and provides support for the application layer. Crust’s architecture also has the capability of supporting a decentralized computing layer and building a decentralized cloud ecosystem.
Based on TEE (Trusted Execution Environment), Crust proposed MPoW (Meaningful Proof-of-Work). Crust storage nodes will periodically report to the entire network how much storage “I” provided, what user files “I” stored, and other information. With the help of the MPOW mechanism, the workload in the report is verifiable and credible.
Crust storage nodes will record and pack the workload report along with other transactions into the block.
At the same time, Crust designed a PoS consensus algorithm that defines the staking amount by storage resources, called GPoS (Guaranteed Proof of Stake). Through the workload report provided by the first layer MPoW mechanism, the storage workload of all nodes can be obtained on Crust, and the GPoS algorithm of the second layer is to calculate the Staking quota for each node according to the workload of the node. According to this quota, then it carries on the PoS consensus.
Crust will provide a user-oriented storage service. You can compare it to common cloud services, such as iCloud to regular users, and AliCloud, Amazon cloud to enterprise users.

-> Crust provides a native cross-chain communication pallet based on XCMP, called xStorage.

-> The protocol also supports most smart contract platforms, including Ethereum, with its cross-chain dStorage solution `https://wiki.crust.network/docs/en/buildCrossChainSolution`.

What’s the relationship between Crust and IPFS?

The relationship between Crust and IPFS is quite similar to the relationship between incentive layer and storage layer. Filecoin protocols encourage nodes in the network to provide storage capacity through incentives, which IPFS can exactly provide this kind of storage ability.
Looking at the blockchain ecosystem, many chains have relationships with IPFS. All Polkadot official websites, apps, Wikis, etc. are all placed on IPFS for decentralized versions, including the Ethereum official website, which are closely related to IPFS. It can be said that IPFS has become an indispensable part of decentralization in the Web3 ecosystem.
The “decentralized website deployment and operation” function launched on the Crust Preview Network can deploy a local website to any location that can be globally accessed. It only needs a PC and a Crust account, and no cloud storage server is required. The principle behind this is to transfer the created website to the IPFS network, and place an order through the Crust decentralized storage market, so that the storage nodes can help users store it locally, and then through the IPFS network, users can access the website anywhere.
The previous website deployment requires a cloud server, whether it is Ali Cloud or AWS, the process will be more complicated. Traditional cloud storage services cannot bring convenience for users.

What’s the difference between Crust and FileCoin?

The following is a comparison of the differences between Filecoin and Crust in the decentralized storage incentive layer.
At this stage, there are two key problems in the incentive layer of decentralized storage: proof and incentive. Proof mainly refers to how to correctly reflect the off-chain storage state on-chain, and incentive is the incentive plan for the storage node, that is, the way the node obtains rewards. The methods and technical paths used to solve these two problems in Crust and Filecoin are very different.
In terms of the Proof, Filecoin uses Proof of Replication (PoR) and Proof-of-Spacetime (PoSt). The sophisticated zero knowledge proof and VDF algorithms are very novel techniques in the Filecoin technical path. Nevertheless, from the view of the data of Filecoin testnet, a large number of complex proof algorithms bring huge computing cost, such as storage nodes demanding 128G of memory with matched GPU to prove the calculation.
Compare this to Crust, which designed a meaningful workload proof (MPoW) consensus mechanism by introducing TEE technology. The MPoW proof process can be very simple and efficient, and many PCs are available to support TEE. That is to say, to solve the same proof problem, the cost and threshold of the Crust network are lower, which is a different idea to solve the proof.
Next let’s talk about incentives. At the early stage, the node incentive of Filecoin and Crust mainly comes from block rewards, but the difference is that in Filecoin current economic model, the block rewards of Filecoin are assigned to the block node, and the probability of generating blocks is proportional to the workload of the node. Its core is a PoW Base model with more work and more gain.
With Crust’s GPoS mechanism, the block reward is proportional to the mortgage amount of each node, and the upper limit of the staking amount of each node is limited by the storage amount provided by the node. The essence of GPoS is based on PoS. This is also a reference to the Polkadot NPoS model for innovation and design. Meanwhile, as the acceptable staking amount of nodes is limited by the amount of storage, GPoS can avoid the centralization concerns of PoS.
In Crust, in addition to holding CRU, a staking amount is also required. The staking amount is based on the second layer mechanism MPoW, which is a bit similar to PoST on Filecoin. This layer mechanism uses TEE technology, and there is an off-chain protocol to confirm the miner’s local storage status. For example, if a node/miner has a storage capacity of 1,000 TB, then after he is checked by the TEE program, that is sWorker, he will make a signed workload report and submit it to the chain, and then convert the 1,000 TB to a staking amount of 1,000 CRU.
The miner has 1,000 staked CRUs, then he can compete to produce blocks. This is guaranteed by these two-layer mechanisms.
In addition, Crust has opened the decentralized storage market. Users can place orders in the market, and they will be pulled locally by the miner node through IPFS, and the files will be sealed through sWorker. Then users can perform a series of operations such as liquidation on the chain.

For more question about crust network: `https://medium.com/crustnetwork/frequently-asked-questions-crust-network-e7f6db5e34a5`

4. Substrate Files : Substrate Files is a decentralized storage module which allows substrate-based chains(including Polkadot/Kusama/Crust/Acala/Clover/Moonbeam/Astar/Phala/...) users upload their files to IPFS W3Auth Gateway and decentralized pin their files on Crust Network by using the standard IPFS W3Auth Pinning Service. This module is a 100% IPFS compatible file storage module, users can pin NFT files, host DApps or store on-chain data in totally DECENTRALIZED way(guaranteed by Crust protocol).
   Also, the Pinning Service is compatible with several Platforms like Ethereum, Polygon, Solana and Near, and funded by Decentralized Cloud Foundation. So currently, Substrate Files is FREE for all the substrate-based chains!

Pull request of substrate files: `https://github.com/polkadot-js/apps/pull/6106`

Question need to be addressed:
How to integrate crust network in our chain?
Do we use substrate files which already used crust network?
