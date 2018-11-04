FROM rustlang/rust:nightly
RUN set -ex;\
    apt-get update;\
    apt-get install -y --no-install-recommends \
    libsqlite3-dev \
    libssl-dev;\
    rm -rf /var/lib/apt/lists/*
RUN cargo install diesel_cli
