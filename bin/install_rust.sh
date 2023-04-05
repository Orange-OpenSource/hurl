#!/bin/bash
set -Eeuo pipefail

rust_version=$(grep '^rust-version' packages/hurl/Cargo.toml | cut -f2 -d'"')
curl https://sh.rustup.rs -sSfkL | sh -s -- -y --default-toolchain "$rust_version"
~/.cargo/bin/rustc --version
~/.cargo/bin/cargo --version

