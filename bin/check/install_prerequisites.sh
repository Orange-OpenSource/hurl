#!/bin/bash
set -Eeuo pipefail

echo "# Install Python packages..."
python3 -m pip install \
    --requirement bin/requirements-frozen.txt \
    zizmor \
    requests
