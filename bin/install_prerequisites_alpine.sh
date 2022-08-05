#!/bin/sh
set -eu
echo "----- install prerequisite packages -----"
apk update --quiet
apk add --quiet bash curl curl-dev build-base libffi-dev libxml2-dev libxml2-utils openssl-dev python3 python3-dev py3-pip

# FIXME: chrono 0.4.20 basically requires one to set up a correct timezone
# We set /etc/localtime in a Docker context
# see https://github.com/chronotope/chrono/issues/755
apk add tzdata
ln -s /usr/share/zoneinfo/Europe/Paris /etc/localtime

python3 -m pip install --upgrade pip --quiet
