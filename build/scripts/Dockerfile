
# This is the build stage for the imbue collator. Here we create the binary in a temporary image.
FROM rust:1.65-slim as builder

WORKDIR /hero
COPY . /hero
RUN apt-get update && apt-get install -y git cmake pkg-config libssl-dev clang libclang-dev gnupg2 ca-certificates awscli wget curl vim
RUN rustup default nightly && rustup target add wasm32-unknown-unknown
RUN cargo build --release

# This is the 2nd stage: a very small image where we copy the imbue binary."
# FROM docker.io/library/ubuntu:20.04
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y git awscli wget curl vim

LABEL description="Multistage Docker image for the imbue collator" \
	image.type="builder" \
	image.authors="sam@imbue.network tech@imbue.network" \
	image.vendor="Imbue Network" \
	image.description="Imbue Network Collator" \
	image.source="https://github.com/imbuenetwork/imbue/Dockerfile" \
	image.documentation="https://github.com/imbuenetwork/imbue"

COPY --from=builder /hero/target/release/hero /usr/local/bin

RUN mkdir -p /data /hero/.local/share && \
	ln -s /data /imbue/.local/share/imbue && \
# check if executable works in this container
	/usr/local/bin/hero --version

EXPOSE 30333 9944 9945 9615
VOLUME ["/data"]