#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
pacman -Syy --noconfirm
pacman -Sy --noconfirm bash sudo curl icu base-devel libxml2 python3 python3-venv glibc openbsd-netcat squid
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true
