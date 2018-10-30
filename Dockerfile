FROM rustlang/rust:nightly as api_build
ARG api_url
COPY ./femapi ./femapi
RUN (cd femapi && cargo build --release)
RUN mkdir -p /build-out
RUN cp femapi/target/release/femapi /build-out/
RUN cp femapi/.env /build-out/
RUN cp femapi/Rocket.toml /build-out/
RUN cp femapi/diesel.toml /build-out/

FROM debian:stretch
RUN set -ex;\
    apt-get update;\
    apt-get install -y --no-install-recommends \
    libsqlite3-dev \
    libssl-dev;\
    rm -rf /var/lib/apt/lists/*
COPY --from=api_build /build-out/* /
COPY --from=frontend_build /build-out/dist /www
CMD ROCKET_ENV=production /femapi
