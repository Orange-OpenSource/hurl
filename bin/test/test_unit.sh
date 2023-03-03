#!/bin/bash
set -Eeuo pipefail

echo "----- unit tests  -----"
PATH="$HOME"/.cargo/bin:$PATH
export PATH
cargo test --release
