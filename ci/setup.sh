#!/bin/bash

set -e

#ROOT_DIR=$(pwd)

rustup component add clippy
rustup component add rustfmt

cargo clippy --version
cargo fmt --version
cargo install cargo-deb

# Python/Flask
sudo apt-get install -y python3-pip
python3 -V
pip3 install Flask
(cd integration && python3 server.py&)


# Mitmproxy
wget https://snapshots.mitmproxy.org/5.2/mitmproxy-5.2-linux.tar.gz -O - | tar -xz
./mitmdump -p 8888 --modify-header "/From-Proxy/Hello" &

sleep 2
netstat -an | grep LISTEN | grep -E '8000|8888'


