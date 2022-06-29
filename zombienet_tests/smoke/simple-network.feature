Description: Simple Network Test
Network: ./simple-network.toml
Creds: /tmp/output/kubeconfig-v1.21.2-k3s1.yaml

# to run this file: zombienet-linux -p kubernetes test tests/integration/simple-network.feature

# well know functions
alice: is up
bob: is up
alice: parachain 2000 is registered within 225 seconds

# logs
bob: log line matches glob "*rted #1*" within 10 seconds
bob: log line matches "Imported #[0-9]+" within 10 seconds
bob: log line matches "Imported new block." within 10 seconds

alice: parachain 2000 block height is at least 6 within 150 seconds

# metrics
alice: reports node_roles is 4
alice: reports sub_libp2p_is_major_syncing is 0

# system events
bob: system event contains "A candidate was included" within 20 seconds
alice: system event matches "\"paraId\":[0-9]+" within 10 seconds
