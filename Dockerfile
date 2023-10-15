# build image
FROM rust:1.73.0-slim-buster as build
WORKDIR /web-api
COPY . .
RUN --mount=type=cache,target=/web-api/target cargo build --release && cp target/release/host /host

# app image
FROM debian:buster-slim
COPY --from=build /host /host
CMD ["/host"]