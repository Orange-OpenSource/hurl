#!/bin/bash
set -Eeuo pipefail

cargo install grcov
rustup component add llvm-tools-preview
cargo clean

