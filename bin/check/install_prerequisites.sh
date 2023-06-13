#!/bin/bash
set -Eeuo pipefail

bin/install_rust.sh
python3 -m pip install --requirement bin/requirements-frozen.txt
sudo apt-get update && sudo apt-get install -y libxml2-utils

