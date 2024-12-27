#!/bin/bash
# shellcheck source=/dev/null
set -Eeuo pipefail

echo "----- activate python3 venv -----"
python3 -m venv /tmp/hurl-python3-venv
source /tmp/hurl-python3-venv/bin/activate
python3 -m pip install --upgrade pip --quiet

