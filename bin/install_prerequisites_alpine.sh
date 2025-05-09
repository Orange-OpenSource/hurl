#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
apk update --quiet
apk add --quiet \
    bash \
    sudo \
    expect \
    netcat-openbsd \
    curl \
    curl-dev \
    build-base \
    libidn2 \
    libffi-dev \
    libxml2-dev \
    libxml2-utils \
    openssl-dev \
    python3 \
    python3-dev \
    cargo \
    squid \
    jq

# libxml crate >= 0.3.4 uses bindgen
apk add --quiet clang-dev

sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true
