#!/bin/sh
set -eu

echo "----- Bench suite -----"

# hurl infos
command -v hurl
command -v hurlfmt
hurl --version
hurlfmt --version

# bench
pip3 install --requirement bin/requirements-frozen.txt
cd bench
python3 server.py >server.log 2>&1 &
sleep 5
netstat -anpe | grep 8000
./run.sh
