FROM rust:1.98.1-slim-trixie AS base

# Setup build environment
RUN apt-get update && \
    apt-get install --yes --no-install-recommends musl musl-dev musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    rustup component add rustfmt

WORKDIR /representer

COPY . .

# Build rust-representer
RUN cargo build --release --target=x86_64-unknown-linux-musl && \
    cp target/x86_64-unknown-linux-musl/release/rust-representer ./bin/

FROM alpine:3.24.3@sha256:be2656061b536c1883c4034790e21d0a09c5aaaf51968b89368a3c0371ea2c9a

WORKDIR /opt/representer

COPY --from=base /representer/bin /opt/representer/bin

ENTRYPOINT ["bin/run.sh"]
