#!/bin/bash
set -Eeuo pipefail

# Install packages
apt update
apt -y install bash curl sudo libcurl4-openssl-dev libxml2-utils libxml2-dev libssl-dev python3 python3-pip netcat squid
python3 -m pip install --upgrade pip --quiet

