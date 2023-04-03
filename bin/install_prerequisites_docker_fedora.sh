#!/bin/bash
set -Eeuo pipefail

# Install packages
yum install -y python38 procps gcc libxml2-devel openssl-devel libcurl-devel nc squid
curl https://bootstrap.pypa.io/get-pip.py -o /tmp/get-pip.py
USER="$(whoami)"
export USER
python3 /tmp/get-pip.py

