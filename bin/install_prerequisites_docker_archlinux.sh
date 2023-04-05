#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
pacman -Syy --noconfirm
pacman -Sy --noconfirm bash sudo python3 python-pip icu base-devel libxml2 glibc openbsd-netcat squid
python3 -m pip install --upgrade pip --quiet
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

