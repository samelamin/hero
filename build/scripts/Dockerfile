FROM docker.io/library/ubuntu:20.04

LABEL description="Docker image for Hero" \
	io.hero.image.type="collator" \
	io.hero.image.authors="obrienalaribe@gmail.com, devops@master.ventures" \
	io.hero.image.vendor="Hero Chain" \
	io.hero.image.description="Hero: a substrate blockchain" \
	io.hero.image.documentation="https://github.com/PAIDNetwork/hero"

RUN apt update && apt install -y telnet

# create user to run target with
RUN useradd -m -u 1000 -U -s /bin/bash -d /hero hero

# copy target & chain spec files
COPY ./target/release/hero /usr/local/bin/hero-collator
COPY ./res               /hero/res

RUN hero-collator build-spec --disable-default-bootnode > /hero/res/rococo-local-parachain-plain.json \
	&& hero-collator build-spec --chain /hero/res/rococo-local-parachain-plain.json --raw --disable-default-bootnode > /hero/res/rococo-local-parachain-2000-raw.json \
	&& hero-collator export-genesis-wasm --chain /hero/res/rococo-local-parachain-2000-raw.json > /hero/res/para-2000-wasm \
	&& hero-collator export-genesis-state --chain /hero/res/rococo-local-parachain-2000-raw.json > /hero/res/para-2000-genesis

# minimize attack surface && test target
# RUN rm -rf /usr/bin /usr/sbin

USER hero

EXPOSE 40333-40335 8844-8848 6965-6969 30343-30345 9977-9979

CMD ["hero-collator", "--alice", "--collator", "--force-authoring", \
"--chain", "/hero/res/rococo-local-parachain-2000-raw.json", \
 "--base-path", "/tmp/parachain/alice", \
 "--port", "40333", "--ws-port", "8844", "--unsafe-ws-external", "--unsafe-rpc-external", \
 "--rpc-cors", "all", "--rpc-external", "--discover-local", \
 "--rpc-methods", "Unsafe", "--prometheus-external", \
 "--", "--execution", "wasm", \ 
 "--chain", "/hero/res/rococo-custom-2-raw.json", "--port", "30343",\
 "--ws-port", "9977"]
