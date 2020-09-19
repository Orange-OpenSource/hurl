#!/bin/bash

set -e

#ROOT_DIR=$(pwd)

rustup component add clippy
cargo clippy --version
cargo install cargo-deb

# Python/Flask
sudo apt-get install python3-pip
python3 -V
pip3 install Flask
(cd integration && python3 server.py&)


# Mitmproxy
wget https://snapshots.mitmproxy.org/5.2/mitmproxy-5.2-linux.tar.gz -O - | tar -xz
./mitmdump &

sleep 2
netstat -an | grep LISTEN | grep -E '8000|8080'


