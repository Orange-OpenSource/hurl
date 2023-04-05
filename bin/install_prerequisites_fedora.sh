#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
yum install -y bash sudo procps gcc libxml2-devel openssl-devel libcurl-devel python3-devel python3-pip nc squid
python3 -m pip install --upgrade pip --quiet
sudo squid -k kill > /dev/null 2>&1 || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

