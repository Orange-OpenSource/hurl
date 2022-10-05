#!/bin/sh
set -eu

# Install packages
apt update
apt -y install curl sudo libcurl4-openssl-dev libxml2-utils libxml2-dev libssl-dev python3 python3-pip
python3 -m pip install --upgrade pip --quiet
