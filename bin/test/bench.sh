#!/bin/sh
export PATH="$PWD/target/release:$PATH"
pip3 install --requirement integration/requirements-frozen.txt
cd bench || exit 1
python3 server.py >server.log 2>&1 &
sleep 2
netstat -an | grep 8000
./run.sh
