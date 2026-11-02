#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
pacman -Syu --noconfirm \
    bash \
    sudo \
    gcc \
    openssl \
    python3 \
    icu \
    base-devel \
    libxml2 \
    glibc \
    openbsd-netcat \
    jq

"$(dirname "$0")"/install_mitmproxy.sh

# libxml crate >= 0.3.4 uses bindgen
pacman -Syu --noconfirm clang

# Temporary install to patch a python3/pip crash
pacman -Syu --noconfirm expat
