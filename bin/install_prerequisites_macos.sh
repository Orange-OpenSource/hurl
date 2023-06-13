#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
brew install curl pkg-config squid
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

