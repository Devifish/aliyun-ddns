ARG RUST_VERSION=1.43
ARG DEBIAN_VERSION=buster

#Build image
FROM rust:${RUST_VERSION}-${DEBIAN_VERSION} as builder
WORKDIR /src

#Build code
COPY . .
RUN cargo install --path .

#Runner image
FROM debian:${DEBIAN_VERSION}-slim
MAINTAINER Devifish <devifish@outlook.com>

#Run application
COPY --from=builder /src/bin/aliyun-ddns /usr/bin/aliyun-ddns
ENTRYPOINT aliyun-ddns --mode env
