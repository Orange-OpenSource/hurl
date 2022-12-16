#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
pacman -Syy --noconfirm
pacman -Sy --noconfirm bash curl icu base-devel libxml2 python3 glibc openbsd-netcat

python3 get-pip.py

