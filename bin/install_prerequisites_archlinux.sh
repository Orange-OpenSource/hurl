#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
pacman -Syu --noconfirm \
    bash \
    sudo \
    gcc \
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
pacman -Syu --noconfirm expat
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

