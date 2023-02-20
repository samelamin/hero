
FROM rust:1.65-slim as builder
RUN apt-get update && apt-get install -y git cmake pkg-config libssl-dev clang libclang-dev gnupg2 ca-certificates awscli wget curl vim
RUN rustup default nightly && rustup target add wasm32-unknown-unknown


RUN git clone -b add_docker_file https://github.com/samelamin/hero
WORKDIR /hero
RUN cargo build --release

# FROM docker.io/library/ubuntu:20.04
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y git awscli wget curl vim

LABEL description="Multistage Docker image for the hero collator" \
	image.type="builder" \
	image.description="Hero Collator" \

COPY --from=builder /hero/target/release/hero /usr/local/bin

RUN mkdir -p /data /hero/.local/share && \
	ln -s /data /hero/.local/share/hero && \
# check if executable works in this container
	/usr/local/bin/hero --version

EXPOSE 30333 9944 9945 9615
VOLUME ["/data"]