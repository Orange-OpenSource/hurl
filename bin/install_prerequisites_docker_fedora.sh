#!/bin/bash
set -Eeuo pipefail

# Install packages
yum install -y python38 procps gcc libxml2-devel openssl-devel libcurl-devel
alternatives --install /usr/bin/python3 python3 /usr/bin/python3.8 0
alternatives --install /usr/bin/python3 python3 /usr/bin/python3.11 1
alternatives --set python3 /usr/bin/python3.8
curl https://bootstrap.pypa.io/get-pip.py -o /tmp/get-pip.py
USER="$(whoami)"
export USER
python3 /tmp/get-pip.py

