FROM gcr.io/paid-network-202104/rust:2004ubuntu220530 as builder
ARG PROFILE=release

WORKDIR /builds/collator

COPY . .

RUN . $HOME/.cargo/env && cargo build --${PROFILE}

EXPOSE 40333-40335 8844-8848 6965-6969 30343-30345 9977-9979

# TODO: Make this run the CMD used in the docker-compose-collator.yaml
CMD tail -f /dev/null