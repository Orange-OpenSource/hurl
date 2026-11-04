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

# libxml2 >= 2.14 bumps soname to libxml2.so.16; install libxml2.so.2 compat
# package (if available) so that binaries linked against libxml2.so.2 still run
# (for instance, generic linux package built on Ubuntu).
if yum info libxml2_2.13 >/dev/null 2>&1; then
    yum install -y libxml2_2.13
fi

"$(dirname "$0")"/install_mitmproxy.sh

# libxml crate >= 0.3.4 uses bindgen
yum install -y clang-devel
