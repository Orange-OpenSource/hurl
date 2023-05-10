#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
apk update --quiet
apk add --quiet sudo bash curl curl-dev build-base libffi-dev libxml2-dev libxml2-utils openssl-dev python3 python3-dev py3-pip squid
python3 -m pip install --upgrade pip --quiet
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

