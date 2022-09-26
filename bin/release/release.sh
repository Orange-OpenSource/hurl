#!/bin/sh
set -e

PATH="$HOME"/.cargo/bin:$PATH
export PATH
cargo build --release --verbose --locked

PATH="$PWD/target/release:$PATH"
export PATH
which hurl
hurl --version
