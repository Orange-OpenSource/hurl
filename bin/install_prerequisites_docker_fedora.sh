#!/bin/sh
set -eu

# Install packages
yum install -y python3-devel python3-pip
python3 -m pip install --upgrade pip --quiet
