FROM rust:1.72-slim as base

WORKDIR /representer

COPY . .

# Setup build environment
RUN apt-get update && \
    apt-get install -y musl musl-dev musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    rustup component add rustfmt

# Build rust-representer
RUN cargo build --release --target=x86_64-unknown-linux-musl && \
    cp target/x86_64-unknown-linux-musl/release/rust-representer ./bin/

FROM alpine:3.18.4

WORKDIR /opt/representer

COPY --from=base /representer/bin /opt/representer/bin

ENTRYPOINT ["bin/run.sh"]