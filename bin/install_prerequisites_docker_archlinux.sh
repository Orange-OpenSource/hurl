#!/bin/bash
set -Eeuo pipefail

# Install packages
pacman -Syy --noconfirm
pacman -Sy --noconfirm bash python3 python-pip icu base-devel libxml2 glibc
python3 -m pip install --upgrade pip --quiet

