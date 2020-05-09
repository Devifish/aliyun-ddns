ARG RUST_VERSION=1.43
ARG ALPINE_VERSION=3.11

#Build image
FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} as builder
WORKDIR /src

#Build code
COPY . .
RUN cargo install --path .

#Runner image
FROM alpine:${ALPINE_VERSION}
MAINTAINER Devifish <devifish@outlook.com>

#Run application
COPY --from=builder /src/bin/aliyun-ddns /usr/bin/aliyun-ddns
ENTRYPOINT aliyun-ddns --mode env
