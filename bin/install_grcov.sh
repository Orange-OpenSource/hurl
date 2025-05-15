#!/bin/bash
set -Eeuo pipefail

# See <https://github.com/mozilla/grcov/issues/1351>
# grcov v0.9.1 installation fails
cargo install grcov --version 0.8.20
rustup component add llvm-tools-preview
cargo clean

