#!/bin/sh
set -eu
echo "----- install prerequisite packages -----"
pacman -Syy --noconfirm
pacman -Sy --noconfirm curl icu base-devel libxml2 python3 glibc

# FIXME: chrono 0.4.20 basically requires one to set up a correct timezone
# We set /etc/localtime in a Docker context
# see https://github.com/chronotope/chrono/issues/755
ln -s /usr/share/zoneinfo/Europe/Paris /etc/localtime

curl -O https://bootstrap.pypa.io/get-pip.py
python3 get-pip.py
