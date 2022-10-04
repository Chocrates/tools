FROM ghcr.io/cross-rs/x86_64-unknown-linux-gnu:edge
RUN apt-get update && \
    apt-get install --yes libssl-dev pkg-config
