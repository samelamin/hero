## Zombienet

### Installation

Install [Docker](https://docs.docker.com/engine/install/)

Install [kubectl](https://kubernetes.io/docs/tasks/tools/)

Install [minikube](https://minikube.sigs.k8s.io/docs/start/)

Install the latest [Zombienet release](https://github.com/paritytech/zombienet/releases)

Make the downloaded Zombienet release file executable:
`chmod +x zombienet-linux`

Move it in another folder and add that folder into your shell path:
`export PATH=/.../zombienet-releases/:$PATH`

Restart your shell: `source ~/.bashrc`

---

### Login and Get Authorized in Google Cloud

` sudo usermod -a -G docker ${USER}`

Then logout and login back into the operating system

` gcloud auth login`
... now logged in as [raymond@master.ventures].
... Your current project is [paidchain-3242].

` gcloud config list`
...gcloud iam service-accounts [core]
...account = johndoe@master.ventures
...disable_usage_reporting = True
...project = paidchain-3242
...Your active configuration is: [default]

` gcloud config set project paid-network-202104`

` gcloud auth login`

` gcloud auth configure-docker`
... Adding credentials for all GCR repositories.
... your new Docker config file will be saved at [/home/user1/.docker/config.json]:
{
"credHelpers": {
"gcr.io": "gcloud",
"us.gcr.io": "gcloud",
"eu.gcr.io": "gcloud",
"asia.gcr.io": "gcloud",
"staging-k8s.gcr.io": "gcloud",
"marketplace.gcr.io": "gcloud"
}
}

---

### Build Rust Docker image

```
  docker build -f zombienet/Dockerfile.base -t gcr.io/paid-network-202104/rust:2004ubuntu220530 .
  docker push gcr.io/paid-network-202104/rust:2004ubuntu220530
```

---

### Build & Push Collator Docker image

```
  docker build -f docker/Dockerfile.build -t gcr.io/paid-network-202104/collator:build .
  docker build -f zombienet/Dockerfile.zombie -t gcr.io/paid-network-202104/collator:zxx .
  docker images
  docker push gcr.io/paid-network-202104/collator:zxx
```

---

### Start minikube

`minikube start`

### Running Zombienet Tests

Assuming the zombienet-linux executable is in thie repo.

-   Run only the network definition spec

```
  export POLKADOT_VERSION=v0.9.xx
  export COLLATOR_VERSION=zxx
  zombienet-linux -p kubernetes spawn zombienet/0001-small-network.toml
```

-   Run the test file(.feature file extension)

```
  export POLKADOT_VERSION=v0.9.xx
  export COLLATOR_VERSION=zxx
  zombienet-linux1236 -p kubernetes test zombienet/0001-small-network.feature
```

If all is successful, it should show something like this in the commandline:

```
	 Network launched 🚀🚀
		 In namespace zombie-38bea45391d4dff4d95532e368906b02 with kubernetes provider

		 Node name: alice
		 Node direct link: https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A40477#/explorer

		 Node prometheus link: http://127.0.0.1:32905/metrics
---
		 Node name: bob

		 Node direct link: https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A44713#/explorer

		 Node prometheus link: http://127.0.0.1:45299/metrics
---
	 Parachain ID: 2000
		 Node name: collator01

		 Node direct link: https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A39737#/explorer

		 Node prometheus link: http://127.0.0.1:39143/metrics
---

    ✔ alice: is up (114ms)
    ✔ bob: is up (85ms)
2022-05-30 19:47:23        API/INIT: RPC methods not decorated: beefy_getFinalizedHead
    ✔ alice: parachain 2000 is registered within 225 seconds (3151ms)
    ✔ bob: log line matches glob "*rted #1*" within 10 seconds (132ms)
    ✔ bob: log line matches "Imported #[0-9]+" within 10 seconds (94ms)
    ✔ bob: log line matches "Imported new block." within 10 seconds (99ms)
    ✔ alice: parachain 2000 block height is at least 6 within 150 seconds (72295ms)
    ✔ alice: reports node_roles is 4
    ✔ alice: reports sub_libp2p_is_major_syncing is 0
2022-05-30 19:48:38        API/INIT: RPC methods not decorated: beefy_getFinalizedHead
    ✔ bob: system event contains "A candidate was included" within 20 seconds (787ms)
2022-05-30 19:48:39        API/INIT: RPC methods not decorated: beefy_getFinalizedHead
    ✔ alice: system event matches "\"paraId\":[0-9]+" within 10 seconds (474ms)

	 Node's logs are available in /tmp/zombie-38bea45391d4dff4d95532e368906b02_-42199-UW0QoDP5a7o6/logs

	 Deleting network
```

Now we can read the collator logs:
`kubectl logs -f collator01 -c collator01 -n zombie-38bea45391d4dff4d95532e368906b02`

We can also click on the collator URL to see the collator node's settings and other blockchain information via polkadot.js.org:
`https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A39737#/explorer`

Click on Network -> Explorer -> Node Info: confirm you can see 2 peers. For example:

```
12D3KooWRkZhiRhsqmrQ28rt73K7V3aCBpqKrLGSXmZ99PTcTZby	authority	130	0xad3a492a22e12ff865fea93f3e3e44df8dbb8ffd9559749f8a0569062cd2a7bf
12D3KooWQCkBm1BYtkHpocxCwMgR8yjitEeHGx8spzcDLGt2gkBm	authority	130	0xad3a492a22e12ff865fea93f3e3e44df8dbb8ffd9559749f8a0569062cd2a7bf
```

`12D3KooWQCkBm1BYtkHpocxCwMgR8yjitEeHGx8spzcDLGt2gkBm` matches relaychain node Alice as Added Boot Nodes: `/ip4/172.17.0.6/tcp/30333/ws/p2p/12D3KooWQCkBm1BYtkHpocxCwMgR8yjitEeHGx8spzcDLGt2gkBm`

Click on Network -> Parachains:

```
parachains		          lifecycle		included	backed	timeout	chain	in/out	leases
2,000                                 Parachain
Val. Group 0 (1)
5HEGXS…9AWfnh (0)
Non-voters (0)
```

Click on Network -> Explorer -> ChainInfo, confirm you can see many blocks being produced:

```
recent blocks
397  0xf5c6e28b506171ea78940d1708ea37c5624f64a997f3fd5f3341f11034345fb1	 5HEGXS…9AWfnh
396  0x2249d6ba127e3369c21c58e56e6b1f4dff385eb6f908e0ba7c4c415f1958b50d	 5GKbyS…DQBe64
```

Confirm that 5HEGXS…9AWfnh and 5GKbyS…DQBe64 match what you saw in your command line logs:

```
🧹 Starting with a fresh authority set...
	  👤 Added Genesis Authority alice - 5HEGXSoZGCkj1dYC1VLKEfd1tU7c9SyyiDcT7HHC4n9AWfnh
	  👤 Added Genesis Authority bob - 5GKbySSE8rm1QE5XPYgK1ewFM4nKQCrW1RWUnXdGReDQBe64
```

> NOTE: if the test fails, you should see something like:
> `/tmp/zombie-f8c1ff353b229713cdf72d1e3d27cf7f_-240705-tF0mvN38Aga5/logs`, then you can look up the relay chain nodes' logs via VS Code: `code /tmp/zombie-f8c1ff353b229713cdf72d1e3d27cf7f_-240705-tF0mvN38Aga5/logs`

---

### Test objectives and their corresponding test code

-   Check the relay chain node 1 is up: satisfied by "alice: is up" in the test feature file

-   Check the relay chain node 2 is up: satisfied by "bob: is up" in the test feature file

-   Check the parachain is registered within certain time: satisfied by "alice: parachain 2000 is registered within 225 seconds" in the test feature file

-   Make sure the parachain produces blocks: satisfied by "alice: parachain 2000 block height is at least 6 within 150 seconds" in the test feature file

### Test Feature file

alice: is up
bob: is up
alice: parachain 2000 is registered within 225 seconds

bob: log line matches glob "_rted #1_" within 10 seconds
bob: log line matches "Imported #[0-9]+" within 10 seconds
bob: log line matches "Imported new block." within 10 seconds

alice: parachain 2000 block height is at least 6 within 150 seconds

alice: reports node_roles is 4
alice: reports sub_libp2p_is_major_syncing is 0
