FROM rust:1.36-stretch as base

WORKDIR /representer

COPY . .

RUN apt-get update && \
    apt-get install -y musl musl-dev musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    rustup component add rustfmt && \
    cargo build --release --target=x86_64-unknown-linux-musl && \
    cp target/x86_64-unknown-linux-musl/release/rust-representer ./bin/

FROM alpine:latest

WORKDIR /opt/representer

COPY --from=base /representer/bin/* ./bin/

ENTRYPOINT ["bin/generate.sh"]