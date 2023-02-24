FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /hero
RUN git clone -b add_docker_file https://github.com/samelamin/hero
WORKDIR /hero/hero
RUN cargo build --locked --release

FROM docker.io/library/ubuntu:20.04

COPY --from=builder /hero/hero/target/release/hero /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /hero hero && \
	mkdir -p /data /hero/.local/share && \
	chown -R hero:hero /data && \
	ln -s /data /hero/.local/share/hero && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/hero --version

USER hero

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/hero"]
