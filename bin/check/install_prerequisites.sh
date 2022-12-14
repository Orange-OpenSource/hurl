#!/bin/bash
set -Eeuo pipefail

bin/install_rust_latest.sh
pip3 install -r bin/requirements-frozen.txt
sudo apt update && sudo apt install -y libxml2-utils

