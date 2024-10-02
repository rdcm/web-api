# build image
FROM rust:1.79.0-slim-buster as build
WORKDIR /web-api
COPY . .

# installing deps for building librdkafka
RUN apt-get update && apt-get install -y build-essential \
     openssl libssl-dev \
     zlib1g \
     cmake \
     curl

# caching application build deps
RUN --mount=type=cache,target=/web-api/target cargo build --release && cp target/release/host /host

# api image
FROM debian:buster-slim as api

# installing netcat for check kafka readiness
RUN apt-get update && apt-get install -y netcat

COPY --from=build /host /host
CMD ["/host"]