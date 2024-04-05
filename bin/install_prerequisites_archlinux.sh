#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
pacman -Syu --noconfirm bash sudo icu base-devel libxml2 python3 python3-venv glibc openbsd-netcat squid jq
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true
