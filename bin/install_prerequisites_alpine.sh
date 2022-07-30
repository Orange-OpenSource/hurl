#!/bin/sh
set -eu
echo "----- install prerequisite packages -----"
apk update --quiet
apk add --quiet bash curl curl-dev build-base libffi-dev libxml2-dev libxml2-utils openssl-dev python3 python3-dev py3-pip
python3 -m pip install --upgrade pip --quiet
