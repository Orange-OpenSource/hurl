#!/bin/bash
set -Eeuo pipefail

bin/install_rust.sh
pip install --break-system-packages -r bin/requirements-frozen.txt
sudo apt-get update && sudo apt-get install -y libxml2-utils

