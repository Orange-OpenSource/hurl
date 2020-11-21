#!/bin/bash
set -e

#cargo build --features "strict"
ci/check_version.sh

cargo build
cargo test
cargo doc --document-private-items


cargo clippy -- -D warnings

cargo fmt -- --check

echo
echo "!!! Build successful !!!"


