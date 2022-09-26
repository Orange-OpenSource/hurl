#!/bin/sh
set -e

echo "----- unit tests  -----"
PATH="$HOME"/.cargo/bin:$PATH
export PATH
cargo test --release --features strict --tests
