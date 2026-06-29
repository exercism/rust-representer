FROM rust:1.95.0-slim-trixie AS base

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

FROM alpine:3.23.5@sha256:fd791d74b68913cbb027c6546007b3f0d3bc45125f797758156952bc2d6daf40

WORKDIR /opt/representer

COPY --from=base /representer/bin /opt/representer/bin

ENTRYPOINT ["bin/run.sh"]
