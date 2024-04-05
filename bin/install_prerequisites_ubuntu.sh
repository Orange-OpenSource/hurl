#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
sudo apt-get update
sudo apt-get -y install bash libcurl4-openssl-dev libxml2-utils netcat-openbsd python3 python3-pip python3-venv net-tools squid jq
sudo service squid stop || true
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

