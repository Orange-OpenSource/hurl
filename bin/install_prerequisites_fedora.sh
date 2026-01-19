#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
yum install -y \
    bash \
    sudo \
    which \
    procps \
    gcc \
    libxml2-devel \
    openssl-devel \
    libcurl-devel \
    python3.11-devel \
    nc \
    squid \
    jq
ln -sf /usr/bin/python3.11 /usr/bin/python3
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

# libxml crate >= 0.3.4 uses bindgen
yum install -y clang-devel
