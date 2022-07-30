#!/bin/sh
set -e
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
sh rustup.sh -y
# shellcheck source=/dev/null
PATH="$HOME/.cargo/bin:$PATH"
export PATH
rustc --version
cargo --version


