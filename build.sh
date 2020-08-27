#!/bin/bash
set -e


ROOT_DIR="$(dirname "$0")"

#cargo clean
cargo build --features "strict"
cargo test
cargo doc --document-private-items

touch src/lib.rs
cargo clippy -- -D warnings

DOCS_DIR="$ROOT_DIR/docs"
MAN_DIR="$ROOT_DIR/target/man"
mkdir -p "$MAN_DIR"
cp "$DOCS_DIR/hurl.1" "$MAN_DIR"
cp "$DOCS_DIR/hurlfmt.1" "$MAN_DIR"
gzip "$MAN_DIR"/hurl.1
gzip "$MAN_DIR"/hurlfmt.1


echo
echo "!!! Build successful !!!"


