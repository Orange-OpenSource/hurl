#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
yum install -y sudo python38 procps gcc libxml2-devel openssl-devel libcurl-devel nc squid
curl https://bootstrap.pypa.io/get-pip.py -o /tmp/get-pip.py
USER="$(whoami)"
export USER
python3 /tmp/get-pip.py
sudo squid -k kill > /dev/null 2>&1 || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true
