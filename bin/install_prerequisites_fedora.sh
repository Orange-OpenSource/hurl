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
    jq
ln -sf /usr/bin/python3.11 /usr/bin/python3

"$(dirname "$0")"/install_mitmproxy.sh

# libxml crate >= 0.3.4 uses bindgen
yum install -y clang-devel
