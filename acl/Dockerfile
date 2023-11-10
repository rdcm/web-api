# build image
FROM rust:1.73.0-slim-buster as build
WORKDIR /web-api
COPY . .
RUN apt-get update && apt-get install -y build-essential \
     openssl libssl-dev \
     zlib1g \
     cmake

RUN --mount=type=cache,target=/web-api/target cargo build --release && cp target/release/acl /acl

# acl image
FROM debian:buster-slim as acl
COPY --from=build /acl /acl
CMD ["/acl"]