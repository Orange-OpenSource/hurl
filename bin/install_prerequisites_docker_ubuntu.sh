#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
apt update
apt -y install bash sudo curl libcurl4-openssl-dev libxml2-utils libxml2-dev libssl-dev python3 python3-pip netcat squid
python3 -m pip install --upgrade pip --quiet
sudo squid -k kill > /dev/null 2>&1 || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

