FROM rustembedded/cross:x86_64-unknown-linux-gnu
RUN apt-get update && \
    apt-get install --yes libssl-dev pkg-config
