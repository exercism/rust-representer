FROM rust:1.45-stretch as base

WORKDIR /representer

COPY . .

# Download exercism tooling webserver
RUN wget -P /usr/local/bin https://github.com/exercism/local-tooling-webserver/releases/latest/download/exercism_local_tooling_webserver && \
    chmod +x /usr/local/bin/exercism_local_tooling_webserver

# Setup build environment
RUN apt-get update && \
    apt-get install -y musl musl-dev musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    rustup component add rustfmt

# Build rust-representer
RUN cargo build --release --target=x86_64-unknown-linux-musl && \
    cp target/x86_64-unknown-linux-musl/release/rust-representer ./bin/

FROM alpine:latest

WORKDIR /opt/representer

COPY --from=base /representer/bin/* ./bin/
COPY --from=base /usr/local/bin/ /usr/local/bin/

ENTRYPOINT ["bin/generate.sh"]