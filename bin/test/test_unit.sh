#!/bin/sh
set -e
echo "----- unit tests  -----"
PATH="$HOME"/.cargo/bin:$PATH
export PATH
cargo test --features strict --tests
