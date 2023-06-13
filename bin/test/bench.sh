#!/bin/bash
set -Eeuo pipefail

echo "----- Bench suite -----"

# hurl infos
command -v hurl || (echo "ERROR - hurl not found" ; exit 1)
command -v hurlfmt || (echo "ERROR - hurlfmt not found" ; exit 1)
hurl --version
hurlfmt --version

# bench
pip install --break-system-packages --requirement bin/requirements-frozen.txt
cd bench
python3 server.py > server.log 2>&1 &
sleep 5
netstat -anpe | grep 8000
./run.sh

