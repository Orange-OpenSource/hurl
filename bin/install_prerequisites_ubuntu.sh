#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
if ! command -V sudo  ; then
    apt-get update
    apt-get -y install sudo
fi
sudo apt-get update
sudo apt-get -y install \
    bash \
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
    python3 \
    python3-pip \
    python3-venv \
    netcat-openbsd \
    squid \
    jq
sudo service squid stop || true
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

