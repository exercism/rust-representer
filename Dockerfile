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

FROM alpine:3.23.4@sha256:5b10f432ef3da1b8d4c7eb6c487f2f5a8f096bc91145e68878dd4a5019afde11

WORKDIR /opt/representer

COPY --from=base /representer/bin /opt/representer/bin

ENTRYPOINT ["bin/run.sh"]
