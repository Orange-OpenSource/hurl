#!/bin/sh
set -e

PATH="$HOME"/.cargo/bin:$PATH
export PATH
cargo build --release --verbose --locked

target/release/hurl --version

