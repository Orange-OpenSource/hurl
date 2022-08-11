#!/bin/bash
set -e
cargo install grcov
rustup component add llvm-tools-preview
cargo clean
