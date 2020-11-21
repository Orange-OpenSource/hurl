#!/bin/bash
set -e

#cargo build --features "strict"
cargo --version

cargo build
cargo test
cargo doc --document-private-items


cargo clippy -- -D warnings

cargo fmt -- --check

echo
echo "!!! Build successful !!!"


