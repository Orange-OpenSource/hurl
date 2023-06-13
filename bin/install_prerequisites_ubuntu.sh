#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
sudo apt-get update
sudo apt-get -y install bash libcurl4-openssl-dev libxml2-utils netcat-openbsd python3 python3-pip net-tools squid
python3 -m pip install --upgrade pip --break-system-packages --quiet
sudo service squid stop || true
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

