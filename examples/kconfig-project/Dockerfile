FROM debian:trixie-slim

RUN apt-get update && apt-get install -y \
    kconfig-frontends make \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /tmp/kconfig-project
COPY . .

