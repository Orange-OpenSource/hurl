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
    software-properties-common \
    expect \
    curl \
    net-tools \
    g++-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    libxml2-dev \
    pkg-config \
    libcurl4-openssl-dev \
    libxml2-utils \
    libxml2-dev \
    libssl-dev \
    netcat-openbsd \
    squid \
    jq \
    python3 \
    python3-distutils \
    python3-venv \
    python3-dev
sudo service squid stop || true
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

