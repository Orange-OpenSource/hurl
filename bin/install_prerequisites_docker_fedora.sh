#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
yum install -y sudo which python3 python3-pip procps gcc libxml2-devel openssl-devel libcurl-devel nc squid
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

