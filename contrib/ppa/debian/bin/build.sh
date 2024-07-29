#!/bin/bash
set -Eeuo pipefail

echo "## build:"
cargo build --release --frozen --verbose --package hurl
