#!/bin/bash
set -e

cargo build --features "strict"
cargo test
cargo doc --document-private-items

touch src/lib.rs
cargo clippy -- -D warnings

cargo fmt -- --check

echo
echo "!!! Build successful !!!"


