FROM docker.io/library/ubuntu:20.04

LABEL description="Docker image for Hero" \
	io.hero.image.type="collator" \
	io.hero.image.authors="obrienalaribe@gmail.com, devops@master.ventures" \
	io.hero.image.vendor="Hero Chain" \
	io.hero.image.description="Hero: a substrate blockchain" \
	io.hero.image.documentation="https://github.com/PAIDNetwork/hero"

# create user to run target with
RUN useradd -m -u 1000 -U -s /bin/bash -d /hero hero

# copy target & chain spec files
COPY ./target/release/hero /usr/local/bin/hero-collator
COPY ./res               /hero/res

# minimize attack surface && test target
# RUN rm -rf /usr/bin /usr/sbin

USER hero

EXPOSE 40333-40335 8844-8848 6965-6969 30343-30345 9977-9979

CMD ["hero-collator", "--alice", "--collator", "--base-path", "/tmp/parachain/alice", \
 "--port", "40333", "--ws-port", "8844", "--unsafe-ws-external", "--unsafe-rpc-external", \
 "--rpc-cors", "all", "--rpc-external", "--discover-local", \
 "--rpc-methods", "Unsafe", "--prometheus-external" \
 "--", "--execution", "wasm", \ 
 "--chain", "/hero/res/rococo-custom-2-raw.json", "--port", "30343",\
 "--ws-port", "9977"]
