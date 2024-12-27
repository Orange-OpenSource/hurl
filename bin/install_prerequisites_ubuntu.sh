#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
if ! command -V sudo  ; then
    echo ":: Installing sudo"
    apt-get update
    DEBIAN_FRONTEND=noninteractive apt-get -y install sudo
fi
sudo apt-get update
sudo DEBIAN_FRONTEND=noninteractive apt-get -y install \
    bash \
    expect \
    curl \
    net-tools \
    build-essential \
    g++-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    libxml2-dev \
    pkg-config \
    libcurl4-openssl-dev \
    libxml2-utils \
    libxml2-dev \
    libssl-dev \
    python3 \
    python3-venv \
    netcat-openbsd \
    squid \
    jq
sudo service squid stop || true
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true
