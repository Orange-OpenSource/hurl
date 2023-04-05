#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
sudo apt update
sudo apt -y install bash sudo libcurl4-openssl-dev libxml2-utils netcat net-tools squid
python3 -m pip install --upgrade pip --quiet
sudo squid -k kill > /dev/null 2>&1 || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

