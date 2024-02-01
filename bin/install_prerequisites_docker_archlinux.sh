#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
pacman -Sy --noconfirm bash sudo python3 python-pip icu base-devel libxml2 glibc openbsd-netcat squid
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

