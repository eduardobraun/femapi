FROM debian:stretch
RUN set -ex;\
    apt-get update;\
    apt-get install -y --no-install-recommends \
    libsqlite3-dev \
    libpq5 \
    libssl-dev;\
    rm -rf /var/lib/apt/lists/*
COPY ./diesel.toml /diesel.toml
RUN touch /.env
COPY ./.env /.env
COPY ./target/release/femapi /femapi
COPY ./templates /templates
COPY ./dist /www
CMD /femapi
