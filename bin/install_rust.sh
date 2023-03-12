#!/bin/bash
set -Eeuo pipefail

rust_version=$(grep '^rust-version' packages/hurl/Cargo.toml | cut -f2 -d'"')

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs >/tmp/rustup.sh
sh /tmp/rustup.sh -y --default-toolchain "$rust_version"
~/.cargo/bin/rustc --version
~/.cargo/bin/cargo --version

