#!/bin/sh
set -e

PATH="$HOME"/.cargo/bin:$PATH
export PATH
cargo build --release --verbose --locked
strip target/release/hurl
strip target/release/hurlfmt

target/release/hurl --version

