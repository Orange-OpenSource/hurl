#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
brew install curl pkg-config squid
python3 -m pip install --upgrade pip --quiet
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

