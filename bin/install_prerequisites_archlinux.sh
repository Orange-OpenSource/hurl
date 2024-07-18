#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
pacman -Sy --noconfirm \
    bash \
    sudo \
    expect \
    openssl \
    python3 \
    python-pip \
    icu \
    base-devel \
    libxml2 \
    glibc \
    openbsd-netcat \
    squid \
    jq
# Temporary install to patch a python3/pip crash
pacman -Sy --noconfirm expat
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

