#!/bin/bash
set -e
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
sh rustup.sh -y
~/.cargo/bin/rustc --version
~/.cargo/bin/cargo --version

