#!/bin/sh
set -eu
sudo apt update
# Install libcurl dev so that hurl can be built dynamically with libcurl
sudo apt install libcurl4-openssl-dev libxml2-utils
python3 -m pip install --upgrade pip --quiet