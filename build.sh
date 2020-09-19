#!/bin/bash
set -e

cargo build --features "strict"
cargo test
cargo doc --document-private-items

touch src/lib.rs
cargo clippy -- -D warnings

echo
echo "!!! Build successful !!!"


