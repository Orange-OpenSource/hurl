#!/bin/sh
set -eu

# Install packages
pacman -Syy --noconfirm
pacman -Sy --noconfirm python3 python-pip
python3 -m pip install --upgrade pip --quiet
